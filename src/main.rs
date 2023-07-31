pub mod inputs;
mod story;

use std::{
    io::{self, Write},
    thread, time,
};

use rand::{thread_rng, Rng};

use dialoguer::{theme::ColorfulTheme, Select};

fn options<'a>(selections: &'a [&'a str]) -> &'a str {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(selections)
        .interact()
        .unwrap();

    selections[selection]
}

fn roll_dice(sides: i32) -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(0..=sides)
}

fn higher_roll(rolls: (i32, i32)) -> i32 {
    if rolls.0 > rolls.1 {
        rolls.0
    } else {
        rolls.1
    }
}

fn roll_against_ai(player: i32) -> &'static str {
    let mut AI_RNG = thread_rng();

    let ai_guess = AI_RNG.gen_range(0..=20);

    if ai_guess > player {
        "AI won"
    } else {
        "Player won"
    }
}

fn double_dice_roll(sides: i32) -> (i32, i32) {
    let mut rng = thread_rng();

    let mut rng2 = thread_rng();

    (rng.gen_range(0..=sides), rng2.gen_range(0..=sides))
}

fn main() {
    println!("One roll: {} ", roll_dice(20));

    let (a, b) = double_dice_roll(20);
    println!("double rolls a:{}, b:{}", a, b);

    println!("highest roll from the double roll: {}", higher_roll((a, b)));

    println!("AI: {}", roll_against_ai(roll_dice(20)));

    //
    // println!("Your roll: {}", roll_dice(20));
    //
    //  let (one, two) = double_dice_roll(20);
    //
    //  println!("Rolle one: {}, roll 2: {}", one, two);

    // wait(1000);
    // typewriter("hello world again", 100);
    // wait(500);
    // typewriter("Please enter a number:", 100);

    // typewriter("U are cold waht u wanna do?", 100);

    // let quest = &["North", "West", "South"];

    // let option = options(quest);

    // typewriter(format!("you chose {option}").as_str(), 100);

    // clearscreen::clear().unwrap();
}

fn typewriter(text: &str, delay: u64) {
    for c in text.chars() {
        print!("{c}");
        io::stdout().flush().expect("error to flush!");
        wait(delay);
    }
    println!();
}

fn wait(delay: u64) {
    thread::sleep(time::Duration::from_millis(delay));
}
