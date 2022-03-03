trait Animal {
    fn speak(&self) -> String;

    fn introdution(&self) -> String {
        format!("I am an animal, and I am {}", self.speak())
    }
}

struct Cat {
    name: String,
    color: String,
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{}", self.name)
    }
    fn introdution(&self) -> String {
        format!("I am a cat, and my name is {}", self.name)
    }
}

fn main() {
    let beibei_cat = Cat {
        name: "Beibei".to_string(),
        color: "white".to_string(),
    };
    println!("{}", beibei_cat.introdution());
    println!("{}", beibei_cat.color);
}
