use std::io;
use pokemon_rs;
fn main() {
        loop {
                println!("type break to stop");
                println!("search any pokemon:");
                let pokemon_names = pokemon_rs::get_all(None);

                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)
                .expect("can't read input :C, try again");
                // |
                if user_input.trim().to_uppercase() == "BREAK" { break; }

                for pokemon in pokemon_names {
                        if pokemon.trim().to_uppercase().contains(&user_input.trim().to_uppercase()){
                                println!("{pokemon}");

                        } else {
                                continue;
                        }
                }
        }
}