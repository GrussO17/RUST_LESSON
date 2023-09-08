fn main() {
    let fizzybuzzy = fizzbuzz(5);
    println!("{fizzybuzzy}");
}


// If you have never heard about fizzbuzz, it is very simple.
// We are counting up to the number passed in, we will print the number unless
// one of the following conditions are met...
// add fizz on every number divisible by 3
// add buzz on every numebr divisible by 5
// add fizzbuzz if divisible by 3 and 5
// if fizzbuzz(15) were called it would return.
// "1 2 fizz 4 buzz fizz 7 8 fizz buzz 11 fizz 13 14 fizzbuzz"
fn fizzbuzz(num: i32) -> String {
    let mut answer:String = "1".to_string();
    return answer;
}

#[cfg(test)]
mod test;