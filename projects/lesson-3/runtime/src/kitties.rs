use support::{decl_module, decl_storage, StorageValue, StorageMap, dispatch::Result};
use codec::{Encode, Decode, alloc::vec::Vec};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(get_kitty): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;
		/// Stores all the ownerships of kitties, key is the kitty id
		pub KittyOwner get(get_owner): map u32 => T::AccountId;

		/// Stores the count of kitties of one specific account
		pub OwnCount get(own_count): map T::AccountId => u32;
		/// Stores kitty id of specific index owned by specific accout
		pub OwnKitty get(own_kitty): map (T::AccountId, u32) => u32;
		pub Nonce get(get_nonce): u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) -> Result {
			let sender = ensure_signed(origin)?;

			//kitty check
			let count = Self::kitties_count();
			let kitty_id = count;
			let new_count = count.checked_add(1).ok_or("Kitty count overflow")?;

			let payload = (<system::Module<T>>::random_seed(), sender.clone(), <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);

			//user check
			let new_user = !<OwnCount<T>>::exists(&sender);
			let mut own_id = 0;
			if !new_user {
				own_id = <OwnCount<T>>::get(&sender);
				own_id = own_id.checked_add(1).ok_or("Own count overflow")?;
			}

			//write
			if new_user {
				<OwnCount<T>>::insert(&sender, 0);
			} else {
				<OwnCount<T>>::mutate(&sender, |n| *n+=1);
			}
			<OwnKitty<T>>::insert((sender.clone(), own_id), kitty_id);

			Kitties::insert(kitty_id, kitty);
			KittiesCount::put(new_count);
			<KittyOwner<T>>::insert(kitty_id, sender.clone());

			Ok(())
		}

		/// generate a new kitty from our own kitties
		pub fn gen_new(origin, own_id_1: u32, own_id_2: u32) -> Result {
			let sender = ensure_signed(origin)?;

			if own_id_2 == own_id_1 {
				return Err("Kitty must not be equal");
			}

			if !<OwnKitty<T>>::exists(&(sender.clone(), own_id_1)) {
				return Err("Kitty1 not found");
			}

			if !<OwnKitty<T>>::exists(&(sender.clone(), own_id_2)) {
				return Err("Kitty2 not found");
			}

			let kitty_id_1 = <OwnKitty<T>>::get(&(sender.clone(), own_id_1));
			let kitty_id_2 = <OwnKitty<T>>::get(&(sender.clone(), own_id_2));
			let kitty_1 = Kitties::get(kitty_id_1);
			let kitty_2 = Kitties::get(kitty_id_2);

			let mut own_id = <OwnCount<T>>::get(&sender);
			own_id = own_id.checked_add(1).ok_or("Own count overflow")?;

			let mut count = Self::kitties_count();
			let kitty_id = count;
			count = count.checked_add(1).ok_or("Kitty count overflow")?;

			let new_kitty = Self::next_gen(&sender, &kitty_1, &kitty_2);

			//write
			let nonce = Self::get_nonce();
			Nonce::put(nonce.wrapping_add(1));

			<OwnCount<T>>::mutate(&sender, |n| *n+=1);
			<OwnKitty<T>>::insert((sender.clone(), own_id), kitty_id);

			Kitties::insert(kitty_id, new_kitty);
			KittiesCount::put(count);
			<KittyOwner<T>>::insert(kitty_id, sender.clone());

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	fn next_gen(owner: &T::AccountId, kitty1: &Kitty, kitty2: &Kitty) -> Kitty {
		//let new_kitty = Kitty(kitty1.0);
		let mut new_dna = [0; 16];

		let nonce = Self::get_nonce();
		let payload = (<system::Module<T>>::random_seed(), owner.clone(), <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number(), nonce);
		let gen_mutation = payload.using_encoded(blake2_128);

		for (idx, gen_byte) in gen_mutation.iter().enumerate() {
			if (gen_byte % 2) == 0 {
				new_dna[idx] = kitty1.0[idx];
			} else {
				new_dna[idx] = kitty2.0[idx];
			}
		}

		Kitty(new_dna)
	}
}