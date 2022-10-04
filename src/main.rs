use std::{env, process::exit, io::Read};
use std::fs::File; 
use std::str;
use miniz_oxide::inflate::decompress_to_vec_zlib;
use minifb::{Window, Key, WindowOptions};

struct Chunk{ 
    chunk_type: [u8; 4],
    chunk_data: Vec<u8>, 
    crc: [u8; 4]
}

impl Chunk {

    fn length(&self) -> usize{
        self.chunk_type().len()
    }

    fn chunk_type(&self) -> &str{
        str::from_utf8(&self.chunk_type).unwrap()
    }

    fn is_end(&self) -> bool { 
        return self.chunk_type() == "IEND"
    }

    fn verify_crc(&self) -> bool { 

        println!("TODO: verify CRC {:?}",self.crc);
        return true
    }
    
}

struct Chunks{
    data: Vec<Chunk>,
}

impl Chunks{

    fn new() -> Chunks{
        Chunks{
            data: Vec::new(),
        }
    }

    fn add_chunk(& mut self, img_file: &mut File) -> bool{

        let mut length_buffer =  [0;4]; 
        img_file.read_exact(& mut length_buffer).expect("Unable to read chunk length"); 


        let mut chunk_type_buffer = [0;4];
        img_file.read_exact(& mut chunk_type_buffer).expect("Unable to read chunk type"); 


        let chunk_length: u32 = u32::from_be_bytes(length_buffer);
        let mut chunk_data_buffer: Vec<u8> = vec![0; chunk_length as usize];
        img_file.read(& mut chunk_data_buffer).expect("Unable to read chunk data");
    

        let mut crc_buffer = [0;4]; 
        img_file.read(&mut crc_buffer).expect("Unable to read crc");

        let curr_chunk = Chunk{
            chunk_type: chunk_type_buffer, 
            chunk_data: chunk_data_buffer, 
            crc: crc_buffer
        };

        let is_end = curr_chunk.is_end();

        println!("Chunk type: {}", curr_chunk.chunk_type());

        match curr_chunk.verify_crc(){
            true => {
                self.data.push(curr_chunk);
            },
            false => {
                println!("Couldn't Verify CRC");
            }
        }

        is_end

        
    }

    fn display_header(&self){

        let header = &self.data[0];
        assert_eq!(header.chunk_type(), "IHDR");

        assert!(header.length() > 0);

        let width = u32::from_be_bytes(header.chunk_data[0..4].try_into().unwrap());
        let height = u32::from_be_bytes(header.chunk_data[4..8].try_into().unwrap());
        let bit_depth = u8::from_be_bytes(header.chunk_data[8..9].try_into().unwrap());
        let color_type = u8::from_be_bytes(header.chunk_data[9..10].try_into().unwrap());
        let compression_method = u8::from_be_bytes(header.chunk_data[10..11].try_into().unwrap());
        let filter_method = u8::from_be_bytes(header.chunk_data[11..12].try_into().unwrap());
        let interlace_method = u8::from_be_bytes(header.chunk_data[12..13].try_into().unwrap());

        println!("-----IMAGE METADATA-----");
        println!("Image Width: {}, Image Height: {}", width, height);
        println!("Bit Depth: {}, Color type: {}, compression_method: {}, filter_method: {}, interplace_method: {}", bit_depth, color_type, compression_method,filter_method, interlace_method );
        println!("------------------------");
    }

    fn get_dimensions(&self) -> [usize;2]{
        let header = &self.data[0];
        assert_eq!(header.chunk_type(), "IHDR");

        let width = u32::from_be_bytes(header.chunk_data[0..4].try_into().unwrap()) as usize;
        let height = u32::from_be_bytes(header.chunk_data[4..8].try_into().unwrap()) as usize;

        return [width, height]
    }

    fn get_all_idat_data(&self) -> Vec<u8>{
        let mut result: Vec<u8> = Vec::new(); 

        for chunk in &self.data{
            if chunk.chunk_type() == "IDAT"{
                for val in &chunk.chunk_data{
                    result.push(val.clone());
                }
            }
        }

        result
    }

    fn read_file(& mut self, img_file: & mut File){

        loop{
            if self.add_chunk(img_file){
                break
            }
        }
    }

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
    
    
    assert_eq!(file_signature, [137, 80, 78, 71, 13, 10, 26, 10], "Invalid PNG Header");

    let mut chunks = Chunks::new();

    chunks.read_file(& mut img_file);

    chunks.display_header();
    
    let [width, height] = chunks.get_dimensions();
    let idat_data = chunks.get_all_idat_data();
    let decompress:Vec<u8> = decompress_to_vec_zlib(&idat_data).unwrap();

    let mut pixels: Vec<u32> = Vec::new();

    let stride = width * 4 + 1;

    for i in 0.. decompress.len()/stride as usize {

        let start = stride * i  + 1; 
        let end  = stride * i + stride;

        let row = &decompress[start..end ];
        
        for j in 0 ..row.len()/4 {

            let a = row[j * 4 + 3] as u32 /255 ;

            let mut r = row[j * 4] as u32 * a;
            // dbg!(r);
            r = r << 16;
            let mut g = row[j * 4 + 1] as u32 * a;
            // dbg!(g);
            g = g << 8;
            let b = row[j * 4 + 2] as u32 * a;
            // dbg!(b);

            let color = r + g + b;

            pixels.push(color);
        }
    }

    let mut window = Window::new( 
        "test - Esc to exit", 
        width, 
        height, 
        WindowOptions::default()
    ).expect("Could not open window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(10)));

    while window.is_open() && !window.is_key_down(Key::Escape) {

        window
            .update_with_buffer(&pixels, width, height)
            .unwrap();
    }
}
