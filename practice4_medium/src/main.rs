fn main() {
    let fizzybuzzy = fizzbuzz(1);
    println!("{fizzybuzzy}");
}



// add fizz on every number divisible by 3
// add buzz on every numebr divisible by 5
// add fizzbuzz if divisible by 3 and 5
// string will look like the following when printed
// 1 2 fizz 4 buzz fizz 7 8 fizz buzz 11
fn fizzbuzz(num: i32) -> &'static str {
    "1"
}

#[cfg(test)]
mod test;