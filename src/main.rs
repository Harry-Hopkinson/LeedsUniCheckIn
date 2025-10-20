use image::Luma;
use qrcode::QrCode;

fn get_exe_path() -> std::path::PathBuf {
    return std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
}
fn main() {
    let code = QrCode::new(b"01234567").unwrap();

    let image = code.render::<Luma<u8>>().build();
    println!("{:?}", get_exe_path());

    image.save(get_exe_path().join("qrcode.png")).unwrap();
}
