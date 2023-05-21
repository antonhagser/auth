#[rustfmt::skip]
#[allow(unused_imports, dead_code, clippy::all)]
pub mod prisma;

#[rustfmt::skip]
#[allow(unused_imports, dead_code, clippy::all)]
pub use prisma::PrismaClient;

pub mod application;
pub mod error;
pub mod user;

/// A model value that might not exist.
///
/// This is used to represent a value that might not exist or has not been loaded.
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Copy, PartialOrd, Eq, Ord, Debug, Hash)] //  PartialOrd, Eq, Ord
pub enum ModelValue<T> {
    /// The value **might** exist but it has not been loaded.
    NotLoaded,
    /// The value does not exist.
    NotSet,
    /// The value exists.
    Loaded(T),
}

impl<T> ModelValue<T> {
    pub const fn is_some(&self) -> bool {
        matches!(*self, ModelValue::Loaded(_))
    }

    pub const fn is_not_loaded(&self) -> bool {
        matches!(*self, ModelValue::NotLoaded)
    }

    pub const fn is_not_set(&self) -> bool {
        matches!(*self, ModelValue::NotSet)
    }

    pub fn unwrap(self) -> T {
        match self {
            ModelValue::Loaded(x) => x,
            ModelValue::NotLoaded => panic!("called `ModelValue::unwrap()` on a `NotLoaded` value"),
            ModelValue::NotSet => panic!("called `ModelValue::unwrap()` on a `NotSet` value"),
        }
    }

    pub const fn as_ref(&self) -> ModelValue<&T> {
        match *self {
            ModelValue::Loaded(ref x) => ModelValue::Loaded(x),
            ModelValue::NotLoaded => ModelValue::NotLoaded,
            ModelValue::NotSet => ModelValue::NotSet,
        }
    }
}

impl<T> Clone for ModelValue<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        match self {
            ModelValue::Loaded(x) => ModelValue::Loaded(x.clone()),
            ModelValue::NotLoaded => ModelValue::NotLoaded,
            ModelValue::NotSet => ModelValue::NotSet,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (ModelValue::Loaded(to), ModelValue::Loaded(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}

pub trait SpecOptionPartialEq: Sized {
    fn eq(l: &ModelValue<Self>, other: &ModelValue<Self>) -> bool;
}

impl<T: PartialEq> SpecOptionPartialEq for T {
    #[inline]
    fn eq(l: &ModelValue<T>, r: &ModelValue<T>) -> bool {
        match (l, r) {
            (ModelValue::Loaded(l), ModelValue::Loaded(r)) => *l == *r,
            (ModelValue::NotLoaded, ModelValue::NotLoaded) => true,
            (ModelValue::NotSet, ModelValue::NotSet) => true,
            _ => false,
        }
    }
}

impl<T: PartialEq> PartialEq for ModelValue<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        SpecOptionPartialEq::eq(self, other)
    }
}
