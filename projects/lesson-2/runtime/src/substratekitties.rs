//reference: 
//          https://www.shawntabrizi.com/substrate-collectables-workshop/#/zh-cn/2/refactoring-our-code

use support::{
	decl_storage, 
	decl_module, 
	StorageValue, 
	StorageMap,
    dispatch::Result, 
	ensure, 
	decl_event
};
use system::ensure_signed;
use sr_primitives::{
  traits::{
    Zero, Hash
  }
};
use codec::{Encode, Decode};

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Kitty<Hash, Balance> {
    id: Hash,
    dna: u128,
    price: Balance,
    gen: u64,
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
    // kitties
    pub Kitties get(kitties): map T::Hash => Kitty<T::Hash, T::Balance>;
    pub KittyOwner get(owner_of): map T::Hash => Option<T::AccountId>;

    // kitties list
    AllKittiesArray get(kitty_by_index): map u64 => T::Hash;
    AllKittiesCount get(kitties_count): u64;
    AllKittiesIndex: map T::Hash => u64;

    // owners
    OwnedKittiesArray get(index_kitty_owner): map (T::AccountId, u64) => T::Hash;
    OwnedKittiesCount get(owned_kitties_count): map T::AccountId => u64;
    OwnedKittiesIndex: map T::Hash => u64;

    // Nonce
    Nonce: u128;
	}
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
        Created(AccountId, Hash),
    }
);

//function diapatch
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
    fn deposit_event() = default;

    fn create_kitty(origin) -> Result {
      let sender = ensure_signed(origin)?;

      //new one kitty
      let new_id = Self::generate_hash(&sender);
      let new_dna = Self::generate_dna_hashrandom(&new_id);
      let new_kitty = Self::create_zero_kitty(&new_id, new_dna);

      // mint
      Self::mint(sender, new_id, new_kitty)?;

      Ok(())
    }
	}
}


impl<T: Trait> Module<T> {
  fn generate_hash (sender: &T::AccountId) -> T::Hash {
    let random_seed = <system::Module<T>>::random_seed();
    let nonce = <Self as Store>::Nonce::get();

    (random_seed, sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash)
  }

  fn generate_dna_hashrandom<M: AsRef<[u8]>> (random_hash: &M) -> u128 {
    let rand = random_hash.as_ref();
    let mut flag = false;
    let mut ret: u128 = 0;
    for element in rand.iter() {
      if !flag {
        ret += *element as u128;
      } 
	  else {
        ret <<= 8;
      }
      flag = !flag;
    }
    ret
  }

  // create gen zero kitty
  fn create_zero_kitty (id: &T::Hash, dna_data: u128) -> Kitty<T::Hash, T::Balance> {
    Kitty {
      id: id.clone(),
      dna: dna_data,
      price: <T as balances::Trait>::Balance::zero(),
      gen: 0,
    }
  }

  // mint a new Kitty
  fn mint (kitty_owner: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
    ensure!(!<Kitties<T>>::exists(kitty_id), "This kitty id already exists");
    //index
    let curr_count_index = Self::kitties_count();
    let new_count = curr_count_index.checked_add(1).ok_or("Overflow adding a new kitty")?;

    let owner_count = Self::owned_kitties_count(&kitty_owner);
    let new_count = owner_count.checked_add(1).ok_or("Overflow adding a new kitty of owner")?;

    // insert kitties and owner
    <Kitties<T>>::insert(kitty_id, new_kitty);
    <KittyOwner<T>>::insert(kitty_id, &kitty_owner);
    // add kitty to list
    <AllKittiesArray<T>>::insert(curr_count_index, kitty_id);
    <AllKittiesIndex<T>>::insert(kitty_id, curr_count_index);
    <Self as Store>::AllKittiesCount::put(new_count);
    // insert kitty to owner
    <OwnedKittiesArray<T>>::insert((kitty_owner.clone(), owner_count), kitty_id);
    <OwnedKittiesCount<T>>::insert(&kitty_owner, new_count);
    <OwnedKittiesIndex<T>>::insert(kitty_id, owner_count);

    <Self as Store>::Nonce::mutate(|n| *n += 1);
    
    Self::deposit_event(RawEvent::Created(kitty_owner, kitty_id));

    Ok(())
  }
}