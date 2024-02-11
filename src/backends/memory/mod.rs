use bytes::Bytes;
use std::fmt::Display;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::{cell::RefCell, collections::HashMap};

use crate::internal::*;
use crate::storage::StorageBackendSearch;
use crate::{label::DefaultLabel, object::DefaultObject, storage::StorageBackend};

pub type Item = DefaultObject;
pub type MetadataItem = DefaultLabel;

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Ident(String);

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<Bytes> for Ident {
    type Error = Error;

    fn try_from(value: Bytes) -> std::prelude::v1::Result<Self, Self::Error> {
        let this = String::from_utf8(value.to_vec())?;
        Ok(Self(this))
    }
}

impl TryInto<Bytes> for Ident {
    type Error = Error;

    fn try_into(self) -> std::prelude::v1::Result<Bytes, Self::Error> {
        let this = self.0.into_bytes();
        Ok(Bytes::from(this))
    }
}

type Idents = Vec<Ident>;
type MetadataItems = Vec<MetadataItem>;

#[derive(Clone)]
pub struct MemoryBackend {
    /// Store the objects
    objects: RefCell<HashMap<Ident, Item>>,

    /// Store key=value label -> Vec<Ident>
    metadata: RefCell<HashMap<String, Idents>>,

    /// Store value=key label -> Vec<Ident>
    metadata_rtl: RefCell<HashMap<String, Idents>>,

    /// Store Ident -> Vec<key=value>
    object_metadata: RefCell<HashMap<Ident, MetadataItems>>,
}

impl StorageBackend for MemoryBackend {
    type Backend = sled::Db;
    type Result<T> = Result<T>;
    type Error = Error;
    type Item = Item;
    type MetadataItem = MetadataItem;
    type Ident = Ident;

    fn open(_path: &std::path::Path) -> Self::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            objects: RefCell::new(HashMap::new()),
            metadata: RefCell::new(HashMap::new()),
            metadata_rtl: RefCell::new(HashMap::new()),
            object_metadata: RefCell::new(HashMap::new()),
        })
    }

    fn sync(&self) -> Self::Result<()> {
        Ok(())
    }

    fn new_ident(&self, item: &Self::Item) -> crate::error::Result<Self::Ident> {
        let mut h = DefaultHasher::new();
        item.0.hash(&mut h);
        Ok(Ident(format!("{}", h.finish())))
    }

    fn get(&self, ident: Self::Ident) -> crate::error::Result<Option<Self::Item>> {
        let mut objects = self.objects.try_borrow_mut()?;
        match objects.entry(ident) {
            std::collections::hash_map::Entry::Occupied(this) => Ok(Some(this.get().clone())),
            std::collections::hash_map::Entry::Vacant(_) => Ok(None),
        }
    }

    fn put(
        &self,
        item: Self::Item,
        meta: Vec<Self::MetadataItem>,
    ) -> crate::error::Result<Self::Ident> {
        let mut objects = self.objects.try_borrow_mut()?;
        let mut metadata = self.metadata.try_borrow_mut()?;
        let mut metadata_rtl = self.metadata_rtl.try_borrow_mut()?;
        let mut object_metadata = self.object_metadata.try_borrow_mut()?;

        let ident = self.new_ident(&item)?;
        objects
            .entry(ident.clone())
            .and_modify(|this| *this = item.clone())
            .or_insert(item);

        for item in &meta {
            metadata
                .entry(format!("{item}"))
                .and_modify(|this| {
                    this.push(ident.clone());
                    this.sort_by(|a, b| b.0.cmp(&a.0));
                    this.dedup();
                })
                .or_insert(vec![ident.clone()]);

            metadata_rtl
                .entry(format!("{item}"))
                .and_modify(|this| {
                    this.push(ident.clone());
                    this.sort_by(|a, b| b.0.cmp(&a.0));
                    this.dedup();
                })
                .or_insert(vec![ident.clone()]);
        }
        object_metadata
            .entry(ident.clone())
            .and_modify(|this| *this = meta.clone())
            .or_insert(meta.clone());

        Ok(ident)
    }

    fn delete(&self, ident: Self::Ident) -> crate::error::Result<Option<Self::Item>> {
        let mut objects = self.objects.try_borrow_mut()?;
        let mut metadata = self.metadata.try_borrow_mut()?;
        let mut metadata_rtl = self.metadata_rtl.try_borrow_mut()?;
        let mut object_metadata = self.object_metadata.try_borrow_mut()?;

        let old = match objects.remove_entry(&ident) {
            Some((_, old)) => old,
            None => return Err(Error::ObjectNotFound(format!("{ident}"))),
        };
        let (_, meta_items) = match object_metadata.remove_entry(&ident) {
            Some(this) => this.to_owned(),
            None => (Ident(String::new()), vec![]),
        };

        for label in meta_items {
            metadata
                .entry(format!("{}={}", &label.0, &label.1))
                .and_modify(|idents| idents.retain(|e| e != &ident));
            metadata_rtl
                .entry(format!("{}={}", &label.1, &label.0))
                .and_modify(|idents| idents.retain(|e| e != &ident));
        }

        Ok(Some(old))
    }
}

impl StorageBackendSearch for MemoryBackend {
    fn find_by_metadata(
        &self,
        metadata_items: Vec<Self::MetadataItem>,
    ) -> Self::Result<Option<Vec<Self::Ident>>> {
        let metadata = self.metadata.try_borrow()?;

        let mut idents = vec![];
        for label in metadata_items {
            if let Some(label_idents) = metadata.get(&format!("{label}")) {
                idents.extend_from_slice(label_idents);
            }
        }
        if idents.is_empty() {
            Ok(None)
        } else {
            idents.sort();
            idents.dedup();
            Ok(Some(idents))
        }
    }
}
