use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use image::Luma;
use owo_colors::OwoColorize;
use qrcode::QrCode;

mod path;

const ROOMS: &[&str] = &[
    "11-14 Blenheim Terrace SR (1.01)",
    "11-14 Blenheim Terrace SR (1.17)",
    "11-14 Blenheim Terrace SR (G.02)",
    "11-14 Blenheim Terrace SR (G.03)",
    "11-14 Blenheim Terrace SR (G.06)",
    "11-14 Blenheim Terrace SR (G.08)",
    "11-14 Blenheim Terrace SR (G.12)",
];

const DATA: &str = r#"{"locationName": "Worsley SR (8.49n)", "description": "Worsley SR (8.49n)"}"#;

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
        .items(&ROOMS[..])
        .interact()
        .unwrap();

    let room = ROOMS[selection];
    println!("You have selected room: {}", room.blue().italic().bold());

    let code = QrCode::new(DATA).expect("Failed to create QR code");
    let image = code.render::<Luma<u8>>().build();

    let output_path = path::get_exe_path().join("qrcode.png");
    image
        .save(&output_path)
        .expect("Failed to save QR code image");

    println!(
        "QR code saved to {:?}",
        output_path.yellow().italic().bold()
    );

    println!("{}", "Have fun at your lesson 😉".black().bold());
}
