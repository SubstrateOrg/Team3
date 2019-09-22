use support::{decl_storage, decl_module, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use sr_primitives::traits::{Hash};
use codec::{Encode, Decode};

//Kitty data structure
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
	dna: Hash,
    price: Balance,
    gen: u64,
}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        TestValue get(getTestValue): u64;
        MyValue: map T::AccountId => u64;
        KittyCount: map T::AccountId => u64;
        MyKitties get(getKitties): map (T::AccountId, u64) => Option<Kitty<T::Hash, T::Balance>>;

        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn setValue(origin, value: u64) -> Result {
            let sender = ensure_signed(origin)?;

            <TestValue>::put(value);
            <MyValue<T>>::insert(sender, value);

            Ok(())
        }

        fn createKitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let hash_of_zero = <T as system::Trait>::Hashing::hash_of(&0);
            let my_zero_balance = T::Balance::from(0u32);

            let new_kitty = Kitty {
                id: hash_of_zero,
                dna: hash_of_zero,
                price: my_zero_balance,
                gen: 0,
            };

            //because of owner issue, the idea of (accout, id) is not finished yet
            <MyKitties<T>>::insert((sender, 0), new_kitty);

            //use Nonce as random seed?
            //Nonce

            //<MyKitties<T>>::insert((sender, new_kitty_count), new_kitty);
            //<KittyCount<T>>::insert(sender, new_kitty_count);

            Ok(())
        }

        fn removeKitty(origin, gen: u64) -> Result {

            Ok(())
        }
    }
}