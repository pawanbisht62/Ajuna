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

use mock::Event;

use frame_support::{assert_noop, assert_ok, error::BadOrigin};

type NftPallet = Pallet<Test>;

#[test]
fn create_class_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));
		let event = tests::Event::Nft(crate::Event::NFTTokenClassCreated(ALICE, CLASS_ID));
		assert_eq!(last_event(), event);
	})
}

#[test]
fn create_class_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			NftPallet::create_class(Origin::none(), "a class".as_bytes().to_vec(), Default::default()),
			BadOrigin
		);
	})
}

#[test]
fn mint_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));
		let event = tests::Event::Nft(crate::Event::NFTTokenClassCreated(ALICE, CLASS_ID));
		assert_eq!(last_event(), event);

		assert_ok!(NftPallet::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			0, // not locked
			TEST_QUANTITY,
		));
		let event = tests::Event::Nft(crate::Event::NFTTokenMinted(ALICE, CLASS_ID, TEST_QUANTITY));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn mint_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));
		let event = tests::Event::Nft(crate::Event::NFTTokenClassCreated(ALICE, CLASS_ID));
		assert_eq!(last_event(), event);

		assert_noop!(
			NftPallet::mint(
				Origin::signed(BOB),
				0,
				"a token".as_bytes().to_vec(),
				0, // not locked
				TEST_QUANTITY,
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn transfer_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));

		assert_ok!(NftPallet::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			0, // not locked
			TEST_QUANTITY,
		));

		assert_ok!(NftPallet::transfer(Origin::signed(ALICE), BOB, (CLASS_ID, TOKEN_ID)));
		let event = tests::Event::Nft(crate::Event::NFTTokenTransferred(ALICE, BOB, CLASS_ID, TOKEN_ID));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn transfer_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));

		assert_ok!(NftPallet::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			0, // not locked
			TEST_QUANTITY,
		));

		assert_noop!(
			NftPallet::transfer(Origin::signed(BOB), ALICE, (CLASS_ID, TOKEN_ID)),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn burn_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));

		assert_ok!(NftPallet::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			0, // not locked
			TEST_QUANTITY,
		));

		assert_ok!(NftPallet::burn(Origin::signed(ALICE), (CLASS_ID, TOKEN_ID)));
		let event = tests::Event::Nft(crate::Event::NFTTokenBurned(ALICE, CLASS_ID, TOKEN_ID));
		assert_eq!(last_event(), event);
	});
}

#[test]
fn burn_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));

		assert_ok!(NftPallet::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			0, // not locked
			TEST_QUANTITY,
		));

		assert_noop!(
			NftPallet::burn(Origin::signed(BOB), (CLASS_ID, TOKEN_ID)),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn destroy_class_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));

		assert_ok!(NftPallet::destroy_class(Origin::signed(ALICE), CLASS_ID));
	});
}

#[test]
fn destroy_class_fails() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(NftPallet::create_class(
			Origin::signed(ALICE),
			"a class".as_bytes().to_vec(),
			Default::default()
		));

		assert_ok!(NftPallet::mint(
			Origin::signed(ALICE),
			0,
			"a token".as_bytes().to_vec(),
			0, // not locked
			TEST_QUANTITY,
		));

		assert_noop!(
			NftPallet::destroy_class(Origin::signed(ALICE), CLASS_ID),
			Error::<Test>::CannotDestroyClass
		);
	});
}

