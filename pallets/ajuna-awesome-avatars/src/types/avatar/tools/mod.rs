mod v1;
mod v2;

use crate::*;
use frame_support::pallet_prelude::*;
use sp_std::{boxed::Box, vec::Vec};

/// Trait used to implement generic minting logic for an entity.
pub(crate) trait MintProvider<T: Config> {
	fn get_minter(&self) -> Box<dyn Minter<T>>;
	fn with_minter<F, R>(&self, func: F) -> R
	where
		F: Fn(Box<dyn Minter<T>>) -> R;
}

impl<T> MintProvider<T> for AvatarVersion
where
	T: Config,
{
	fn get_minter(&self) -> Box<dyn Minter<T>> {
		match self {
			AvatarVersion::V1 => Box::new(v1::AvatarMinterV1::<T>(PhantomData)),
			AvatarVersion::V2 => Box::new(v2::AvatarMinterV2::<T>(PhantomData)),
		}
	}

	fn with_minter<F, R>(&self, func: F) -> R
	where
		F: Fn(Box<dyn Minter<T>>) -> R,
	{
		func(self.get_minter())
	}
}

/// A tuple containing and avatar identifier with its represented avatar, returned as mint output.
pub(crate) type MintOutput<T> = (AvatarIdOf<T>, Avatar);

pub(crate) trait Minter<T: Config> {
	fn mint_avatar_set(
		&self,
		player: &T::AccountId,
		season_id: &SeasonId,
		season: &SeasonOf<T>,
		mint_option: &MintOption,
	) -> Result<Vec<MintOutput<T>>, DispatchError>;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ForgeType {
	None = 0,
	Stack = 1,
	Tinker = 2,
	Build = 3,
	Assemble = 4,
	Breed = 5,
	Equip = 6,
	Mate = 7,
	Special = 8,
}

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
		season_id: SeasonId,
		season: &SeasonOf<T>,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError>;

	/// Validates that all inputs can be used in the forging process.
	fn can_be_forged(
		&self,
		season: &SeasonOf<T>,
		input_leader: &ForgeItem<T>,
		input_sacrifices: &[ForgeItem<T>],
	) -> Result<ForgeType, DispatchError>;

	/// Used to obtain the RarityTier of a given avatar as an u8.
	fn min_tier(&self, target: &Avatar) -> u8;

	/// Used to get the ForceType of a given avatar as an u8.
	fn last_variation(&self, target: &Avatar) -> u8;
}
