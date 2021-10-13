pub const FOLDER_TYPE_CODE: u32 = u32::from_be_bytes(b"fldr");

/// Hotline file fork magics
#[repr(u32)]
pub enum HLFileForkType {
	Data = u32::from_be_bytes(b"DATA"),
	MacResource = u32::from_be_bytes(b"MACR"), // obsolete
}

/// File fork metadata
#[derive(Copy, Clone, Debug)]
pub struct HLFileForkInfo {
	pub fork: HLFileForkType,
	pub size: u32,
}

/// Information on a single file
#[derive(Clone, Debug)]
pub struct HLFile {
	pub mac_kind: u32,
	pub mac_creator: u32,
	pub comment: Vec<u8>,
	pub fork_info: [HLFileForkInfo; 2],
}