fn main() {
    ex1();   
}

fn ex1() {
     //This turns every word in a given sentance or string to the word "🍞".
     let i = "this is a totally normal sentance.";
     println!("Sentance Before: {}", i);
 
    //THIS IS MY SINGLE LINE
    println!("Sentance after: {}",i.split(" ").map(|_x| "🍞 ").into_iter().collect::<String>());
}