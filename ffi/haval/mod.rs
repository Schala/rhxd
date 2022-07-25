mod internal {
	#![allow(non_camel_case_types)]
	include!("bindings.rs");
}

/// Generates a HAVAL-128 checksum for a file
pub fn haval_file(filename: &[u8]) -> Result<[u8; 16], i32> {
	let mut fingerprint = [0; 16];
	unsafe {
		let res = internal::haval_file(filename.as_ptr() as *mut i8,
			fingerprint.as_mut_ptr());
		match res {
			0 => Ok(fingerprint),
			_ => Err(res),
		}
	}
}

/// Generates a HAVAL-128 checksum for a byte string
pub fn haval_bytes(data: &[u8]) -> [u8; 16] {
	let mut output = [0; 16];
	unsafe {
		internal::haval_string(data.as_ptr() as *mut i8, output.as_mut_ptr());
	}
	output
}

#[test]
fn test_haval() {
	println!("{:02X?}", haval_bytes("hello world".as_bytes()));
}
