use std::io;

fn main() {
    let x = 2.0;

    let y: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("{truncated}");

    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tuple;

    println!("The value of y is: {y}");

    let five_hundred = tuple.0;

    let six_point_four = tuple.1;

    let one = tuple.2;

    // no se pueden cambiar de tamanio, los vectores si
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // tipo y tamanio
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    // valor de llenado y tamanio
    let c = ["si"; 5];

    let elements_joined = c.join("");

    println!("{elements_joined}");

    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

