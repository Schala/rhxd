use bitflags::bitflags;

bitflags! {
	/// User privileges. These should have reversed bits on little endian platforms.
	/// This includes the extra GLoarb extensions.
	pub struct HLUserAccess: u64 {
		const DELETE_FILES = 1 << 1;
		const UPLOAD_FILES = 1 << 2;
		const DOWNLOAD_FILES = 1 << 3;
		const RENAME_FILES = 1 << 4;
		const MOVE_FILES = 1 << 5;
		const CREATE_FOLDERS = 1 << 6;
		const DELETE_FOLDERS = 1 << 7;
		const RENAME_FOLDERS = 1 << 8;

		const MOVE_FOLDERS = 1 << 9;
		const READ_CHAT = 1 << 10;
		const SEND_CHAT = 1 << 11;
		const CREATE_CHATS = 1 << 12;
		const CLOSE_CHATS = 1 << 13;
		const SHOW_IN_LIST = 1 << 14;
		const CREATE_USERS = 1 << 15;
		const DELETE_USERS = 1 << 16;

		const OPEN_USERS = 1 << 17;
		const EDIT_USERS = 1 << 18;
		const EDIT_OWN_PASSWORD = 1 << 19;
		const SEND_PRIVATE_MESSAGES = 1 << 20;
		const READ_NEWS = 1 << 21;
		const POST_NEWS = 1 << 22;
		const KICK_USERS = 1 << 23;
		const CANT_BE_KICKED = 1 << 24;

		const GET_USER_INFO = 1 << 25;
		const UPLOAD_ANYWHERE = 1 << 26;
		const ANY_NAME = 1 << 27;
		const NO_AGREEMENT = 1 << 28;
		const SET_FILE_COMMENTS = 1 << 29;
		const SET_FOLDER_COMMENTS = 1 << 30;
		const VIEW_DROP_BOXES = 1 << 31;
		const MAKE_ALIASES = 1 << 32;

		const BROADCAST = 1 << 33;
		const DELETE_NEWS_ARTICLES = 1 << 34;
		const CREATE_NEWS_CATEGORIES = 1 << 35;
		const DELETE_NEWS_CATEGORIES = 1 << 36;
		const CREATE_NEWS_FOLDERS = 1 << 37;
		const DELETE_NEWS_FOLDERS = 1 << 38;
		const UPLOAD_FOLDERS = 1 << 39;
		const DOWNLOAD_FOLDERS = 1 << 40;

		const SEND_MESSAGES = 1 << 41;
		const FAKE_RED = 1 << 42;
		const AWAY = 1 << 43;
		const EDIT_NAME = 1 << 44;
		const EDIT_ICON = 1 << 45;
		const SPEAK_BEFORE = 1 << 46;
		const REFUSE_CHAT_INVITES = 1 << 47;
		const BLOCK_DOWNLOADS = 1 << 48;

		const VISIBILITY = 1 << 49;
		const VIEW_INVISIBLE = 1 << 50;
		const FLOOD = 1 << 51;
		const VIEW_OWN_DROP_BOX = 1 << 52;
		const DONT_QUEUE = 1 << 53;
		const ADMIN_INSPECTOR = 1 << 54;
		const POST_BEFORE = 1 << 55;
	}
}

/// Hotline user account. This is included in the HOPE "derivitive"
pub struct HLAccount {
	pub name: Vec<u8>,
	pub login: Vec<u8>,
	pub password: Vec<u8>,
	pub access: HLUserAccess,
}
