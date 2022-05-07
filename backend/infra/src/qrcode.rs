use std::path::Path;
pub fn verify_qr_code(filepath: String, input_code: String) -> bool {
    let mut is_valid = false;
    let file_result = image::open(Path::new(&filepath)).ok();
    if file_result.is_none() {
        println!("File not found {}", &filepath);
        return is_valid;
    }
    if let Some(img) = file_result {
        let img_luma = img.to_luma();
        let mut prepared = rqrr::PreparedImage::prepare(img_luma);
        // Search for grids, without decoding
        let grids = prepared.detect_grids();
        assert_eq!(grids.len(), 1);
        // Decode the grid
        let (meta, content) = grids[0].decode().unwrap();
        println!("{:?}", meta);
        println!("{:?}", content);
    }
    return is_valid;
}
