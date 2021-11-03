use alloc::vec::Vec;
use rlp_derive::{RlpEncodable, RlpDecodable};
use ethereum_types::{H160, H256};
use scale_info::TypeInfo;

#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable, TypeInfo)]
#[cfg_attr(feature = "codec", derive(codec::Encode, codec::Decode))]
pub struct Log {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}
