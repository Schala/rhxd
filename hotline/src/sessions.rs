use bitflags::bitflags;

bitflags! {
	/// User message flags
	pub struct HLUserMsgFlags: u16 {
		pub const REFUSE_PRIVATE_MSGS = 1;
		pub const REFUSE_PRIVATE_CHAT = 2;
		pub const AUTO_REPLY = 4;
	}

	/// User status
	pub struct HLUserFlags: u16 {
		pub const AWAY = 1;
		pub const ADMIN = 2;
		pub const REFUSE_PRIVATE_MSGS = 4;
		pub const REFUSE_PRIVATE_CHAT = 8;
	}
}

#[derive(Debug)]
pub struct HLSession {
	pub id: u16,
	pub icon: u16,
	pub msg_flags: HLUserMsgFlags,
	pub flags: HLUserFlags,
	pub version: u16,
	pub account: Arc<HLAccount>,
	pub alias: Vec<u8>,
}
