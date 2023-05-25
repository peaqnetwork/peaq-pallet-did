use std::convert::From;
use std::sync::Arc;

use codec::Codec;
//use codec::{Decode, Encode};
use jsonrpsee::{
    core::{async_trait, Error as JsonRpseeError, RpcResult},
    proc_macros::rpc,
    types::error::{CallError, ErrorObject},
};
use peaq_pallet_did::structs::Attribute;
pub use peaq_pallet_did_runtime_api::PeaqDIDApi as PeaqDIDRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

/*
#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
pub struct RPCAttribute<BlockNumber, Moment> {
    pub name: Bytes,
    pub value: Bytes,
    pub validity: BlockNumber,
    pub created: Moment,
}

impl<BlockNumber, Moment> From<Attribute<BlockNumber, Moment>>
    for RPCAttribute<BlockNumber, Moment>
{
    fn from(item: Attribute<BlockNumber, Moment>) -> Self {
        RPCAttribute {
            name: item.name.into(),
            value: item.value.into(),
            validity: item.validity,
            created: item.created,
        }
    }
}
*/

#[rpc(client, server)]
pub trait PeaqDIDApi<BlockHash, AccountId, BlockNumber, Moment> {
    #[method(name = "peaqdid_readAttribute")]
    fn read_attribute(
        &self,
        did_account: AccountId,
        name: Bytes,
        at: Option<BlockHash>,
    ) -> RpcResult<Option<Attribute<BlockNumber, Moment>>>;
}

/// A struct that implements the [`PeaqDIDApi`].
pub struct PeaqDID<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> PeaqDID<C, B> {
    /// Create new `PeaqDID` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        PeaqDID {
            client,
            _marker: Default::default(),
        }
    }
}

pub enum Error {
    RuntimeError,
}

impl From<Error> for i32 {
    fn from(e: Error) -> i32 {
        match e {
            Error::RuntimeError => 1,
        }
    }
}

#[async_trait]
impl<C, Block, AccountId, BlockNumber, Moment>
    PeaqDIDApiServer<<Block as BlockT>::Hash, AccountId, BlockNumber, Moment> for PeaqDID<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: PeaqDIDRuntimeApi<Block, AccountId, BlockNumber, Moment>,
    AccountId: Codec,
    BlockNumber: Codec,
    Moment: Codec,
{
    fn read_attribute(
        &self,
        did_account: AccountId,
        name: Bytes,
        at: Option<<Block as BlockT>::Hash>,
    ) -> RpcResult<Option<Attribute<BlockNumber, Moment>>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or(
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash,
        ));
        api.read(&at, did_account, name.to_vec())
            .map(|o| o.map(Attribute::from))
            .map_err(|e| {
                JsonRpseeError::Call(CallError::Custom(ErrorObject::owned(
                    Error::RuntimeError.into(),
                    "Unable to get value.",
                    Some(format!("{:?}", e)),
                )))
            })
    }
}



