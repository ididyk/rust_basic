// arrays  - fixed list where elemebts are the same data types

use std::mem;

pub fn run(){

    let mut numbers: [i32; 5] = [1,2,3,4,5];

    numbers[2]=111;

    println!("{:?}",numbers);
    println!("single value = {}",numbers[3]);
    println!("Array length = {}", numbers.len());
    println!("\nArrays are stack allocated.\nOccupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}