mod avatar_combinator;
mod avatar_mutator;
mod avatar_utils;
mod constants;
mod slot_roller;
mod types;

pub(self) use avatar_combinator::*;
pub(self) use avatar_mutator::*;
pub(self) use avatar_utils::*;
pub(self) use constants::*;
pub(self) use slot_roller::*;
pub(self) use types::*;

use super::*;
use crate::{
	pallet::SeasonOf,
	types::{MintOption, SeasonId},
	Config,
};
use sp_runtime::DispatchError;
use sp_std::marker::PhantomData;

pub(super) struct AvatarMinterV2<'a, T: Config>(pub PhantomData<&'a T>);

impl<'a, T> Minter<T> for AvatarMinterV2<'a, T>
where
	T: Config,
{
	fn mint_avatar_set(
		&self,
		player: &T::AccountId,
		season_id: &SeasonId,
		season: &SeasonOf<T>,
		mint_option: &MintOption,
	) -> Result<Vec<MintOutput<T>>, DispatchError> {
		Ok(self.mint_avatar_set_for(player, season_id, mint_option)?)
	}
}

impl<'a, T> AvatarMinterV2<'a, T>
where
	T: Config,
{
	fn generate_base_avatar_dna(&self, player: &T::AccountId) -> Result<Dna, DispatchError> {
		let base_hash = Pallet::<T>::random_hash(b"mint_avatar_v2", player);

		Dna::try_from(base_hash.as_ref()[0..32].to_vec())
			.map_err(|_| Error::<T>::IncorrectDna.into())
	}

	fn get_mutator_from_item_type(
		&self,
		pack_type: PackType,
		item_type: ItemType,
	) -> Box<dyn AvatarMutator<T>> {
		match item_type {
			ItemType::Pet => Box::new(SlotRoller::<T>::roll_on_pack_type(
				pack_type,
				&PACK_TYPE_MATERIAL_PET_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_EQUIPMENT_PET_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_SPECIAL_PET_ITEM_TYPE_PROBABILITIES,
			)),
			ItemType::Material => Box::new(SlotRoller::<T>::roll_on_pack_type(
				pack_type,
				&PACK_TYPE_MATERIAL_MATERIAL_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_EQUIPMENT_MATERIAL_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_SPECIAL_MATERIAL_ITEM_TYPE_PROBABILITIES,
			)),
			ItemType::Essence => Box::new(SlotRoller::<T>::roll_on_pack_type(
				pack_type,
				&PACK_TYPE_MATERIAL_ESSENCE_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_EQUIPMENT_ESSENCE_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_SPECIAL_ESSENCE_ITEM_TYPE_PROBABILITIES,
			)),
			ItemType::Equipable => Box::new(SlotRoller::<T>::roll_on_pack_type(
				pack_type,
				&PACK_TYPE_MATERIAL_EQUIPABLE_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_EQUIPMENT_EQUIPABLE_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_SPECIAL_EQUIPABLE_ITEM_TYPE_PROBABILITIES,
			)),
			ItemType::Blueprint => Box::new(SlotRoller::<T>::roll_on_pack_type(
				pack_type,
				&PACK_TYPE_MATERIAL_BLUEPRINT_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_EQUIPMENT_BLUEPRINT_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_SPECIAL_BLUEPRINT_ITEM_TYPE_PROBABILITIES,
			)),
			ItemType::Special => Box::new(SlotRoller::<T>::roll_on_pack_type(
				pack_type,
				&PACK_TYPE_MATERIAL_SPECIAL_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_EQUIPMENT_SPECIAL_ITEM_TYPE_PROBABILITIES,
				&PACK_TYPE_SPECIAL_SPECIAL_ITEM_TYPE_PROBABILITIES,
			)),
		}
	}

	fn mint_avatar_set_for(
		&self,
		player: &T::AccountId,
		season_id: &SeasonId,
		mint_option: &MintOption,
	) -> Result<Vec<MintOutput<T>>, DispatchError> {
		let roll_amount = mint_option.count as usize;

		let rolled_item_type = SlotRoller::<T>::roll_on_pack_type(
			mint_option.mint_pack,
			&PACK_TYPE_MATERIAL_ITEM_PROBABILITIES,
			&PACK_TYPE_EQUIPMENT_ITEM_PROBABILITIES,
			&PACK_TYPE_SPECIAL_ITEM_PROBABILITIES,
		);

		let mut minted_avatars = Vec::with_capacity(roll_amount);

		for _ in 0..roll_amount {
			let avatar_id = Pallet::<T>::random_hash(b"create_avatar", player);

			let base_dna = self.generate_base_avatar_dna(player)?;
			let base_avatar = Avatar {
				season_id: *season_id,
				version: mint_option.mint_version,
				dna: base_dna,
				souls: SoulCount::zero(),
			};

			let avatar = self
				.get_mutator_from_item_type(mint_option.mint_pack, rolled_item_type)
				.mutate_from_base(base_avatar);

			minted_avatars.push((avatar_id, avatar));
		}

		Ok(minted_avatars)
	}
}

pub(crate) struct AvatarForgerV2<'a, T: Config>(pub PhantomData<&'a T>);

impl<'a, T> Forger<T> for AvatarForgerV2<'a, T>
where
	T: Config,
{
	fn forge_with(
		&self,
		player: &T::AccountId,
		season_id: SeasonId,
		season: &SeasonOf<T>,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		let forge_type = self.can_be_forged(season, &input_leader, &input_sacrifices)?;

		AvatarCombinator::<T>::combine_avatars_in(
			forge_type,
			player,
			season_id,
			season,
			input_leader,
			input_sacrifices,
		)
	}

	fn can_be_forged(
		&self,
		season: &SeasonOf<T>,
		input_leader: &ForgeItem<T>,
		input_sacrifices: &[ForgeItem<T>],
	) -> Result<ForgeType, DispatchError> {
		if input_sacrifices
			.iter()
			.all(|(_, avatar)| avatar.version == input_leader.1.version)
		{
			let leader = &input_leader.1;
			let sacrifices: Vec<&Avatar> =
				input_sacrifices.iter().map(|sacrifice| &sacrifice.1).collect();

			match self.determine_forge_type(leader, sacrifices.as_slice()) {
				ForgeType::None => Err(Error::<T>::InvalidForgeComponents.into()),
				other => Ok(other),
			}
		} else {
			Err(Error::<T>::IncompatibleAvatarVersions.into())
		}
	}

	fn min_tier(&self, target: &Avatar) -> u8 {
		todo!()
	}

	fn last_variation(&self, target: &Avatar) -> u8 {
		todo!()
	}
}

impl<'a, T> AvatarForgerV2<'a, T>
where
	T: Config,
{
	fn determine_forge_type(
		&self,
		input_leader: &Avatar,
		input_sacrifices: &[&Avatar],
	) -> ForgeType {
		// Extracting ItemType from the Avatar's DNA
		match AvatarUtils::read_attribute(input_leader, AvatarAttributes::ItemType) {
			// ItemType::Pet
			1 => {
				if input_sacrifices
					.iter()
					.all(|sacrifice| AvatarUtils::is_same_type_as(input_leader, sacrifice))
				{
					ForgeType::Stack
				} else {
					ForgeType::None
				}
			},
			// ItemType::Material
			2 => {
				if input_sacrifices
					.iter()
					.all(|sacrifice| AvatarUtils::is_same_type_as(input_leader, sacrifice))
				{
					ForgeType::Stack
				} else {
					ForgeType::None
				}
			},
			// ItemType::Essence
			3 => ForgeType::None,
			// ItemType::Equipable
			4 => ForgeType::None,
			// ItemType::Blueprint
			5 => ForgeType::None,
			// ItemType::Special
			6 => ForgeType::None,
			// Other non-match
			_ => ForgeType::None,
		}
	}
}
