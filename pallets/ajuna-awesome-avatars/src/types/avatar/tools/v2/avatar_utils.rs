use super::{constants::PROGRESS_VARIATIONS, types::*, ByteType};
use crate::types::{
	avatar::tools::v2::constants::{BASE_PROGRESS_PROBABILITY, MAX_SACRIFICE},
	Avatar, AvatarVersion, Dna, SeasonId, SoulCount,
};
use frame_support::traits::Len;
use sp_runtime::SaturatedConversion;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AvatarAttributes {
	ItemType,
	ItemSubType,
	ClassType1,
	ClassType2,
	CustomType1,
	CustomType2,
	RarityType,
	Quantity,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AvatarSpecBytes {
	SpecByte1,
	SpecByte2,
	SpecByte3,
	SpecByte4,
	SpecByte5,
	SpecByte6,
	SpecByte7,
	SpecByte8,
	SpecByte9,
	SpecByte10,
	SpecByte11,
	SpecByte12,
	SpecByte13,
	SpecByte14,
	SpecByte15,
	SpecByte16,
}

pub(crate) struct AvatarBuilder {
	inner: Avatar,
}

impl AvatarBuilder {
	pub fn with_dna(season_id: SeasonId, dna: Dna) -> Self {
		Self { inner: Avatar { season_id, version: AvatarVersion::V2, dna, souls: 0 } }
	}

	pub fn with_base_avatar(avatar: Avatar) -> Self {
		Self { inner: avatar }
	}

	pub fn with_attribute<T>(mut self, attribute: AvatarAttributes, value: T) -> Self
	where
		T: IntoByte,
	{
		self.with_attribute_raw(attribute, value.into_byte())
	}

	pub fn with_attribute_raw(mut self, attribute: AvatarAttributes, value: u8) -> Self {
		AvatarUtils::write_attribute(&mut self.inner, attribute, value);
		self
	}

	pub fn with_spec_byte(mut self, spec_byte: AvatarSpecBytes, value: u8) -> Self {
		AvatarUtils::write_spec_byte(&mut self.inner, spec_byte, value);
		self
	}

	pub fn with_soul_count(mut self, soul_count: SoulCount) -> Self {
		self.inner.souls = soul_count;
		self
	}

	pub fn with_progress_array(mut self, progress_array: [u8; 11]) -> Self {
		AvatarUtils::write_progress_array(&mut self.inner, progress_array);
		self
	}

	pub fn add_quantity(mut self, quantity: u8) -> Self {
		let current_quantity = AvatarUtils::read_attribute(&self.inner, AvatarAttributes::Quantity);
		AvatarUtils::write_attribute(
			&mut self.inner,
			AvatarAttributes::Quantity,
			current_quantity.saturating_add(quantity),
		);
		self
	}

	pub fn into_pet(mut self, pet_type: PetItemType) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Pet)
			.with_attribute(AvatarAttributes::ItemSubType, pet_type)
	}

	pub fn into_material(mut self, material_type: MaterialItemType) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Material)
			.with_attribute(AvatarAttributes::ItemSubType, material_type)
	}

	pub fn into_essence(mut self, essence_type: EssenceItemType, quantity: u8) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Essence)
			.with_attribute(AvatarAttributes::ItemSubType, essence_type)
			.with_attribute(AvatarAttributes::ClassType1, HexType::X0)
			.with_attribute(AvatarAttributes::ClassType2, HexType::X0)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X1)
			.with_attribute(AvatarAttributes::CustomType2, HexType::X0)
			.with_attribute(AvatarAttributes::RarityType, RarityType::Uncommon)
			.with_attribute_raw(AvatarAttributes::Quantity, quantity)
			.with_soul_count(quantity as u32 * HexType::X1 as u32)
	}

	pub fn into_equipable(
		mut self,
		equipable_type: EquipableItemType,
		pet_type: PetType,
		slot_type: SlotType,
		rarity_type: RarityType,
		soul_points: SoulCount,
	) -> Self {
		let armor_assemble_progress = if EquipableItemType::is_armor(equipable_type) {
			AvatarUtils::enums_to_bits(&vec![equipable_type])
		} else {
			0
		};

		self.with_attribute(AvatarAttributes::ItemType, ItemType::Equipable)
			.with_attribute(AvatarAttributes::ItemSubType, equipable_type)
			.with_attribute(AvatarAttributes::ClassType1, slot_type)
			.with_attribute(AvatarAttributes::ClassType2, pet_type)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X0)
			.with_attribute(AvatarAttributes::RarityType, rarity_type)
			.with_attribute_raw(AvatarAttributes::Quantity, 1)
			// Unused
			.with_attribute(AvatarAttributes::CustomType2, HexType::X0)
			.with_spec_byte(AvatarSpecBytes::SpecByte1, armor_assemble_progress)
			.with_spec_byte(AvatarSpecBytes::SpecByte2, 0)
			.with_spec_byte(AvatarSpecBytes::SpecByte3, 0)
			.with_spec_byte(AvatarSpecBytes::SpecByte4, 0)
			.with_spec_byte(AvatarSpecBytes::SpecByte5, 0)
			.with_spec_byte(AvatarSpecBytes::SpecByte6, 0)
			.with_spec_byte(AvatarSpecBytes::SpecByte7, 0)
			.with_spec_byte(AvatarSpecBytes::SpecByte8, 0)
			.with_soul_count(soul_points)
	}

	pub fn into_blueprint(
		mut self,
		blueprint_type: BlueprintItemType,
		pet_type: PetType,
		slot_type: SlotType,
		equipable_item_type: EquipableItemType,
		pattern: Vec<MaterialItemType>,
		soul_points: SoulCount,
	) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Blueprint)
			.with_attribute(AvatarAttributes::ItemSubType, blueprint_type)
			.with_attribute(AvatarAttributes::ClassType1, slot_type)
			.with_attribute(AvatarAttributes::ClassType2, pet_type)
			.with_attribute(AvatarAttributes::CustomType1, HexType::X1)
			.with_attribute(AvatarAttributes::RarityType, RarityType::Rare)
			.with_attribute_raw(AvatarAttributes::Quantity, soul_points as u8)
			// Unused
			.with_attribute(AvatarAttributes::CustomType2, HexType::X0)
			.with_spec_byte(AvatarSpecBytes::SpecByte1, AvatarUtils::enums_to_bits(&pattern))
			.with_spec_byte(AvatarSpecBytes::SpecByte2, AvatarUtils::enums_order_to_bits(&pattern))
			.with_spec_byte(AvatarSpecBytes::SpecByte3, equipable_item_type.into_byte())
			// TODO SpecByte
			.with_spec_byte(AvatarSpecBytes::SpecByte4, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte5, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte6, 1)
			.with_spec_byte(AvatarSpecBytes::SpecByte7, 1)
			.with_soul_count(soul_points)
	}

	pub fn into_special(mut self, special_type: SpecialItemType) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Special)
			.with_attribute(AvatarAttributes::ItemSubType, special_type)
	}

	pub fn build(self) -> Avatar {
		self.inner
	}
}

/// Struct to wrap DNA interactions with Avatars from V2 upwards.
/// Don't use with Avatars with V1.
pub(crate) struct AvatarUtils;

impl AvatarUtils {
	pub fn has_attribute_with_same_value_as(
		avatar: &Avatar,
		other: &Avatar,
		attribute: AvatarAttributes,
	) -> bool {
		Self::read_attribute(avatar, attribute) == Self::read_attribute(other, attribute)
	}

	pub fn has_attribute_set_with_same_values_as(
		avatar: &Avatar,
		other: &Avatar,
		attribute_set: &[AvatarAttributes],
	) -> bool {
		attribute_set
			.iter()
			.all(|attribute| Self::has_attribute_with_same_value_as(avatar, other, *attribute))
	}

	fn read_dna_strand(avatar: &Avatar, position: usize, byte_type: ByteType) -> u8 {
		Self::read_dna_at(avatar.dna.as_slice(), position, byte_type)
	}

	#[inline]
	fn read_dna_at(dna: &[u8], position: usize, byte_type: ByteType) -> u8 {
		match byte_type {
			ByteType::Full => dna[position],
			ByteType::High => dna[position] >> 4,
			ByteType::Low => dna[position] & ByteType::High as u8,
		}
	}

	fn write_dna_strand(avatar: &mut Avatar, position: usize, byte_type: ByteType, value: u8) {
		match byte_type {
			ByteType::Full => avatar.dna[position] = value,
			ByteType::High =>
				avatar.dna[position] =
					(avatar.dna[position] & (ByteType::High as u8)) | (value << 4),
			ByteType::Low =>
				avatar.dna[position] = (avatar.dna[position] & (ByteType::Low as u8)) |
					(value & (ByteType::High as u8)),
		}
	}

	#[inline]
	fn write_dna_at(dna: &mut [u8], position: usize, byte_type: ByteType, value: u8) {
		match byte_type {
			ByteType::Full => dna[position] = value,
			ByteType::High =>
				dna[position] = (dna[position] & (ByteType::High as u8)) | (value << 4),
			ByteType::Low =>
				dna[position] =
					(dna[position] & (ByteType::Low as u8)) | (value & (ByteType::High as u8)),
		}
	}

	pub fn has_attribute_with_value<T>(
		avatar: &Avatar,
		attribute: AvatarAttributes,
		value: T,
	) -> bool
	where
		T: IntoByte,
	{
		Self::read_attribute(avatar, attribute) == value.into_byte()
	}

	pub fn has_attribute_with_value_lower_than<T>(
		avatar: &Avatar,
		attribute: AvatarAttributes,
		lower_than: T,
	) -> bool
	where
		T: IntoByte,
	{
		Self::read_attribute(avatar, attribute) < lower_than.into_byte()
	}

	pub fn read_attribute_as<T>(avatar: &Avatar, attribute: AvatarAttributes) -> T
	where
		T: FromByte,
	{
		T::from_byte(Self::read_attribute(avatar, attribute))
	}

	pub fn read_attribute(avatar: &Avatar, attribute: AvatarAttributes) -> u8 {
		match attribute {
			AvatarAttributes::ItemType => Self::read_dna_strand(avatar, 0, ByteType::High),
			AvatarAttributes::ItemSubType => Self::read_dna_strand(avatar, 0, ByteType::Low),
			AvatarAttributes::ClassType1 => Self::read_dna_strand(avatar, 1, ByteType::High),
			AvatarAttributes::ClassType2 => Self::read_dna_strand(avatar, 1, ByteType::High),
			AvatarAttributes::CustomType1 => Self::read_dna_strand(avatar, 2, ByteType::High),
			AvatarAttributes::CustomType2 => Self::read_dna_strand(avatar, 4, ByteType::Full),
			AvatarAttributes::RarityType => Self::read_dna_strand(avatar, 2, ByteType::Low),
			AvatarAttributes::Quantity => Self::read_dna_strand(avatar, 3, ByteType::Full),
		}
	}

	pub fn write_attribute(avatar: &mut Avatar, attribute: AvatarAttributes, value: u8) {
		match attribute {
			AvatarAttributes::ItemType => Self::write_dna_strand(avatar, 0, ByteType::High, value),
			AvatarAttributes::ItemSubType =>
				Self::write_dna_strand(avatar, 0, ByteType::Low, value),
			AvatarAttributes::ClassType1 =>
				Self::write_dna_strand(avatar, 1, ByteType::High, value),
			AvatarAttributes::ClassType2 =>
				Self::write_dna_strand(avatar, 1, ByteType::High, value),
			AvatarAttributes::CustomType1 =>
				Self::write_dna_strand(avatar, 2, ByteType::High, value),
			AvatarAttributes::CustomType2 =>
				Self::write_dna_strand(avatar, 4, ByteType::Full, value),
			AvatarAttributes::RarityType => Self::write_dna_strand(avatar, 2, ByteType::Low, value),
			AvatarAttributes::Quantity => Self::write_dna_strand(avatar, 3, ByteType::Full, value),
		}
	}

	pub fn read_full_spec_bytes(avatar: &Avatar) -> [u8; 16] {
		let mut out = [0; 16];
		out.copy_from_slice(&avatar.dna[5..21]);
		out
	}

	pub fn read_spec_byte(avatar: &Avatar, spec_byte: AvatarSpecBytes) -> u8 {
		match spec_byte {
			AvatarSpecBytes::SpecByte1 => Self::read_dna_strand(avatar, 5, ByteType::Full),
			AvatarSpecBytes::SpecByte2 => Self::read_dna_strand(avatar, 6, ByteType::Full),
			AvatarSpecBytes::SpecByte3 => Self::read_dna_strand(avatar, 7, ByteType::Full),
			AvatarSpecBytes::SpecByte4 => Self::read_dna_strand(avatar, 8, ByteType::Full),
			AvatarSpecBytes::SpecByte5 => Self::read_dna_strand(avatar, 9, ByteType::Full),
			AvatarSpecBytes::SpecByte6 => Self::read_dna_strand(avatar, 10, ByteType::Full),
			AvatarSpecBytes::SpecByte7 => Self::read_dna_strand(avatar, 11, ByteType::Full),
			AvatarSpecBytes::SpecByte8 => Self::read_dna_strand(avatar, 12, ByteType::Full),
			AvatarSpecBytes::SpecByte9 => Self::read_dna_strand(avatar, 13, ByteType::Full),
			AvatarSpecBytes::SpecByte10 => Self::read_dna_strand(avatar, 14, ByteType::Full),
			AvatarSpecBytes::SpecByte11 => Self::read_dna_strand(avatar, 15, ByteType::Full),
			AvatarSpecBytes::SpecByte12 => Self::read_dna_strand(avatar, 16, ByteType::Full),
			AvatarSpecBytes::SpecByte13 => Self::read_dna_strand(avatar, 17, ByteType::Full),
			AvatarSpecBytes::SpecByte14 => Self::read_dna_strand(avatar, 18, ByteType::Full),
			AvatarSpecBytes::SpecByte15 => Self::read_dna_strand(avatar, 19, ByteType::Full),
			AvatarSpecBytes::SpecByte16 => Self::read_dna_strand(avatar, 20, ByteType::Full),
		}
	}

	pub fn read_spec_byte_as<T>(avatar: &Avatar, spec_byte: AvatarSpecBytes) -> T
	where
		T: FromByte,
	{
		T::from_byte(Self::read_spec_byte(avatar, spec_byte))
	}

	pub fn write_full_spec_bytes(avatar: &mut Avatar, value: [u8; 16]) {
		(&mut avatar.dna[5..16]).copy_from_slice(&value);
	}

	pub fn write_spec_byte(avatar: &mut Avatar, spec_byte: AvatarSpecBytes, value: u8) {
		match spec_byte {
			AvatarSpecBytes::SpecByte1 => Self::write_dna_strand(avatar, 5, ByteType::Full, value),
			AvatarSpecBytes::SpecByte2 => Self::write_dna_strand(avatar, 6, ByteType::Full, value),
			AvatarSpecBytes::SpecByte3 => Self::write_dna_strand(avatar, 7, ByteType::Full, value),
			AvatarSpecBytes::SpecByte4 => Self::write_dna_strand(avatar, 8, ByteType::Full, value),
			AvatarSpecBytes::SpecByte5 => Self::write_dna_strand(avatar, 9, ByteType::Full, value),
			AvatarSpecBytes::SpecByte6 => Self::write_dna_strand(avatar, 10, ByteType::Full, value),
			AvatarSpecBytes::SpecByte7 => Self::write_dna_strand(avatar, 11, ByteType::Full, value),
			AvatarSpecBytes::SpecByte8 => Self::write_dna_strand(avatar, 12, ByteType::Full, value),
			AvatarSpecBytes::SpecByte9 => Self::write_dna_strand(avatar, 13, ByteType::Full, value),
			AvatarSpecBytes::SpecByte10 =>
				Self::write_dna_strand(avatar, 14, ByteType::Full, value),
			AvatarSpecBytes::SpecByte11 =>
				Self::write_dna_strand(avatar, 15, ByteType::Full, value),
			AvatarSpecBytes::SpecByte12 =>
				Self::write_dna_strand(avatar, 16, ByteType::Full, value),
			AvatarSpecBytes::SpecByte13 =>
				Self::write_dna_strand(avatar, 17, ByteType::Full, value),
			AvatarSpecBytes::SpecByte14 =>
				Self::write_dna_strand(avatar, 18, ByteType::Full, value),
			AvatarSpecBytes::SpecByte15 =>
				Self::write_dna_strand(avatar, 19, ByteType::Full, value),
			AvatarSpecBytes::SpecByte16 =>
				Self::write_dna_strand(avatar, 20, ByteType::Full, value),
		}
	}

	pub fn read_progress_array(avatar: &Avatar) -> [u8; 11] {
		let mut out = [0; 11];
		out.copy_from_slice(&avatar.dna[21..32]);
		out
	}

	pub fn match_progress_arrays(array_1: [u8; 11], array_2: [u8; 11]) -> Option<Vec<u8>> {
		let mut matches = Vec::new();

		let mut mirror = usize::MIN;

		let lowest_1 = Self::read_dna_at(&array_1, 0, ByteType::High);

		for index in array_1 {
			let rarity_1 = Self::read_dna_at(&array_1, 1, ByteType::High);
			let variation_1 = Self::read_dna_at(&array_1, 1, ByteType::Low);

			let rarity_2 = Self::read_dna_at(&array_2, 1, ByteType::High);
			let variation_2 = Self::read_dna_at(&array_2, 1, ByteType::Low);

			let have_same_rarity = rarity_1 == rarity_2;
			let is_maxed = rarity_1 > lowest_1 || lowest_1 == RarityType::Legendary.into_byte();

			if have_same_rarity && !is_maxed && Self::match_progress_byte(variation_1, variation_2)
			{
				matches.push(index);
			}

			if is_maxed && (variation_1 == variation_2) {
				mirror = mirror.saturating_add(1);
			}
		}

		let total_matches = matches.len();
		let handicap = match total_matches {
			1 => 4,
			2 => 2,
			_ => 0,
		};
		(total_matches > 0 && (total_matches + mirror) >= handicap + 3).then_some(matches)
	}

	pub fn match_progress_byte(byte_1: u8, byte_2: u8) -> bool {
		let diff = byte_1.saturating_sub(byte_2);
		diff == 1 || diff == (PROGRESS_VARIATIONS - 1)
	}

	pub fn write_progress_array(avatar: &mut Avatar, value: [u8; 11]) {
		(&mut avatar.dna[21..11]).copy_from_slice(&value);
	}

	pub fn can_use_avatar(avatar: &Avatar, quantity: u8) -> bool {
		Self::read_attribute(avatar, AvatarAttributes::Quantity) >= quantity
	}

	pub fn use_avatar(avatar: &mut Avatar, quantity: u8) -> (bool, SoulCount) {
		let current_qty = Self::read_attribute(avatar, AvatarAttributes::Quantity);

		if current_qty < quantity {
			return (false, 0)
		}

		let new_qty = current_qty - quantity;
		Self::write_attribute(avatar, AvatarAttributes::Quantity, new_qty);

		let ouput_soul_points = if new_qty == 0 {
			let soul_points = avatar.souls;
			avatar.souls = 0;
			soul_points
		} else {
			let diff = Self::read_attribute(avatar, AvatarAttributes::CustomType1)
				.saturating_mul(quantity) as SoulCount;
			avatar.souls = avatar.souls.saturating_sub(diff);
			diff
		};

		(true, ouput_soul_points)
	}

	pub fn enums_to_bits<T>(enum_list: &Vec<T>) -> u8
	where
		T: Copy + IntoByte,
	{
		enum_list
			.iter()
			.fold(0_u8, |acc, entry| acc | (1 << (entry.clone().into_byte().saturating_sub(1))))
	}

	pub fn enums_order_to_bits<T>(enum_list: &Vec<T>) -> u8
	where
		T: Ord + IntoByte + Clone,
	{
		let mut sorted_list = (*enum_list).clone();
		sorted_list.sort();

		let mut byte_buff = 0;
		let mut buff_fill_size = 0;

		for entry in enum_list {
			if let Ok(index) = sorted_list.binary_search(entry) {
				byte_buff |= (index as u32);
				let fill_amount = (usize::BITS - index.leading_zeros());
				byte_buff <<= fill_amount;
				buff_fill_size += fill_amount;
			}
		}

		(byte_buff >> (buff_fill_size.saturating_sub(8))) as u8
	}

	pub fn bits_to_enums<T>(bits: u8) -> Vec<T>
	where
		T: FromByte,
	{
		(0..u8::BITS as u8)
			.into_iter()
			.filter(|n| (bits & (1 << n)) != 0)
			.map(|n| T::from_byte(n))
			.collect()
	}

	pub fn bits_order_to_enum<T>(bit_order: u8, mut enum_list: Vec<T>) -> Vec<T>
	where
		T: Copy + Ord + FromByte,
	{
		enum_list.sort();
		let mut output_enums = Vec::new();
		let bit_index_array: [u8; 8] = [
			0b10000000, 0b01000000, 0b00100000, 0b00010000, 0b00001000, 0b00000100, 0b00000010,
			0b00000001,
		];

		let bit_size = u8::BITS as usize;

		for i in (0..bit_size).step_by(2) {
			// We extract the i and i+1 bits from the 'bit_order' parameter, then slide them
			// to the right so that we can get a position from those bits -> 00 to 11 -> 0 to 3
			let bit_position = ((bit_order & (bit_index_array[i] | bit_index_array[i + 1])) >>
				(bit_size - i)) as usize;
			// TODO: This will always be true
			// Probably bug
			if bit_size > bit_position {
				output_enums.push(enum_list[bit_position]);
			}
		}

		output_enums
	}

	pub fn write_progress_bytes(
		rarity_type: RarityType,
		probability: f32,
		mut progress_bytes: [u8; 11],
	) -> [u8; 11] {
		for i in 0..progress_bytes.len() {
			let random_value = Self::read_dna_at(&progress_bytes, i, ByteType::Full);
			let mut new_rarity = rarity_type.clone().into_byte();

			if random_value < (u8::MAX as f32 * probability) as u8 {
				new_rarity = new_rarity.saturating_add(1);
			}

			Self::write_dna_at(&mut progress_bytes, i, ByteType::High, new_rarity);
			Self::write_dna_at(
				&mut progress_bytes,
				i,
				ByteType::Low,
				random_value % PROGRESS_VARIATIONS,
			);
		}

		Self::write_dna_at(&mut progress_bytes, 10, ByteType::High, rarity_type.into_byte());

		progress_bytes
	}

	pub fn read_lowest_progress_byte(progress_bytes: &[u8; 11], byte_type: ByteType) -> u8 {
		let mut result = u8::MAX;

		for i in 0..progress_bytes.len() {
			let value = Self::read_dna_at(progress_bytes, i, byte_type);
			if result > value {
				result = value;
			}
		}

		result
	}
}
