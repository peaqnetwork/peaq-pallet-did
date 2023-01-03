#![cfg_attr(not(feature = "std"), no_std)]
// The `too_many_arguments` warning originates from `decl_runtime_apis` macro.
#![allow(clippy::too_many_arguments)]
// The `unnecessary_mut_passed` warning originates from `decl_runtime_apis` macro.
#![allow(clippy::unnecessary_mut_passed)]

use codec::Codec;
use peaq_pallet_did::structs::Attribute;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
    pub trait PeaqDIDApi<AccountId, BlockNumber, Moment> where
        AccountId: Codec,
        BlockNumber: Codec,
        Moment: Codec,
        {
            fn read(did_account: AccountId, name: Vec<u8>) -> Option<Attribute<BlockNumber, Moment>>;
        }
}
