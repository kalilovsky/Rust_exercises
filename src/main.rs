use std::collections::HashMap;

fn main() {
    // count_occurrences()
    iterators()
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
