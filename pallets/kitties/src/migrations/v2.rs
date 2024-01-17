use frame_support::{
	pallet_prelude::*, storage::StoragePrefixedMap, traits::GetStorageVersion, weights::Weight,
};

use frame_support::{migration::storage_key_iter, Blake2_128Concat};
use frame_system::pallet_prelude::*;

use crate::*;

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct V0Kitty(pub [u8; 16]);

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct V1Kitty {
	pub dna: [u8; 16],
	pub name: [u8; 4],
}

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();
	if on_chain_version != 0 {
		return Weight::zero();
	}
	if current_version != 1 {
		return Weight::zero();
	}
	if on_chain_version == 0 {
		let module = Kitties::<T>::module_prefix();
		let item = Kitties::<T>::storage_prefix();

		for (index, kitty) in
			storage_key_iter::<KittyId, V0Kitty, Blake2_128Concat>(module, item).drain()
		{
			let new_kitty = Kitty { dna: kitty.0, name: *b"abcdxxxx" };
			Kitties::<T>::insert(index, &new_kitty);
		}
	}
	if on_chain_version == 1 {
		let module = Kitties::<T>::module_prefix();
		let item = Kitties::<T>::storage_prefix();
		for (index, kitty) in
			storage_key_iter::<KittyId, V1Kitty, Blake2_128Concat>(module, item).drain()
		{
			let mut result = [0; 8];
			result[..4].copy_from_slice(&kitty.name[..4]);
			let v2_kitty = Kitty { name: result, dna: kitty.dna };
			Kitties::<T>::insert(index, &v2_kitty);
		}
	}
	Weight::zero()
}
