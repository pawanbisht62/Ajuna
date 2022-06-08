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

use super::*;
use crate::{mock::*, Error};

use frame_support::{assert_noop, assert_ok};

use pallet_ajuna_nft::TokenData;

pub type AuctionsModule = Pallet<Test>;
pub type NftPallet = pallet_ajuna_nft::Pallet<Test>;

fn create_nft() {
	assert_ok!(NftPallet::create_class(Origin::signed(100), "Class1".as_bytes().to_vec(), 0));
	assert_ok!(NftPallet::mint(
		Origin::signed(100),
		0,
		"Class1_mint1".as_bytes().to_vec(),
		TokenData { locked: false },
		1
	));
}

#[test]
fn can_create_auction() {
	new_test_ext().execute_with(|| {
		let auction_info = AuctionInfo {
			name: "Aukce1".as_bytes().to_vec(),
			last_bid: None,
			start: 1,
			end: 20,
			owner: 100,
			auction_type: AuctionType::English,
			token_id: (0, 0),
			minimal_bid: 50,
		};
		assert_noop!(
			AuctionsModule::create_auction(Origin::signed(100), auction_info.clone()),
			Error::<Test>::NotATokenOwner
		);
		create_nft();
		assert_ok!(AuctionsModule::create_auction(Origin::signed(100), auction_info));
	});
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(200, 500)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut t: sp_io::TestExternalities = t.into();

	t.execute_with(|| System::set_block_number(1));
	t
}
