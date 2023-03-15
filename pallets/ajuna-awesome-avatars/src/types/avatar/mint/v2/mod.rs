mod constants;
mod dna_mutator;
mod slot_roller;

use crate::{
	types::avatar::{
		mint::v2::{constants::*, dna_mutator::AvatarMutator, slot_roller::SlotRoller},
		types::ItemType,
	},
	*,
};
use sp_runtime::DispatchError;

pub(crate) struct AvatarMinterV2<'a, T: Config>(pub PhantomData<&'a T>);

#[allow(unused_variables)]
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
