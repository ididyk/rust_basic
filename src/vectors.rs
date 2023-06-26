use std::mem;

pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[2]=111;

    numbers.push(42);

    numbers.pop();

    println!("{:?}",numbers);
    println!("single value = {}",numbers[3]);
    println!("Vector length = {}", numbers.len());
    println!("Vector are stack allocated.\nOccupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);


    // loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop & mutate val
    for elem in numbers.iter_mut() {
        *elem *= 2;
    }

    println!("Numbers vector: {:?}", numbers);

}