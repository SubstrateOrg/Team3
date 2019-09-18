/// A runtime module avalon with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap,dispatch::Result,ensure};
use system::ensure_signed;
use codec::{Codec, Encode, Decode};
use sr_primitives::traits::{Hash, Zero};

// NOTE: We have added this struct template for you
#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Gundam<Hash,Balance> {
	id: Hash,
    dna: Hash,
	price: Balance,
    gen: u64,
}
/*
128 bit DNA
[0] [1] [2] ... [15] bytes from random hash
[0] = stars  [Value mod 5] + 1  from 1 to 5
# level
# want to make  5 starts harder to get then 1
[1] = type   [Value mod 3] + 1  
# 1: Sword, 2: Sniper, 3: Speed 
# Speed weak sniper, Sniper weak Sword, Sword weak Speed. 
# ref 'weak' in FGO, not resist
# how about a 'duel function' for two gundams fight, loser deleted
Gundam1 adn 2 generate new one
hash = get_new_hash()
new[0] = (Gundam1 [0] + Gundam2[0])/2;
new[0] = new[0] + providence(); //providence in [-1 , 0 , 1]
if(new[0]) > 5 new[0] = 5
if(new[0]) < 1 new[0] = 1
if(Gundam1[1] == Gundam2[1])
new[1] = Gundam1[1];
else 
new[1] = [Value mod 3] + 1;
*/
pub trait Trait: balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

/// The module's configuration trait.
// pub trait Trait: system::Trait {
// 	// TODO: Add other types and constants required configure this module.

// 	/// The overarching event type.
// 	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
// }
//pub trait Trait: balances::Trait {};

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as AvalonModule {
		Something get(something): Option<u32>;
		AvU32 get(avu32):Option<u32>;
        AvBool get(my_bool_getter): Option<bool>;
		//pub ReservedBalance get(reserved_balance): map T::AccountId => T::Balance;
		AccValue get(acc_bal): map T::AccountId => Option<u32>;
		//
		Gundams get(gundam): map T::Hash => Gundam<T::Hash, T::Balance>;
		GundamOwner get(owner_of): map T::Hash => Option<T::AccountId>;

        AllGundamsArray get(gundam_by_index): map u64 => T::Hash;
        AllGundamsCount get(all_gundams_count): u64;
        AllGundamsIndex: map T::Hash => u64;

		OwnedGundam get(gundam_of_owner): map T::AccountId => T::Hash;
		Nonce: u64;
	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event() = default;

		// Just a dummy entry point.
		// function that can be called by the external world as an extrinsics call
		// takes a parameter of the type `AccountId`, stores it and emits an event
		pub fn do_something(origin, something: u32) -> Result {
			// TODO: You only need this if you want to check it was signed.
			let who = ensure_signed(origin)?;

			// TODO: Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// here we are raising the Something event
			Self::deposit_event(RawEvent::SomeU32Stored(something, who));
			Ok(())
		}
		//avalon test fn 1
		pub fn do_avbool(origin, input_bool: bool) -> Result {
			let who = ensure_signed(origin)?;
			 AvBool::put(input_bool);
			 Self::deposit_event(RawEvent::SomeBoolStored(input_bool, who));
			 Ok(())
		}
		//avalon test fn 2
		pub fn do_avu32(origin, input_u32: u32) -> Result {
			let who = ensure_signed(origin)?;
			 AvU32::put(input_u32);
			 Self::deposit_event(RawEvent::SomeU32Stored(input_u32, who));
			 Ok(())
		}
		//avalon test fn 3
		pub fn do_mapu32(origin,input_value: u32) -> Result {
			let who = ensure_signed(origin)?;
			 <AccValue<T>>::insert(who,input_value);
			 Ok(())
		}
		//avalon fn 1
		pub fn create_gundam(origin) -> Result {
            let sender = ensure_signed(origin)?;
			let all_gundams_count = Self::all_gundams_count();
			let new_all_gundams_count = all_gundams_count.checked_add(1)
                .ok_or("Overflow adding a new gundam to total supply")?;

            let nonce = Nonce::get();
            let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
                .using_encoded(<T as system::Trait>::Hashing::hash);

            ensure!(!<GundamOwner<T>>::exists(random_hash), "Gundam already exists");

            let zaku = Gundam {
                id: random_hash,
                dna: random_hash,
				price: Zero::zero(),
				gen:0
            };

			<Gundams<T>>::insert(random_hash, zaku);
			<GundamOwner<T>>::insert(random_hash, &sender);

            <AllGundamsArray<T>>::insert(all_gundams_count, random_hash);
            AllGundamsCount::put(new_all_gundams_count);
            <AllGundamsIndex<T>>::insert(random_hash, all_gundams_count);

            <OwnedGundam<T>>::insert(&sender, random_hash);

            Nonce::mutate(|n| *n += 1);
			Self::deposit_event(RawEvent::Created(sender, random_hash));
            Ok(())
        }
	}
}

/*
 pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash
    {
        Created(AccountId, Hash),
    }
*/

decl_event!(
	pub enum Event<T> 
		where 
			AccountId = <T as system::Trait>::AccountId,
			<T as system::Trait>::Hash
		{
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
		Created(AccountId, Hash),
		SomeBoolStored(bool, AccountId),
		SomeU32Stored(u32, AccountId),
	}
);

// /// tests for this module
// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	use runtime_io::with_externalities;
// 	use primitives::{H256, Blake2Hasher};
// 	use support::{impl_outer_origin, assert_ok, parameter_types};
// 	use sr_primitives::{traits::{BlakeTwo256, IdentityLookup}, testing::Header};
// 	use sr_primitives::weights::Weight;
// 	use sr_primitives::Perbill;

// 	impl_outer_origin! {
// 		pub enum Origin for Test {}
// 	}

// 	// For testing the module, we construct most of a mock runtime. This means
// 	// first constructing a configuration type (`Test`) which `impl`s each of the
// 	// configuration traits of modules we want to use.
// 	#[derive(Clone, Eq, PartialEq)]
// 	pub struct Test;
// 	parameter_types! {
// 		pub const BlockHashCount: u64 = 250;
// 		pub const MaximumBlockWeight: Weight = 1024;
// 		pub const MaximumBlockLength: u32 = 2 * 1024;
// 		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
// 	}
// 	impl system::Trait for Test {
// 		type Origin = Origin;
// 		type Call = ();
// 		type Index = u64;
// 		type BlockNumber = u64;
// 		type Hash = H256;
// 		type Hashing = BlakeTwo256;
// 		type AccountId = u64;
// 		type Lookup = IdentityLookup<Self::AccountId>;
// 		type Header = Header;
// 		type WeightMultiplierUpdate = ();
// 		type Event = ();
// 		type BlockHashCount = BlockHashCount;
// 		type MaximumBlockWeight = MaximumBlockWeight;
// 		type MaximumBlockLength = MaximumBlockLength;
// 		type AvailableBlockRatio = AvailableBlockRatio;
// 		type Version = ();
// 	}
// 	impl Trait for Test {
// 		type Event = ();
// 	}
// 	type TemplateModule = Module<Test>;

// 	// This function basically just builds a genesis storage key/value store according to
// 	// our desired mockup.
// 	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
// 		system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
// 	}

// 	#[test]
// 	fn it_works_for_default_value() {
// 		with_externalities(&mut new_test_ext(), || {
// 			// Just a dummy test for the dummy funtion `do_something`
// 			// calling the `do_something` function with a value 42
// 			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 			// asserting that the stored value is equal to what we stored
// 			assert_eq!(TemplateModule::something(), Some(42));
// 		});
// 	}
// }
