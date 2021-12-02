pub mod ethereum;
pub use ethereum::{Ethereum, EthereumMiddleware};

use ethers::{
    core::types::transaction::eip2930::AccessList,
    prelude::{Address, Bytes, NameOrAddress, Signature, H256, U256, U64},
};

use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Default)]
pub struct Eip1559Fees {
    pub max_fee_per_gas: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
}

pub trait Txn: Default + serde::Serialize + Debug + Clone + Send + Sync {
    fn recommend_1559(&self) -> bool {
        false
    }

    fn get_1559_fees(&self) -> Eip1559Fees;
    fn set_1559_fees(&mut self, fees: &Eip1559Fees);

    fn from(&self) -> Option<&Address>;
    fn set_from(&mut self, from: Address);
    fn to(&self) -> Option<&NameOrAddress>;
    fn set_to<T: Into<NameOrAddress>>(&mut self, to: T);
    fn nonce(&self) -> Option<&U256>;
    fn set_nonce<T: Into<U256>>(&mut self, nonce: T);
    fn value(&self) -> Option<&U256>;
    fn set_value<T: Into<U256>>(&mut self, value: T);
    fn gas(&self) -> Option<&U256>;
    fn set_gas<T: Into<U256>>(&mut self, gas: T);
    fn gas_price(&self) -> Option<U256>;
    fn set_gas_price<T: Into<U256>>(&mut self, gas_price: T);
    fn data(&self) -> Option<&Bytes>;
    fn access_list(&self) -> Option<&AccessList>;
    fn set_access_list(&mut self, access_list: AccessList);
    fn set_data(&mut self, data: Bytes);
    fn rlp_signed<T: Into<U64>>(&self, chain_id: T, signature: &Signature) -> Bytes;
    fn rlp<T: Into<U64>>(&self, chain_id: T) -> Bytes;
    fn sighash<T: Into<U64>>(&self, chain_id: T) -> H256;
}

pub trait Network: Send + Sync + Debug {
    type TransactionRequest: Txn;
}
