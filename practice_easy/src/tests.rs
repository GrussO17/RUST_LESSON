use super::count_above;

fn reference_count_above(input: &Vec<i32>, min: i32) -> u32{ 
    let mut ret: u32 = 0;
    for val in input{
        if *val >= min {
            ret += 1;
        }
    }
    return ret;
}

#[test]
fn easy() {
    let list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ret = count_above(&list, 3);
    let answer = reference_count_above(&list, 3); 
    assert!(ret == answer); // 7
}
#[test]
fn hard() {
    let list: Vec<i32> = (-13..100).collect();
    let ret = count_above(&list, 3);
    let answer = reference_count_above(&list, 3);
    assert!(ret == answer); // 97
}

