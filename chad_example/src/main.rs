fn main() {

    //This turns every word in a given sentance or string to the word "🍞".
    let i = "this is a totally normal sentance.";
    println!("Sentance Before: {}", i);

    println!("Sentance after: {}",i.split(" ").map(|_x| "🍞 ").into_iter().collect::<String>());
}
