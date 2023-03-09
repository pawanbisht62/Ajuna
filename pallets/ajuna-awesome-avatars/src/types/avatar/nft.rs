use crate::types::AvatarCodec;
use codec::Encode;
use pallet_ajuna_nft_transfer::traits::{AttributeCode, NftConvertible};
use sp_std::prelude::*;

pub const DNA_ATTRIBUTE_CODE: u16 = 10;
pub const SOUL_POINTS_ATTRIBUTE_CODE: u16 = 11;
pub const RARITY_ATTRIBUTE_CODE: u16 = 12;
pub const FORCE_ATTRIBUTE_CODE: u16 = 13;

impl NftConvertible for AvatarCodec {
	const ASSET_CODE: u16 = 0;

	fn get_attribute_table() -> Vec<AttributeCode> {
		vec![
			DNA_ATTRIBUTE_CODE,
			SOUL_POINTS_ATTRIBUTE_CODE,
			RARITY_ATTRIBUTE_CODE,
			FORCE_ATTRIBUTE_CODE,
		]
	}

	fn get_encoded_attributes(&self) -> Vec<(AttributeCode, Vec<u8>)> {
		vec![
			(DNA_ATTRIBUTE_CODE, self.dna.clone().encode()),
			(SOUL_POINTS_ATTRIBUTE_CODE, self.soul_points.encode()),
			(RARITY_ATTRIBUTE_CODE, self.rarity.encode()),
			(FORCE_ATTRIBUTE_CODE, self.force.encode()),
		]
	}
}
