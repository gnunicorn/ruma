use bytes::BufMut;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::IntoHttpError;

/// TODO: DOCS
#[allow(clippy::exhaustive_structs)]
#[derive(Default)]
pub struct RawHttpBody(pub Vec<u8>);

/// Types that can be converted to the raw bytes of an http request or response body.
pub trait IntoHttpBody {
    /// TODO: DOCS
    fn into_buf<B>(self) -> Result<B, IntoHttpError>
    where
        B: Default + BufMut;

    // FIXME: To support alternative transports, the user should be allowed to pass a serialization
    // function similar to a `for<T: Serialize> FnOnce(&mut buf::Writer<B>, T) -> Result<B, E>`
    // where `E: Into<IntoHttpError>`. To make this work without higher-ranked trait bounds which
    // are only implemented for lifetimes so far, a separate trait will have to be involved.
}

impl<T: Serialize> IntoHttpBody for T {
    fn into_buf<B>(self) -> Result<B, IntoHttpError>
    where
        B: Default + BufMut,
    {
        let mut buf = B::default().writer();
        serde_json::to_writer(&mut buf, &self)?;
        Ok(buf.into_inner())
    }
}

impl IntoHttpBody for RawHttpBody {
    fn into_buf<B>(self) -> Result<B, IntoHttpError>
    where
        B: Default + BufMut,
    {
        let mut buf = B::default();
        buf.put_slice(&self.0);
        Ok(buf)
    }
}

/// Types that can be converted from the raw bytes of an http request or response body.
pub trait FromHttpBody<Error>: Sized {
    /// TODO: DOCS
    fn from_buf(body: &[u8]) -> Result<Self, Error>;
}

impl<T: DeserializeOwned, Error> FromHttpBody<Error> for T
where
    Error: From<serde_json::Error>,
{
    fn from_buf(body: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(body)?)
    }
}

impl<Error> FromHttpBody<Error> for RawHttpBody {
    fn from_buf(body: &[u8]) -> Result<Self, Error> {
        Ok(Self(body.to_owned()))
    }
}
