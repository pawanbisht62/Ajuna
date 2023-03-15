use crate::{types::avatar::forge::ForgeType, *};
use sp_runtime::DispatchError;
use sp_std::{marker::PhantomData, vec::Vec};

pub(crate) struct AvatarForgerV2<'a, T: Config>(pub PhantomData<&'a T>);

#[allow(unused_variables)]
impl<'a, T> Forger<T> for AvatarForgerV2<'a, T>
where
	T: Config,
{
	fn forge_with(
		&self,
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn can_be_forged(
		&self,
		input_leader: &ForgeItem<T>,
		input_sacrifices: &[ForgeItem<T>],
		season: &SeasonOf<T>,
	) -> Result<(), DispatchError> {
		todo!()
	}

	fn min_tier(&self, target: &Dna) -> u8 {
		todo!()
	}

	fn last_variation(&self, target: &Dna) -> u8 {
		todo!()
	}
}

impl<'a, T> AvatarForgerV2<'a, T>
where
	T: Config,
{
	#[allow(unused_variables)]
	fn determine_forge_type(
		input_leader: &ForgeItem<T>,
		input_sacrifices: &[ForgeItem<T>],
	) -> ForgeType {
		// Inspect leader type, based on that forge type should be defined
		// If mismatch logic to discard should be in each specific sub-forge method
		todo!()
	}
}
