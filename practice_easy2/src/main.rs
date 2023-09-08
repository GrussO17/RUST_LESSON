use std::collections::LinkedList;

fn main() {
    let vec = fib_vec(2);
    let ll = fib_ll(2);
    println!("Vector: {vec:?}");
    println!("LinkedList: {ll:?}");
}

//Vector practice
//generate vectors of n length fibonacci sequence
//if you get stuck there is an implementation in the test file.
fn fib_vec(len: i32) -> Vec<i32> {
    return vec![0, 1];
}

//any harder to do with a linked list??
fn fib_ll(len: i32) -> LinkedList<i32> {
    return LinkedList::from([0, 1])
}


#[cfg(test)]
mod tests;