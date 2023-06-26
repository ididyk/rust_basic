/*
Primitive types: 
- integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
- floats: f32, f64
- Boolean: (bool)
- Characters: (char)
- Tuples
- Arrays

Rust is a statically typed lang, witch means that it must know the types of all variables at compile time,
however, the compiler can ussually infer what type we want to use based on the value and how we use it.
*/



pub fn run(){
    // default "i32"
    let a = 1;


    // defailt "f64"
    let b = 2.5;

    // add explicit type
    let c : i64 = 45454545454545;

    // find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // get bool from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';

    let b1 = '\u{1F600}';

    println!("{:?}", (a,b,c,is_active, is_greater, a1, b1))
}