pub trait NonEmptyExt {
    type Item;
    fn is_empty(&self) -> bool;
    fn first(&self) -> &Self::Item;
    fn last(&self) -> &Self::Item;
}

impl<T> NonEmptyExt for Vec<T> {
    type Item = T;
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn first(&self) -> &Self::Item {
        &self[0]
    }

    fn last(&self) -> &Self::Item {
        &self.as_slice().last().expect("ha-ha, impossible")
    }
}

#[cfg(test)]
mod test {
    use crate::non_empty::NonEmpty;
    use super::NonEmptyExt;

    #[test]
    fn test_vec() {
        let a = vec![1];
        let a = NonEmpty::new(a).unwrap();
        println!("first: {}", a.first());
    }
}