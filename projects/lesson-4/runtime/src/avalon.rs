/// A runtime module avalon with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs

use support::{decl_module, decl_storage, StorageValue, ensure,StorageMap,dispatch::Result,Parameter};
use system::ensure_signed;
use codec::{ Encode, Decode};
use sr_primitives::traits::{SimpleArithmetic, Bounded};
use runtime_io::{blake2_128,print};
use rstd::result;

// NOTE: We have added this struct template for you
#[derive(Encode, Decode)]
pub struct Gundam(pub [u8; 16]);

pub trait Trait: system::Trait {
	type GundamNumber: Parameter + SimpleArithmetic + Bounded + Default + Copy;
}

// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as AvalonModule {
		/// Store all gundams, key is index
		pub Gundams get(gundam): map T::GundamNumber => Option<Gundam>;
		/// Store gundam count
		pub GundamsCount get(gundams_count): T::GundamNumber;

		///Get gundam ID by account ID and index
		pub OwnedGundams get(owned_gundam): map (T::AccountId, T::GundamNumber) => T::GundamNumber;
		///Get number of gundam per account
		pub OwnedGundamCount get(owned_gundam_count): map T::AccountId => T::GundamNumber;

	}
}

// The module's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		///Create new
		pub fn create(origin){
			print("enter  create");
            let sender = ensure_signed(origin)?;
			Self::do_create(sender)?;
        }
		//Combine to getnew
		pub fn breed(origin,gundam_id_1:T::GundamNumber,gundam_id_2:T::GundamNumber){
			print("enter  breed");
			let sender = ensure_signed(origin)?;
			Self::do_breed(sender,gundam_id_1,gundam_id_2)?;
		}
	}
}

impl<T: Trait> Module<T> {

	fn next_id()->result::Result<T::GundamNumber,&'static str>{
		let new_gundam_index = Self::gundams_count();
		if new_gundam_index == T::GundamNumber::max_value(){
				return Err("count overflow");
		}
		Ok(new_gundam_index)
	}

	fn random_value(sender: &T::AccountId) -> [u8; 16] {
		let payload = (<system::Module<T>>::random_seed(), sender, <system::Module<T>>::extrinsic_index(), <system::Module<T>>::block_number());
		payload.using_encoded(blake2_128)
	}

	// 128 bit DNA
	// [0] [1] [2] ... [15] bytes from random hash
	// [0] = stars  [Value mod 5] + 1  from 1 to 5
	// # level
	// # want to make  5 starts harder to get then 1
	// [1] = type   [Value mod 3] + 1  
	// # 1: Sword, 2: Sniper, 3: Speed 
	// # Speed weak sniper, Sniper weak Sword, Sword weak Speed. 
	// # ref 'weak' in FGO, not resist
	// # how about a 'duel function' for two gundams fight, loser deleted
	// Gundam1 adn 2 generate new one
	// hash = get_new_hash()
	// new[0] = (Gundam1 [0] + Gundam2[0])/2;
	// if(Gundam1[1] == Gundam2[1])
	// new[1] = Gundam1[1];
	// else 
	// new[1] = [Value mod 3] + 1;
	fn gen_dna(rng:&[u8]) -> [u8;16]{
		let mut dna:[u8;16] = [0u8;16];
		let mut i = 0;
		while i<16 {
			dna[i] = rng[i];
			i = i + 1; 
		}
		dna[0] = rng[0]%5 + 1;
		dna[1] = rng[1]%3 + 1;

		dna
	}

	fn combine_dna(dna1:&[u8], dna2:&[u8], rng:&[u8]) -> [u8;16]{
		let mut dna:[u8;16] = [0u8;16];
		let mut i = 0;
		while i<16 {
			dna[i] = rng[i];
			i = i + 1; 
		}
		dna[0] = ( dna1[0] + dna2[0] )/2;
		if dna1[1] == dna2[1]{
			dna[1] = dna1[1];
		}
		else{
			dna[1] = rng[1]%3 + 1;
		}
		
		dna
	}
	fn check_and_save(owner:T::AccountId, gundam_id:T::GundamNumber, gundam:Gundam){
		<Gundams<T>>::insert(gundam_id,gundam);
		<GundamsCount<T>>::put(gundam_id+1.into());

		let user_gundams_id = Self::owned_gundam_count(owner.clone());
		<OwnedGundams<T>>::insert((owner.clone(),user_gundams_id),user_gundams_id);
		<OwnedGundamCount<T>>::insert(owner,user_gundams_id+1.into());
	}

	fn do_create(owner:T::AccountId)->Result{
		print("enter do create");
		let new_gundam_index = Self::next_id()?;
		print("next_id id");
		let rng = Self::random_value(&owner);
		let dna = Self::gen_dna(&rng);
		let gundam = Gundam(dna);
		Self::check_and_save(owner,new_gundam_index,gundam);
		Ok(())
	}

	fn do_breed(owner:T::AccountId, gundam_id_1:T::GundamNumber, gundam_id_2:T::GundamNumber) -> Result{
		print("enter do do_breed");
		let gundam1 = Self::gundam(gundam_id_1);
		let gundam2 = Self::gundam(gundam_id_2);
		ensure!(gundam1.is_some(),"Invalid gundam1");
		ensure!(gundam2.is_some(),"Invalid gundam2");

		let mut rng = Self::random_value(&owner);
		let dna = Self::combine_dna(&gundam1.unwrap().0, &gundam2.unwrap().0, &mut rng);

		let gundam_new = Gundam(dna);
		let new_index = Self::next_id()?;
		Self::check_and_save(owner,new_index,gundam_new);
		Ok(())
	}



}