/// A runtime module avalon with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap,dispatch::Result};
use system::ensure_signed;

/// The module's configuration trait.
pub trait Trait: system::Trait {
	// TODO: Add other types and constants required configure this module.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}
//pub trait Trait: balances::Trait {};

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as AvalonModule {
		Something get(something): Option<u32>;
		AvU32: u32;
        AvBool get(my_bool_getter): bool;
		//pub ReservedBalance get(reserved_balance): map T::AccountId => T::Balance;
		AccValue get(acc_bal): map T::AccountId => u32;
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
		//avalon fn 1
		pub fn do_avbool(origin, input_bool: bool){
			let who = ensure_signed(origin)?;
			 <AvBool>::put(input_bool);
			 Self::deposit_event(RawEvent::SomeBoolStored(input_bool, who));
		}
		//avalon fn 2
		pub fn do_avu32(origin, input_u32: u32){
			let who = ensure_signed(origin)?;
			 <AvU32>::put(input_u32);
			 Self::deposit_event(RawEvent::SomeU32Stored(input_u32, who));
		}
		//avalon fn 3
		pub fn do_mapu32(origin,input_value: u32){
			let who = ensure_signed(origin)?;
			 <AccValue<T>>::insert(who,input_value);
		}
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		// Just a dummy event.
		// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		// To emit this event, we call the deposit funtion, from our runtime funtions
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
