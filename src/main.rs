mod linked_list;

use linked_list::List;
use linkedlist_rust::doubly_linked_list::Dlist;

fn main() {
    let mut list: List<i32> = List::new();

    list.insert(10);
    list.insert(11);
    list.insert(12);

    println!("{}", list[2]);

    println!("{}", list);

    println!("{}", list[2]);

    list.delete_nth(1).unwrap();

    println!("{}", list);

    list.insert_at(50, 1).unwrap();

    println!("{}", list);

    print!("doubly linked list:\n\n");

    let mut dlist :Dlist<i32> = Dlist::new();
    dlist.insert(10);
    dlist.insert(11);
    dlist.insert(12);
    dlist.insert(13);


    dlist.insert_at(50, 4).unwrap();

    dlist.print();
    dlist.print_reverse();
    println!("##");

    dlist.delete_nth(1).unwrap();
    dlist.print();
    dlist.print_reverse();

    dlist.delete_nth(0).unwrap();
    println!("##");
    dlist.print();
    dlist.print_reverse();

    dlist.delete_nth(2).unwrap();
    println!("##");
    dlist.print();
    dlist.print_reverse();
}

