use super::*;

#[test]
fn easy() {
    let fizzybuzzy = fizzbuzz(5);
    let answer = "1 2 fizz 4 buzz";
    assert_eq!(fizzybuzzy, answer);

}
#[test]
fn hard() {
    let fizzybuzzy = fizzbuzz(15);
    let answer = "1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz";
    assert_eq!(fizzybuzzy, answer);
}
