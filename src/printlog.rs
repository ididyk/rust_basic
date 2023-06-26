pub fn run(){
    // print to console;
    println!("Hello from printlog file");


    // println!("Number: {}", 1);

      // basic formatting
      println!("{} is from {}", 1,2);

      println!();

      // positional args
      println!("var1 = {0}, var2 = {1}, var3 =  {0}, var4 = {2}", "ASD", "DSA", 2);

      println!();

      // named args
      println!(
        "var1 = {var1}, var2 = {var2}",
        var1 = 123, 
        var2 = 456
    );

     println!();

      // palceholder traits
      println!(
        "Binary = {:b}, hex = {:x}, Octal = {:o}",
        10,10,10
    );

    println!();

      // palceholder for debug trait
      println!(
        "{:?}",
        (12, true, "Hello")
    );

    println!();

      // Basic math
      println!(
        "10+10={}",
        10+10
    )
}