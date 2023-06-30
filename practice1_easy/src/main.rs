mod tests;


fn main() {

    //Write a function that takes in a vector of ints, and returns the number of elements above the 'min'
    let list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let ret = count_above(&list, 3);
    println!("number of items above 3: {}", ret);

}

//don't include values that are equal
fn count_above(input: &Vec<i32>, min: i32) -> u32{ 
    //TODO this

    1 //Last line (wihtout a semicolon) is the return value
}