
enum Movement{
    UP, DOWN, LEFT, RIGHT
}


fn move_avatar(m: Movement){
    // perform action depends on info

    match m {
        Movement::UP => println!("Avatar moving UP"),
        Movement::DOWN => println!("Avatar moving DOWN"),
        Movement::LEFT => println!("Avatar moving LEFT"),
        Movement::RIGHT => println!("Avatar moving RIGHT")
    }

}


pub fn run(){
    let avatar1 = Movement::LEFT;
    let avatar2 = Movement::RIGHT;
    let avatar3 = Movement::UP;
    let avatar4 = Movement::DOWN;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}