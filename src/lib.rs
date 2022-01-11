use core::ops::Deref;

pub struct Annex<'a, T: Annexable>(&'a mut T);

impl<'a, T: Annexable> Annex<'a, T> {
    pub fn new(item: &'a mut T) -> Annex<'a, T> {
        Annex(item)
    }

    pub fn push_annex(&mut self, item: T::NewItem) -> Annex<T> {
        self.0.push_annex(item)
    }
}

pub trait Annexable {
    type NewItem;

    fn push_annex(&mut self, item: Self::NewItem) -> Annex<Self>
    where
        Self: Sized;
    fn revert_annex(&mut self);
}

impl<T> Annexable for std::vec::Vec<T> {
    type NewItem = T;

    fn push_annex(&mut self, item: Self::NewItem) -> Annex<Self> {
        self.push(item);
        Annex(self)
    }

    fn revert_annex(&mut self) {
        self.pop();
    }
}

impl<T: Annexable> Deref for Annex<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

// TODO could do this but allowing it my break the symmetry if self is mutated between the drop
// impl<T: Annexable> DerefMut for Annex<'_, T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

impl<'a, T: Annexable> Drop for Annex<'a, T> {
    fn drop(&mut self) {
        Annexable::revert_annex(self.0);
    }
}

#[cfg(test)]
mod tests {
    use super::Annexable;

    #[test]
    fn it_works() {
        let mut vec1 = vec![1, 2, 3];
        assert_eq!(vec1, [1, 2, 3]);
        {
            let mut new_vec_ref = vec1.push_annex(4);
            assert_eq!(*new_vec_ref, [1, 2, 3, 4]);
            {
                let new_new_vec_ref = new_vec_ref.push_annex(8);
                assert_eq!(*new_new_vec_ref, [1, 2, 3, 4, 8]);
            }
            assert_eq!(*new_vec_ref, [1, 2, 3, 4]);
        }
        assert_eq!(vec1, [1, 2, 3]);
    }
}
