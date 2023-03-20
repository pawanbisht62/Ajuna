use crate::types::{avatar::tools::v2::types::ByteType, Avatar};

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

/// Struct to wrap DNA interactions with Avatars from V2 upwards.
/// Don't use with Avatars with V1.
pub(crate) struct AvatarWrapper;

impl AvatarWrapper {
	pub fn is_same_type_as(avatar: &Avatar, other: &Avatar) -> bool {
		Self::read_attribute(avatar, AvatarAttributes::ItemType) ==
			Self::read_attribute(other, AvatarAttributes::ItemType)
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
