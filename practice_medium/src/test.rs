use super::*;

fn reference_fizzbuzz(num: i32) -> String {
    let mut ret:String = "".to_string();
    for i in 1..num + 1{
        if i % 3 == 0 {
            ret.push_str("fizz");
        }
        if i % 5 == 0 {
            ret.push_str("buzz");
        }
        if !(i % 5 == 0 || i % 3 == 0) {
            ret.push_str(&i.to_string());
        }
        if i < num {
            ret.push_str(" ");
        }
    }
    ret
}


#[test]
fn easy() {
    let fizzybuzzy = fizzbuzz(5);
    let answer = reference_fizzbuzz(5);
    //let answer = "1 2 fizz 4 buzz";
    assert_eq!(fizzybuzzy, answer);

}
#[test]
fn hard() {
    let fizzybuzzy = fizzbuzz(15);
    let answer = reference_fizzbuzz(15);
    //let answer = "1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz";
    assert_eq!(fizzybuzzy, answer);
}
