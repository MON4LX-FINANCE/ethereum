use crate::Bytes;
use scale_info::TypeInfo;
use alloc::vec::Vec;
use scale_info::TypeInfo;
use ethereum_types::{H160, H256};
use rlp_derive::{RlpDecodable, RlpEncodable};


#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable, TypeInfo)]
#[cfg_attr(
	feature = "with-codec",
	derive(codec::Encode, codec::Decode)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Log {
	pub address: H160,
	pub topics: Vec<H256>,
	pub data: Bytes,
}
