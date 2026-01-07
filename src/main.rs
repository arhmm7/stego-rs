use std::{env, fs};
use image::{Rgb, open};

fn encode(text_path: &str, image_path: &str, output_path: &str) {
    let message = fs::read_to_string(text_path).unwrap();
    let data = message.as_bytes();
    
    let mut img = open(image_path).unwrap().to_rgb8();
    let (width, height) = img.dimensions();
    
    let max_bytes = (width * height * 3) / 8;
    if data.len() + 4 > max_bytes as usize {
        panic!("Message too large for image. Max {} bytes", max_bytes - 4);
    }
    
    let len_bytes = (data.len() as u32).to_le_bytes();
    let mut all_data = Vec::new();
    all_data.extend_from_slice(&len_bytes);
    all_data.extend_from_slice(data);
    
    let mut bit_idx = 0;
    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let mut new_pixel = [pixel[0], pixel[1], pixel[2]];
            
            for j in 0..3 {
                let byte_idx = bit_idx / 8;
                let bit_pos = 7 - (bit_idx % 8);
                
                if byte_idx < all_data.len() {
                    let bit = (all_data[byte_idx] >> bit_pos) & 1;
                    new_pixel[j] = (new_pixel[j] & 0xFE) | bit;
                    bit_idx += 1;
                } else {
                    break 'outer;
                }
            }
            img.put_pixel(x, y, Rgb(new_pixel));
        }
    }
    
    img.save(output_path).expect("Failed to save image");
    println!("Encoded {} bytes to {}", data.len(), output_path);
}

fn decode(image_path: &str, output_path: &str) {
    let img = open(image_path).unwrap().to_rgb8();
    let (width, height) = img.dimensions();
    
    let mut bits = Vec::new();
    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            for j in 0..3 {
                bits.push(pixel[j] & 1);
                if bits.len() >= 32 + (1024 * 1024 * 8) {
                    break 'outer;
                }
            }
        }
    }
    
    let mut len_bytes = [0u8; 4];
    for i in 0..4 {
        let mut byte = 0u8;
        for j in 0..8 {
            byte |= bits[i * 8 + j] << (7 - j);
        }
        len_bytes[i] = byte;
    }
    let msg_len = u32::from_le_bytes(len_bytes) as usize;
    
    if msg_len == 0 || msg_len > (bits.len() / 8) - 4 {
        println!("No valid message found or corrupted data");
        fs::write(output_path, &[]).unwrap();
        return;
    }
    
    let mut message = Vec::new();
    for i in 0..msg_len {
        let mut byte = 0u8;
        for j in 0..8 {
            byte |= bits[32 + i * 8 + j] << (7 - j);
        }
        message.push(byte);
    }
    
    fs::write(output_path, &message).unwrap();
    println!("Decoded {} bytes to {}", message.len(), output_path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage:");
        println!("  Encode: {} encode <text_file> <input_image> <output_image>", args[0]);
        println!("  Decode: {} decode <image_file> <output_text>", args[0]);
        return;
    }
    
    match args[1].as_str() {
        "encode" => {
            if args.len() != 5 {
                println!("Usage: {} encode <text_file> <input_image> <output_image>", args[0]);
                return;
            }
            encode(&args[2], &args[3], &args[4]);
        }
        "decode" => {
            if args.len() != 4 {
                println!("Usage: {} decode <image_file> <output_text>", args[0]);
                return;
            }
            decode(&args[2], &args[3]);
        }
        _ => {
            println!("Invalid command. Use 'encode' or 'decode'");
        }
    }
}