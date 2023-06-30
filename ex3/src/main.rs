fn main() {
    let mut homemade_chicken_parm: ChickenParm = Food::new("homemade chicken parm");
    // now why can't we do this????
    // let mut random_food = Food::new("new_food");

    let noise = homemade_chicken_parm.eat();
    println!("{noise}");

    // so what good is traits anyways???

    // lets write a function that can take in any object of a certain trait!

    // this is used very often
    // and you can implement more than one trait

    // all of the ::from functions are really the trait From
}


// Traditional thought process behind how we would do some stacking of things, but we have traits?? they are kind of neat!

struct Farm{
    animals: Vec<Animal>,
}

struct Animal{
    name:   String,
    age:    i16,
    weight: i16,
    sound:  String,
}

//Traits
trait Food{
    //how to create food
    fn new(name: &'static str) -> Self;

    //get attributes about the food
    fn name(&self) -> &'static str;
    fn calories(&self) -> i16;
    fn is_a_sandwhich(&self) -> bool;

    //munch
    fn eat(&self) -> &'static str;
}

//We need something that has this trait??
//I made one and we will build one together
struct ChickenParm {
    name: &'static str,
    calories: i16,
    noise_i_make_while_eating: &'static str,
    with_pasta: bool,
}






impl ChickenParm {
    fn with_pasta(&self) -> bool {
        self.with_pasta
    }
    fn add_pasta(&self) {
        self.with_pasta = true
    }
}

impl Food for ChickenParm {
    fn new(name: &'static str) -> ChickenParm {
        ChickenParm{name: name, calories: 1115, noise_i_make_while_eating: "OM NOM NOM", with_pasta: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }
    fn calories(&self) -> i16 {
        self.calories
    }
    fn is_a_sandwhich(&self) -> bool {
        !self.with_pasta
    }
    fn eat(&self) -> &'static str {
        self.noise_i_make_while_eating
    }
}














