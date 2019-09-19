use support::{decl_storage, decl_module,StorageValue, StorageMap, dispatch::Result,ensure,decl_event};
use system::ensure_signed;
use sr_primitives::traits::{Hash,Zero};
use codec::{Codec,Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: Hash,
    price: Balance,
    gen: u64,
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
        Created(AccountId, Hash),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
        KittyOwner get(owner_of_kitty): map T::Hash => Option<T::AccountId>;
        OwnedKitty get(kitty_of_owner): map T::AccountId => T::Hash;
        Nonce : u64;
        AllKittiesArray get(kitties_array): map u64=> T::Hash;
        AllKittiesCount get(kitties_count): u64 ;
        AllKittiesIndex : map T::Hash => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let all_kitties_count = AllKittiesCount::get();
            let new_all_kitties_count = all_kitties_count.checked_add(1).ok_or("Overflow adding a new person")?;

            let nonce = <Nonce>::get();
            let random_seed = <system::Module<T>>::random_seed();
            let new_id = (random_seed, &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!<Kitties<T>>::exists(new_id), "This new id already exists");

            let new_kitty = Kitty {
                id: new_id,
                dna: new_id,
                price: Zero::zero(),
                gen: 0,
            };

            <Kitties<T>>::insert(&new_id, new_kitty);
            <KittyOwner<T>>::insert(&new_id, &sender);

            <AllKittiesArray<T>>::insert(all_kitties_count,&new_id);
            <AllKittiesCount>::put(new_all_kitties_count);
            <AllKittiesIndex<T>>::insert(&new_id,all_kitties_count);

            <OwnedKitty<T>>::insert(&sender, new_id);

            <Nonce>::mutate(|n| *n += 1);
            Self::deposit_event(RawEvent::Created(sender, new_id));

            Ok(())
        }
    }
}
