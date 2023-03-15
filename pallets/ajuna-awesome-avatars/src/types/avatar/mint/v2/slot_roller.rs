use crate::{types::PackType, Config};
use frame_support::traits::Randomness;
use std::marker::PhantomData;

/// Represents a ‰ value, which goes from 1 to 1000
pub type SlotPerMille = u32;
pub type Slot<T> = (T, SlotPerMille);
pub type ProbabilitySlots<T, const N: usize> = [Slot<T>; N];

pub(crate) struct SlotRoller<'a, T: Config>(pub PhantomData<&'a T>);

impl<'a, T> SlotRoller<'a, T>
where
	T: Config,
{
	/// Rolls number between 1 and 1000, representing a range of 0.1% increments in probability.
	pub(crate) fn roll_number() -> u32 {
		let (random_hash, _) = T::Randomness::random(b"roll");

		// TODO: Improve random generation logic
		(u32::from_ne_bytes(random_hash.as_ref()[0..4].try_into().unwrap_or_default()) % 1000) + 1
	}

	pub(crate) fn roll_on<S, const N: usize>(slots: &ProbabilitySlots<S, N>) -> S
	where
		S: Copy + Clone + Default,
	{
		let mut item_rolled = S::default();
		let mut roll = Self::roll_number();

		for (slot_item, slot_probability) in slots {
			roll = roll.saturating_sub(*slot_probability);

			if roll == 0 {
				item_rolled = *slot_item;
				break
			}
		}

		item_rolled
	}

	/// Rolls and picks from one of the three slots used as arguments, based on the value of
	/// pack_type
	pub(crate) fn roll_on_pack_type<S, const N: usize>(
		pack_type: PackType,
		on_material: &ProbabilitySlots<S, N>,
		on_equipment: &ProbabilitySlots<S, N>,
		on_special: &ProbabilitySlots<S, N>,
	) -> S
	where
		S: Copy + Clone + Default,
	{
		let slots = match pack_type {
			PackType::Material => on_material,
			PackType::Equipment => on_equipment,
			PackType::Special => on_special,
		};

		Self::roll_on(&slots)
	}
}
