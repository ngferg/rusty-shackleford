use anyhow::Result; // Automatically handle the error types
use opencv::{highgui, prelude::*, videoio}; // Note, the namespace of OpenCV is changed (to better or worse). It is no longer one enormous.

fn main() -> Result<()> {
    // // Open a GUI window
    // highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    // Open the web-camera (assuming you have one)
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the web-cam data
    cam.read(&mut frame)?;
    // let pixels = frame.data_bytes()?.len() / 3;
    const ROW_LEN: usize = 1920;
    const COL_LEN: usize = 1080;
    const PIXEL_STEP: usize = 30;

    // Read the camera and display in the window
    loop {
        cam.read(&mut frame)?;

        for y in (0..COL_LEN).step_by(PIXEL_STEP) {
            for x in (1..ROW_LEN).step_by(PIXEL_STEP) {
                // print!("({:0>4},{:0>4})", x, y);
                // print!("{:0>8},", x*3+y*ROW_LEN*3);
                print!(
                    "{}",
                    map_pixel_to_ascii(
                        frame
                            .data_bytes()?
                            .get(x * 3 + y * ROW_LEN * 3)
                            .unwrap_or(&0)
                    )
                )
            }
            println!(" ");
        }
    }
    Ok(())
}

fn map_pixel_to_ascii(pixel: &u8) -> char {
    match pixel {
        0..16 => '.',
        16..32 => ',',
        32..48 => '_',
        48..64 => '-',
        64..80 => '•',
        80..96 => '+',
        96..112 => '=',
        112..128 => 'o',
        128..144 => 'O',
        144..160 => '*',
        160..176 => '#',
        176..192 => '%',
        192..208 => '&',
        208..224 => '$',
        224..240 => '£',
        240.. => '@',
    }
}
