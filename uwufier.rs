use std::io;

fn main() {
    let mut input = String::new();
    println!("Welcome to UwUfier!\nInsert String to UwUfy");
    loop {
        println!("Paste text here:");
        std::io::stdin().read_line(&mut input);
        println!(
            "{}",
            input
                .replace("u", "uwu")
                .replace("r", "w")
                .replace("l", "w")
        );
    }
}
