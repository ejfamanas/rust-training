trait Animal {
    // static function
    fn create(name: &'static str) -> Self;
    // abstract funtion with no default implementation
    fn name(&self) -> &'static str;
    // abstract function with default implementation
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

// human implements animal
// does not implement function w/ default function assigned
impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        return self.name;
    }
    fn talk(&self) {
        println!("{} says hello", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        return self.name;
    }
    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

// adding an extra function to a native class
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result: i32 = 0;
        for x in self {
            result += *x;

        }
        return result;
    }
}

fn traits() {
    // using the static factory method of create
    let h = Human::create("John");
    h.talk();
    // instantiating using a property
    let c = Cat { name: "dumb shit" };
    c.talk();
    // can also do by accessing the base class
    let new_human: Human = Animal::create("Doe");
    new_human.talk();

    let a = vec![1,2,3];
    // but what if we want the sum?
    // create trait, implement for native class
    // now you can do
    println!("the sum of a = {}", a.sum());
}

fn main() {
    println!("Hello, world!");
    traits();
}
