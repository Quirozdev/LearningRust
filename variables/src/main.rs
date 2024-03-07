fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing
    let x = 5;

    let x = x + 1;

    {
        // 12
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // 6
    println!("The value of x is: {x}");

    // con shadowing se puede cambiar tipo
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    // Con mut no
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
