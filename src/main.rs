use std::collections::HashMap;
use crate::custom_link_list::CustomLinkList;

mod custom_link_list;

fn main() {
    // count_occurrences()
    // iterators()
    link_list();
}

fn link_list(){
    let mut int_link_list = CustomLinkList::new();
    int_link_list.insert(15);
    int_link_list.insert(45);
    println!("{:?}", int_link_list);
    println!("{:?}", int_link_list.remove());
    int_link_list.custom_print();

    let mut string_linked_list = CustomLinkList::new();
    string_linked_list.insert("khalil".to_string());
    string_linked_list.insert("zaza".to_string());
    string_linked_list.insert("faifai".to_string());
    println!("{:?}", string_linked_list);
    println!("{:?}", string_linked_list.remove());
    string_linked_list.custom_print();
}

fn iterators(){
    let mut vector = vec![1,2,3,4,5,6,7,8,9,10];

    let filtered_vector = vector.iter().filter(|&x| x % 2 == 0).collect::<Vec<&i32>>();


    println!("Filtered Vector is {:?}", filtered_vector);
    vector[1] = 100;
    println!("Non filtered Vector is {:?}", vector);

}

fn count_occurrences() {
    let vector = Vec::from([1,2,3,12,1,1,1]);
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for i in vector.iter(){
        let frequency = hash.entry(*i).or_insert(0);
        *frequency += 1 ;
    }

    println!("Occurrences of the Vector are {:?}", hash);
}
