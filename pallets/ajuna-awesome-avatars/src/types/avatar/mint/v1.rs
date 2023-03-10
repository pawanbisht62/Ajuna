use crate::*;
use sp_runtime::DispatchError;

pub(crate) struct AvatarMinterV1<'a, T: Config>(pub PhantomData<&'a T>);

impl<'a, T> Minter<T> for AvatarMinterV1<'a, T>
where
	T: Config,
{
	fn mint_avatar_set(
		&self,
		player: &T::AccountId,
		season_id: &SeasonId,
		season: &SeasonOf<T>,
		mint_option: &MintOption,
	) -> Result<Vec<MintOutput<T>>, DispatchError> {
		let is_batched = mint_option.count.is_batched();
		(0..mint_option.count as usize)
			.map(|_| {
				let avatar_id = Pallet::<T>::random_hash(b"create_avatar", player);
				let dna = self.random_dna(&avatar_id, season, is_batched)?;
				let souls = (dna.iter().map(|x| *x as SoulCount).sum::<SoulCount>() % 100) + 1;
				let avatar =
					Avatar { season_id: *season_id, version: mint_option.mint_version, dna, souls };
				Ok((avatar_id, avatar))
			})
			.collect::<Result<Vec<MintOutput<T>>, _>>()
	}
}

impl<'a, T> AvatarMinterV1<'a, T>
where
	T: Config,
{
	#[inline]
	fn random_dna(
		&self,
		hash: &T::Hash,
		season: &SeasonOf<T>,
		batched_mint: bool,
	) -> Result<Dna, DispatchError> {
		let dna = (0..season.max_components)
			.map(|i| {
				let (random_tier, random_variation) =
					Self::random_component(season, hash, i as usize * 2, batched_mint);
				((random_tier << 4) | random_variation) as u8
			})
			.collect::<Vec<_>>();
		Dna::try_from(dna).map_err(|_| Error::<T>::IncorrectDna.into())
	}

	#[inline]
	fn random_component(
		season: &SeasonOf<T>,
		hash: &T::Hash,
		index: usize,
		batched_mint: bool,
	) -> (u8, u8) {
		let hash = hash.as_ref();
		let random_tier = {
			let random_prob = hash[index] % MAX_PERCENTAGE;
			let probs =
				if batched_mint { &season.batch_mint_probs } else { &season.single_mint_probs };
			let mut cumulative_sum = 0;
			let mut random_tier = season.tiers[0] as u8;
			for i in 0..probs.len() {
				let new_cumulative_sum = cumulative_sum + probs[i];
				if random_prob >= cumulative_sum && random_prob < new_cumulative_sum {
					random_tier = season.tiers[i] as u8;
					break
				}
				cumulative_sum = new_cumulative_sum;
			}
			random_tier
		};
		let random_variation = hash[index + 1] % season.max_variations;
		(random_tier, random_variation)
	}
}
