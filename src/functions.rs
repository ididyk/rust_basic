
// functions - used to store blocks of code for re-use
pub fn run(){
    greetings("Oleh");

    // bind func value to variable
    let sum = add(1,2);
    println!("Sum: {}", sum);

    // closure
    let n3: i32 = 10;
    let add_nums = |n1:i32, n2: i32| n1 + n2 + n3;
    println!("closure sum: {}", add_nums(2,3));


}


fn greetings(greet: &str){
    println!("Hello, {}\nNice to meet you!", greet);
}

fn add(n1:i32, n2:i32) ->i32{
    n1 + n2
}