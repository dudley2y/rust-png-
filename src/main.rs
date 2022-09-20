use std::{env, process::exit, io::Read};
use image::{io::Reader };
use minifb::{Key, Window, WindowOptions};
use std::fs::File; 
use std::str;

pub fn ihdr_decode(chunk: Vec<u8> ){
    println!("Decoding IHDR!");

    let mut width_buffer = [0 ;4];
    width_buffer[0..4].clone_from_slice(&chunk[0..4]);
    let width = u32::from_be_bytes(width_buffer);


    let mut height_buffer = [0 ;4];
    height_buffer[0..4].clone_from_slice(&chunk[4..8]);
    let height = u32::from_be_bytes(height_buffer);

    let mut bit_depth_buffer = [0 ;1];
    bit_depth_buffer[0..1].clone_from_slice(&chunk[8..9]);
    let bit_depth  = bit_depth_buffer[0];

    let mut color_type_buffer = [0 ;1];
    color_type_buffer[0..1].clone_from_slice(&chunk[9..10]);
    let color_type = color_type_buffer[0];

    let mut compression_method_buffer = [0 ;1];
    compression_method_buffer[0..1].clone_from_slice(&chunk[10..11]);
    let compression_method = compression_method_buffer[0];

    let mut filter_method_buffer = [0 ;1];
    filter_method_buffer[0..1].clone_from_slice(&chunk[11..12]);
    let filter_method = filter_method_buffer[0];

    let mut interplace_method_buffer = [0 ;1];
    interplace_method_buffer[0..1].clone_from_slice(&chunk[12..13]);
    let interplace_method = interplace_method_buffer[0];

    println!("Image Width: {}, Image Height: {}", width, height);
    println!("Bit Depth: {}, Color type: {}, compression_method: {}, filter_method: {}, interplace_method: {}", bit_depth, color_type, compression_method,filter_method, interplace_method );

    // TODO: READ 4.1.2: http://www.libpng.org/pub/png/spec/1.2/PNG-Chunks.html#C.Critical-chunks
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("Usage: %s <input.png>");
        println!("Error: no input file is provided");
        exit(1);
    }
    else if args.len() > 2 {
        println!("Usage: %s <input.png>");
        println!("Error: too many parameters");
        exit(1);
    }

    let filename = &args[1]; 

    println!("Inspected file is {}", filename);

    let mut img_file = File::open(filename).expect("Unable to open file");
    let mut file_signature = [0;8];
    img_file.read_exact(& mut file_signature).expect("Unable to read PNG signature");
    
    match file_signature { 
        [137, 80, 78, 71, 13, 10, 26, 10] => println!("Valid PNG signature!"),
        _ => panic!("Invalid PNG file signature")
    }

    loop{

        let mut length_buffer =  [0;4]; 
        img_file.read_exact(& mut length_buffer).expect("Unable to read chunk length"); 
        let chunk_length: u32 = u32::from_be_bytes(length_buffer);
    
        let mut chunk_type_buffer = [0;4];
        img_file.read_exact(& mut chunk_type_buffer).expect("Unable to read chunk type"); 
        let chunk_type = str::from_utf8(&chunk_type_buffer).unwrap();
    
        let mut chunk_data_buffer: Vec<u8> = vec![0; chunk_length as usize];
        img_file.read(& mut chunk_data_buffer).expect("Unable to read chunk data");
    
        let mut crc_buffer = [0;4]; 
        img_file.read(&mut crc_buffer).expect("Unable to read crc");
        // TODO: VALIDATE CRC 

        match chunk_type {
            "IHDR" => ihdr_decode(chunk_data_buffer),
            "IEND" => break,
            _ => println!("Not Implemented: {}", chunk_type)
        }
    }




    // let reader = Reader::open(filename).expect("Unable to open file");
    // let file_data = reader.decode().expect("Failed to read image");
    // dbg!(file_data.color());
    // let mut buffer: Vec<u32> = file_data.as_rgba8().expect("Could not get rgb").pixels().map(
    //     |x| {

    //         let blue = x.0[2] as u32;   

    //         let mut green = x.0[1] as u32; 
    //         green  = green << 8;

    //         let mut red = x.0[0] as u32;
    //         red = red << 16;

    //         blue + green + red 
    //     }
    // ).collect();

    // let width = usize::try_from(file_data.width()).unwrap(); 
    // let height = usize::try_from(file_data.height()).unwrap();

    // let width_blocks_size: usize = 100; 
    // let height_blocks_size: usize = 100;

    // let number_width_blocks = width / width_blocks_size ; 
    // let number_height_blocks = height / height_blocks_size;

    // println!("Trying to create window with Width: {}, Height: {}", width, height);
    // println!("Image has {} pixels (width * height = {})", buffer.len(), width * height);
    

    // let mut window = Window::new( 
    //     "test - Esc to exit", 
    //     width, 
    //     height, 
    //     WindowOptions::default()
    // ).expect("Could not open window");

    // window.limit_update_rate(Some(std::time::Duration::from_micros(10)));

    // while window.is_open() && !window.is_key_down(Key::Escape) {

    //     buffer = buffer.into_iter().map(
    //         |x| {
    //             if x == u32::MAX{
    //                 0
    //             }
    //             else{
    //                 x+1
    //             }
    //         }
    //     ).collect();

    //     window
    //         .update_with_buffer(&buffer, width, height)
    //         .unwrap();
    // }

}
