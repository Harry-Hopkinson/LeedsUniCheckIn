use image::Luma;
use qrcode::QrCode;
use std::path::PathBuf;

fn get_exe_path() -> PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

const DATA: &str = r#"{"locationName": "EC Stoner SR (7.83)", "description": "EC Stoner SR (7.83)", "locationId": 0, "campusID": "{'createdBy': 'Data Services','createDate': '15/08/2024 13:14:09','FDCM_index': 'bb356f29e557e952','FDCM_subIndex': 'bb356f29e557e952' }", "FDCM_index": "bb356f29e557e952", "FDCM_subIndex": "8735a4b5030693c4"}"#;

fn main() {
    println!("Data loaded:\n{}", DATA);

    let code = QrCode::new(DATA).expect("Failed to create QR code");

    let image = code.render::<Luma<u8>>().build();

    let output_path = get_exe_path().join("qrcode.png");
    image
        .save(&output_path)
        .expect("Failed to save QR code image");

    println!("QR code saved to {:?}", output_path);
}
