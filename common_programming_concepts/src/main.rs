fn main() {

    // Mutable variables
    let mut number: i32 = 10;
    println!("Number before mutation: {}", number);
    number = 20;
    println!("Number after mutation: {}", number);

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Amount of seconds in three hours: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing
    let guess: i32 = 30;
    println!("Initial variable value: {}", guess);
    let guess: i32 = 40;
    println!("Shadowed variable value: {}", guess);

    // Float numbers
    let float_number_32: f32 = 12.3;
    println!("32-bit float number value: {}", float_number_32);
    let float_number_64: f64 = 10.2;
    println!("32-bit float number value: {}", float_number_64);

    // Booleans
    let bool_1: bool = true;
    println!("Bool 1 value {}", bool_1);
    let bool_2: bool = false;
    println!("Bool 2 value {}", bool_2);

    // Defined type Tuples
    let td_tuple: (i32, f64, u8) = (500, 6.4, 1);
    // Destructuring tuples values
    let (a, b, c) = td_tuple; 
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    println!("The value of c is: {}", c);
    // Inferred types tuples
    let ti_tuple = (10, 3.2, 8);
    // Access values by destructuring
    let (f, g, h) = ti_tuple;
    println!("The value of f is: {}", f);
    println!("The value of g is: {}", g);
    println!("The value of h is: {}", h);
    // Access values by index
    let value_by_index = ti_tuple.1;
    println!("The value of the second element is: {}", value_by_index);

    // Type and length defined array
    let td_arr: [i32; 3] = [1, 2, 3];
    // Type and length inferred array
    let ti_arr = [4, 5, 6];
    // Same value length-defined array
    let sv_arr = [5;6];
    println!("Accessing values by index in the first array: {}", td_arr[0]);
    println!("Accessing values by index in the second array: {}", ti_arr[0]);
    println!("Accessing values by index in the third array: {}", sv_arr[0]);

    // if and else expressions
    let number: i32 = 5;
    if number <= 5 {
        println!("The number is five :)");
    } else {
        println!("The number is not five :(");
    }

    // if and else if expressions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let variable declaration
    let condition: bool = true;
    let conditional_number: i32 = if condition { 5 } else { 6 };
    println!("The conditional number is: {}", conditional_number);
}
