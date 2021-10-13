#![allow(non_camel_case_types)]
include!("bindings.rs");

use std::env::args;

/// Generates a HAVAL-128 checksum for a file
pub fn haval_file_checksum(filename: &[u8]) -> Option<[u8; 16]> {
	let mut fingerprint: [u8; 16] = [0; 16];
	unsafe {
		match haval_file(filename.as_ptr() as *mut i8, fingerprint.as_mut_ptr()) {
			0 => Some(fingerprint),
			_ => None,
		}
	}
}

fn main() {
	let argv: Vec<String> = args().collect();
	let sum = haval_file_checksum(argv[1].as_bytes());

	println!("{:02X?}", sum);
}
