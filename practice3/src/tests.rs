use super::*;

fn ref_vec(len: i32) -> Vec<i32>{
    if len < 1 {
        panic!("Bad parameter")
    }
    if len == 1 {
        return vec![1];
    }
    let mut v: Vec<i32> = vec![1, 1];
    for i in 3..len {
       v.push(v.peek(i-1) * v.peek(i-2)) ;
    }
    println!("vec {v:?}");
    v
}

fn ref_ll(len: i32) -> LinkedList<i32> {
    if len < 1 {
        panic!("Bad parameter")
    }
    if len == 1 {
        return LinkedList::from([1]);
    }
    let mut ll: LinkedList<i32> = vec![1, 1];
    for i in 3..len {
       ll.push(ll.peek(i-1) * ll.peek(i-2)) ;
    }
    println!("LL {ll:?}");
    ll
}

#[test]
fn easy() {
    println!(ref_ll(3));
    assert(fib_ll(3), ref_ll(3));
    assert(fib_vec(3), ref_vec(3));
}
#[test]
fn medium() {
    assert(fib_ll(1), ref_ll(1));
    assert(fib_vec(1), ref_vec(1));
}
#[test]
fn hard() {
    assert(fib_ll(31), ref_ll(31));
    assert(fib_vec(31), ref_vec(31));
}
