use std::collections::LinkedList;

fn main() {
    let vec = fib_vec(2);
    let ll = fib_ll(2);
    println!("Vector: {vec:?}");
    println!("LinkedList: {ll:?}");
}

//Vector practice
//generate vectors of n length fibonacci sequence
fn fib_vec(len: i32) -> Vec<i32> {
    return vec![1, 1];
}

//any harder to do with a linked list??
fn fib_ll(len: i32) -> LinkedList<i32> {
    return LinkedList::from([1, 1])
}
