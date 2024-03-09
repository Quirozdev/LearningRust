fn main() {
    println!("Hello, world!");
}

fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}
// error
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

fn stringify_name_with_title(name: &Vec<String>) -> String {
    // let mut name_clone = name.clone();
    // name_clone.push(String::from("Esq."));
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
// This version would make the input name unusable, which is very annoying to a caller
// fn stringify_name_with_title(mut name: Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }
// correcto, pero muta!!!
// Functions should not mutate their inputs if the caller would not expect it
// fn stringify_name_with_title(name: &mut Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }
// error
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

// guardando solo el tamanio del string mas grande,
// se evita el problema del borrow checker y siguie siendo
// muy performant
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest {
            dst.push(s.clone());
        }
    }
}
// performance issues
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone());
//         }
//     }
// }
