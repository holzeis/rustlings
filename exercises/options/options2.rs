// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    match optional_word {
        Some(word) => println!("The word is: {}", word),
        None => println!("The optional word doesn't contain anything"),
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    for option in optional_integers_vec.iter() {
        match option {
            Some(integer) => println!("current value: {}", integer),
            None => (),
        }
    }
}
