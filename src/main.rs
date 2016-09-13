use std::io;


     fn main() {
         let mut i = String::new();
         i = match io::stdin().read_line(&mut i) {
             Ok(_) => i.trim().to_string(),
             _ => "".to_string()
         };
         println!("Hello {}!", i);
     }