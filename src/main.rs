// entry point into the program
fn main() {
    //immutability
    let mut x = 5;
    println!("X is currently: {}", x);
    x = 6;
    println!("X is now: {}", x);

    //constants
    const Y: u32 = 67;

    println!("Const y is {}", Y);

    //shadowing
    let s = 66;

    let s = s + 4;

    let s = s * 5;

    println!("S's value is: {}", s);

    //Integers
    let decimal_no = 45_556;
    let hex_no = 0xC0;
    let octal_no = 0o76;
    let binary_no = 0b1111_0000;
    let byte_no = b'C';

    println!(
        "Decimal, hex, octal, binary, byte:  {} {} {} {} {}",
        decimal_no, hex_no, octal_no, binary_no, byte_no
    );
    //Floating point numbers
    let six_four = 2.7;

    let three_two: f32 = 3.4;

    println!("Floating point numbers: {} {}", six_four, three_two);

    //Numeric operatiion

    //addition
    let sum = 67 + 90;

    //subtraction
    let diff = 4556.78 - 4.5;

    //multiplication
    let product = 78 * 50;

    //divider
    let quotient = 45.8 / 8.7;

    //remainder
    let modulus = 23 % 6;

    println!(
        "Sum, difference, quotient, modulus {} {} {} {} {}",
        sum, diff, product, quotient, modulus
    );

    /*
     * Booleans
     */
    let f = true;

    let t: bool = false; //uses an annotation

    println!("Bools: {} {}", f, t);
    /*
     * Characters
     * This type is four bytes in size and represents a unicode scalar
     */
    let z = 'z';

    println!("Character: {}", z);

    /*
     * Compound Types
     */

    //Tuple
    let tup: (i32, f64, u8) = (5600, 3.5, 1);

    //pattern match out of a tuple
    let (p, q, r) = tup;

    println!("Tuple values:  ({}, {}, {})", p, q, r);

    //Arrays
    //In rust arrays have a fixed length and must contain the same data kinds of item
    let a: [i32; 5] = [1, 2, 4, 5, 6];

    println!(
        "Array elements: {} {} {} {} {}",
        a[0], a[1], a[2], a[3], a[4]
    );

    new_function();
    iffy(t);
    iffy(f);

    loopy()
}

fn new_function() {
    println!("Wow from a new function");
    parametized(32);
}

fn parametized(x: i32) {
    println!("The value of x is: {}", x);
    exprs()
}

// a function that returns a () [unit] since it ends in a statement an a semicolon
fn exprs() {
    //a statement
    let _x = 5;

    // a statement that contains an expression
    let y = {
        let x = three();
        // an expression notice the lack of a delimiting semicolon
        plus_one(x)
    };

    println!("The value of y is: {}", y);
}

// a function that returns a value
fn three() -> i32 {
    3
}

// A function that takes a paremeter and returns a value
fn plus_one(x: i32) -> i32 {
    x + 1
}

//Control flow code
fn iffy(b: bool) {
    // if statement
    if b {
        println!("B is true!");
    } else {
        println!("B is false!");
    }

    let number = 6;

    // if with if else
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    // if within let
    let me = if b { "Mee!" } else { "Youuu!" };

    println!("{}", me);
}

fn loopy() {
    let mut counter = 0;

    // Returning a result from a loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}, The counter is {}", result, counter);

    //while loop
    let mut num = 3;
    while num != 0 {
        println!("{}", num);

        num -= 1;
    }

    println!("ALL SYSTEMS ARE GO");

    // For loop
    // Perhaps considered a comprehension?
    let a = [2, 4, 5, 6, 6, 77, 7, 90];

    // Using an iterator
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
