use crate::types::{avatar::types::ByteType, Avatar};

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
pub(crate) struct WrappedAvatar<'a> {
	pub inner: &'a mut Avatar,
}

impl<'a> WrappedAvatar<'a> {
	pub fn from(avatar: &'a mut Avatar) -> Self {
		Self { inner: avatar }
	}

	pub fn is_same_type_as(&self, other: &WrappedAvatar) -> bool {
		self.read_attribute(AvatarAttributes::ItemType) ==
			other.read_attribute(AvatarAttributes::ItemType)
	}

	fn read_dna_strand(&self, position: usize, byte_type: ByteType) -> u8 {
		let dna = &self.inner.dna;

		match byte_type {
			ByteType::Full => dna[position],
			ByteType::High => dna[position] >> 4,
			ByteType::Low => dna[position] & ByteType::High as u8,
		}
	}

	fn write_dna_strand(&mut self, position: usize, byte_type: ByteType, value: u8) {
		let dna = &mut self.inner.dna;
		match byte_type {
			ByteType::Full => dna[position] = value,
			ByteType::High =>
				dna[position] = (dna[position] & (ByteType::High as u8)) | (value << 4),
			ByteType::Low =>
				dna[position] =
					(dna[position] & (ByteType::Low as u8)) | (value & (ByteType::High as u8)),
		}
	}

	pub fn read_attribute(&self, attribute: AvatarAttributes) -> u8 {
		match attribute {
			AvatarAttributes::ItemType => self.read_dna_strand(0, ByteType::High),
			AvatarAttributes::ItemSubType => self.read_dna_strand(0, ByteType::Low),
			AvatarAttributes::ClassType1 => self.read_dna_strand(1, ByteType::High),
			AvatarAttributes::ClassType2 => self.read_dna_strand(1, ByteType::High),
			AvatarAttributes::CustomType1 => self.read_dna_strand(2, ByteType::High),
			AvatarAttributes::CustomType2 => self.read_dna_strand(4, ByteType::Full),
			AvatarAttributes::RarityType => self.read_dna_strand(2, ByteType::Low),
			AvatarAttributes::Quantity => self.read_dna_strand(3, ByteType::Full),
		}
	}

	pub fn write_attribute(&mut self, attribute: AvatarAttributes, value: u8) {
		match attribute {
			AvatarAttributes::ItemType => self.write_dna_strand(0, ByteType::High, value),
			AvatarAttributes::ItemSubType => self.write_dna_strand(0, ByteType::Low, value),
			AvatarAttributes::ClassType1 => self.write_dna_strand(1, ByteType::High, value),
			AvatarAttributes::ClassType2 => self.write_dna_strand(1, ByteType::High, value),
			AvatarAttributes::CustomType1 => self.write_dna_strand(2, ByteType::High, value),
			AvatarAttributes::CustomType2 => self.write_dna_strand(4, ByteType::Full, value),
			AvatarAttributes::RarityType => self.write_dna_strand(2, ByteType::Low, value),
			AvatarAttributes::Quantity => self.write_dna_strand(3, ByteType::Full, value),
		}
	}

	pub fn read_full_spec_bytes(&self) -> [u8; 16] {
		let mut out = [0; 16];
		out.copy_from_slice(&self.inner.dna[5..21]);
		out
	}

	pub fn read_spec_byte(&self, spec_byte: AvatarSpecBytes) -> u8 {
		match spec_byte {
			AvatarSpecBytes::SpecByte1 => self.read_dna_strand(5, ByteType::Full),
			AvatarSpecBytes::SpecByte2 => self.read_dna_strand(6, ByteType::Full),
			AvatarSpecBytes::SpecByte3 => self.read_dna_strand(7, ByteType::Full),
			AvatarSpecBytes::SpecByte4 => self.read_dna_strand(8, ByteType::Full),
			AvatarSpecBytes::SpecByte5 => self.read_dna_strand(9, ByteType::Full),
			AvatarSpecBytes::SpecByte6 => self.read_dna_strand(10, ByteType::Full),
			AvatarSpecBytes::SpecByte7 => self.read_dna_strand(11, ByteType::Full),
			AvatarSpecBytes::SpecByte8 => self.read_dna_strand(12, ByteType::Full),
			AvatarSpecBytes::SpecByte9 => self.read_dna_strand(13, ByteType::Full),
			AvatarSpecBytes::SpecByte10 => self.read_dna_strand(14, ByteType::Full),
			AvatarSpecBytes::SpecByte11 => self.read_dna_strand(15, ByteType::Full),
			AvatarSpecBytes::SpecByte12 => self.read_dna_strand(16, ByteType::Full),
			AvatarSpecBytes::SpecByte13 => self.read_dna_strand(17, ByteType::Full),
			AvatarSpecBytes::SpecByte14 => self.read_dna_strand(18, ByteType::Full),
			AvatarSpecBytes::SpecByte15 => self.read_dna_strand(19, ByteType::Full),
			AvatarSpecBytes::SpecByte16 => self.read_dna_strand(20, ByteType::Full),
		}
	}

	pub fn write_full_spec_bytes(&mut self, value: [u8; 16]) {
		(&mut self.inner.dna[5..16]).copy_from_slice(&value);
	}

	pub fn write_spec_byte(&mut self, spec_byte: AvatarSpecBytes, value: u8) {
		match spec_byte {
			AvatarSpecBytes::SpecByte1 => self.write_dna_strand(5, ByteType::Full, value),
			AvatarSpecBytes::SpecByte2 => self.write_dna_strand(6, ByteType::Full, value),
			AvatarSpecBytes::SpecByte3 => self.write_dna_strand(7, ByteType::Full, value),
			AvatarSpecBytes::SpecByte4 => self.write_dna_strand(8, ByteType::Full, value),
			AvatarSpecBytes::SpecByte5 => self.write_dna_strand(9, ByteType::Full, value),
			AvatarSpecBytes::SpecByte6 => self.write_dna_strand(10, ByteType::Full, value),
			AvatarSpecBytes::SpecByte7 => self.write_dna_strand(11, ByteType::Full, value),
			AvatarSpecBytes::SpecByte8 => self.write_dna_strand(12, ByteType::Full, value),
			AvatarSpecBytes::SpecByte9 => self.write_dna_strand(13, ByteType::Full, value),
			AvatarSpecBytes::SpecByte10 => self.write_dna_strand(14, ByteType::Full, value),
			AvatarSpecBytes::SpecByte11 => self.write_dna_strand(15, ByteType::Full, value),
			AvatarSpecBytes::SpecByte12 => self.write_dna_strand(16, ByteType::Full, value),
			AvatarSpecBytes::SpecByte13 => self.write_dna_strand(17, ByteType::Full, value),
			AvatarSpecBytes::SpecByte14 => self.write_dna_strand(18, ByteType::Full, value),
			AvatarSpecBytes::SpecByte15 => self.write_dna_strand(19, ByteType::Full, value),
			AvatarSpecBytes::SpecByte16 => self.write_dna_strand(20, ByteType::Full, value),
		}
	}

	pub fn read_progress_array(&self) -> [u8; 11] {
		let mut out = [0; 11];
		out.copy_from_slice(&self.inner.dna[21..32]);
		out
	}

	pub fn write_progress_array(&mut self, value: [u8; 11]) {
		(&mut self.inner.dna[21..11]).copy_from_slice(&value);
	}
}
