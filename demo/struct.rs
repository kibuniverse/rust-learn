
enum SexKind {
    Man,
    Woman,
}

struct People {
    name: String,
    age: u8,
    sex: SexKind,
    ident_id: String,
}
struct Student {
    id: u8,
    grade: u8,
    is_graduate: bool,
    people_info: People,
}


impl Student {
    fn say_name(&self) -> &str {
        println!("my name is {}", self.people_info.name);
        &self.people_info.name
    }
}

fn main() {
    let liming = Student {
        id: 1,
        grade: 2,
        is_graduate: false,
        people_info: People {
            name: String::from("liming"),
            age: 18,
            sex: SexKind::Man,
            ident_id: String::from("610424200009121234"),
        },
    };
    let liming_name = liming.say_name();
    println!("return name is  {}", liming_name);
}