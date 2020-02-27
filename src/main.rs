use std::io;

fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Ok to continue !");
        },
        Err(e) => {
            println!("Oups, something went wrong: {}", e);
        }
    }
}
