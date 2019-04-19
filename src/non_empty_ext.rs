pub trait NonEmptyExt<T> {
    fn is_empty(&self) -> bool;
    fn first(&self) -> &T;
    fn last(&self) -> &T;
}

impl<T> NonEmptyExt<T> for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn first(&self) -> &T {
        &self[0]
    }

    fn last(&self) -> &T {
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