pub fn run(){
    // let mut count : i32 = 0;

    // infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 300{
    //         break;
    //     }
    // }

    // fizzbuzz - while loop
    // while count <= 100{
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz");
    //     } else if count % 5 == 0 {
    //         println!("buzz");
    //     }else {
    //         println!("{}", count);
    //     }
        
    //     count += 1;
    // }

        for count in 0..100{
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }else {
            println!("{}", count);
        }
        
    }

}