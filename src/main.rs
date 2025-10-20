use image::Luma;
use qrcode::QrCode;

fn get_exe_path() -> std::path::PathBuf {
    return std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
}

fn load_txt_data() -> Vec<String> {
    let file_path = get_exe_path().join("../../data/data.txt");
    let file_contents = std::fs::read_to_string(file_path).unwrap();
    file_contents.lines().map(|line| line.to_string()).collect()
}

fn main() {
    let code = QrCode::new(b"01234567").unwrap();

    println!("{}", load_txt_data().join("\n"));

    let image = code.render::<Luma<u8>>().build();
    println!("{:?}", get_exe_path());

    image.save(get_exe_path().join("qrcode.png")).unwrap();
}
