use crate::non_empty_ext::NonEmptyExt;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::non_empty_ext;

pub struct NonEmpty<T, V> where T: NonEmptyExt<V> { inner: T, marker: PhantomData<V> }

impl<T, V> NonEmpty<T, V> where T: NonEmptyExt<V> {

    pub fn new(t: T) -> Result<Self, &'static str> {
        if !t.is_empty() {
            Ok(Self { inner: t, marker: PhantomData, })
        } else {
            Err("empty collection")
        }
    }

    pub fn take(self) -> T {
        self.inner
    }

    pub fn first(&self) -> &V {
        self.inner.first()
    }

    pub fn last(&self) -> &V {
        self.inner.last()
    }
}