use ajuna_solo_runtime::AccountId;
use frame_support::assert_ok;

mod ajuna_node;
mod traits;
mod sidechain;

// Some useful accounts
pub const SIDECHAIN_SIGNING_KEY: [u8; 32] = [0x1; 32];
pub const SUDO: [u8; 32] = [0x2; 32];
pub const PLAYER_1: [u8; 32] = [0x3; 32];
pub const PLAYER_2: [u8; 32] = [0x4; 32];

use crate::{
	ajuna_node::AjunaNode,
	traits::{BlockProcessing, RuntimeBuilding},
	sidechain::{AjunaBoard, Guess, SideChain, SigningKey},
};
use ajuna_solo_runtime::{GameRegistry, Origin};

struct Player {
	account_id: AccountId,
}

impl Player {
	pub fn queue(&self) {
		assert_ok!(GameRegistry::queue(Origin::signed(self.account_id.clone())));
	}
	pub fn play_turn(&self, guess: Guess) {
		assert_ok!(AjunaBoard::play_turn(
			sidechain::Origin::signed(self.account_id.clone()),
			guess
		));
	}
}

struct Network {}

struct SideChainSigningKey;

impl SigningKey for SideChainSigningKey {
	fn account() -> AccountId {
		SIDECHAIN_SIGNING_KEY.into()
	}
}

impl Network {
	pub fn process(number_of_blocks: u8) {
		for _ in 0..number_of_blocks {
			// Produce a block at the node
			AjunaNode::move_forward();
			// Produce a sidechain block
			SideChain::<SideChainSigningKey>::move_forward();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{PLAYER_1, PLAYER_2, SIDECHAIN_SIGNING_KEY, SUDO};

	use crate::{
		sidechain::{SideChain, THE_NUMBER}, AjunaNode, Network, Player, RuntimeBuilding, SideChainSigningKey,
	};
	use ajuna_solo_runtime::{GameRegistry, Observers};
	use frame_support::assert_ok;
	use ajuna_common::RunnerState;
	use ajuna_solo_runtime::{pallet_ajuna_gameregistry::Game, AccountId, GameRegistry, Runner};
	use codec::Decode;

	fn last_event() -> ajuna_solo_runtime::Event {
		frame_system::Pallet::<ajuna_solo_runtime::Runtime>::events()
			.pop()
			.expect("Event expected")
			.event
	}

	#[test]
	fn play_a_guessing_game() {
		SideChain::<SideChainSigningKey>::build().execute_with(|| {
			AjunaNode::default()
				.account(SUDO.into())
				.players(vec![PLAYER_1.into(), PLAYER_2.into()])
				.sidechain(SIDECHAIN_SIGNING_KEY.into())
				.build()
				.execute_with(|| {
					// Queue players
					let player_1 = Player { account_id: PLAYER_1.into() };
					let player_2 = Player { account_id: PLAYER_2.into() };
					player_1.queue();
					assert!(GameRegistry::queued().is_none());
					player_2.queue();
					assert!(GameRegistry::queued().is_some());
					// We want to move forward by one block so the sidechain imports
					Network::process(1);
					// Game would be acknowledged by sidechain
					assert!(GameRegistry::queued().is_none());

					// This is the first game in this test, hence 1
					let game_id = 1;
					assert_eq!(
						last_event(),
						ajuna_solo_runtime::Event::Runner(
							pallet_ajuna_runner::Event::StateAccepted { runner_id: game_id }
						),
						"We should have our game(runner) accepted with the game id"
					);
					// Game should be created now and we can play
					player_1.play_turn(100);
					player_2.play_turn(101);
					// Win the game
					player_1.play_turn(THE_NUMBER);
					// We want to move forward by one block so the sidechain imports
					Network::process(1);

					// TODO - bug here where the event stack is cleared on the ajuna_solo_runtime
					// when we win a game in ajuna-board

					// We should expect the runner for this game
					// has now finished with our final assert_eq!(
					// 	last_event(),
					// 	ajuna_solo_runtime::Event::Runner(
					// 		pallet_ajuna_runner::Event::StateFinished { runner_id: game_id }
					// 	),
					// 	"We should have our game(runner) finished with the game id"
					// );

					// The runner state is now finished with the completed state from the board
					match Runner::runners(game_id).expect("the game should be a runner") {
						RunnerState::Finished(mut final_state) => {
							let game = Game::<AccountId>::decode(&mut final_state)
								.expect("game state as finished");

							assert_eq!(
								&game.winner.expect("winner for game"),
								player_1.account_id(),
								"winner should be player 1"
							);
						},
						_ => panic!("State should be Finished for runner"),
					}
				});
		});
	}
}
