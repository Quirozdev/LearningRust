fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // String slices
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s;

    let s = String::from("hello");

    // equals
    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();

    // equals
    let slice = &s[3..len];
    let slice = &s[3..];

    // equals
    let slice = &s[0..len];
    let slice = &s[..];

    // String literals are slices, immutable
    let s = "Hello, world!";

    // Array slices!
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    let s = String::from("hello");
    let s2: &String = &s;
    let s3: &str = &s[..];

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

// con string slices
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }
