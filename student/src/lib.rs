pub struct Student {
    name: String,
    age: u8,
}

impl Student {
    pub fn new(name: String, age: u8) -> Student {
        Student { name, age }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn say_hello(&self) {
        println!(
            "Hello, my name is {} and I am {} years old",
            self.name, self.age
        );
    }
}
