use rhxd_hotline::files::HLFile;

/// File hash information
#[derive(Clone, Debug)]
pub struct HOPEFileHashInfo {
	pub md5: [u8; 16],
	pub haval: [u8; 16],
	pub sha1: [u8; 20],
}

/// HOPE file information extension
#[derive(Copy, Clone, Debug)]
pub struct HOPEFile {
	pub file: HLFile;
	pub hash_info: HOPEFileHashInfo,
}
