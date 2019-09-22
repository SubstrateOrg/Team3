use support::{decl_storage, decl_module,StorageValue, StorageMap, dispatch::Result,ensure,decl_event};
use system::ensure_signed;
use sr_primitives::traits::{Hash,Zero};
use codec::{Encode, Decode};
use runtime_io::blake2_128;
use rstd::cmp;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: [u8; 16],
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
        OwnedKittiesArray get(kitty_of_owner_by_index): map (T::AccountId, u64) => T::Hash;
        OwnedKittiesCount get(kitty_of_owner_count): map T::AccountId => u64;
        OwnedKittiesIndex : map T::Hash => u64;
        Nonce : u64;
        AllKittiesArray get(kitties_array): map u64 => T::Hash;
        AllKittiesCount get(kitties_count): u64;
        AllKittiesIndex : map T::Hash => u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn deposit_event() = default;

        fn create_kitty(origin) -> Result {
            let sender = ensure_signed(origin)?;

            let nonce = <Nonce>::get();
            let random_seed = <system::Module<T>>::random_seed();
            let new_id = (random_seed, &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);

            let dna = (random_seed,
                       <system::Module<T>>::block_number(),
                       nonce,
                       sender.clone()).using_encoded(blake2_128);

            let new_kitty = Kitty {
                id: new_id,
                dna: dna,
                price: Zero::zero(),
                gen: 0,
            };

            Self::mint(sender, new_id, new_kitty)?;

            <Nonce>::mutate(|n| *n += 1);

            Ok(())
        }

        fn spawn_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> Result{
            let sender = ensure_signed(origin)?;

            ensure!(<Kitties<T>>::exists(kitty_id_1), "kitty_id_1 cat does not exist");
            ensure!(<Kitties<T>>::exists(kitty_id_2), "kitty_id_2 cat does not exist");

            let nonce = <Nonce>::get();
            let new_id = (<system::Module<T>>::random_seed(),&sender, nonce)
                    .using_encoded(<T as system::Trait>::Hashing::hash);

            let kitty_1 = Self::kitty(kitty_id_1);
            let kitty_2 = Self::kitty(kitty_id_2);

            let mut final_dna = kitty_1.dna;
            for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(new_id.as_ref().iter()).enumerate() {
                if r % 2 == 0 {
                    final_dna.as_mut()[i] = *dna_2_element;
                }
            }

            let new_kitty = Kitty {
                id: new_id,
                dna: final_dna,
                price: Zero::zero(),
                gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
            };

            Self::mint(sender, new_id, new_kitty)?;

            <Nonce>::mutate(|n| *n += 1);

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn mint(sender: T::AccountId, new_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {

        ensure!(!<Kitties<T>>::exists(new_id), "This new id already exists");

        let owned_kitty_count = Self::kitty_of_owner_count(&sender);
        let new_owned_kitty_count = owned_kitty_count.checked_add(1).ok_or("overflow adding a new kitty to account balance")?;

        let all_kitties_count = AllKittiesCount::get();
        let new_all_kitties_count = all_kitties_count.checked_add(1).ok_or("overflow adding a new kitty to total supply")?;

        <Kitties<T>>::insert(&new_id, new_kitty);
        <KittyOwner<T>>::insert(&new_id, &sender);

        <AllKittiesArray<T>>::insert(all_kitties_count,&new_id);
        <AllKittiesCount>::put(new_all_kitties_count);
        <AllKittiesIndex<T>>::insert(&new_id,all_kitties_count);

        <OwnedKittiesArray<T>>::insert((sender.clone(),owned_kitty_count), &new_id);
        <OwnedKittiesCount<T>>::insert(&sender, new_owned_kitty_count);
        <OwnedKittiesIndex<T>>::insert(&new_id, owned_kitty_count);

        Self::deposit_event(RawEvent::Created(sender, new_id));
        Ok(())
    }
}