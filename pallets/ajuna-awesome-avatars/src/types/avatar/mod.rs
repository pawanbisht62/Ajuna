// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

mod avatar_codec;
mod force;
mod forge;
mod nft;
mod rarity_tier;

pub use avatar_codec::*;
pub use force::*;
pub use forge::*;
pub use nft::*;
pub use rarity_tier::*;

use crate::Config;
use frame_support::pallet_prelude::*;

pub type SeasonId = u16;
pub type Dna = BoundedVec<u8, ConstU32<100>>;
pub type SoulCount = u32;

#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Clone, Debug, Default, PartialEq)]
pub struct Avatar {
	pub season_id: SeasonId,
	pub version: AvatarForgeVersion,
	pub dna: Dna,
	pub souls: SoulCount,
}

impl Avatar {
	#[inline]
	pub(crate) fn min_tier<T: Config>(&self) -> u8 {
		self.version.with_forger(|forger: Box<dyn Forger<T>>| forger.min_tier(self))
	}

	#[inline]
	pub(crate) fn last_variation<T: Config>(&self) -> u8 {
		self.version
			.with_forger(|forger: Box<dyn Forger<T>>| forger.last_variation(self))
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::mock::*;
	use pallet_ajuna_nft_transfer::traits::NftConvertible;

	impl Avatar {
		pub(crate) fn season_id(mut self, season_id: SeasonId) -> Self {
			self.season_id = season_id;
			self
		}
		pub(crate) fn dna(mut self, dna: &[u8]) -> Self {
			self.dna = Dna::try_from(dna.to_vec()).unwrap();
			self
		}
		pub(crate) fn souls(mut self, souls: SoulCount) -> Self {
			self.souls = souls;
			self
		}
	}

	#[test]
	fn codec_works() {
		let avatar_codec = AvatarCodec::from::<Test>(
			Avatar::default().season_id(123).dna(&[0x31, 0x32, 0x33, 0x34]).souls(321),
		);
		let encoded = avatar_codec.clone().encode_into();

		// check encoding
		assert_eq!(
			encoded,
			AvatarCodec {
				season_id: avatar_codec.season_id,
				version: avatar_codec.version,
				dna: avatar_codec.dna.clone(),
				soul_points: avatar_codec.soul_points,
				rarity: RarityTier::Epic as u8,
				force: Force::Astral as u8,
			}
			.encode()
		);

		// check decoding
		assert_eq!(AvatarCodec::decode_from(encoded), Ok(avatar_codec));
	}
}
