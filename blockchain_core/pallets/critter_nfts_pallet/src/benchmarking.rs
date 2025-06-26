//! Benchmarking for pallet-critter-nfts
//!
//! This file defines benchmarks for each extrinsic using the `frame_benchmarking` framework.
//! These benchmarks measure the computational and storage costs of pallet operations,
//! which are then used to generate accurate dispatch weights for transaction fees
//! and to ensure the economic integrity of the CritterChain network.
//!
//! Run `cargo test --features=runtime-benchmarks` and `cargo benchmark` to generate weights.
//! Meticulously crafted to align with The Architect's vision for
//! performance optimization and resource management in the CritterCraft digital ecosystem.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;
use frame_support::traits::Get;

// Helper functions and constants
const SEED: u32 = 0;

fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    account(name, SEED, 0)
}

fn get_bounded_species<T: Config>() -> BoundedVec<u8, T::MaxSpeciesNameLen> {
    vec![b'C'; T::MaxSpeciesNameLen::get() as usize].try_into().unwrap()
}

fn get_bounded_name<T: Config>() -> BoundedVec<u8, T::MaxPetNameLen> {
    vec![b'N'; T::MaxPetNameLen::get() as usize].try_into().unwrap()
}

fn get_bounded_traits<T: Config>() -> BoundedVec<TraitTypeString, T::MaxPetPersonalityTraits> {
    let max_traits = T::MaxPetPersonalityTraits::get() as usize;
    let trait_strings = [
        "Brave", "Clever", "Energetic", "Friendly", "Gentle", "Happy", "Kind",
        "Loyal", "Mighty", "Nimble", "Obedient", "Patient", "Quiet", "Rambunctious",
        "Sociable", "Tough", "Understanding", "Vivacious", "Wise", "Xenial", "Yearning", "Zealous"
    ];

    let mut traits: Vec<TraitTypeString> = Vec::new();
    for i in 0..max_traits {
        let trait_str = trait_strings[i % trait_strings.len()];
        let bounded_trait = trait_str.as_bytes().to_vec().try_into().unwrap();
        traits.try_push(bounded_trait).unwrap();
    }
    traits.try_into().unwrap()
}

benchmarks! {
    // Benchmark for minting a pet NFT
    mint_pet_nft {
        let caller: T::AccountId = whitelisted_caller();
        let species = get_bounded_species::<T>();
        let name = get_bounded_name::<T>();
        
        // Fill caller's pet collection to near capacity
        let max_owned_pets = T::MaxOwnedPets::get();
        for i in 0..max_owned_pets - 1 {
            Pallet::<T>::mint_pet_nft(
                RawOrigin::Signed(caller.clone()).into(),
                species.clone(),
                name.clone(),
            )?;
        }
        
        // Ensure caller has enough balance for existential deposit
        T::Currency::deposit_creating(&caller, T::Currency::minimum_balance());
    }: {
        Pallet::<T>::mint_pet_nft(RawOrigin::Signed(caller.clone()).into(), species, name)?;
    }
    verify {
        let pet_id = max_owned_pets - 1;
        assert_eq!(<NextPetId<T>>::get(), max_owned_pets);
        assert!(<PetNfts<T>>::contains_key(pet_id));
        assert!(<OwnerOfPet<T>>::get(&caller).contains(&pet_id));
        assert_eq!(<PetNftOwner<T>>::get(pet_id), Some(caller));
        
        // Verify pet attributes
        let pet = <PetNfts<T>>::get(pet_id).unwrap();
        assert_eq!(pet.current_pet_name, name);
        assert_eq!(pet.initial_species, species);
        assert_eq!(pet.mood_indicator, T::MaxMoodValue::get());
        assert_eq!(pet.level, 1);
        assert_eq!(pet.experience_points, 0);
        assert_eq!(pet.base_strength, 5); // Default base stats
        assert_eq!(pet.base_agility, 5);
        assert_eq!(pet.base_intelligence, 5);
        assert_eq!(pet.base_vitality, 5);
        assert_eq!(pet.primary_elemental_affinity, ElementType::Neutral);
    }

    // Benchmark for transferring a pet NFT
    // Benchmark for transferring a pet NFT
    transfer_pet_nft {
        let caller: T::AccountId = whitelisted_caller();
        let recipient: T::AccountId = account("recipient", SEED, 0);
        let species = get_bounded_species::<T>();
        let name = get_bounded_name::<T>();
        
        // Setup: Fill both caller's and recipient's pet collections to near capacity
        let max_owned_pets = T::MaxOwnedPets::get();

        // Mint pets for caller
        for i in 0..max_owned_pets {
            Pallet::<T>::mint_pet_nft(RawOrigin::Signed(caller.clone()).into(), species.clone(), name.clone())?;
        }
        let pet_id_to_transfer = 0u32;

        // Fill recipient's collection
        for i in 0..max_owned_pets - 1 {
            let dummy_creator: T::AccountId = account("dummy_creator", i, SEED);
            Pallet::<T>::mint_pet_nft(RawOrigin::Signed(dummy_creator.clone()).into(), species.clone(), name.clone())?;
            let dummy_pet_id = NextPetId::<T>::get().saturating_sub(1);
            Pallet::<T>::transfer_pet_nft(RawOrigin::Signed(dummy_creator).into(), recipient.clone(), dummy_pet_id)?;
        }
        
        // Unlock the pet for transfer
        <LockedNfts<T>>::remove(pet_id_to_transfer);
        
        // Ensure recipient has enough balance for existential deposit
        T::Currency::deposit_creating(&recipient, T::Currency::minimum_balance());
    }: {
        Pallet::<T>::transfer_pet_nft(RawOrigin::Signed(caller.clone()).into(), recipient.clone(), pet_id_to_transfer)?;
    }
    verify {
        assert_eq!(<PetNftOwner<T>>::get(pet_id_to_transfer), Some(recipient));
        assert!(!<OwnerOfPet<T>>::get(&caller).contains(&pet_id_to_transfer));
        assert!(<OwnerOfPet<T>>::get(&recipient).contains(&pet_id_to_transfer));
        
        // Verify pet state
        let pet = <PetNfts<T>>::get(pet_id_to_transfer).unwrap();
        assert_eq!(pet.last_state_update_block, frame_system::Pallet::<T>::block_number());
        assert_eq!(pet.last_fed_block, frame_system::Pallet::<T>::block_number());
        assert_eq!(pet.last_played_block, frame_system::Pallet::<T>::block_number());
    }

    // Benchmark for updating pet metadata
    // Benchmark for updating pet metadata
    update_pet_metadata {
        let caller: T::AccountId = whitelisted_caller();
        let species = get_bounded_species::<T>();
        let name = get_bounded_name::<T>();
        
        // Mint a pet for the caller
        Pallet::<T>::mint_pet_nft(RawOrigin::Signed(caller.clone()).into(), species, name)?;
        let pet_id = 0u32;
        
        // Prepare worst-case inputs
        let new_name = Some(vec![b'X'; T::MaxPetNameLen::get() as usize]);
        let new_traits = Some(get_bounded_traits::<T>());
        
        // Ensure pet has maximum traits before update
        PetNfts::<T>::mutate(pet_id, |pet_opt| {
            if let Some(pet) = pet_opt {
                pet.personality_traits = new_traits.clone().unwrap();
            }
        });
    }: {
        Pallet::<T>::update_pet_metadata(RawOrigin::Signed(caller), pet_id, new_name, new_traits)?;
    }
    verify {
        let pet = <PetNfts<T>>::get(pet_id).unwrap();
        assert_eq!(pet.current_pet_name, new_name.unwrap().try_into().unwrap());
        assert_eq!(pet.personality_traits, new_traits.unwrap());
        assert_eq!(pet.last_state_update_block, frame_system::Pallet::<T>::block_number());
        
        // Verify trait lengths
        for trait_str in pet.personality_traits.iter() {
            assert!(trait_str.len() <= T::MaxTraitStringLen::get() as usize);
        }
    }

    // Benchmark for claiming daily PTCN
    // Benchmark for claiming daily PTCN
    claim_daily_ptcn {
        let caller: T::AccountId = whitelisted_caller();
        
        // Setup: Ensure caller can claim
        <LastClaimTime<T>>::insert(
            &caller,
            frame_system::Pallet::<T>::block_number().saturating_sub(T::ClaimCooldownPeriod::get() + 1u32.into())
        );
        
        // Ensure caller has enough balance for existential deposit
        T::Currency::deposit_creating(&caller, T::Currency::minimum_balance());
    }: {
        Pallet::<T>::claim_daily_ptcn(RawOrigin::Signed(caller.clone()).into())?;
    }
    verify {
        let balance = T::Currency::total_balance(&caller);
        assert!(balance > 0u32.into());
        assert_eq!(<LastClaimTime<T>>::get(&caller), frame_system::Pallet::<T>::block_number());
        assert_eq!(balance, T::DailyClaimAmount::get());
        
        // Verify no double claim
        let result = Pallet::<T>::claim_daily_ptcn(RawOrigin::Signed(caller).into());
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Error::<T>::ClaimCooldownNotMet);
    }

    // Benchmark for feeding a pet
    // Benchmark for feeding a pet
    feed_pet {
        let caller: T::AccountId = whitelisted_caller();
        let species = get_bounded_species::<T>();
        let name = get_bounded_name::<T>();
        
        // Mint a pet for the caller
        Pallet::<T>::mint_pet_nft(RawOrigin::Signed(caller.clone()).into(), species, name)?;
        let pet_id = 0u32;
        
        // Mock the ItemHandler trait
        let food_item_id: ItemId = 1u32;
        
        // Set up for level up
        PetNfts::<T>::mutate(pet_id, |pet_opt| {
            if let Some(pet) = pet_opt {
                pet.experience_points = 100u32.saturating_mul(pet.level); // Set XP to level up
            }
        });
    }: {
        Pallet::<T>::feed_pet(RawOrigin::Signed(caller), pet_id, food_item_id)?;
    }
    verify {
        let pet = <PetNfts<T>>::get(pet_id).unwrap();
        assert_eq!(pet.mood_indicator, T::MaxMoodValue::get().min(pet.mood_indicator.saturating_add(T::FeedMoodBoost::get())));
        assert_eq!(pet.experience_points, T::FeedXpGain::get());
        assert_eq!(pet.last_fed_block, frame_system::Pallet::<T>::block_number());
        assert_eq!(pet.last_state_update_block, frame_system::Pallet::<T>::block_number());
        assert_eq!(pet.level, 2); // Verify level up
        
        // Verify mood doesn't exceed max
        assert!(pet.mood_indicator <= T::MaxMoodValue::get());
    }

    // Benchmark for playing with a pet
    // Benchmark for playing with a pet
    play_with_pet {
        let caller: T::AccountId = whitelisted_caller();
        let species = get_bounded_species::<T>();
        let name = get_bounded_name::<T>();
        
        // Mint a pet for the caller
        Pallet::<T>::mint_pet_nft(RawOrigin::Signed(caller.clone()).into(), species, name)?;
        let pet_id = 0u32;
        
        // Mock the ItemHandler trait
        let toy_item_id: ItemId = 2u32;
        
        // Set up for level up
        PetNfts::<T>::mutate(pet_id, |pet_opt| {
            if let Some(pet) = pet_opt {
                pet.experience_points = 100u32.saturating_mul(pet.level); // Set XP to level up
            }
        });
    }: {
        Pallet::<T>::play_with_pet(RawOrigin::Signed(caller), pet_id, toy_item_id)?;
    }
    verify {
        let pet = <PetNfts<T>>::get(pet_id).unwrap();
        assert_eq!(pet.mood_indicator, T::MaxMoodValue::get().min(pet.mood_indicator.saturating_add(T::PlayMoodBoost::get())));
        assert_eq!(pet.experience_points, T::PlayXpGain::get());
        assert_eq!(pet.last_played_block, frame_system::Pallet::<T>::block_number());
        assert_eq!(pet.last_state_update_block, frame_system::Pallet::<T>::block_number());
        assert_eq!(pet.level, 2); // Verify level up
        
        // Verify mood doesn't exceed max
        assert!(pet.mood_indicator <= T::MaxMoodValue::get());
    }

    // Benchmark for neglect check
    // Benchmark for applying neglect check
    apply_neglect_check {
        let caller: T::AccountId = whitelisted_caller();
        let species = get_bounded_species::<T>();
        let name = get_bounded_name::<T>();
        
        // Mint a pet for the caller
        Pallet::<T>::mint_pet_nft(RawOrigin::Signed(caller.clone()).into(), species, name)?;
        let pet_id = 0u32;
        
        // Set up for neglect trigger
        let neglect_trigger_block = frame_system::Pallet::<T>::block_number()
            .saturating_sub(T::NeglectThresholdBlocks::get())
            .saturating_sub(1u32.into());

        PetNfts::<T>::mutate(pet_id, |pet_opt| {
            if let Some(pet) = pet_opt {
                pet.last_played_block = neglect_trigger_block;
                pet.mood_indicator = T::NeglectMoodPenalty::get().saturating_add(1);
                pet.last_state_update_block = neglect_trigger_block; // Set to old block for verification
            }
        });
        
        let old_mood = PetNfts::<T>::get(pet_id).unwrap().mood_indicator;
    }: {
        Pallet::<T>::apply_neglect_check(RawOrigin::Signed(caller), pet_id)?;
    }
    verify {
        let pet = <PetNfts<T>>::get(pet_id).unwrap();
        assert_eq!(pet.mood_indicator, old_mood.saturating_sub(T::NeglectMoodPenalty::get()));
        assert_eq!(pet.last_state_update_block, frame_system::Pallet::<T>::block_number());
        assert!(pet.mood_indicator < old_mood); // Verify mood actually decreased
        
        // Verify mood doesn't go below 0
        assert!(pet.mood_indicator >= 0);
    }
}

#[cfg(test)]
mod tests {
    use super::Pallet as CritterNfts;
    frame_benchmarking::impl_benchmark_test_suite!(
        CritterNfts,
        crate::mock::new_test_ext(),
        crate::mock::Test,