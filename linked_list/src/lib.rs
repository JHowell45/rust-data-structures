#![allow(dead_code)]

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Clone)]
struct SingleLinkedList<T> {
    payload: T,
    child: Option<Box<SingleLinkedList<T>>>,
}

impl<T> SingleLinkedList<T> where T: Copy {
    fn new(payload: T, next: Option<Box<SingleLinkedList<T>>>) -> SingleLinkedList<T> {
        return SingleLinkedList { payload: payload, child: next };
    }

    fn add_child(&mut self, child: Box<SingleLinkedList<T>>) {
        self.child = Some(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_linked_list_instance() {
        let linked_list: SingleLinkedList<i32> = SingleLinkedList::new(10, None);
        assert_eq!(linked_list.payload , 10);
        assert_eq!(linked_list.child, None);
    }

    #[test]
    fn test_add_child() {
        let mut linked_list: SingleLinkedList<i32> = SingleLinkedList::new(10, None);
        assert_eq!(linked_list.child, None);
        let child: SingleLinkedList<i32> = SingleLinkedList::new(22, None);
        let child_box: Box<SingleLinkedList<i32>> = Box::new(child);
        linked_list.add_child(child_box.clone());
        assert_eq!(linked_list.child.unwrap(), child_box);

    }
}
