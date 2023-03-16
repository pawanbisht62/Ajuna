#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum ByteType {
	Full = 0b1111_1111,
	High = 0b0000_1111,
	Low = 0b1111_0000,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum HexType {
	X0 = 0b0000,
	X1 = 0b0001,
	X2 = 0b0010,
	X3 = 0b0011,
	X4 = 0b0100,
	X5 = 0b0101,
	X6 = 0b0110,
	X7 = 0b0111,
	X8 = 0b1000,
	X9 = 0b1001,
	XA = 0b1010,
	XB = 0b1011,
	XC = 0b1100,
	XD = 0b1101,
	XE = 0b1110,
	XF = 0b1111,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum NibbleType {
	X0 = 0b0000,
	X1 = 0b0001,
	X2 = 0b0010,
	X3 = 0b0011,
	X4 = 0b0100,
	X5 = 0b0101,
	X6 = 0b0110,
	X7 = 0b0111,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum ItemType {
	#[default]
	Pet = 1,
	Material = 2,
	Essence = 3,
	Equipable = 4,
	Blueprint = 5,
	Special = 6,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum PetItemType {
	#[default]
	Pet = 1,
	PetPart = 2,
	Egg = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum EquipableItemType {
	#[default]
	ArmorBase = 1,
	ArmorComponent1 = 2,
	ArmorComponent2 = 3,
	ArmorComponent3 = 4,
	WeaponVersion1 = 5,
	WeaponVersion2 = 6,
	WeaponVersion3 = 7,
}

impl EquipableItemType {
	pub fn is_armor(item: EquipableItemType) -> bool {
		item == EquipableItemType::ArmorBase ||
			item == EquipableItemType::ArmorComponent1 ||
			item == EquipableItemType::ArmorComponent2 ||
			item == EquipableItemType::ArmorComponent3
	}

	pub fn is_weapon(item: EquipableItemType) -> bool {
		item == EquipableItemType::WeaponVersion1 ||
			item == EquipableItemType::WeaponVersion2 ||
			item == EquipableItemType::WeaponVersion3
	}
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum PetType {
	#[default]
	TankyBulldog = 1,
	FoxishDude = 2,
	WierdFerry = 3,
	FireDino = 4,
	BigHybrid = 5,
	GiantWoodStick = 6,
	CrazyDude = 7,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum PetPartType {
	#[default]
	Horns = 1,
	Furs = 2,
	Wings = 3,
	Scales = 4,
	Claws = 5,
	Sticks = 6,
	Eyes = 7,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum SlotType {
	#[default]
	Head = 1,
	Breast = 2,
	ArmFront = 3,
	ArmBack = 4,
	LegFront = 5,
	LegBack = 6,
	WeaponFront = 8,
	WeaponBack = 9,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum MaterialItemType {
	#[default]
	Polymers = 1,
	Electronics = 2,
	PowerCells = 3,
	Optics = 4,
	Metals = 5,
	Ceramics = 6,
	Superconductors = 7,
	Nanomaterials = 8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum EssenceItemType {
	#[default]
	Glimmer = 1,
	ColorSpark = 2,
	GlowSpark = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum BlueprintItemType {
	#[default]
	Blueprint = 1,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub(crate) enum SpecialItemType {
	#[default]
	Special = 1,
}
