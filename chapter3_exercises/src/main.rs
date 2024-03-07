fn main() {
    let fahr = 12.0;
    let x = convert_to_celcius(fahr);

    println!("{fahr} fahrenheits equals {x} celcius");

    let celc = 68.0;
    let y = convert_to_fahrenheits(celc);

    println!("{celc} celcius equals {y} fahrenheits");

    let fibonacci_sequence_position = 3;
    let fibonacci_number = generate_nth_fibonacci_number(fibonacci_sequence_position);

    println!(
        "The fibonacci number at position: {fibonacci_sequence_position} is: {fibonacci_number}"
    );

    print_twelve_days_of_christmas_lyrics();
}

fn convert_to_celcius(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) / (9.0 / 5.0);
}

fn convert_to_fahrenheits(celcius: f64) -> f64 {
    celcius * (9.0 / 5.0) + 32.0
}

fn generate_nth_fibonacci_number(nth: i64) -> i64 {
    if nth == 0 || nth == 1 {
        return nth;
    }

    let mut current_value = 1;
    let mut previous_value = 0;

    for _ in 1..nth {
        let temp = current_value;
        current_value = current_value + previous_value;
        previous_value = temp;
    }
    return current_value;
}

const VERSES: [&str; 12] = [
    "And a partridge in a pear tree.",
    "Two turtle doves",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,",
];

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eightth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn print_twelve_days_of_christmas_lyrics() {
    for index in 0..VERSES.len() {
        println!("On the {} day of Christmas", DAYS[index]);
        println!("My true love gave to me");
        if index == 0 {
            println!("{}", "A partridge in a pear tree.");
            print!("\n");
            continue;
        }

        for reverse_index in (0..index + 1).rev() {
            println!("{}", VERSES[reverse_index]);
        }
        print!("\n")
    }
}
