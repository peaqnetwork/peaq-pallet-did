use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// #[cfg(feature = "std")]
// use sp_rpc::number::NumberOrHex;


/// Attributes of a DID.
#[derive(
	PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
// #[cfg_attr(
// 	feature = "std",
// 	serde(
// 		rename_all = "camelCase",
// 		bound(serialize = "BlockNumber: Serialize, Moment: Serialize"),
// 		bound(deserialize = "BlockNumber: Deserialize<'de>, Moment: Deserialize<'de>")
// 	)
// )]
pub struct Attribute<BlockNumber, Moment> {
	pub name: Vec<u8>,
	pub value: Vec<u8>,
	pub validity: BlockNumber,
	pub created: Moment,
}
