use std::fs::File;
use std::io::prelude::*;

// converts 8 bit rgba vals into a 4 byte int for storage as an img/pixel array
fn pack_colour(r: u32, g: u32, b: u32, a: u32) -> u32 {
    (a << 24) + (b << 16) + (g << 8) + r
}

// takes a 4 byte int colour and returns the 8 bit rgba values for easy use
fn unpack_colour(colour: u32) -> (u32, u32, u32) {
    let get_val = |bits| ((colour >> bits) & 255u32);
    (get_val(0), get_val(8), get_val(16))
}

// create an image from a vector of pixels
fn drop_ppm_image(filename: &str, image: Vec<u32>, w: usize, h: usize) -> std::io::Result<()> {
    assert!(image.len() == w*h);
    let mut file = File::create(filename)?;
    file.write(format!("P6\n{} {}\n255\n", w, h).as_bytes())?;
    for i in 0..h*w {
        // write each colour pixel in image to the file
        let (r, g, b) = unpack_colour(image[i]);
        file.write(&[r as u8])?;
        file.write(&[g as u8])?;
        file.write(&[b as u8])?;
    }
    Ok(())
}

fn generate_map(w: usize, h: usize) -> [u32; 16*16] {
    //const map: Vec<u32> = vec!
    [
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
        0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
        0,1,1,1,1,1,1,0,0,0,0,0,0,1,1,0,
        0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,0,
        0,1,1,1,1,1,0,1,1,1,1,1,1,1,1,0,
        0,1,1,1,1,1,0,1,1,1,0,1,1,1,1,0,
        0,1,1,0,0,0,0,0,0,0,1,1,1,1,1,0,
        0,1,1,1,0,1,1,1,1,0,1,1,1,1,1,0,
        0,1,1,1,0,1,1,1,1,0,0,0,0,1,1,0,
        0,1,1,1,0,1,1,1,1,0,1,1,1,1,1,0,
        0,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0,
        0,1,1,1,1,1,1,1,1,0,1,1,0,0,0,0,
        0,1,1,1,1,1,1,1,1,0,1,1,1,1,1,0,
        0,1,0,0,0,0,0,0,0,0,0,1,1,1,1,0,
        0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,
        0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
    //map
}

fn main() -> std::io::Result<()> {
    // window height/width
    const WIN_W: usize = 512; 
    const WIN_H: usize = 512;
    // map height width
    let map_w: usize = 16;
    let map_h: usize = 16; 
    // pixels per map tile
    let tile_w = WIN_W/map_w;
    let tile_h = WIN_H/map_h;
    // default value for a in rgba
    const DEFAULT_A: u32 = 255; 
    // wall colour for map generation
    let (wall_r, wall_g, wall_b): (u32, u32, u32) = (0, 255, 255);
    // wall identifier in map
    const WALL: u32 = 0;

    let mut framebuffer = vec![255; WIN_W*WIN_H]; // the pixel array
    
    // make a gradient by varying rgb vals along the width and height
    let map = generate_map(map_w, map_h);
    for i in 0..WIN_W {
        for j in 0..WIN_H {
            // check if this pixel is on a wall tile or not
            let map_i = i/tile_w;
            let map_j = j/tile_h;
            let (r, g, b): (u32, u32, u32);
            if map[map_i + map_j*map_h] == WALL {
                r = wall_r;
                g = wall_g;
                b = wall_b;
            } else {
                r = (255*i/WIN_W) as u32;
                g = (255*j/WIN_H) as u32;
                b = 0;
            }
            framebuffer[i + j*WIN_H] = pack_colour(r, g, b, DEFAULT_A)
        }
    }


    // output the colour image to a file
    drop_ppm_image("./out.ppm", framebuffer, WIN_W, WIN_H)
}