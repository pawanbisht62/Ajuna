use super::*;
use sp_runtime::DispatchError;
use std::{io::Read, marker::PhantomData};

pub(crate) struct AvatarCombinator<'a, T: Config>(pub PhantomData<&'a T>);

impl<'a, T> AvatarCombinator<'a, T>
where
	T: Config,
{
	pub(crate) fn combine_avatars_in(
		forge_type: ForgeType,
		player: &T::AccountId,
		season_id: SeasonId,
		season: &SeasonOf<T>,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		match forge_type {
			ForgeType::Stack =>
				Self::stack_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Tinker =>
				Self::tinker_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Build =>
				Self::build_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Assemble =>
				Self::assemble_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Breed =>
				Self::breed_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Equip =>
				Self::equip_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Mate =>
				Self::mate_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::Special =>
				Self::special_avatars(player, input_leader, input_sacrifices, season_id, season),
			ForgeType::None => Err(Error::<T>::InvalidForgeComponents.into()),
		}
	}

	fn match_avatars(
		input_leader: ForgeItem<T>,
		sacrifices: Vec<ForgeItem<T>>,
	) -> (ForgeItem<T>, Vec<ForgeItem<T>>, Vec<AvatarIdOf<T>>) {
		let (leader_id, mut leader) = input_leader;
		let mut matches: u8 = 0;
		let mut no_fit: u8 = 0;

		let mut matching_score = Vec::new();
		let mut matching_sacrifices = Vec::new();
		let mut non_matching_sacrifices = Vec::new();

		let mut leader_progress_array = AvatarUtils::read_progress_array(&leader);

		sacrifices.iter().for_each(|(sacrifice_id, sacrifice)| {
			let sacrifice_progress_array = AvatarUtils::read_progress_array(&sacrifice);

			if let Some(matched_indexes) =
				AvatarUtils::match_progress_arrays(leader_progress_array, sacrifice_progress_array)
			{
				if AvatarUtils::has_attribute_set_with_same_values_as(
					&leader,
					&sacrifice,
					&[AvatarAttributes::ItemType, AvatarAttributes::ItemSubType],
				) {
					matching_score.extend(matched_indexes);
					matches += 1;
					non_matching_sacrifices.push(*sacrifice_id);
				} else {
					matching_sacrifices.push(*sacrifice_id);
				}
			} else {
				no_fit += 1;
			}
		});

		if !matching_score.is_empty() {
			let rolls = matches + no_fit;

			let match_probability = (1_f32 - BASE_PROGRESS_PROBABILITY) / MAX_SACRIFICE as f32;
			let probability_match = (((BASE_PROGRESS_PROBABILITY + matches as f32) *
				match_probability) *
				255_f32) as u8;

			let sacrifice_count = sacrifices.len();

			for i in 0..rolls as usize {
				let random_hash = sacrifices[i % sacrifice_count].1.dna.as_slice();

				if random_hash[(random_hash[i] % 32) as usize] < probability_match {
					let pos = matching_score[random_hash[i] as usize % matching_score.len()];

					leader_progress_array[pos as usize] += 0x10; // 16

					matching_score.retain(|item| *item != pos);

					if matching_score.is_empty() {
						break
					}
				}
			}

			AvatarUtils::write_progress_array(&mut leader, leader_progress_array);
		}

		leader.souls += sacrifices.iter().map(|(_, sacrifice)| sacrifice.souls).sum::<SoulCount>();

		(
			(leader_id, leader),
			sacrifices
				.into_iter()
				.filter(|(sacrifice_id, _)| {
					matching_sacrifices.iter().any(|match_id| match_id == sacrifice_id)
				})
				.collect(),
			non_matching_sacrifices,
		)
	}

	fn stack_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		let (avatar_id, mut avatar) = input_leader;

		let (new_quantity, new_souls) = input_sacrifices
			.iter()
			.map(|sacrifice| {
				(
					AvatarUtils::read_attribute(&sacrifice.1, AvatarAttributes::Quantity),
					sacrifice.1.souls,
				)
			})
			.reduce(|(acc_qty, acc_souls), (qty, souls)| {
				(acc_qty.saturating_add(qty), acc_souls.saturating_add(souls))
			})
			.unwrap_or_default();
		AvatarUtils::write_attribute(&mut avatar, AvatarAttributes::Quantity, new_quantity);

		let mut essence_avatar: Option<Avatar> = None;

		let stack_probability = (STACK_PROBABILITY * 256_f32).abs() as u8;

		for i in 0..input_sacrifices.len() {
			if stack_probability > avatar.dna[i] {
				essence_avatar = match essence_avatar {
					None => {
						let dna =
							AvatarMinterV2::<T>(PhantomData).generate_base_avatar_dna(player)?;
						Some(
							AvatarBuilder::with_dna(season_id, dna)
								.into_essence(EssenceItemType::Glimmer, 1)
								.build(),
						)
					},
					Some(entry) =>
						Some(AvatarBuilder::with_base_avatar(entry).add_quantity(1).build()),
				}
			}
		}

		avatar.souls += new_souls;

		let output_vec: Vec<ForgeOutput<T>> = input_sacrifices
			.into_iter()
			.map(|(sacrifice_id, _)| ForgeOutput::Consumed(sacrifice_id))
			.chain(essence_avatar.map(|minted_avatar| ForgeOutput::Minted(minted_avatar)))
			.collect();

		Ok((LeaderForgeOutput::Forged((avatar_id, avatar), 0), output_vec))
	}

	fn tinker_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		mut input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		let mut output_sacrifices = Vec::with_capacity(0);

		let sacrifice_pattern = input_sacrifices
			.iter()
			.map(|(_, sacrifice)| {
				AvatarUtils::read_attribute_as::<MaterialItemType>(
					sacrifice,
					AvatarAttributes::ItemSubType,
				)
			})
			.collect::<Vec<MaterialItemType>>();

		let leader_spec_bytes = AvatarUtils::read_full_spec_bytes(&input_leader.1);

		let unord_1 = AvatarUtils::bits_to_enums::<MaterialItemType>(leader_spec_bytes[0]);
		let ord_1 = AvatarUtils::bits_order_to_enum(leader_spec_bytes[1], unord_1);
		let pat_1_flag = sacrifice_pattern == ord_1;

		let unord_2 = AvatarUtils::bits_to_enums::<MaterialItemType>(leader_spec_bytes[2]);
		let ord_2 = AvatarUtils::bits_order_to_enum(leader_spec_bytes[3], unord_2);
		let pat_2_flag = sacrifice_pattern == ord_2;

		let unord_3 = AvatarUtils::bits_to_enums::<MaterialItemType>(leader_spec_bytes[4]);
		let ord_3 = AvatarUtils::bits_order_to_enum(leader_spec_bytes[5], unord_3);
		let pat_3_flag = sacrifice_pattern == ord_3;

		let unord_4 = AvatarUtils::bits_to_enums::<MaterialItemType>(leader_spec_bytes[6]);
		let ord_4 = AvatarUtils::bits_order_to_enum(leader_spec_bytes[7], unord_4);
		let pat_4_flag = sacrifice_pattern == ord_4;

		let mut soul_points = 0;

		let correct_pattern = (pat_1_flag || pat_2_flag || pat_3_flag || pat_4_flag) &&
			input_sacrifices
				.iter()
				.all(|(_, sacrifice)| AvatarUtils::can_use_avatar(sacrifice, 1));

		if correct_pattern {
			let mut success = true;

			for (sacrifice_id, mut sacrifice) in input_sacrifices.into_iter() {
				let (use_result, out_soul_points) = AvatarUtils::use_avatar(&mut sacrifice, 1);
				success &= use_result;
				soul_points += out_soul_points;

				let sacrifice_output =
					if AvatarUtils::read_attribute(&sacrifice, AvatarAttributes::Quantity) == 0 {
						ForgeOutput::Consumed(sacrifice_id)
					} else {
						ForgeOutput::Forged((sacrifice_id, sacrifice), 0)
					};

				output_sacrifices.push(sacrifice_output);
			}

			if !success || soul_points > u8::MAX as SoulCount {
				todo!()
			}

			let equipable_item_type = {
				if pat_1_flag {
					EquipableItemType::ArmorBase
				} else if pat_2_flag {
					EquipableItemType::ArmorComponent1
				} else if pat_3_flag {
					EquipableItemType::ArmorComponent2
				} else if pat_4_flag {
					EquipableItemType::ArmorComponent3
				} else {
					todo!()
				}
			};

			let pet_type = AvatarUtils::read_attribute_as::<PetType>(
				&input_leader.1,
				AvatarAttributes::ClassType2,
			);

			let slot_type = AvatarUtils::read_attribute_as::<SlotType>(
				&input_leader.1,
				AvatarAttributes::ClassType1,
			);

			let dna = AvatarMinterV2::<T>(PhantomData).generate_base_avatar_dna(player)?;
			let generated_blueprint = AvatarBuilder::with_dna(season_id, dna)
				.into_blueprint(
					BlueprintItemType::Blueprint,
					pet_type,
					slot_type,
					equipable_item_type,
					sacrifice_pattern,
					soul_points as SoulCount,
				)
				.build();

			output_sacrifices.push(ForgeOutput::Minted(generated_blueprint));
		}

		Ok((LeaderForgeOutput::Forged(input_leader, 0), output_sacrifices))
	}

	fn build_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		let mut output_sacrifices = Vec::with_capacity(input_sacrifices.len());

		let leader_spec_bytes = AvatarUtils::read_full_spec_bytes(&input_leader.1);

		let unord_1 = AvatarUtils::bits_to_enums::<MaterialItemType>(leader_spec_bytes[0]);
		let pat_1 = AvatarUtils::bits_order_to_enum(leader_spec_bytes[1], unord_1);

		// TODO: First quantity likely applies to leader avatar not sacrifices
		let quantities = [
			1_u8,
			leader_spec_bytes[3],
			leader_spec_bytes[4],
			leader_spec_bytes[5],
			leader_spec_bytes[6],
		];

		let sacrifice_pattern = input_sacrifices
			.iter()
			.map(|(_, sacrifice)| {
				AvatarUtils::read_attribute_as::<MaterialItemType>(
					sacrifice,
					AvatarAttributes::ItemSubType,
				)
			})
			.collect::<Vec<MaterialItemType>>();

		let mut soul_points = 0 as SoulCount;

		if sacrifice_pattern == pat_1 &&
			input_sacrifices.iter().enumerate().all(|(index, (_, sacrifice))| {
				AvatarUtils::can_use_avatar(sacrifice, quantities[index])
			}) {
			let mut success = true;

			for (sacrifice_id, mut sacrifice) in input_sacrifices.into_iter() {
				let (use_result, out_soul_points) = AvatarUtils::use_avatar(&mut sacrifice, 1);
				success &= use_result;
				soul_points += out_soul_points;

				let sacrifice_output =
					if AvatarUtils::read_attribute(&sacrifice, AvatarAttributes::Quantity) == 0 {
						ForgeOutput::Consumed(sacrifice_id)
					} else {
						ForgeOutput::Forged((sacrifice_id, sacrifice), 0)
					};

				output_sacrifices.push(sacrifice_output);
			}

			if !success {
				todo!()
			}

			let mut max_build = 6_usize;
			let mut build_prop = u8::MAX;

			let mut generated_equipables = Vec::with_capacity(3);

			for i in 0..max_build {
				// TODO: Add random hash
				if (build_prop >= input_leader.1.dna[(i + 6) % 32]) && soul_points > 0 {
					// Create new equipable avatar

					let pet_type = AvatarUtils::read_attribute_as::<PetType>(
						&input_leader.1,
						AvatarAttributes::ClassType2,
					);

					let slot_type = AvatarUtils::read_attribute_as::<SlotType>(
						&input_leader.1,
						AvatarAttributes::ClassType1,
					);

					let equipable_item_type = AvatarUtils::read_spec_byte_as::<EquipableItemType>(
						&input_leader.1,
						AvatarSpecBytes::SpecByte3,
					);

					let dna = AvatarMinterV2::<T>(PhantomData).generate_base_avatar_dna(player)?;
					let generated_equipable = AvatarBuilder::with_dna(season_id, dna)
						.into_equipable(
							equipable_item_type,
							pet_type,
							slot_type,
							RarityType::Common,
							1,
						)
						.build();

					generated_equipables.push(generated_equipable);

					soul_points = soul_points.saturating_sub(1);
				}

				// 38 :~ u8::MAX * 0.15;
				build_prop = build_prop.saturating_sub(38);
			}

			for i in 0..soul_points as usize {
				let sacrifice_index =
					(input_leader.1.dna[(i + 9) % 32] as usize % output_sacrifices.len());
				(&mut generated_equipables[sacrifice_index]).souls.saturating_inc();
			}

			output_sacrifices
				.extend(generated_equipables.into_iter().map(|gen| ForgeOutput::Minted(gen)));
		} else {
			todo!()
		}

		Ok((LeaderForgeOutput::Forged(input_leader, 0), output_sacrifices))
	}

	fn assemble_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn breed_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn equip_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		let (leader_id, mut leader) = input_leader;

		let mut new_souls = SoulCount::MIN;

		let mut leader_spec_bytes = AvatarUtils::read_full_spec_bytes(&leader);
		let equipped_slots = leader_spec_bytes.map(|byte| byte & ByteType::Low as u8);

		for (_, sacrifice) in input_sacrifices.iter() {
			new_souls += sacrifice.souls;

			let slot_type =
				AvatarUtils::read_attribute(sacrifice, AvatarAttributes::ClassType1) as usize;
			let filled_slots =
				equipped_slots.clone().into_iter().filter(|slot| *slot > 0).count() as u8;
			let slot_empty = equipped_slots[slot_type] == 0;
			if filled_slots >= MAX_EQUIPPED_SLOTS && slot_empty {
				continue
			}

			leader_spec_bytes[slot_type] =
				AvatarUtils::read_spec_byte(sacrifice, AvatarSpecBytes::SpecByte1);
		}

		AvatarUtils::write_full_spec_bytes(&mut leader, leader_spec_bytes);

		leader.souls += new_souls;

		let output_vec: Vec<ForgeOutput<T>> = input_sacrifices
			.into_iter()
			.map(|(sacrifice_id, _)| ForgeOutput::Consumed(sacrifice_id))
			.collect();

		Ok((LeaderForgeOutput::Forged((leader_id, leader), 0), output_vec))
	}

	fn mate_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}

	fn special_avatars(
		player: &T::AccountId,
		input_leader: ForgeItem<T>,
		input_sacrifices: Vec<ForgeItem<T>>,
		season_id: SeasonId,
		season: &SeasonOf<T>,
	) -> Result<(LeaderForgeOutput<T>, Vec<ForgeOutput<T>>), DispatchError> {
		todo!()
	}
}
