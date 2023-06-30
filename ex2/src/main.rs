fn main() {




    //Ownership
    let mut x = 3; //Allowed with primitives
    let mut y: i32 = x;
    
    x += 1;
    //What is x??
    



















    let s1 = String::from("abc");
    let s2 = s1; 
    //Moved s1 to s2, since it is a variable sized object
    // s1 no longer owns that object, s2 does!
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    // How do we make another reference to the same var?
    let s3 = String::from("cde");
    let s4: &String; // HERE??
    println!("s3: {}", s3);
    // println!("s4: {}", s4);

    // What if we want a copy?
    let mut s5 = String::from("fgh");
    let s6:String; // HERE??
    s5 += "i";
    println!("s5: {}", s5);
    // println!("s6: {}", s6);

    //This works for all objects, Vectors, etc.
    //So now we can talk about stack vs heap memory.
    //we don't have to worry about it! remember garbage collection!

    string_fun();
    return;








    //Pointer fun! (not as fun as C)
    let mut v:Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6];
    let v_ref1 = &v;

    //QUESTION??
    //Which of the below will work? Both??
    v_ref1.push(7);
    v.push(7);

    //slices can be considered view only portions of other objects
    let v_slice: &[i32] = &v[3..5];
    
    println!("V: {:?}", v_ref1);
    println!("v_slice: {:?}", v_slice);
    //return;


    string_fun();

}

//Which one is right?
//The goal is to be able to modify the passed in parameter (pass by reference)
fn append_a(vect: Vec<i32>) {
    //vect.push(7);
}

fn append_b(vect: & Vec<i32>) {
    //vect.push(7);
}

fn append_c(vect: &mut Vec<i32>) {
    //vect.push(7);
}



fn string_fun() {

    //Alright time for more fun
    //What is the difference between these?
    let string:String = String::from("I am a string");
    let string2 = "I am not a string";


    //What should the type hint be??
    let new_str = "help me, i don't know who i am!";


    //How can i get only the "help me" part of the last string?
    let help_me = "help me";
    
    println!("{help_me}");
     





    //String slices
    let strslice:&str = &string[2..4]; //&str -> a string slice


    let first_word = word_finder(&string, 1); //we can pass in a string 
    let second_word = word_finder(string2, 2); //or a string slice
    println!("first word: {}, second word {}", first_word, second_word);
    


}


fn word_finder(input: &str , num_word: u32) -> &str {
    
    let mut prev_word = 0;
    let mut count = 0;
    for (idx, &item) in input.as_bytes().iter().enumerate(){ //fancy iterator stuff!!
        if (item == b' ') {
            count += 1;
            if (count == num_word) {
                return &input[prev_word..idx];
            } else {
                prev_word = idx;
            }
        }
    }
    return input;
}