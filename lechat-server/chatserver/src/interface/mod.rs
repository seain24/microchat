use anyhow::Context;

use crate::base::response::Result;

pub mod user;

pub trait Validate {
    fn validate(&mut self) -> Result<()>;
}

#[allow(unused)]
#[inline]
fn deserialize<M: protobuf::Message>(src: &[u8]) -> anyhow::Result<M> {
    M::parse_from_bytes(src).context("deserialized data failed")
}
