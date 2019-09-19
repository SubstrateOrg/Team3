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
        KittyCount get(getKittyCount): map T::AccountId => u32;
        MyKitties get(getKitties): map (T::AccountId, u32) => Kitty;
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
            let my_zero_balance: T::Balance = 0u8;

            let current_kitty_count = match <KittyCount<T>>::get(sender);
            let new_kitty_count = current_kitty_count.checked_add(1).ok_or("Overflow adding kitty count")?;

            let new_kitty = Kitty {
                id: hash_of_zero,
                dna: hash_of_zero,
                price: my_zero_balance,
                gen: current_kitty_count,
            };

            <MyKitties<T>>::insert(sender, new_kitty);
            <KittyCount<T>>::insert(sender, new_kitty_count);

            Ok(())
        }

        fn removeKitty(origin, gen: u32) -> Result {
            let sender = ensure_signed(origin)?;

            <MyKitties<T>>::remove(sender);

            Ok(())
        }
    }
}