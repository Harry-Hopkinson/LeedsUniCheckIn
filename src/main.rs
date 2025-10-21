use std::path::PathBuf;

use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use image::Luma;
use owo_colors::OwoColorize;
use qrcode::QrCode;

fn load_rooms() -> Vec<&'static str> {
    include_str!("rooms.txt").lines().collect()
}

fn get_exe_path() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn main() {
    println!(
        "{}",
        "Welcome to LeedsUniCheckIn!".cyan().bold().underline()
    );

    println!(
        "{}",
        "Just select the room and it will generate a QR code for you to scan using the UniLeeds app.".cyan().bold()
    );

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .default(0)
        .items(&load_rooms())
        .interact()
        .unwrap();

    let room = load_rooms()[selection];
    println!("You have selected room: {}", room.blue().italic().bold());

    let data = format!(
        r#"{{"locationName": "{}", "description": "{}"}}"#,
        room, room
    );

    let code = QrCode::new(data).expect("Failed to create QR code");
    let image = code.render::<Luma<u8>>().build();

    let output_path = get_exe_path().join("qrcode.png");
    image
        .save(&output_path)
        .expect("Failed to save QR code image");

    println!(
        "QR code saved to {:?}",
        output_path.yellow().italic().bold()
    );

    println!("{}", "Have fun at your lesson 😉".red().bold());
}
