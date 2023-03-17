use crate::{
	types::avatar::{forge::ForgeType, types::ItemType},
	*,
};
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
	#[allow(unused_variables)]
	fn determine_forge_type(
		input_leader: WrappedAvatar,
		input_sacrifices: &[WrappedAvatar],
	) -> ForgeType {
		// Extracting ItemType from the Avatar's DNA
		match input_leader.read_attribute(AvatarAttributes::ItemType) {
			// ItemType::Pet
			1 => {
				if input_sacrifices
					.iter()
					.all(|sacrifice| sacrifice.is_same_type_as(&input_leader))
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
					.all(|sacrifice| sacrifice.is_same_type_as(&input_leader))
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
