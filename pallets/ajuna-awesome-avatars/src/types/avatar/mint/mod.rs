use crate::*;
use frame_support::pallet_prelude::*;
use sp_std::{boxed::Box, vec::Vec};

mod v1;
mod v2;

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
pub(crate) type MintOutput<T> = (AvatarIdOf<T>, Dna);

pub(crate) trait Minter<T: Config> {
	fn mint_avatar_set(
		&self,
		player: &T::AccountId,
		season_id: &SeasonId,
		season: &SeasonOf<T>,
		mint_option: &MintOption,
	) -> Result<Vec<MintOutput<T>>, DispatchError>;
}
