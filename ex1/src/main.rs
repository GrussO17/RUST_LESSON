use rand::prelude::*;

fn main() {
    let mut r = rand::thread_rng();
    let x: f64 = r.gen(); //random 0-1
    let y: u32 = r.gen_range(0..100);
    println!("X: {x}, and Y: {y}");


    let cow = animal{age: 3, weight: 133, name: String::from("stacy")};
    println!("{}", cow.who())
}

//takes nothing, returns nothing
fn example() {

}

//Takes 3 parameters
fn example1(_p1:u32, _p2:u32, _p3:u32) {

}

//returns an unsigned int
fn example2() -> u32 {
    1 //last line of function is ret (no ;)
}

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

struct animal {
    age: u32,
    weight: u32,
    name: String
}

impl animal {
    fn who(&self) -> String{
        format!("I am {}, i am {} years old", self.name, self.age)
    }
    
}