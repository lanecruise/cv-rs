//! The test suite in this file is adapted from:
//! https://docs.opencv.org/3.1.0/d3/df2/tutorial_py_basic_ops.html
//!
//! N.B. Blue pixel value at (100, 100) is actually 156, NOT 157...

extern crate cv;
mod utils;

#[test]
fn test_accessing_pixel() {
    let img = utils::load_messi_color();

    let pixel = img.at2::<(u8, u8, u8)>(100, 100);
    assert_eq!(pixel.0, 156);
    assert_eq!(pixel.1, 166);
    assert_eq!(pixel.2, 200);

    let blue = img.at3::<u8>(100, 100, 0);
    assert_eq!(blue, 156);

    // img.set(100, 100, (255, 255, 255));
    // assert!(img.at(100, 100), (255, 255, 255));
}
