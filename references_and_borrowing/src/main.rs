fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);

    // Dereferencing
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // a = 1
    *x += 1; // x = 2

    let r1: &Box<i32> = &x; //
    let b: i32 = **r1;

    let r2: &i32 = &*x;
    let c: i32 = *r2;

    let mut v: Vec<i32> = vec![1, 2, 3];
    // error porque al hacer push y pasar de la capacidad de 3 del
    // vector, este array en heap se desaloca y cambia de ubicacion
    // en memoria, lo que invalida el puntero de num
    // let num: &i32 = &v[2];
    // v.push(4);
    // println!("Third element is {}", *num);

    // mutable references
    let num: &mut i32 = &mut v[2];

    *num += 1;
    println!("Third element is {}", *num);

    println!("Vector is now{:?}", v);

    let chars: &mut Vec<char> = &mut vec!['W', 'a', 'o', 's'];
    ascii_capitalize(chars);
}

// reference to a string -> &String
// references are now-owning pointers
fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v);
    }
}
