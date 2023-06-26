// variables are immutable by default
// rust is a block-scoped lang
// variables hold primitive data or references to data

pub fn run(){
    let name = "Valera";
    let mut age = 18;
    println!("User name is {} and user age {}", name,age);

    age = 19;
    println!("User name is {} and user age {}", name,age);

    // Define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Valentyn", 19);
    println!("Name: {}, Age: {}", my_name, my_age);

    

}