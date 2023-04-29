use uuid::Uuid;


use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn gen_uuid() -> String{

	let uid = Uuid::new_v4();
    uid.to_string()
}
