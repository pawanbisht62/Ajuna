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
use crate as pallet_auction;

use sp_core::H256;

use frame_support::{
	parameter_types,
};

use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

mod auction {
	// Re-export needed for `impl_outer_event!`.
	pub use super::super::*;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Auctions: pallet_auction::{Pallet, Call, Storage, Event<T>},
		OrmlNft: orml_nft::{Pallet, Storage, Config<T>},
		Nft: pallet_ajuna_nft::{Pallet, Call, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1_000_000);
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

/// Balance of an account.
pub type Balance = u128;

impl pallet_ajuna_nft::Config  for Test {
	type Event = Event;
	type WeightInfo = pallet_ajuna_nft::weights::SubstrateWeight<Test>;
}

parameter_types! {
	pub const MaxClassMetadata: u32 = 100;
	pub const MaxTokenMetadata: u32 = 100;
}

impl orml_nft::Config for Test {
	type ClassId = u64;
	type TokenId = u64;
	type ClassData = u32;
	type TokenData = pallet_ajuna_nft::TokenData;
	type MaxClassMetadata = MaxClassMetadata;
	type MaxTokenMetadata = MaxTokenMetadata;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const ArbitraryUpperBound: u32 = 1_000_000;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ArbitraryUpperBound;
	type MaxReserves = ArbitraryUpperBound;
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_auction::Config for Test {
	type Event = Event;
	type Balance = Balance;
	type AuctionId = u64;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type WeightInfo = pallet_auction::weights::SubstrateWeight<Test>;
}
