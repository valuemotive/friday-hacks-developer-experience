use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Info {
    name: String,
    age: i32,
    rating: Option<i32>,
}

fn main() {
    let mut my_info = Info {
        name: String::from("Rust"),
        age: 9,
        rating: None,
    };

    // working with potentially absent values
    match my_info.rating {
        Some(r) => println!("The rating is {}", r),
        None => println!("Not rated!"),
    };

    let just_some_string = Some(String::from("sTuPiD cAsE"));
    println!("Original string: {:?}", just_some_string);

    let new_string = just_some_string.map(|s| s.to_lowercase());
    println!("New string: {:?}", new_string);

    // working with operations that may fail
    match write_info(&my_info) {
        Ok(_f) => println!("File written"),
        Err(e) => println!("Something went wrong: {}", e),
    };

    let mut file = File::open("my_best_friends.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    println!("File contents:\n{:}", contents);

    // working with libs
    let mut rng = thread_rng();
    let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
    println!(
        "Lets go in this direction: {}",
        arrows_iter.choose(&mut rng).unwrap()
    );

    // some on the fly testing
    println!("{}", double(my_info.age));
    do_nothing_with_struct(&mut my_info);
    println!("{:?}", my_info);
}

fn do_nothing(s: String) -> String {
    s
}

fn do_nothing_with_struct(s: &mut Info) -> () {
    println!("{:?}", s);
    s.age = 12
}

fn write_info(info: &Info) -> io::Result<(File)> {
    let mut file = File::create("my_best_friends.txt")?;
    file.write_all(format!("name: {}\n", info.name).as_bytes())?;
    file.write_all(format!("age: {}\n", info.age).as_bytes())?;
    file.write_all(format!("rating: {}\n", info.rating.unwrap_or_default()).as_bytes())?;
    Ok(file)
}

// a function that expects a reference as an argument
fn double(num: i32) -> i32 {
    num * 2
}
