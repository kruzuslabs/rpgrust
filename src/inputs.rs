pub mod inputs {
    use std::io;

    pub fn string_input() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        input
    }

    pub fn int_input() -> i32 {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read user input");
    
    
            match input.trim().parse::<i32>() {
                Ok(number) => return number, 
                Err(_) => {
                    println!("Invalid input. Please enter a valid number:");
                    continue;
                }
            }
        }
    }
}