use super::*;


#[test]
fn easy() {
    let check = is_prime(5);
    let answer = true;
    assert_eq!(check, answer);

}
#[test]
fn medium() {
    let check = is_prime(300);
    let answer = false;
    assert_eq!(check, answer);
}
#[test]
fn hard() {
    let check = is_prime(7919);
    let answer = true;
    assert_eq!(check, answer);
}
