use crate::non_empty_ext::NonEmptyExt;
use std::fmt::Debug;
use std::marker::PhantomData;
use crate::non_empty_ext;

pub struct NonEmpty<T>(T) where T: NonEmptyExt;

impl<T> NonEmpty<T> where T: NonEmptyExt {

    pub fn new(t: T) -> Result<Self, &'static str> {
        if !t.is_empty() {
            Ok(Self(t))
        } else {
            Err("empty collection")
        }
    }

    pub fn take(self) -> T {
        self.0
    }

    pub fn first(&self) -> &<T as non_empty_ext::NonEmptyExt>::Item {
        self.0.first()
    }

    pub fn last(&self) -> &<T as non_empty_ext::NonEmptyExt>::Item {
        self.0.last()
    }
}