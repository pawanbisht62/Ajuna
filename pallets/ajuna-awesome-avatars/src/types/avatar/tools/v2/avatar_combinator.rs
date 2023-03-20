use super::*;
use sp_runtime::{DispatchError, SaturatedConversion};
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
			ForgeType::Equipment =>
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

		let new_quantity = input_sacrifices
			.iter()
			.map(|sacrifice| {
				AvatarWrapper::read_attribute(&sacrifice.1, AvatarAttributes::Quantity)
			})
			.reduce(|acc, qty| acc.saturating_add(qty))
			.unwrap_or_default();
		AvatarWrapper::write_attribute(&mut avatar, AvatarAttributes::Quantity, new_quantity);

		let stack_probability = 12_u8/* Constants.StackProbabilities * 256 => StackProbabilities = 0.05; */;

		let mut essence_avatar: Option<Avatar> = None;

		for i in 0..input_sacrifices.len() {
			if stack_probability > avatar.dna[i] {
				match essence_avatar {
					None => {
						/*let dna = self.random_dna(&avatar_id, season, is_batched)?;
						let souls =
							(dna.iter().map(|x| *x as SoulCount).sum::<SoulCount>() % 100) + 1;
						let avatar = Avatar { season_id, version: AvatarVersion::V2, dna, souls };*/
					},
					Some(ref mut entry) => {
						AvatarWrapper::write_attribute(
							entry,
							AvatarAttributes::Quantity,
							AvatarWrapper::read_attribute(entry, AvatarAttributes::Quantity)
								.saturating_add(1),
						);
					},
				}
			}
		}

		avatar.souls += input_sacrifices
			.into_iter()
			.map(|sacrifice| sacrifice.1.souls)
			.reduce(|acc, qty| acc.saturating_add(qty))
			.unwrap_or_default();

		Ok((LeaderForgeOutput::Forged((avatar_id, avatar), 0), vec![]))
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
