use crate::*;
use frame_support::pallet_prelude::*;
use sp_std::{boxed::Box, vec::Vec};

mod v1;
mod v2;

/// Trait used to implement generic forging logic for an entity.
pub(crate) trait ForgeProvider<T: Config> {
	fn get_forger(&self) -> Box<dyn Forger<T>>;
	fn with_forger<F, R>(&self, func: F) -> R
	where
		F: Fn(Box<dyn Forger<T>>) -> R;
}

impl<T> ForgeProvider<T> for AvatarVersion
where
	T: Config,
{
	fn get_forger(&self) -> Box<dyn Forger<T>> {
		match self {
			AvatarVersion::V1 => Box::new(v1::AvatarForgerV1::<T>(PhantomData)),
			AvatarVersion::V2 => Box::new(v2::AvatarForgerV2::<T>(PhantomData)),
		}
	}

	fn with_forger<F, R>(&self, func: F) -> R
	where
		F: Fn(Box<dyn Forger<T>>) -> R,
	{
		func(self.get_forger())
	}
}

/// A tuple containing and avatar identifier with its represented avatar, used as forging inputs.
pub(crate) type ForgeItem<T> = (AvatarIdOf<T>, Avatar);
/// Number of components upgraded after a forge in a given Avatar.
pub(crate) type UpgradedComponents = u8;
/// Enum used to express the possible results of the forge on the leader avatar.
#[allow(dead_code)]
pub(crate) enum LeaderForgeOutput<T: Config> {
	/// The leader avatar was forged (mutated) in some way.
	Forged(ForgeItem<T>, UpgradedComponents),
	/// The leader avatar was consumed in the forging process.
	Consumed(AvatarIdOf<T>),
}
/// Enum used to express the possible results of the forge on the other avatars, also called
/// sacrifices.
#[allow(dead_code)]
pub(crate) enum ForgeOutput<T: Config> {
	/// The avatar was forged (mutate) in some way.
	Forged(ForgeItem<T>, UpgradedComponents),
	/// A new avatar was created from the forging process.
	Minted(Avatar),
	/// The avatar was consumed in the forging process.
	Consumed(AvatarIdOf<T>),
}

/// Trait used to define the surface logic of the forging algorithm.
pub(crate) trait Forger<T: Config> {
	/// Tries to use the supplied inputs and forge them.
	fn forge_with(
		&self,
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError>;

	/// Validates that all inputs can be used in the forging process.
	fn can_be_forged(
		&self,
		input_leader: &ForgeItem<T>,
		input_sacrifices: &[ForgeItem<T>],
		season: &SeasonOf<T>,
	) -> Result<(), DispatchError>;

	/// Used to obtain the RarityTier of a given avatar as an u8.
	fn min_tier(&self, target: &Avatar) -> u8;

	/// Used to get the ForceType of a given avatar as an u8.
	fn last_variation(&self, target: &Avatar) -> u8;
}
