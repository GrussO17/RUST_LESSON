fn main() {

    //Write a function that takes in a vector of ints, and returns the number of elements above the 'min'
    let list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let ret = count_above(&list, 3);
    println!("number of items above 3: {}", ret);

}

//The goal of coSunt above is to take in a refernece to a vector and return the number of integers that are
//above the passed min. The count includes the minimum
//
// input: Vector of integers
// min: integer to compare against
//
// return: number of values >= min.
fn count_above(input: &Vec<i32>, min: i32) -> u32{ 
    //TODO this

    1 //Last line (wihtout a semicolon) is the return value
}

#[cfg(test)]
mod tests;