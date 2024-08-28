use crate::{Config, MomentTime};
use crate::{MAX_NAME_SIZE, MAX_VALUE_SIZE};
use parity_scale_codec::{Decode, Encode};
use frame_support::pallet_prelude::ConstU32;
use frame_support::pallet_prelude::MaxEncodedLen;
use frame_support::BoundedVec;
use scale_info::TypeInfo;
use sp_core::RuntimeDebug;
use frame_system::pallet_prelude::BlockNumberFor;

pub type AttributeOf<T> = Attribute<
    BlockNumberFor<T>,
    <<T as Config>::Time as MomentTime>::Moment,
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
pub struct Attribute<BlockNumber, Moment> {
    pub name: BoundedVec<u8, ConstU32<{ MAX_NAME_SIZE as u32 }>>,
    pub value: BoundedVec<u8, ConstU32<{ MAX_VALUE_SIZE as u32 }>>,
    pub validity: BlockNumber,
    pub created: Moment,
}
