/// This module contains Rust-specific utilities that are separate from the main library logic, but necessary to work with Rust.
///
/// Most of this is sourced from official Rust documentation or other Rust guides.
use std::{
    any::Any,
    cmp::Ordering,
    hash::{Hash, Hasher},
};

// Allow dynamic traits to implement PartialEq.
// Based-on: https://quinedot.github.io/rust-learning

pub trait AsDynEq: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_dyn_eq(&self) -> &dyn DynEq;
}

impl<T: Any + DynEq> AsDynEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_dyn_eq(&self) -> &dyn DynEq {
        self
    }
}

pub trait DynEq: AsDynEq {
    fn dyn_eq(&self, other: &dyn DynEq) -> bool;
}

impl<T: Any + PartialEq + Eq> DynEq for T {
    fn dyn_eq(&self, other: &dyn DynEq) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

impl Eq for dyn DynEq {}

impl PartialEq<dyn DynEq> for dyn DynEq {
    fn eq(&self, other: &dyn DynEq) -> bool {
        self.dyn_eq(other)
    }
}

// Allow dynamic traits to implement PartialOrd.
// Based-on: https://quinedot.github.io/rust-learning

pub trait AsDynOrd: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_dyn_ord(&self) -> &dyn DynOrd;
}

impl<T: Any + DynOrd> AsDynOrd for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_dyn_ord(&self) -> &dyn DynOrd {
        self
    }
}

pub trait DynOrd: AsDynOrd {
    fn dyn_eq(&self, other: &dyn DynOrd) -> bool;
    fn dyn_partial_cmp(&self, other: &dyn DynOrd) -> Option<Ordering>;
}

impl<T: Any + PartialOrd + Eq> DynOrd for T {
    fn dyn_eq(&self, other: &dyn DynOrd) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn dyn_partial_cmp(&self, other: &dyn DynOrd) -> Option<Ordering> {
        other
            .as_any()
            .downcast_ref::<Self>()
            .and_then(|other| self.partial_cmp(other))
    }
}

impl Eq for dyn DynOrd {}

impl PartialEq<dyn DynOrd> for dyn DynOrd {
    fn eq(&self, other: &dyn DynOrd) -> bool {
        self.dyn_eq(other)
    }
}

impl PartialOrd<dyn DynOrd> for dyn DynOrd {
    fn partial_cmp(&self, other: &dyn DynOrd) -> Option<Ordering> {
        self.dyn_partial_cmp(other)
    }
}

// Allow dynamic traits to implement Hash.
// Based-on: https://quinedot.github.io/rust-learning

pub trait DynHash {
    fn dyn_hash(&self, state: &mut dyn Hasher);
}

// impl<T: ?Sized + Hash> DynHash for T {
impl<T: Hash> DynHash for T {
    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state)
    }
}

impl Hash for dyn DynHash + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}
