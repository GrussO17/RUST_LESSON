use super::*;

fn ref_vec(len: i32) -> Vec<i32>{
    if len < 1 {
        panic!("Bad parameter")
    }
    if len == 1 {
        return vec![0];
    }
    let mut v: Vec<i32> = vec![0, 1];
    for i in 3..len + 1 {
        let first = v[(i - 1) as usize];
        let second = v[(i - 2) as usize];

       v.push(first + second);
    }
    println!("vec {v:?}");
    v
}

fn ref_ll(len: i32) -> LinkedList<i32> {
    if len < 1 {
        panic!("Bad parameter")
    }
    if len == 1 {
        return LinkedList::from([0]);
    }
    let mut ll: LinkedList<i32> =  LinkedList::from([0, 1]);
    let mut first = 0;
    let mut second = 1;
    for _ in 3..len + 1 {
        ll.push_back(first + second);
        first = second;
        second = *ll.back().unwrap();
    }
    println!("LL {ll:?}");
    ll
}

#[test]
fn easy() {
    assert_eq!(fib_ll(3), ref_ll(3));
    assert_eq!(fib_vec(3), ref_vec(3));
}
#[test]
fn medium() {
    assert_eq!(fib_ll(1), ref_ll(1));
    assert_eq!(fib_vec(1), ref_vec(1));
}
#[test]
fn hard() {
    assert_eq!(fib_ll(13), ref_ll(13));
    assert_eq!(fib_vec(13), ref_vec(13));
}
