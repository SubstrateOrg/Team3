use support::{decl_module, decl_storage, StorageValue, StorageMap,ensure};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;
use byteorder::{LittleEndian};

pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Default)]
pub struct Kitty{
	dna: u128,
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id / index
		pub Kitties get(kitty): map u32 => Kitty;
		/// Stores the total number of kitties. i.e. the next kitty index
		pub KittiesCount get(kitties_count): u32;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		/// Create a new kitty
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let count = Self::kitties_count();
			let new_count = count.checked_add(1).ok_or("Kitties count overflow")?;
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let  dna = payload.using_encoded(blake2_128);
			let final_dna=LittleEndian::read_u128(&dna);
			let kitty = Kitty{
				dna: final_dna,
			};
			Kitties::insert(count, kitty);
			KittiesCount::put(new_count);
		}

		/// Breed a new kitty
		pub fn breed(origin,kitty_id_1: u32, kitty_id_2: u32) {
			let sender = ensure_signed(origin)?;

			ensure!(Kitties::exists(kitty_id_1), "This cat 1 does not exist");
            ensure!(Kitties::exists(kitty_id_2), "This cat 2 does not exist");

			let kitty_1 = Self::kitty(kitty_id_1);
            let kitty_2 = Self::kitty(kitty_id_2);

					
			//generate new dna
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let random_hash = payload.using_encoded(blake2_128);

            let kitty_1 = Self::kitty(kitty_id_1);
            let kitty_2 = Self::kitty(kitty_id_2);

            let mut final_dna = kitty_1.dna/2+kitty_2.dna/2;
            

			ensure!(!Kitties::exists(final_dna), "Kitty already exists");
			
			let count = Self::kitties_count();
			let new_count = count.checked_add(1).ok_or("Kitties count overflow")?;
	
			let kitty = Kitty{
				dna: final_dna,
			};
			Kitties::insert(count, kitty);
			KittiesCount::put(new_count);
		}
	}
}
