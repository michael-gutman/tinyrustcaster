// converts 8 bit rgba vals into a 4 byte int for storage as an img/pixel array
fn pack_colour(r: u8, g: u8, b: u8, a: u8) -> i32 {
    ((a as i32) << 24) + ((b as i32) << 16) + ((g as i32) << 8) + (r as i32)
}

// takes a 4 byte int colour and returns the 8 bit rgba values for easy use
fn unpack_colour(colour: i32) -> (u8, u8, u8, u8) {
    let get_val = |bits| ((colour >> bits) & 255) as u8;
    (get_val(0), get_val(8), get_val(16), get_val(24))
}

fn main() {
    println!("{}", pack_colour(11,246,75,77));
    println!("{:?}", unpack_colour(-1))
}