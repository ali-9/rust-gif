
fn main() {
  // use std::fs::File;

  use gif::SetParameter;
  use std::fs::File;
  use std::io::BufWriter;
  use std::path::Path;
  use std::time::Instant;
  let mut decoder = gif::Decoder::new(File::open("test3.gif").unwrap());
  use gif_dispose::{Screen, RGBA8};
  // Configure the decoder such that it will expand the image to RGBA.
  // decoder.set(gif::ColorOutput::RGBA);
  decoder.set(gif::ColorOutput::Indexed);
  // Read the file header
  let mut decoder = decoder.read_info().unwrap();
  let mut i: i32 = 0;

  let now = Instant::now();
  let mut screen = Screen::new_reader(&decoder);
  println!("{} ,{}", decoder.width(), decoder.height());

  loop {
    match decoder.read_next_frame() {
      Ok(option_frame) => {
         let frame =match option_frame {
          None => break ,
          Some(frame) => frame
         };
          // Process every frame
          let stri: String = i.to_string();
          let dir: String = "./imgs/image".to_string() + &stri + &".png".to_string();
          println!("{}", dir);
         
             i+=1;
         
          let path = Path::new(&dir);
          let file = File::create(path).unwrap();
          let ref mut w = BufWriter::new(file);

          let mut encoder = png::Encoder::new(
            w,
            screen.pixels.width() as u32,
            screen.pixels.height() as u32,
          ); // Width is 2 pixels and height is 1.
          encoder.set_color(png::ColorType::RGBA);
          encoder.set_depth(png::BitDepth::Eight);
          let mut writer = encoder.write_header().unwrap();
          //      println!("{:?}",frame.dispose );
          screen.blit_frame(&frame).unwrap();
          //   screen.blit(&frame);
          // screen.pixels // that's the frame now

          let data: &Vec<RGBA8> = &*screen.pixels.buf(); //.into_iter().map(|rgba| rgba).rev();
          let c: Vec<u8> = data.iter().flat_map(|x| x.iter()).collect();
          // let o :Vec<u8>=c.iter().map(|x| **x).clone().collect();
          let o1 = &c[..];

          writer.write_image_data(o1).unwrap(); // Save
        
      }
      Err(err) => {
        println!("{:?}",err);
        break 
      },
    };
  
  }

  println!("{} sec elapsed!", now.elapsed().as_secs());
}
