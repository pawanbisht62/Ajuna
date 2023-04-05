use crate::{
	pallet::AvatarIdOf,
	types::{Avatar, AvatarVersion, LeaderForgeOutput},
	Config, Pallet,
};
use sp_core::{bounded::BoundedVec, H256};

#[inline]
pub fn create_random_avatar<T, F>(
	creator: &T::AccountId,
	avatar_build_fn: Option<F>,
) -> (AvatarIdOf<T>, Avatar)
where
	F: Fn(Avatar) -> Avatar,
	T: Config,
{
	let base_avatar = Avatar {
		season_id: 0,
		version: AvatarVersion::V2,
		dna: BoundedVec::try_from([0_u8; 32].to_vec()).expect("Should create DNA!"),
		souls: 0,
	};

	let avatar = match avatar_build_fn {
		None => base_avatar,
		Some(f) => f(base_avatar),
	};
	(Pallet::<T>::random_hash(b"mock_avatar", creator), avatar)
}

#[inline]
pub fn is_leader_forged<T>(output: &LeaderForgeOutput<T>) -> bool
where
	T: Config,
{
	if let LeaderForgeOutput::Forged(_, _) = output {
		true
	} else {
		false
	}
}

#[inline]
pub fn is_leader_consumed<T>(output: &LeaderForgeOutput<T>) -> bool
where
	T: Config,
{
	if let LeaderForgeOutput::Consumed(_) = output {
		true
	} else {
		false
	}
}

#[inline]
pub fn is_forged<T>(output: &LeaderForgeOutput<T>) -> bool
where
	T: Config,
{
	if let LeaderForgeOutput::Consumed(_) = output {
		true
	} else {
		false
	}
}
