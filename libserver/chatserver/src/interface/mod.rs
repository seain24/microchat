use crate::base::response::{ChatResult, Error};

pub mod api;
pub mod user;

pub trait Validate {
    fn validate(&mut self) -> ChatResult<()>;
}

#[inline]
fn deserialize<M: protobuf::Message>(src: &[u8]) -> ChatResult<M> {
    M::parse_from_bytes(src).map_err(|e| Error::param_invalid(&e.to_string()))
}
