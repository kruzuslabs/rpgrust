use std::{thread, time, io::{self, Write}};

use dialoguer::{theme::ColorfulTheme, Select};


fn options<'a>(selections: &'a [&'a str]) -> &'a str {
    let selection = Select::with_theme(&ColorfulTheme::default())
  
    .default(0)
    .items(&selections[..])
    .interact()
    .unwrap();

    
    return selections[selection];
 
}


fn main() {
    println!("Hello world from rust");
    wait(1000);
    typewriter("hello world again", 100);
    wait(500);
    typewriter("Please enter a number:", 100);
    let num = read_number();
    typewriter(format!("hello {num}").as_str(), 100);
    typewriter("U are cold waht u wanna do?", 100);

    let quest = &[
        "North",
        "West",
        "South"
    ];

    let option = options(quest);

    typewriter(format!("you chose {option}").as_str(), 100);
  

    // clearscreen::clear().unwrap();

}

fn typewriter(text: &str, delay: u64) {
    for c in text.chars() {
        print!("{c}");
        io::stdout().flush().expect("error to flush!");
        wait(delay);
    }
    print!("\n");
}

fn wait(delay: u64) {
    thread::sleep(time::Duration::from_millis(delay));
}

fn read_number() -> i32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");

        // Attempt to parse the input as an integer
        match input.trim().parse::<i32>() {
            Ok(number) => return number, // Return the number if parsing is successful
            Err(_) => {
                // If parsing fails, print an error message and ask for input again
                println!("Invalid input. Please enter a valid number:");
                continue;
            }
        }
    }
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    input
}