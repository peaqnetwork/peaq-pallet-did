use crate::MAX_NAME_SIZE;
use crate::{Config, MomentTime};
use codec::{Decode, Encode};
use frame_support::pallet_prelude::ConstU32;
use frame_support::pallet_prelude::Get;
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::BoundedVec;
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use sp_std::vec::Vec;

pub type AttributeOf<T> = Attribute<
    <T as frame_system::Config>::BlockNumber,
    <<T as Config>::Time as MomentTime>::Moment,
    BoundedVec<u8, <T as Config>::BoundedDataLen>,
>;
/// Attributes of a DID.
#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Encode,
    Decode,
    Default,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
)]
pub struct Attribute<BlockNumber, Moment, BoundedVecT> {
    pub name: BoundedVec<u8, ConstU32<{ MAX_NAME_SIZE as u32 }>>,
    pub value: BoundedVecT,
    pub validity: BlockNumber,
    pub created: Moment,
}
