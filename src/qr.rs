use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

pub fn generate_qr_code(config: &str) {
    let qr = QrCode::encode_text(config, QrCodeEcc::Medium).unwrap();

    for y in 0..qr.size() {
        for x in 0..qr.size() {
            let module = if qr.get_module(x, y) { "██" } else { "  " };
            print!("{}", module);
        }
        println!();
    }
}
