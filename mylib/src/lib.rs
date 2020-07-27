
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use gif::SetParameter;
use gif_dispose::{Screen,RGBA8};
use js_sys::{Array,Uint8Array};


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}



#[wasm_bindgen]
pub fn test (array: &[u8])->  Array{
  

    let mut slice: &[u8] = array;
    let mut decoder = gif::Decoder::new(&mut slice);
    
    decoder.set(gif::ColorOutput::Indexed);
//    decoder.set(gif::ColorOutput::RGBA);
    // Read the file header

    let mut decoder = decoder.read_info().unwrap();
    let mut screen = Screen::new_reader(&decoder);

    let mut i:i32=0;

    let  mut buffers:Vec<Vec<u8>> = Vec::new();

    let  frames_info: Array = Array::new();

  
loop{
  match decoder.read_next_frame() { 
        Ok(next_frame) => {
           if let Some(frame) =  next_frame{
            
            let stri:String = i.to_string();
            let dir:String = "frame".to_string() + &stri ;
           console_log!("{}",dir);
            i+=1;
            screen.blit_frame(&frame).unwrap();
          
            let data: &Vec<RGBA8> =&*screen.pixels.buf();
            let c: Vec<u8> = data.iter().flat_map(|x| x.iter()).collect();
           
            buffers.push(c);
        
    
    
         
        
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"delay".into(), &frame.delay.into()).unwrap();
            js_sys::Reflect::set(&obj, &"top".into(), &frame.top.into()).unwrap();
            js_sys::Reflect::set(&obj, &"left".into(), &frame.left.into()).unwrap();
            js_sys::Reflect::set(&obj, &"width".into(), &frame.width.into()).unwrap();
            js_sys::Reflect::set(&obj, &"height".into(), &frame.height.into()).unwrap();
        
    
            frames_info.push(&obj.into());
           }else {
               break;
           };
        }, 
        Err(err)=> {
            console_log!("{:?}",err);
            break;
        }

    }

}
     
//add gif info 

let obj = js_sys::Object::new();
js_sys::Reflect::set(&obj, &"gif_width".into(), &decoder.width().into()).unwrap();
js_sys::Reflect::set(&obj, &"gif_height".into(), &decoder.height().into()).unwrap();
frames_info.push(&obj.into());



let array: Array = buffers.iter().map(|x| Uint8Array::from(&x[..])).collect();
array.push(&frames_info);
array
}

