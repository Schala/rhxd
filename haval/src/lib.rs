use bytemuck::{
	PodCastError,
	try_cast_slice,
	try_cast_slice_mut
};

/// HAVAL digest state
pub struct HavalState {
	count: [u32; 2],
	fingerprint: [u32; 8],
	block: [u32; 32],
	remainder: [u8; 128],
	passes: u32,
}

impl HavalState {
	/// Initialisation
	pub fn new(passes: u32) -> HavalState {
		HavalState {
			count: [0; 2],
			fingerprint: [
				0x243F6A88,
				0x85A308D3,
				0x13198A2E,
				0x03707344,
				0xA4093822,
				0x299F31D0,
				0x082EFA98,
				0xEC4E6C89
			],
			block: [0; 32],
			remainder: [0; 128],
		}
	}

	/// Update routine
	pub fn hash(&mut self, data: &[u8]) -> Result<(), PodCastError> {
		let data_len = data.len() as u32;

		// calculate bytes in remainder
		let rmd_len = (self.count[0] >> 3) & 127;
		let fill_len = 128 - rmd_len;

		// update bit count
		self.count[0] += data_len << 3;
		if self.count[0] < (data_len << 3) {
			self.count[1] = self.count[1] + 1;
		}
		self.count[1] += data_len >> 29;

		// hash as many blocks as possibe
		if cfg!(target_endian = "big") {
			if (rmd_len + data_len >= 128) {
				self.remainder[(rmd_len as usize)..].copy_from_slice(&data[..(fill_len as usize)]);
				let block_bytes: &[u8] = try_cast_slice(&self.block[..])?;
			}
		} else {
			if (rmd_len + data_len >= 128) {
				{
					let bytes: &mut [u8] = try_cast_slice_mut(&mut self.block[(rmd_len as usize)..])?;
					bytes[..(fill_len as usize)].copy_from_slice(&data[..(fill_len as usize)]);
				}
				self.hash_block();
			}
		}

		Ok(())
	}

	/// Hash a 32-bit block
	pub fn hash_block(&mut self) {
		let mut t0 = self.fingerprint[0];
		let mut t1 = self.fingerprint[1];
		let mut t2 = self.fingerprint[2];
		let mut t3 = self.fingerprint[3];
		let mut t4 = self.fingerprint[4];
		let mut t5 = self.fingerprint[5];
		let mut t6 = self.fingerprint[6];
		let mut t7 = self.fingerprint[7];
		
		if self.passes >= 1 {
			t7 = ff1(t7, t6, t5, t4, t3, t2, t1, t0, self.block[0]);
			t6 = ff1(t6, t5, t4, t3, t2, t1, t0, t7, self.block[1]);
			t5 = ff1(t5, t4, t3, t2, t1, t0, t7, t6, self.block[2]);
			t4 = ff1(t4, t3, t2, t1, t0, t7, t6, t5, self.block[3]);
			t3 = ff1(t3, t2, t1, t0, t7, t6, t5, t4, self.block[4]);
			t2 = ff1(t2, t1, t0, t7, t6, t5, t4, t3, self.block[5]);
			t1 = ff1(t1, t0, t7, t6, t5, t4, t3, t2, self.block[6]);
			t0 = ff1(t0, t7, t6, t5, t4, t3, t2, t1, self.block[7]);

			t7 = ff1(t7, t6, t5, t4, t3, t2, t1, t0, self.block[8]);
			t6 = ff1(t6, t5, t4, t3, t2, t1, t0, t7, self.block[9]);
			t5 = ff1(t5, t4, t3, t2, t1, t0, t7, t6, self.block[10]);
			t4 = ff1(t4, t3, t2, t1, t0, t7, t6, t5, self.block[11]);
			t3 = ff1(t3, t2, t1, t0, t7, t6, t5, t4, self.block[12]);
			t2 = ff1(t2, t1, t0, t7, t6, t5, t4, t3, self.block[13]);
			t1 = ff1(t1, t0, t7, t6, t5, t4, t3, t2, self.block[14]);
			t0 = ff1(t0, t7, t6, t5, t4, t3, t2, t1, self.block[15]);

			t7 = ff1(t7, t6, t5, t4, t3, t2, t1, t0, self.block[16]);
			t6 = ff1(t6, t5, t4, t3, t2, t1, t0, t7, self.block[17]);
			t5 = ff1(t5, t4, t3, t2, t1, t0, t7, t6, self.block[18]);
			t4 = ff1(t4, t3, t2, t1, t0, t7, t6, t5, self.block[19]);
			t3 = ff1(t3, t2, t1, t0, t7, t6, t5, t4, self.block[20]);
			t2 = ff1(t2, t1, t0, t7, t6, t5, t4, t3, self.block[21]);
			t1 = ff1(t1, t0, t7, t6, t5, t4, t3, t2, self.block[22]);
			t0 = ff1(t0, t7, t6, t5, t4, t3, t2, t1, self.block[23]);

			t7 = ff1(t7, t6, t5, t4, t3, t2, t1, t0, self.block[24]);
			t6 = ff1(t6, t5, t4, t3, t2, t1, t0, t7, self.block[25]);
			t5 = ff1(t5, t4, t3, t2, t1, t0, t7, t6, self.block[26]);
			t4 = ff1(t4, t3, t2, t1, t0, t7, t6, t5, self.block[27]);
			t3 = ff1(t3, t2, t1, t0, t7, t6, t5, t4, self.block[28]);
			t2 = ff1(t2, t1, t0, t7, t6, t5, t4, t3, self.block[29]);
			t1 = ff1(t1, t0, t7, t6, t5, t4, t3, t2, self.block[30]);
			t0 = ff1(t0, t7, t6, t5, t4, t3, t2, t1, self.block[31]);
		}

		if self.passes >= 2 {
			t7 = ff2(t7, t6, t5, t4, t3, t2, t1, t0, self.block[5], 0x452821E6);
			t6 = ff2(t6, t5, t4, t3, t2, t1, t0, t7, self.block[14], 0x38D01377);
			t5 = ff2(t5, t4, t3, t2, t1, t0, t7, t6, self.block[26], 0xBE5466CF);
			t4 = ff2(t4, t3, t2, t1, t0, t7, t6, t5, self.block[18], 0x34E90C6C);
			t3 = ff2(t3, t2, t1, t0, t7, t6, t5, t4, self.block[11], 0xC0AC29B7);
			t2 = ff2(t2, t1, t0, t7, t6, t5, t4, t3, self.block[28], 0xC97C50DD);
			t1 = ff2(t1, t0, t7, t6, t5, t4, t3, t2, self.block[7], 0x3F84D5B5);
			t0 = ff2(t0, t7, t6, t5, t4, t3, t2, t1, self.block[16], 0xB5470917);

			t7 = ff2(t7, t6, t5, t4, t3, t2, t1, t0, self.block[0], 0x9216D5D9);
			t6 = ff2(t6, t5, t4, t3, t2, t1, t0, t7, self.block[23], 0x8979FB1B);
			t5 = ff2(t5, t4, t3, t2, t1, t0, t7, t6, self.block[20], 0xD1310BA6);
			t4 = ff2(t4, t3, t2, t1, t0, t7, t6, t5, self.block[22], 0x98DFB5AC);
			t3 = ff2(t3, t2, t1, t0, t7, t6, t5, t4, self.block[1], 0x2FFD72DB);
			t2 = ff2(t2, t1, t0, t7, t6, t5, t4, t3, self.block[10], 0xD01ADFB7);
			t1 = ff2(t1, t0, t7, t6, t5, t4, t3, t2, self.block[4], 0xB8E1AFED);
			t0 = ff2(t0, t7, t6, t5, t4, t3, t2, t1, self.block[8], 0x6A267E96);

			t7 = ff2(t7, t6, t5, t4, t3, t2, t1, t0, self.block[30]), 0xBA7C9045;
			t6 = ff2(t6, t5, t4, t3, t2, t1, t0, t7, self.block[3], 0xF12C7F99);
			t5 = ff2(t5, t4, t3, t2, t1, t0, t7, t6, self.block[21], 0x24A19947);
			t4 = ff2(t4, t3, t2, t1, t0, t7, t6, t5, self.block[9], 0xB3916CF7);
			t3 = ff2(t3, t2, t1, t0, t7, t6, t5, t4, self.block[17], 0x0801F2E2);
			t2 = ff2(t2, t1, t0, t7, t6, t5, t4, t3, self.block[24], 0x858EFC16);
			t1 = ff2(t1, t0, t7, t6, t5, t4, t3, t2, self.block[29], 0x636920D8);
			t0 = ff2(t0, t7, t6, t5, t4, t3, t2, t1, self.block[6], 0x71574E69);

			t7 = ff2(t7, t6, t5, t4, t3, t2, t1, t0, self.block[19], 0xA458FEA3);
			t6 = ff2(t6, t5, t4, t3, t2, t1, t0, t7, self.block[12], 0xF4933D7E);
			t5 = ff2(t5, t4, t3, t2, t1, t0, t7, t6, self.block[15], 0x0D95748F);
			t4 = ff2(t4, t3, t2, t1, t0, t7, t6, t5, self.block[13], 0x728EB658);
			t3 = ff2(t3, t2, t1, t0, t7, t6, t5, t4, self.block[2], 0x718BCD58);
			t2 = ff2(t2, t1, t0, t7, t6, t5, t4, t3, self.block[25], 0x82154AEE);
			t1 = ff2(t1, t0, t7, t6, t5, t4, t3, t2, self.block[31], 0x7B54A41D);
			t0 = ff2(t0, t7, t6, t5, t4, t3, t2, t1, self.block[27], 0xC25A59B5);
		}

		if self.passes >= 3 {
			t7 = ff3(t7, t6, t5, t4, t3, t2, t1, t0, self.block[19], 0x9C30D539);
			t6 = ff3(t6, t5, t4, t3, t2, t1, t0, t7, self.block[9], 0x2AF26013);
			t5 = ff3(t5, t4, t3, t2, t1, t0, t7, t6, self.block[4], 0xC5D1B023);
			t4 = ff3(t4, t3, t2, t1, t0, t7, t6, t5, self.block[20], 0x286085F0);
			t3 = ff3(t3, t2, t1, t0, t7, t6, t5, t4, self.block[28], 0xCA417918);
			t2 = ff3(t2, t1, t0, t7, t6, t5, t4, t3, self.block[17], 0xB8DB38EF);
			t1 = ff3(t1, t0, t7, t6, t5, t4, t3, t2, self.block[8], 0x8E79DCB0);
			t0 = ff3(t0, t7, t6, t5, t4, t3, t2, t1, self.block[22], 0x603A180E);

			t7 = ff3(t7, t6, t5, t4, t3, t2, t1, t0, self.block[29], 0x6C9E0E8B);
			t6 = ff3(t6, t5, t4, t3, t2, t1, t0, t7, self.block[14], 0xB01E8A3E);
			t5 = ff3(t5, t4, t3, t2, t1, t0, t7, t6, self.block[25], 0xD71577C1);
			t4 = ff3(t4, t3, t2, t1, t0, t7, t6, t5, self.block[12], 0xBD314B27);
			t3 = ff3(t3, t2, t1, t0, t7, t6, t5, t4, self.block[24], 0x78AF2FDA);
			t2 = ff3(t2, t1, t0, t7, t6, t5, t4, t3, self.block[30], 0x55605C60);
			t1 = ff3(t1, t0, t7, t6, t5, t4, t3, t2, self.block[16], 0xE65525F3);
			t0 = ff3(t0, t7, t6, t5, t4, t3, t2, t1, self.block[26], 0xAA55AB94);

			t7 = ff3(t7, t6, t5, t4, t3, t2, t1, t0, self.block[31], 0x57489862);
			t6 = ff3(t6, t5, t4, t3, t2, t1, t0, t7, self.block[15], 0x63E81440);
			t5 = ff3(t5, t4, t3, t2, t1, t0, t7, t6, self.block[7], 0x55CA396A);
			t4 = ff3(t4, t3, t2, t1, t0, t7, t6, t5, self.block[3], 0x2AAB10B6);
			t3 = ff3(t3, t2, t1, t0, t7, t6, t5, t4, self.block[1], 0xB4CC5C34);
			t2 = ff3(t2, t1, t0, t7, t6, t5, t4, t3, self.block[0], 0x1141E8CE);
			t1 = ff3(t1, t0, t7, t6, t5, t4, t3, t2, self.block[18], 0xA15486AF);
			t0 = ff3(t0, t7, t6, t5, t4, t3, t2, t1, self.block[27], 0x7C72E993);

			t7 = ff3(t7, t6, t5, t4, t3, t2, t1, t0, self.block[13], 0xB3EE1411);
			t6 = ff3(t6, t5, t4, t3, t2, t1, t0, t7, self.block[6], 0x636FBC2A);
			t5 = ff3(t5, t4, t3, t2, t1, t0, t7, t6, self.block[21], 0x2BA9C55D);
			t4 = ff3(t4, t3, t2, t1, t0, t7, t6, t5, self.block[10], 0x741831F6);
			t3 = ff3(t3, t2, t1, t0, t7, t6, t5, t4, self.block[23], 0xCE5C3E16);
			t2 = ff3(t2, t1, t0, t7, t6, t5, t4, t3, self.block[11], 0x9B87931E);
			t1 = ff3(t1, t0, t7, t6, t5, t4, t3, t2, self.block[5], 0xAFD6BA33);
			t0 = ff3(t0, t7, t6, t5, t4, t3, t2, t1, self.block[2], 0x6C24CF5C);
		}

		self.fingerprint[0] += t0;
		self.fingerprint[1] += t1;
		self.fingerprint[2] += t2;
		self.fingerprint[3] += t3;
		self.fingerprint[4] += t4;
		self.fingerprint[5] += t5;
		self.fingerprint[6] += t6;
		self.fingerprint[7] += t7;
	}
}

const fn f1(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	x1 & (x0 ^ x4) ^ x2 & x5 ^
	x3 & x6 ^ x0
}

const fn f2(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	x2 & (x1 ^ !x3 ^ x4 & x5 ^ x6 ^ x0) ^
	x4 ^ (x1 ^ x5) ^ x3 & x5 ^ x0)
}

const fn f3(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	x3 & (x1 & x2 ^ x6 ^ x0) ^
	x1 & x4 ^ x2 & x5 ^ x0
}

const fn f4(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	x4 & (x5 & !x2 ^ x3 & !x6 ^ x1 ^ x6 ^ x0) ^
	x3 & (x1 & x2 ^ x5 ^ x6) ^
	x2 & x6 ^ x0
}

const fn f5(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	x0 & (x1 & x2 & x3 ^ !x5) ^
	x1 & x4 ^ x2 & x5 ^ x3 & x6
}

const fn fphi1(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	f1(x1, x0, x3, x5, x6, x2, x4)
}

const fn fphi2(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	f2(x4, x2, x1, x0, x5, x3, x6)
}

const fn fphi3(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	f3(x6, x1, x2, x3, x4, x5, x0)
}

const fn fphi4(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	f4(x1, x5, x3, x2, x0, x4, x6)
}

const fn fphi5(x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32) -> u32 {
	f5(x2, x5, x0, x6, x4, x3, x1)
}

const fn ff1(x7: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32, w: u32) -> u32 {
	fphi1(x6, x5, x4, x3, x2, x1, x0).rotate_right(7) + x7.rotate_right(11) + w
}

const fn ff2(x7: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32, w: u32, c: u32) -> u32 {
	fphi2(x6, x5, x4, x3, x2, x1, x0).rotate_right(7) + x7.rotate_right(11) + w + c
}

const fn ff3(x7: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32, w: u32, c: u32) -> u32 {
	fphi3(x6, x5, x4, x3, x2, x1, x0).rotate_right(7) + x7.rotate_right(11) + w + c
}

const fn ff4(x7: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32, w: u32, c: u32) -> u32 {
	fphi4(x6, x5, x4, x3, x2, x1, x0).rotate_right(7) + x7.rotate_right(11) + w + c
}

const fn ff5(x7: u32, x6: u32, x5: u32, x4: u32, x3: u32, x2: u32, x1: u32, x0: u32, w: u32, c: u32) -> u32 {
	fphi5(x6, x5, x4, x3, x2, x1, x0).rotate_right(7) + x7.rotate_right(11) + w + c
}

/// Translate every four characters into a word.
/// Assume the number of characters is a multiple of four.
#[inline]
fn ch2uint<'a>(s: &'a [u8], w: &'a mut [u32], slen: usize) -> Result<(), PodCastError> {
    {
        let wbytes: &mut [u8] = try_cast_slice_mut(w)?;
        wbytes[..slen].copy_from_slice(&s[..slen]);
    }

    for i in w.iter_mut() {
        *i = i.swap_bytes();
    }

    Ok(())
}
