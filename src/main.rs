//! cd to the deser_keybindings repository then do `cargo run`
use {
    crokey::*,
    crokey::crossterm::{
        event::{read, Event},
        terminal,
        style::Stylize,
    },
    serde::Deserialize,
    std::collections::HashMap,
};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
enum Action {
    Increment,
    Decrement,
    Quit,
}

#[derive(Deserialize)]
struct Config {
    keybindings: HashMap<CroKey, Action>,
}

pub fn main() {
    let toml = std::fs::read_to_string(
        std::env::current_dir().unwrap().join("config.toml")
    ).unwrap();
    let config: Config = toml::from_str(&toml).unwrap();
    let fmt = KeyEventFormat::default();
    println!("Key-bindings:");
    for (key, action) in &config.keybindings {
        println!(" {} -> {:?}", fmt.to_string((*key).into()).blue(), action);
    }
    println!("Type any key combination");
    let mut hit_points = 3;
    loop {
        terminal::enable_raw_mode().unwrap();
        let e = read();
        terminal::disable_raw_mode().unwrap();
        match e {
            Ok(Event::Key(key)) => {
                println!("You've hit {} ", fmt.to_string(key).yellow());
                if key == key!(ctrl-c) { // hardcoding a security
                    break;
                }
                match config.keybindings.get(&key.into()) {
                    Some(Action::Increment) => {
                        hit_points += 1;
                    }
                    Some(Action::Decrement) => {
                        hit_points -= 1;
                    }
                    Some(Action::Quit) => {
                        println!("bye!");
                        break;
                    }
                    None => {}
                }
                println!(" You have {hit_points} hit points left");
                if hit_points == 0 {
                    println!(" {}", "You die!".red());
                    break;
                }
            }
            _ => {}
        }
    }
}
