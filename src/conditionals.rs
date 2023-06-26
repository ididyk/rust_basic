
pub fn run(){

    let age: u8 = 21;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("True");
    }else if age < 21 && check_id{
        println!("False");
    }else{
        println!("None");
    }

    let is_of_age = if age >= 21 { true } else { false }; // ternary operator
    println!{"Is of age: {}", is_of_age};
}