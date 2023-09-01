use rand::prelude::*;

fn main() {
    let mut r = rand::thread_rng();
    let x: f64 = r.gen(); //random 0-1
    let y: u32 = r.gen_range(0..100);
    println!("X: {x}, and Y: {y}");

    example();
    let what_i_said = example0(y, x);
    println!("{what_i_said}");
    example1(1, 2, 3);
    example2();

    let a = 33;
    let b = 43;

    let bigger = greater(&a, &b);
    println!("The bigger number is {bigger}");
}

//takes nothing, returns nothing
fn example() {

}

fn example0(a: u32, b: f64) -> &'static str {
    if a > b as u32 {
        return "Hello";
    }
    else {
        return "There";
    }
}

//Takes 3 parameters
fn example1(_p1:u32, _p2:u32, _p3:u32) {

}

//returns an unsigned int
fn example2() -> u32 {
    1 //last line of function is ret (no ;)
}

fn pow(a: i32, b: i32) -> i32{
    let mut sum = 1;
    for _i in 0..b{
        sum *= a;
    }
    return sum;
}


//How to return a reference
fn greater<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i > j {
        i
    } else {
        j
    }
}

fn second_str<'a, 'b>(str1: &'a str, str2: &'b str) -> &'b str {
    str2
}
