use linked_list::SingleLinkedList;

#[test]
fn single_linked_list_instance() {
    let mut linked_list: SingleLinkedList<i32> = SingleLinkedList::new(10, None);
    assert_eq!(linked_list.get_value() , 10);
    assert_eq!(linked_list.get_child(), None);
}

#[test]
fn add_child() {
    let mut linked_list: SingleLinkedList<i32> = SingleLinkedList::new(10, None);
    assert_eq!(linked_list.get_child(), None);
    let child: SingleLinkedList<i32> = SingleLinkedList::new(22, None);
    let child_box: Box<SingleLinkedList<i32>> = Box::new(child);
    linked_list.add_child(child_box.clone());
    assert_eq!(linked_list.get_child().unwrap().get_value(), child_box.get_value());

}

#[test]
fn get_value() {
    let link: SingleLinkedList<i32> = SingleLinkedList::new(1, None);
    assert_eq!(link.get_value(), 1);
}

#[test]
fn get_child() {
    let child: SingleLinkedList<i32> = SingleLinkedList::new(2, None);
    let child_box = Box::new(child);
    let mut link: SingleLinkedList<i32> = SingleLinkedList::new(1, Some(child_box.clone()));
    let child = link.get_child().unwrap();
    assert_eq!(child.get_value(), child_box.get_value());
}

#[test]
fn has_child() {
    let child: SingleLinkedList<i32> = SingleLinkedList::new(2, None);
    let child_box = Box::new(child);
    let mut link: SingleLinkedList<i32> = SingleLinkedList::new(1, None);
    assert_eq!(link.has_child(), false);
    link.add_child(child_box);
    assert_eq!(link.has_child(), true);
}

#[test]
fn update_child_data() {
    let child: SingleLinkedList<i32> = SingleLinkedList::new(2, None);
    let child_box = Box::new(child);
    let mut link: SingleLinkedList<i32> = SingleLinkedList::new(1, Some(child_box.clone()));
    let retrieved_child = link.get_mut_child().unwrap();
    retrieved_child.set_value(10);
    assert_eq!(retrieved_child.get_value(), 10);
    assert_eq!(link.get_child().unwrap().get_value(), 10);
}

#[test]
fn get_length_test_1() {
    let mut link = SingleLinkedList::new(10, None);
    assert_eq!(link.get_length(), 1);
}

#[test]
fn get_length_test_2() {
    let mut link = SingleLinkedList::new(10, None);
    link.add_child_by_value(1);
    assert_eq!(link.get_length(), 2);
}

#[test]
fn get_length_test_3() {
    let mut link = SingleLinkedList::new(10, None);
    link.add_child_by_value(1);
    link.get_mut_child().unwrap().add_child_by_value(2);
    assert_eq!(link.get_length(), 3);
}

#[test]
fn get_length_test_4() {
    let mut link = SingleLinkedList::new(10, None);
    link.add_child_by_value(1);
    link.get_mut_child().unwrap().add_child_by_value(2);
    link.get_mut_child().unwrap().get_mut_child().unwrap().add_child_by_value(2);
    assert_eq!(link.get_length(), 4);
}

#[test]
fn get_length_test_5() {
    let mut link = SingleLinkedList::new(10, None);
    link.add_child_by_value(1);
    link.get_mut_child().unwrap().add_child_by_value(2);
    link.get_mut_child().unwrap().get_mut_child().unwrap().add_child_by_value(2);
    link.get_mut_child().unwrap().get_mut_child().unwrap().get_mut_child().unwrap().add_child_by_value(2);
    assert_eq!(link.get_length(), 5);
}