use super::{types::*, ByteType};
use crate::types::{Avatar, AvatarVersion, Dna, SeasonId, SoulCount};

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

	pub fn into_essence(mut self, essence_type: EssenceItemType) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Essence)
			.with_attribute(AvatarAttributes::ItemSubType, essence_type)
	}

	pub fn into_equipable(mut self, equipable_type: EquipableItemType) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Equipable)
			.with_attribute(AvatarAttributes::ItemSubType, equipable_type)
	}

	pub fn into_blueprint(mut self, blueprint_type: BlueprintItemType) -> Self {
		self.with_attribute(AvatarAttributes::ItemType, ItemType::Blueprint)
			.with_attribute(AvatarAttributes::ItemSubType, blueprint_type)
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
		match byte_type {
			ByteType::Full => avatar.dna[position],
			ByteType::High => avatar.dna[position] >> 4,
			ByteType::Low => avatar.dna[position] & ByteType::High as u8,
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

	pub fn write_progress_array(avatar: &mut Avatar, value: [u8; 11]) {
		(&mut avatar.dna[21..11]).copy_from_slice(&value);
	}
}
