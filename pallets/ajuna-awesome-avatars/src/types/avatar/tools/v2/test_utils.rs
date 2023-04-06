use crate::{
	mock::{MockAccountId, Test},
	pallet::AvatarIdOf,
	types::{
		avatar::tools::v2::{
			avatar_utils::{AvatarBuilder, HashProvider},
			types::{MaterialItemType, PetItemType, PetType, SlotType},
		},
		Avatar, AvatarVersion, ForgeOutput, LeaderForgeOutput,
	},
	Config, Pallet,
};
use sp_core::{bounded::BoundedVec, H256};

pub const HASH_BYTES: [u8; 32] = [
	1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
	97, 101, 103, 107, 109, 113, 127,
];

#[inline]
pub(crate) fn create_random_avatar<T, F>(
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

pub(crate) fn create_random_material(
	account: &MockAccountId,
	material_type: MaterialItemType,
	quantity: u8,
) -> (AvatarIdOf<Test>, Avatar) {
	create_random_avatar::<Test, _>(
		account,
		Some(|avatar| {
			AvatarBuilder::with_base_avatar(avatar)
				.into_material(material_type, quantity)
				.build()
		}),
	)
}

pub(crate) fn create_random_pet_part(
	account: &MockAccountId,
	pet_type: PetType,
	slot_type: SlotType,
	quantity: u8,
) -> (AvatarIdOf<Test>, Avatar) {
	create_random_avatar::<Test, _>(
		account,
		Some(|avatar| {
			AvatarBuilder::with_base_avatar(avatar)
				.into_pet_part(pet_type, slot_type, quantity)
				.build()
		}),
	)
}

#[inline]
pub(crate) fn is_leader_forged<T>(output: &LeaderForgeOutput<T>) -> bool
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
pub(crate) fn is_leader_consumed<T>(output: &LeaderForgeOutput<T>) -> bool
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
pub(crate) fn is_forged<T>(output: &ForgeOutput<T>) -> bool
where
	T: Config,
{
	if let ForgeOutput::Forged(_, _) = output {
		true
	} else {
		false
	}
}

#[inline]
pub(crate) fn is_minted<T>(output: &ForgeOutput<T>) -> bool
where
	T: Config,
{
	if let ForgeOutput::Minted(_) = output {
		true
	} else {
		false
	}
}

#[inline]
pub(crate) fn is_consumed<T>(output: &ForgeOutput<T>) -> bool
where
	T: Config,
{
	if let ForgeOutput::Consumed(_) = output {
		true
	} else {
		false
	}
}
