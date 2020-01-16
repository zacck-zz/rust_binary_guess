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

    /*
     * Characters
     * This type is four bytes in size and represents a unicode scalar
     */
    let z = 'z';
}
