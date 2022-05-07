use infra;

#[test]
fn test_qr_code_verification() {
    let mut is_valid = false;
    let filepath = "./tests/test_image2.jpg";
    let input_code = "123456789";
    is_valid = infra::qrcode::verify_qr_code(filepath.into(), input_code.into());
    assert_eq!(is_valid, false);
}