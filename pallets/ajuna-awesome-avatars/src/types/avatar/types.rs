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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PetItemType {
	Pet = 1,
	PetPart = 2,
	Egg = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EquipableItemType {
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PetType {
	TankyBulldog = 1,
	FoxishDude = 2,
	WierdFerry = 3,
	FireDino = 4,
	BigHybrid = 5,
	GiantWoodStick = 6,
	CrazyDude = 7,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PetPartType {
	Horns = 1,
	Furs = 2,
	Wings = 3,
	Scales = 4,
	Claws = 5,
	Sticks = 6,
	Eyes = 7,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SlotType {
	Head = 1,
	Breast = 2,
	ArmFront = 3,
	ArmBack = 4,
	LegFront = 5,
	LegBack = 6,
	WeaponFront = 8,
	WeaponBack = 9,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MaterialItemType {
	Polymers = 1,
	Electronics = 2,
	PowerCells = 3,
	Optics = 4,
	Metals = 5,
	Ceramics = 6,
	Superconductors = 7,
	Nanomaterials = 8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum EssenceItemType {
	Glimmer = 1,
	ColorSpark = 2,
	GlowSpark = 3,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BlueprintItemType {
	Blueprint = 1,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum SpecialItemType {
	Special = 1,
}
