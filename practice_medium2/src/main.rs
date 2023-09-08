fn main() {
    let a = is_prime(3);
    let b = is_prime(8);
    println!("A is {}, B is {}", a , b);
}


// The goal is simple, check if a given number is prime.
fn is_prime(num:u32) -> bool{
    true
}


#[cfg(test)]
mod test;