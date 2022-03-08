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

    pub fn add_child_by_value(&mut self, payload: T) {
        self.child = Some(Box::new(SingleLinkedList::new(payload, None)));
    }

    pub fn set_value(&mut self, value: T) {
        self.payload = value;
    }

    pub fn get_value(&self) -> T {
        return self.payload.clone();
    }

    pub fn get_child(&self) -> Option<&Box<SingleLinkedList<T>>> {
        return self.child.as_ref();
    }

    pub fn get_mut_child(&mut self) -> Option<&mut Box<SingleLinkedList<T>>> {
        return self.child.as_mut();
    }

    pub fn has_child(&self) -> bool {
        return self.child.is_some();
    }

    pub fn get_length(&self) -> u32 {
        let mut count: u32 = 0;
        if self.has_child() {
            count += self.get_child().unwrap().get_length();
        }
        count += 1;
        return count;
    }

    // pub fn get_node_by_value(&self, value: T) -> Option<&mut Box<SingleLinkedList<T>>> {
    //     if self.get_value() == value {
    //         panic!("fuck");
    //     }
    //     while self.has_child() {

    //     }
    //     return None;
    // }
}