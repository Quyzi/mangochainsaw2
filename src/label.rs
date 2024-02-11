use crate::internal::*;
use bytes::Bytes;

use sled::IVec;
use std::fmt::{Debug, Display};

/// Labels are the basic metadata describing an object.
///
/// Labels are `key=value` pairs.  
/// They should be human readable.
///
/// Objects should be inserted with multiple labels describing the object.
/// Labels are used for organization and indexing.
pub trait Label {
    type Result<T>;
    type Key: Into<String>;
    type Value: Into<String>;

    fn new(lhs: Self::Key, rhs: Self::Value) -> Self;

    fn parse_str(s: &str) -> Result<Self>
    where
        Self: Sized;
    fn parse_string(s: String) -> Result<Self>
    where
        Self: Sized,
    {
        Self::parse_str(&s)
    }

    fn from_bytes(bytes: Bytes) -> Result<Self>
    where
        Self: Sized;
    fn from_bytes_rtl(bytes: Bytes) -> Result<Self>
    where
        Self: Sized;

    fn to_bytes_ltr(&self) -> Result<Bytes>;
    fn to_ivec_ltr(&self) -> Result<IVec> {
        Ok(Self::to_bytes_ltr(self)?.to_vec().into())
    }
    fn to_bytes_rtl(&self) -> Result<Bytes>;
    fn to_ivec_rtl(&self) -> Result<IVec> {
        Ok(Self::to_bytes_rtl(self)?.to_vec().into())
    }
}

#[derive(Clone, Debug, Default)]
pub struct DefaultLabel(pub String, pub String);

impl Display for DefaultLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.0, self.1)
    }
}

impl Label for DefaultLabel {
    type Result<T> = Result<T>;
    type Key = String;
    type Value = String;

    fn new(lhs: Self::Key, rhs: Self::Value) -> Self {
        Self(lhs, rhs)
    }

    fn from_bytes(bytes: Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let this = String::from_utf8(bytes.to_vec())?;
        Self::parse_string(this)
    }

    fn from_bytes_rtl(bytes: Bytes) -> Result<Self>
    where
        Self: Sized,
    {
        let mut this = Self::from_bytes(bytes)?;
        std::mem::swap(&mut this.0, &mut this.1);
        Ok(this)
    }

    fn to_bytes_ltr(&self) -> Result<Bytes> {
        let buf = format!("{}={}", self.0, self.1);
        Ok(Bytes::from(buf.as_bytes().to_owned()))
    }

    fn to_bytes_rtl(&self) -> Result<Bytes> {
        let buf = format!("{}={}", self.1, self.0);
        Ok(Bytes::from(buf.as_bytes().to_owned()))
    }

    fn parse_str(s: &str) -> Result<Self> {
        if let Some(pos) = s.find('=') {
            let (lhs, rhs) = s.split_at(pos);
            let rhs = &rhs[1..];
            Ok(Self(lhs.to_string(), rhs.to_string()))
        } else {
            Err(Error::InvalidLabelString)
        }
    }
}
