fn main() {
    /*
        An array is data inside square brackets []. Arrays:
        - Must not change size
        - Must only contain the same type
     */ 

    let array1 = ["One", "Two"];
    let array2 = ["One", "Two", "Three"];

    // These arrays have same type &str but different size.
    println!("Array on {:?} and array two {:?}", array1, array2);

    // Creating an array of size 10 with same initialized values
    let my_array = ["a"; 10];
    println!("{:?}", my_array);

    let array_of_ten = [1,2,3,4,5,6,7,8,9,10];

    // These are slices
    let three_to_five = &array_of_ten[2..5];
    let start_at_two = &array_of_ten[1..];
    let end_at_five = &array_of_ten[..5];
    let copied = &array_of_ten[..];

    println!("Three to five: {:?}, start at two: {:?}, end at five: {:?}, copy: {:?}", three_to_five, start_at_two, end_at_five, copied);
    
    // Slices work with strings
    let s = String::from("Hello");
    let len = s.len();
    
    let slice = &s[3..len];
    let slice2 = &s[1..];
    
    println!("String is {}, slice1 from 3 to end is {}, slice from 1 to end is {}.", s, slice, slice2);
    
    // Vectors are to arrays what String is to &str.
    let name1 = String::from("Romeo");
    let name2 = String::from("Julia");
    
    let mut my_vec = Vec::new();
    
    my_vec.push(name1); // Now the vec knows that it is Vec<String>, can't push anything else
    my_vec.push(name2);
    
    // Using vec! macro
    let vec_of_ten = [1,2,3,4,5,6,7,8,9,10];
    let three_to_five = &vec_of_ten[2..5];
    let start_at_two = &vec_of_ten[1..];
    let end_at_five = &vec_of_ten[..5];
    let copied = &vec_of_ten[..];
    
    println!("Three to five: {:?}, start at two: {:?}, end at five: {:?}, copy: {:?}", three_to_five, start_at_two, end_at_five, copied);

    // Tuples are collections that can house multiple types
    let random_tuple = ("Matti", 8, vec!["a","b"],"c",[8,9,10],0.1);
    
    println!("Inside the tuple is: {:?}, {:?}, {:?}, {:?}, {:?}, {:?}",
    random_tuple.0,
    random_tuple.1,
    random_tuple.2,
    random_tuple.3,
    random_tuple.4,
    random_tuple.5,
    );

}