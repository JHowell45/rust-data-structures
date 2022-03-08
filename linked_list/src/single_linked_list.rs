#![allow(dead_code)]


#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
pub struct SingleLinkedList<T> {
    payload: T,
    child: Option<Box<SingleLinkedList<T>>>,
}

impl<T> SingleLinkedList<T> where T: Copy {
    pub fn new(payload: T, next: Option<Box<SingleLinkedList<T>>>) -> SingleLinkedList<T> {
        return SingleLinkedList { payload: payload, child: next };
    }

    pub fn add_child(&mut self, child: Box<SingleLinkedList<T>>) {
        self.child = Some(child);
    }

    pub fn set_value(&mut self, value: T) {
        self.payload = value;
    }

    pub fn get_value(&self) -> T {
        return self.payload.clone();
    }

    pub fn get_child(&mut self) -> Option<&mut Box<SingleLinkedList<T>>> {
        return self.child.as_mut();
    }
}