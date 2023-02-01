extern crate core;

use std::borrow::Borrow;
use std::rc::Rc;

pub enum ConsImpl<T> {
    Element(T, Rc<ConsImpl<T>>),
    Root,
}

#[derive(Clone)]
pub struct Cons<T> {
    implementation: Rc<ConsImpl<T>>,
}
impl<T> Default for Cons<T> {
    fn default() -> Self {
        Self {
            implementation: Rc::new(ConsImpl::Root),
        }
    }
}
impl<T> Cons<T> {
    pub fn append(&self, new: T) -> Self {
        Self {
            implementation: Rc::new(ConsImpl::Element(new, self.implementation.clone()))
        }
    }
    pub fn get(&self) -> Option<&T> {
        match self.implementation.borrow() {
            ConsImpl::Root => None,
            ConsImpl::Element(t, _) => Some(t),
        }
    }
    pub fn next(&self) -> Self {
        Self {
            implementation:
            match self.implementation.borrow() {
                ConsImpl::Root => self.implementation.clone(),
                ConsImpl::Element(_, next) => next.clone(),
            }
        }
    }
    pub fn is_root(&self) -> bool {
        self.get().is_none()
    }
}
impl<T: Clone> Cons<T> {
    pub fn delete_next(&mut self) {
        if self.next().is_root() {
            return
        }
        let next = match self.next().implementation.borrow() {
            ConsImpl::Root => Rc::new(ConsImpl::Root),
            _ =>  match self.next().next().implementation.borrow() {
                ConsImpl::Root => Rc::new(ConsImpl::Root),
                _ => self.next().next().implementation.clone(),
            },
        };
        let rc = self.implementation.clone();
        if let ConsImpl::Element(t, _) = rc.borrow() {
            self.implementation = Rc::new(ConsImpl::Element(t.clone(), next));
        }
    }
    pub fn insert_next(&mut self, item: T) {
        if self.next().is_root() {
            return
        }
        let next = match self.implementation.borrow() {
            ConsImpl::Root => Rc::new(ConsImpl::Root),
            _ =>  self.next().implementation.clone(),
        };
        let rc = self.implementation.clone();
        if let ConsImpl::Element(t, _) = rc.borrow() {
            self.implementation = Rc::new(ConsImpl::Element(t.clone(), Rc::new(ConsImpl::Element(item, next))));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let list = Cons::default();
        assert!(list.is_root());

        let list = list.append(1).append(2).append(3);

        assert_eq!(list.get(), Some(&3));
        assert_eq!(list.next().get(), Some(&2));
        assert_eq!(list.next().next().get(), Some(&1));
        assert_eq!(list.next().next().is_root(), false);
        assert_eq!(list.next().next().next().get(), None);
        assert_eq!(list.next().next().next().is_root(), true);
        assert_eq!(list.next().next().next().next().get(), None);
        assert_eq!(list.next().next().next().next().is_root(), true);
    }

    #[test]
    fn branch() {
        let list = Cons::default().append(1).append(2).append(3);

        let branch1 = list.append(41). append(42);
        let branch2 = list.append(51). append(52).append(53);

        assert_eq!(branch1.get(), Some(&42));
        assert_eq!(branch1.next().get(), Some(&41));
        assert_eq!(branch1.next().next().get(), Some(&3));
        assert_eq!(branch1.next().next().next().get(), Some(&2));
        assert_eq!(branch1.next().next().next().next().get(), Some(&1));
        assert_eq!(branch1.next().next().next().next().next().get(), None);
        assert_eq!(branch1.next().next().next().next().next().next().get(), None);


        assert_eq!(branch2.get(), Some(&53));
        assert_eq!(branch2.next().get(), Some(&52));
        assert_eq!(branch2.next().next().get(), Some(&51));
        assert_eq!(branch2.next().next().next().get(), Some(&3));
        assert_eq!(branch2.next().next().next().next().get(), Some(&2));
        assert_eq!(branch2.next().next().next().next().next().get(), Some(&1));
        assert_eq!(branch2.next().next().next().next().next().next().get(), None);
        assert_eq!(branch2.next().next().next().next().next().next().next().get(), None);
    }

    #[test]
    fn delete_next() {
        let mut list = Cons::default().append(1).append(2).append(3);

        let removed_branch = list.next();

        list.delete_next();

        assert_eq!(list.get(), Some(&3));
        assert_eq!(list.next().get(), Some(&1));
        assert_eq!(list.next().next().get(), None);
        assert_eq!(list.next().next().is_root(), true);

        assert_eq!(removed_branch.get(), Some(&2));
        assert_eq!(removed_branch.next().get(), Some(&1));
        assert_eq!(removed_branch.next().next().get(), None);
        assert_eq!(removed_branch.next().next().is_root(), true);

    }

    #[test]
    fn insert_next() {
        let mut list = Cons::default().append(1).append(2).append(3);

        list.insert_next(4);

        assert_eq!(list.get(), Some(&3));
        assert_eq!(list.next().get(), Some(&4));
        assert_eq!(list.next().next().get(), Some(&2));
        assert_eq!(list.next().next().next().get(), Some(&1));
        assert_eq!(list.next().next().next().next().get(), None);
        assert_eq!(list.next().next().next().next().next().get(), None);

    }
}


