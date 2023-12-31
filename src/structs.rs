// traditional struct
struct Color{
    red: u8,
    green: u8,
    blue: u8
}

// tuple struct
struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str){
        self.last_name = last.to_string();

    }

    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}


pub fn run(){

    let mut c = Color{
        red: 255,
        green: 0, 
        blue:0
    };

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TupleColor(255,0,0);
    tc.0 = 121;
    println!("Tuple color: {} {} {}", tc.0, tc.1, tc.2);


    let mut p = Person::new("John", "Doe");
    println!("Person: {} ", p.full_name());
    p.set_last_name("Marry");
    println!("Person: {} ", p.full_name());
    println!("Person: {:?} ", p.to_tuple());

}