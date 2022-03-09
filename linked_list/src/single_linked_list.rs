
// #[derive(Clone, Debug)]
// enum SingleLinkedList<T> {
//     None,
//     Node { payload: T, next: Option<Box<SingleLinkedList<T>>> }
// }


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingleLinkedList<T> {
    payload: Option<T>,
    next: Option<Box<SingleLinkedList<T>>>,
}

impl<T> SingleLinkedList<T> where T: Copy {
    pub fn new() -> SingleLinkedList<T> {
        return SingleLinkedList { payload: None, next: None, };
    }

    pub fn from_value(value: T) -> SingleLinkedList<T> {
        return SingleLinkedList { payload: Some(value), next: None, };
    }

    pub fn from_value_with_next(value: T, next: SingleLinkedList<T>) -> SingleLinkedList<T> {
        return SingleLinkedList { payload: Some(value), next: Some(Box::new(next)), };
    }

    pub fn get_key(&self) -> Option<T> {
        return self.payload;
    }

    pub fn set_key(&mut self, value: T) {
        self.payload = Some(value);
    }

    pub fn get_next(&self) -> Option<&Box<SingleLinkedList<T>>> {
        match &self.next {
            None => { return None; },
            Some(_) => { return self.next.as_ref(); },
        }
    }

    pub fn set_next(&mut self, next: SingleLinkedList<T>) {
        let next = Box::new(next);
        self.next = Some(next);
    }

    pub fn get_mut_next(&mut self) -> Option<&mut Box<SingleLinkedList<T>>> {
        match &self.next {
            None => { return None; },
            Some(_) => { return self.next.as_mut(); },
        }
    }

    // pub fn search(&mut self, value: T) -> Option<&mut Box<SingleLinkedList<T>>> {
    //     if self.payload == value {
    //         return self;
    //     } else {
    //         if self.next.is_sone() {

    //         } else {
    //             panic!("Failed to find the value: {:?}", &value);
    //         }
    //     }
    //     return ;
    // }

    pub fn insert_start() {}

    pub fn insert_end() {}

    pub fn insert() {}

    pub fn delete(&mut self, value: T) {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sll: SingleLinkedList<i32> = SingleLinkedList::new();
        assert_eq!(sll.get_key(), None);
        assert_eq!(sll.get_next(), None);
    }

    #[test]
    fn test_from_value() {
        let sll: SingleLinkedList<i32> = SingleLinkedList::from_value(10);
        assert_eq!(sll.get_key().unwrap(), 10);
        assert_eq!(sll.get_next(), None);
    }

    #[test]
    fn test_from_value_with_next() {
        let child = SingleLinkedList::from_value(100);
        let sll: SingleLinkedList<i32> = SingleLinkedList::from_value_with_next(10, child.clone());
        assert_eq!(sll.get_key().unwrap(), 10);
        assert_eq!(sll.get_next().unwrap(), &Box::new(child));
    }

    #[test]
    fn test_get_key() {
        let sll: SingleLinkedList<i32> = SingleLinkedList::from_value(10);
        assert_eq!(sll.get_key().unwrap(), 10);
    }

    #[test]
    fn test_set_key() {
        let mut sll: SingleLinkedList<i32> = SingleLinkedList::new();
        assert_eq!(sll.get_key(), None);
        sll.set_key(5);
        assert_eq!(sll.get_key(), Some(5));
    }

    #[test]
    fn test_get_next() {
        let child = SingleLinkedList::from_value(100);
        let sll: SingleLinkedList<i32> = SingleLinkedList::from_value_with_next(10, child.clone());
        assert_eq!(sll.get_next().unwrap(), &Box::new(child));

    }

    #[test]
    fn test_set_next() {
        let mut sll: SingleLinkedList<i32> = SingleLinkedList::new();
        assert_eq!(sll.get_next(), None);
        let child = SingleLinkedList::from_value(10);
        sll.set_next(child.clone());
        assert_eq!(sll.get_next().unwrap(), &Box::new(child));
    }
}