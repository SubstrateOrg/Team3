use support::{decl_module, decl_storage, ensure, StorageValue, StorageMap, dispatch::Result, Parameter};
use sr_primitives::traits::{SimpleArithmetic, Bounded};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use rstd::result;
//use rstd::collections::btree_map::BTreeMap;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: system::Trait {
	type KittyIndex: Parameter + SimpleArithmetic + Bounded + Default + Copy;
}

//Account stores accout ID and user kitty index in bmap
//pub OwnedKittiesMap get(owned_kitties_map): map T::AccountId => Option<BTreeMap<(T::AccountId, T::KittyIndex), T::KittyIndex>>;

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map T::KittyIndex => Option<Kitty>;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): T::KittyIndex;

		/// Get kitty ID by account ID and user kitty index
		pub OwnedKitties get(owned_kitties): map (T::AccountId, T::KittyIndex) => Option<T::KittyIndex>;
		/// Get number of kitties by account ID
		pub OwnedKittiesCount get(owned_kitties_count): map T::AccountId => T::KittyIndex;

		
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) -> Result {
			let sender = ensure_signed(origin)?;

			let new_kitty_id = Self::next_kitty_id()?;
			let new_dna = Self::random_value(&sender);

			runtime_io::print("new_dna:");
			for i in &new_dna {
				runtime_io::print(*i);
			}

			Self::insert_kitty(sender, new_kitty_id, Kitty(new_dna));

			Ok(())
		}

		/// Breed kitties
		pub fn breed(origin, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> Result {
			let sender = ensure_signed(origin)?;

			Self::do_breed(sender, kitty_id_1, kitty_id_2)?;

			Ok(())
		}

		/// Transfer kitty
		pub fn transfer(origin, kitty_id: T::KittyIndex, receiver: T::AccountId) -> Result {
			let sender = ensure_signed(origin)?;

			Self::transfer_kitty(sender, kitty_id, receiver)?;

			Ok(())
		}
	}
}

fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
	let mut new_dna = 0;
	let mut bit_mask: u8 = 0x01;

	runtime_io::print(dna1);
	runtime_io::print(dna2);
	runtime_io::print(selector);

	for _ in 0..8 {
		if (bit_mask & selector) != 0 {
			new_dna |= dna1 & bit_mask;
		} else {
			new_dna |= dna2 & bit_mask;
		}

		bit_mask <<= 1;
	}

	runtime_io::print(new_dna);

	return new_dna;
}

impl<T: Trait> Module<T> {
	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
	}

	fn next_kitty_id() -> result::Result<T::KittyIndex, &'static str> {
		let kitty_id = Self::kitties_count();
		if kitty_id == T::KittyIndex::max_value() {
			return Err("Kitties count overflow");
		}
		Ok(kitty_id)
	}

	fn insert_kitty(owner: T::AccountId, kitty_id: T::KittyIndex, kitty: Kitty) {
		// Create and store kitty
		<Kitties<T>>::insert(kitty_id, kitty);
		<KittiesCount<T>>::put(kitty_id + 1.into());

		// Store the ownership information
		let user_kitties_id = Self::owned_kitties_count(&owner);

		//BTreeMap
		// if let Some(mut owner_map) = Self::owned_kitties_map(&owner) {
		// 	owner_map.insert((owner.clone(), user_kitties_id), kitty_id);
		// } else {
		// 	let mut owner_map = BTreeMap::new();
		// 	owner_map.insert((owner.clone(), user_kitties_id), kitty_id);
		// }

		<OwnedKitties<T>>::insert((owner.clone(), user_kitties_id), kitty_id);
		<OwnedKittiesCount<T>>::insert(owner, user_kitties_id + 1.into());
	}

	fn do_breed(sender: T::AccountId, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> Result {
		let kitty1 = Self::kitty(kitty_id_1);
		let kitty2 = Self::kitty(kitty_id_2);

		ensure!(kitty1.is_some(), "Invalid kitty_id_1");
		ensure!(kitty2.is_some(), "Invalid kitty_id_2");
		ensure!(kitty_id_1 != kitty_id_2, "Needs different parent");

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.unwrap().0;
		let kitty2_dna = kitty2.unwrap().0;

		// Generate a random 128bit value
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		// Combine parents and selector to create new kitty
		for i in 0..kitty1_dna.len() {
			new_dna[i] = combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}

		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

		Ok(())
	}

	fn search_own_kitty(sender: T::AccountId, own_kitty_id: T::KittyIndex) -> result::Result<T::KittyIndex, &'static str> {
		// let owner_map = Self::owned_kitties_map(&sender).ok_or("Account dosen't have a kitty!")?;

		// if let Some(kitty_id)= owner_map.get(&(sender, own_kitty_id)) {
		// 	return Ok(*kitty_id);
		// } else {
		// 	return Err("Account dosen't own the kitty!");
		// }

		let kitty_id = Self::owned_kitties(&(sender, own_kitty_id)).ok_or("Account dosen't own the kitty!")?;
		Ok(kitty_id)
	}

	fn remove_own_kitty(owner: T::AccountId, own_kitty_id: T::KittyIndex) {
		let owner_key = &(owner.clone(), own_kitty_id);
		//let kitty_id = Self::owned_kitties(owner_key).ok_or("Account dosen't own the kitty!")?;

		let own_count = Self::owned_kitties_count(owner.clone());
		let owner_final_key = &(owner.clone(), own_count - 1.into());
		//let final_kitty_id = Self::owned_kitties(owner_final_key).ok_or("Account final kitty is missing!")?;
		if let Some(final_kitty_id) = Self::owned_kitties(owner_final_key) {
			<OwnedKitties<T>>::remove(owner_key);
			<OwnedKitties<T>>::remove(owner_final_key);
			if (own_count - 1.into()) > own_kitty_id {
				//move last record to the removed position
				<OwnedKitties<T>>::insert(owner_key, final_kitty_id);
				<OwnedKittiesCount<T>>::insert(owner, own_count - 1.into());
			}
		}
	}

	fn insert_own_kitty(owner: T::AccountId, own_kitty_id: T::KittyIndex, kitty_id: T::KittyIndex) {
		<OwnedKitties<T>>::insert((owner.clone(), own_kitty_id), kitty_id);
		<OwnedKittiesCount<T>>::insert(owner, own_kitty_id + 1.into());
	}

	fn transfer_kitty(sender: T::AccountId, own_kitty_id: T::KittyIndex, receiver: T::AccountId) -> Result {
		let kitty_id = Self::search_own_kitty(sender.clone(), own_kitty_id)?;

		//let send_kitty = Self::kitty(kitty_id).ok_or("Invalid kitty_id")?;
		//let mut owner_map = Self::owned_kitties_map(&sender).ok_or("Account dosen't have a kitty!")?;

		let kitty_count = Self::owned_kitties_count(&receiver);
		if kitty_count == T::KittyIndex::max_value() {
			return Err("Account kitties count overflow");
		}

		//remove
		Self::remove_own_kitty(sender, own_kitty_id);
		//insert
		Self::insert_own_kitty(receiver, kitty_count, kitty_id);

		Ok(())
	}
}
