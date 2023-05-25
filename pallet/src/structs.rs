use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Attributes of a DID.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo,
)]
pub struct Attribute<BlockNumber, Moment> {
    pub name: Vec<u8>,
    pub value: Vec<u8>,
    pub validity: BlockNumber,
    pub created: Moment,
}
