use bytes::Bytes;
use std::path::Path;

use crate::{internal::*, label::Label, object::Object};

pub trait StorageBackend {
    type Backend;
    type Result<T>;
    type Error: std::error::Error;
    type Item: Object;
    type MetadataItem: Label;
    type Ident: TryFrom<Bytes> + TryInto<Bytes>;

    fn open(path: &Path) -> Self::Result<Self>
    where
        Self: Sized;
    fn sync(&self) -> Self::Result<()>;

    fn new_ident(&self, item: &Self::Item) -> Result<Self::Ident>;
    fn get(&self, ident: Self::Ident) -> Result<Option<Self::Item>>;
    fn put(&self, item: Self::Item, metadata: Vec<Self::MetadataItem>) -> Result<Self::Ident>;
    fn delete(&self, ident: Self::Ident) -> Result<Option<Self::Item>>;
}

pub trait StorageBackendSearch: StorageBackend {
    fn find_by_metadata(
        &self,
        metadata: Vec<Self::MetadataItem>,
    ) -> Self::Result<Option<Vec<Self::Ident>>>;
}
