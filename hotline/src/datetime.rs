use chrono::{Datelike,DateTime,Timelike,TimeZone};

const MONTH_SECS: [u32; 12] = [
	0,
	2678400,
	5097600,
	7776000,
	10368000,
	13046400,
	15638400,
	18316800,
	20995200,
	23587200,
	26265600,
	28857600
];

/// Hotline format date-time structure
#[derive(Copy, Clone, Debug)]
pub struct HLDateTime {
	pub year: u16,
	pub msecs: u16,
	pub secs: u32,
}

impl<T: TimeZone> From<DateTime<T>> for HLDateTime {
	fn from(dt: DateTime<T>) -> Self {
		let month = dt.month0() as usize;
		let leap_secs = if (dt.year() % 4 == 0) && month > 1 { 86400 } else { 0 };

		Self {
			year: dt.year() as u16,
			msecs: (dt.nanosecond() / 1000000) as u16,
			secs: MONTH_SECS[month] + leap_secs + (dt.second() + (60 * dt.minute() +
				(60 * dt.hour() + (24 * dt.day0())))),
		}
	}
}
