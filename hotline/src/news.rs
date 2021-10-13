use bytes::Bytes;

mod datetime;
use datetime::HLDateTime;

/// A single Hotline news article post
#[derive(Clone, Debug]
pub struct HLNewsArticle {
	pub id: u32,
	pub timestamp: HLDateTime,
	pub parent_id: u32,
	pub title: Vec<u8>,
	pub poster: Vec<u8>,
	pub flavors: Option<Vec<Vec<u8>>>,
}

impl HLNewsArticle {
	/// Resolves an article's parent ID
	pub fn resolve_parent(&self, list: &[HLNewsArticle]) -> u32 {
		match list.iter().find(|&&article| self.parent_id == article.id) {
			Some(id) => id,
			None => 0,
		}
	}
}
