use ds_and_algorithms::linked_list::LinkedList;

fn main() {

    let mut linkedList = LinkedList::new();

    linkedList.push_back(3);
    linkedList.push_back(12);
    linkedList.push_front(1);

    println!("{:?}", linkedList);
}
