use support::{decl_module, decl_storage, StorageValue, StorageMap};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use system::ensure_signed;

pub trait Trait: system::Trait {
}

#[derive(Encode, Decode, Default)]
pub struct Kitty(pub [u8; 16]);

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
			if count == u32::max_value() {
				return Err("Kitties count overflow");
			}
			let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
			let dna = payload.using_encoded(blake2_128);
			let kitty = Kitty(dna);
			Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);
		}

		pub fn create_child(origin, kitty1_id: [u8; 16], kitty2_id: [u8; 16]) {
		    let sender = ensure_signed(origin)?;

		    ensure!(Kitties::exists(kitty1_id), "kitty 1 不存在");
		    ensure!(Kitties::exists(kitty2_id), "kitty 2 不存在");

            let count = Self::kitties_count();

            ensure!(u32::max_value() <= count, "不能创建 kitty，数量超限")

		    let kitty_1 = Self::kitty(kitty_id_1);
            let kitty_2 = Self::kitty(kitty_id_2);

            let mut final_dna = kitty_1;

            let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
            let dna = payload.using_encoded(blake2_128);

            for (i, (dna_2_element, r)) in kitty_2.as_ref().iter().zip(dna.as_ref().iter()).enumerate() {
                if r % 2 == 0 {
                    final_dna.as_mut()[i] = *dna_2_element;
                }
            }

            let kitty = Kitty(final_dna);
            Kitties::insert(count, kitty);
			KittiesCount::put(count + 1);

			Ok(())
		}
	}
}
