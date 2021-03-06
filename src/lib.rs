extern crate wasm_bindgen;
extern crate hex;
extern crate crypto;

use crypto::{ scrypt };
use wasm_bindgen::prelude::*;
use std::iter::repeat;

#[wasm_bindgen]
pub fn scrypt(password: &str, salt: &str,  n: u32, r: u32, p: u32, dklen: usize) -> String {
    let err_str= String::from("input Error");
    let log_n = (32 - n.leading_zeros() - 1) as u8;
    if log_n as u32 >= r * 16 {
		return String::from("Invalid r");
	}
	if p as u64 > ((u32::max_value() as u64 - 1) * 32)/(128 * (r as u64)) {
		return String::from("Invalid p");
	}
    let mut result: Vec<u8> = repeat(0).take(dklen).collect();
     let params = scrypt::ScryptParams::new(log_n,r,p);
     let pass_ = match hex::decode(password){
         Ok(p) => p,
         Err(_err) => return err_str,
     };
     let salt_ = match hex::decode(salt){
         Ok(p) => p,
         Err(_err) => return err_str,
     };
     scrypt::scrypt(&pass_, &salt_, &params, &mut result);
     hex::encode(result)
}