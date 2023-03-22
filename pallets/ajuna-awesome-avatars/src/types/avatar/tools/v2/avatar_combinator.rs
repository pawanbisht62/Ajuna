use super::*;
use sp_runtime::DispatchError;
use std::marker::PhantomData;

pub(crate) struct AvatarCombinator<'a, T: Config>(pub PhantomData<&'a T>);

impl<'a, T> AvatarCombinator<'a, T>
where
	T: Config,
{
	pub(crate) fn combine_avatars_in(
		forge_type: ForgeType,
		player: &T::AccountId,
		season_id: SeasonId,
		season: &SeasonOf<T>,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		match forge_type {
			ForgeType::Stack =>
				Self::stack_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Tinker =>
				Self::tinker_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Build =>
				Self::build_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Assemble =>
				Self::assemble_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Breed =>
				Self::breed_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Equip =>
				Self::equipment_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Mate =>
				Self::mate_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Special =>
				Self::special_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::None => Err(Error::<T>::InvalidForgeComponents.into()),
		}
	}

	fn stack_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		let (avatar_id, mut avatar) = input_leader;

		let (new_quantity, new_souls) = input_sacrifices
			.iter()
			.map(|sacrifice| {
				(
					AvatarUtils::read_attribute(&sacrifice.1, AvatarAttributes::Quantity),
					sacrifice.1.souls,
				)
			})
			.reduce(|(acc_qty, acc_souls), (qty, souls)| {
				(acc_qty.saturating_add(qty), acc_souls.saturating_add(souls))
			})
			.unwrap_or_default();
		AvatarUtils::write_attribute(&mut avatar, AvatarAttributes::Quantity, new_quantity);

		let mut essence_avatar: Option<Avatar> = None;

		for i in 0..input_sacrifices.len() {
			if STACK_PROBABILITY > avatar.dna[i] {
				essence_avatar = match essence_avatar {
					None => {
						let dna =
							AvatarMinterV2::<T>(PhantomData).generate_base_avatar_dna(player)?;
						Some(
							AvatarBuilder::with_dna(season_id, dna)
								.into_essence(EssenceItemType::Glimmer, 1)
								.build(),
						)
					},
					Some(entry) =>
						Some(AvatarBuilder::with_base_avatar(entry).add_quantity(1).build()),
				}
			}
		}

		avatar.souls += new_souls;

		let output_vec: Vec<ForgeOutput<T>> = input_sacrifices
			.into_iter()
			.map(|(sacrifice_id, _)| ForgeOutput::Consumed(sacrifice_id))
			.chain(essence_avatar.map(|minted_avatar| ForgeOutput::Minted(minted_avatar)))
			.collect();

		Ok((LeaderForgeOutput::Forged((avatar_id, avatar), 0), output_vec))
	}

	fn tinker_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn build_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn assemble_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn breed_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn equipment_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn mate_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn special_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}
}
