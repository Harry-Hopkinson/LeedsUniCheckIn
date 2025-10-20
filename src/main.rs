use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use image::Luma;
use owo_colors::OwoColorize;
use qrcode::QrCode;

mod path;

const ROOMS: &[&str] = &[
    "11-14 Blenheim Terrace SR (1.01)",
    "11-14 Blenheim Terrace SR (1.17)",
    "11-14 Blenheim Terrace SR (G.02)",
];

const DATA: &str =
    r#"{"locationName": "Conference Auditorium 1", "description": "Conference Auditorium 1"}"#;

fn main() {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick what room to check in to")
        .default(0)
        .items(&ROOMS[..])
        .interact()
        .unwrap();

    let room = ROOMS[selection];
    println!("You have selected room: {}", room.italic().bold().blue());

    let code = QrCode::new(DATA).expect("Failed to create QR code");
    let image = code.render::<Luma<u8>>().build();

    let output_path = path::get_exe_path().join("qrcode.png");
    image
        .save(&output_path)
        .expect("Failed to save QR code image");

    println!(
        "QR code saved to {:?}",
        output_path.italic().bold().yellow()
    );
}
