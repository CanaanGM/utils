use uuid::Uuid;
use rand::seq::SliceRandom; 

use wasm_bindgen::prelude::*;



#[wasm_bindgen]
pub fn gen_uuid() -> String{

	let uid = Uuid::new_v4();
    uid.to_string()
}

#[wasm_bindgen]
pub fn gen_custom_uuid() -> String {

    let sections = [8,4,4,4,12];
    let mut custom_id = String::new();

    for sections_size in sections {
        custom_id = format!("{}{}", custom_id,  generate_section(sections_size) );

        // stupid to hardcode, but the spec of uuid will not change anytime soon
        if sections_size != 12{
            custom_id = format!("{}-", custom_id);
        }
    }
    custom_id
}


fn generate_section(size: i32) -> String {
    // numbers are faster than allocating a String ? ? ? look it up
    let chars_numbers =  [
        "q","w","e","r","t","y","u","i","o","p","l","k","a","m",
        "s","n","z","j","x","h","c","b","d","g","f","v","1","2","3","4","5","6","7","8","9","0"
        ];

    let mut rnd_char = String::new();
    
    for _ in 0..size {
        rnd_char =  format!("{}{}", rnd_char, chars_numbers.choose(&mut rand::thread_rng()).unwrap().to_owned() );
    }

    rnd_char
}