pub fn run(){
    let person: (&str, &str, i8) = ("Java", "Python", 5);
    println!("Main stack = {}, second lang = {}, expiriense in years = {}",person.0,person.1,person.2);
}