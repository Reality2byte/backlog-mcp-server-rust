use std::fmt;
use std::marker::PhantomData;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait Identifier {
    type Id;
    fn value(&self) -> Self::Id;
}

/**
 * Convenient implementation for `Identifier`
 */
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct IdentifierFor<T, Id>
where
    Id: Copy + Clone + PartialEq + Eq + Ord + PartialOrd,
    T: Eq + ?Sized,
{
    id: Id,
    _phantom: PhantomData<fn() -> T>,
}

impl<T, Id> IdentifierFor<T, Id>
where
    Id: Copy + Clone + PartialEq + Eq + Ord + PartialOrd,
    T: Eq + ?Sized,
{
    pub fn new(id: Id) -> Self {
        IdentifierFor::<T, Id> {
            id,
            _phantom: PhantomData,
        }
    }
}

impl<T, Id> Identifier for IdentifierFor<T, Id>
where
    Id: Copy + Clone + PartialEq + Eq + Ord + PartialOrd,
    T: Eq + ?Sized,
{
    type Id = Id;
    fn value(&self) -> Self::Id {
        self.id
    }
}

impl<T, Id> fmt::Debug for IdentifierFor<T, Id>
where
    Id: Copy + Clone + Default + PartialEq + Eq + Ord + PartialOrd + std::fmt::Debug,
    T: Eq + ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({:?})", &self.id)
    }
}

impl<'de, T, Id> Deserialize<'de> for IdentifierFor<T, Id>
where
    Id: Copy + Clone + PartialEq + Eq + Ord + PartialOrd + Deserialize<'de>,
    T: Eq + ?Sized,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = Id::deserialize(deserializer)?;
        Ok(IdentifierFor {
            id,
            _phantom: PhantomData,
        })
    }
}

impl<T, Id> Serialize for IdentifierFor<T, Id>
where
    Id: Copy + Clone + PartialEq + Eq + Ord + PartialOrd + Serialize,
    T: Eq + ?Sized,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.id.serialize(serializer)
    }
}
