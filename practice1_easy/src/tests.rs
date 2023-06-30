
use super::count_above;

#[test]
fn easy() {
    let list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ret = count_above(&list, 3);
    assert!(ret == 6);
}
#[test]
fn hard() {
    let list: Vec<i32> = (-13..100).collect();
    let ret = count_above(&list, 3);
    assert!(ret == 96);
}

