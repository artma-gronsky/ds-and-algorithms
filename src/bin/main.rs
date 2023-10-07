use ds_and_algorithms::db_list::DbList;
use ds_and_algorithms::linked_list::LinkedList;
use std::ops::Deref;

fn main() {
    // let mut linkedList = LinkedList::new();
    //
    // linkedList.push_back(3);
    // linkedList.push_back(12);
    // linkedList.push_front(1);
    //
    // println!("{:?}", linkedList);

    let mut db_list = DbList::new();

    db_list.push_front(3);
    db_list.push_front(2);
    db_list.push_front(1);

    db_list.push_back(4);

    println!("{:#?}", db_list);

    println!("{}",db_list.get_last().unwrap().upgrade().unwrap().borrow().get_data());
}
