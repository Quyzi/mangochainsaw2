use crate::internal::*;
use bytes::Bytes;
use sled::IVec;

pub trait Object: Clone {
    type Result<T>;
    type Item: Into<Bytes>;

    fn new(i: Self::Item) -> Result<Self>
    where
        Self: Sized;

    fn to_ivec(&self) -> Self::Result<IVec>;
    fn from_ivec(i: IVec) -> Self::Result<Self>
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct DefaultObject(pub(crate) Bytes);

impl Object for DefaultObject {
    type Result<T> = Result<T>;
    type Item = Bytes;

    fn new(i: Self::Item) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self(i))
    }

    fn to_ivec(&self) -> Self::Result<IVec> {
        Ok(self.0.to_vec().into())
    }

    fn from_ivec(i: IVec) -> Self::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self(Bytes::from(i.to_vec())))
    }
}
