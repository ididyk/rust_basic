pub fn run(){
    let mut hello = String::from("Hello ");
    println!("{}", hello);

    println!("len {}", hello.len());

    // push char
    hello.push('W');

    //push string
    hello.push_str("orld!");

    println!("Capacity: {}", hello.capacity());
    println!("Length: {}", hello.len());
    println!("Is empty: {}", hello.is_empty());    
    println!("Contains 'World': {}", hello.contains("World"));
    println!("Replace 'World': {}", hello.replace("World", "There"));

    //loop through string by whitespace
    for world in hello.split_whitespace(){
        println!("{}",world);
        println!();
    }

    // create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // assertion testing
    assert_eq!(3,s.len());
    assert_eq!(10,s.capacity());

    println!("{}", s);
}