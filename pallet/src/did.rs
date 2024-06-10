use crate::structs::*;
use frame_support::pallet_prelude::Get;

pub enum DidError {
    NotFound,
    AuthorizationFailed,
    NameExceedMaxChar,
    FailedCreate,
    FailedUpdate,
    AlreadyExist,
    MaxBlockNumberExceeded,
    NonAsciiProperty,
}

pub trait Did<AccountId, BlockNumber, Moment, BoundedVecT> {
    fn is_owner(owner: &AccountId, did_address: &AccountId, name: &[u8]) -> Result<(), DidError>;
    fn create(
        owner: &AccountId,
        did_address: &AccountId,
        name: &[u8],
        value: &[u8],
        valid_for: Option<BlockNumber>,
    ) -> Result<(), DidError>;
    fn update(
        owner: &AccountId,
        did_address: &AccountId,
        name: &[u8],
        value: &[u8],
        valid_for: Option<BlockNumber>,
    ) -> Result<(), DidError>;
    fn read(
        did_address: &AccountId,
        name: &[u8],
    ) -> Option<Attribute<BlockNumber, Moment, BoundedVecT>>;
    fn delete(owner: &AccountId, did_address: &AccountId, name: &[u8]) -> Result<(), DidError>;
    fn get_hashed_key_for_attr(did_account: &AccountId, name: &[u8]) -> [u8; 32];
    fn validate_block_number(valid_for: Option<BlockNumber>) -> Result<BlockNumber, DidError>;
}
