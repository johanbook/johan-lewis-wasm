use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	// Set panic hook to log rust backtrace to browser console:
	panic::set_hook(Box::new(console_error_panic_hook::hook));
	Ok(())
}

#[wasm_bindgen]
pub fn decode_qr_from_reader(file_reader: web_sys::FileReader) -> js_sys::Uint8Array {
	let img = get_image(file_reader).expect("Couldn't load image");
	decode_qr(img)
}

#[wasm_bindgen]
pub fn decode_qr_from_array(vec: js_sys::Uint8Array) -> js_sys::Uint8Array {
	let img = image::load_from_memory(&vec.to_vec()).expect("Couldn't load image");
	decode_qr(img)
}

pub fn decode_qr(img: image::DynamicImage) -> js_sys::Uint8Array {
	let img_gray = img.into_luma8();
	web_sys::console::log_1(&format!("{img_gray:?}").into());
	web_sys::console::log_1(&format!("2").into());
	let mut decoder = quircs::Quirc::default();
	let mut codes = decoder.identify(
		img_gray.width() as usize,
		img_gray.height() as usize,
		&img_gray,
	);
	let ret = js_sys::Uint8Array::default();
	ret.copy_from(&codes.next().unwrap().unwrap().decode().unwrap().payload);
	ret
}

fn get_image(file_reader: web_sys::FileReader) -> Option<image::DynamicImage> {
	let array_buffer = file_reader
		.result()
		.expect("Couldn't get file_reader result");
	let vec = js_sys::Uint8Array::new(&array_buffer).to_vec();
	match image::load_from_memory(&vec) {
		Ok(value) => Some(value),
		_ => None,
	}
}
