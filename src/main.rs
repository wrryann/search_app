use std::io;
fn main() {
        let v1 = vec!["sla oq", "2", "3", "bla bla", "42", "45", "64", "86"];
        let mut v1_iter = v1.into_iter();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)
            .expect("sla");

        let total = v1_iter.find(|&x| x == user_input.trim());

        match total {
                Some(value) => println!("{}", value),
                None => println!("{:?}", total)
        }

}