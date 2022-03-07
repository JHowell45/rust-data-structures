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

    fn get_value(&self) -> T {
        return self.payload.clone();
    }

    fn get_child(&mut self) -> Option<&mut Box<SingleLinkedList<T>>> {
        return self.child.as_mut();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_linked_list_instance() {
        let linked_list: SingleLinkedList<i32> = SingleLinkedList::new(10, None);
        assert_eq!(linked_list.get_value() , 10);
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

    #[test]
    fn test_get_value() {
        let link: SingleLinkedList<i32> = SingleLinkedList::new(1, None);
        assert_eq!(link.get_value(), 1);
    }

    #[test]
    fn test_get_child() {
        let child: SingleLinkedList<i32> = SingleLinkedList::new(2, None);
        let child_box = Box::new(child);
        let mut link: SingleLinkedList<i32> = SingleLinkedList::new(1, Some(child_box.clone()));
        let child = link.get_child().unwrap();
        assert_eq!(child.payload, child_box.payload);
    }

    #[test]
    fn test_update_child_data() {
        let child: SingleLinkedList<i32> = SingleLinkedList::new(2, None);
        let child_box = Box::new(child);
        let mut link: SingleLinkedList<i32> = SingleLinkedList::new(1, Some(child_box.clone()));
        let mut retrieved_child = link.get_child().unwrap();
        retrieved_child.payload = 10;
        assert_eq!(retrieved_child.get_value(), 10);
        assert_eq!(link.get_child().unwrap().get_value(), 10);
    }
}
