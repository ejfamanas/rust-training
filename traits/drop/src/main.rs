// drop is a trait but has functions like a destructor
struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} has entered the game", name);
        Creature {name: name.into()}
    }
}

// destructures the value and implements on base class
impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn main() {
    println!("Hello, world!");
    let goblin = Creature::new("Jeff");
    println!("Game proceeds...");
    // destructuring manually
    drop(goblin);
    println!("Minished!");
}
