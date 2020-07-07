use std::fs::File;
use std::io::prelude::*;

// converts 8 bit rgba vals into a 4 byte int for storage as an img/pixel array
fn pack_colour(r: u8, g: u8, b: u8, a: u8) -> u32 {
    ((a as u32) << 24) + ((b as u32) << 16) + ((g as u32) << 8) + (r as u32)
}

// takes a 4 byte int colour and returns the 8 bit rgba values for easy use
fn unpack_colour(colour: u32) -> [u8; 3] {
    let get_val = |bits| ((colour >> bits) & 255u32) as u8;
    [get_val(0), get_val(8), get_val(16)]
}

// create an image from a vector of pixels
fn drop_ppm_image(filename: &str, image: Vec<u32>, w: usize, h: usize) -> std::io::Result<()> {
    assert!(image.len() == w*h);
    let mut file = File::create(filename)?;
    file.write(format!("P6\n{} {}\n255\n", w, h).as_bytes())?;
    for i in 0..h*w {
        // write each colour pixel in image to the file
        let rgba = unpack_colour(image[i]);
        file.write(&rgba)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    const WIN_W: usize  = 512;
    const WIN_H: usize  = 512;
    const DEFAULT_A: u8  = 255; // default value for a in rgba
    const DEFAULT_B: u8 = 0; // default value for b in rgba
    let mut framebuffer = vec![255; WIN_W*WIN_H]; // the pixel array
    
    // make a gradient by varying rgb vals along the width and height
    for i in 0..WIN_W {
        for j in 0..WIN_H {
            let r = (255*i/WIN_W) as u8;
            let g = (255*j/WIN_H) as u8;
            let b = DEFAULT_B;
            framebuffer[i + j*WIN_W] = pack_colour(r, g, b, DEFAULT_A)
        }
    }

    // output the colour image to a file
    drop_ppm_image("./out.ppm", framebuffer, WIN_W, WIN_H)
}