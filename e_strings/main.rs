fn main() {
    // There are two string types in Rust. First one is a str reference &str which is a light-weight and fast. The other is string type which is heavier with additional functionality
    let name = "Bingo";
    let other_name = String::from("Matti Meikäläinen");

    println!("My name is {} and this is {}", name, other_name);

    // The &str is dynamically sized, it is a reference and String is an owned type and has a size
    println!("A String is always {:?}. It is Sized.", std::mem::size_of::<String>());
    println!("An i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    println!("An f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    println!("A &str? It can be anything. 'Bingo' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Bingo"));

    // There are many ways to create strings and one more is format! macro
    let my_name = "Jim Bob";
    let my_country = "USA";
    let my_home = "Alabama";

    let together = format!(
        "I am {} and I come from {}. I live in {}.", my_name, my_country, my_home
    );

    println!("{}", together);
    
}