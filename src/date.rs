
use chrono::prelude::*;

use Month::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
	pub year: Year,
	pub month: Month,
	pub day: Day
}
impl Date {
	pub fn new<Y: Into<Year>, D: Into<Day>>(year: Y, month: Month, day: D) -> Date {
		Date {
			year: year.into(),
			month: month,
			day: day.into(),
		}
	}
	pub fn get_day_of_year(self) -> DayOfYear {
		ymd_to_day_of_year(self)
	}
}
impl From<NaiveDate> for Date {
	fn from(date: NaiveDate) -> Date {
		let y = Year(date.year() as i64);
		let doy = DayOfYear(date.ordinal() as u16);
		doy.to_ymd(y)
	}
}
impl<Y, D> From<(Y, Month, D)> for Date where Y: Into<Year>, D: Into<Day> {
	fn from((y, m, d): (Y, Month, D)) -> Date {
		Date::new(y.into(), m, d.into())
	}
}

/// Day stored 1-indexed from the start of the year
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Into, From)]
pub struct DayOfYear(pub u16);
impl DayOfYear {
	pub fn to_ymd(self, year: Year) -> Date {
		day_of_year_to_ymd(self, year)
	}
	
	pub(crate) fn get_progress_through_year(&self, year: Year) -> f64 {
		let num_days = year.get_num_days() as f64;
		((self.0 - 1) as f64 / num_days).max(0.0).min(1.0)
	}
	
	pub(crate) fn from_progress_through_year(year: Year, mut progress: f64) -> DayOfYear {
		if progress < 0.0 {
			progress = 0.0;
		} else if progress >= 1.0 {
			progress = 1.0;
		}
		let mut doy = DayOfYear((year.get_num_days() as f64 * progress).floor() as u16 + 1);
		if doy.0 > year.get_num_days() {
			doy.0 = year.get_num_days()
		}
		doy
	}
}

/// Day stored 1-indexed from the start of the month
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Into, From)]
pub struct Day(pub u8);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Into, From)]
pub struct Year(pub i64);
impl Year {
	pub fn to_ymd(self, doy: DayOfYear) -> Date {
		day_of_year_to_ymd(doy, self)
	}
	
	pub fn is_leap_year(&self) -> bool {
		if self.0 % 4 != 0 {
			false
		} else if self.0 % 100 != 0 {
			true
		} else if self.0 % 400 != 0 {
			false
		} else {
			true
		}
	}
	
	pub fn get_num_days(&self) -> u16 {
		if self.is_leap_year() {
			return 366;
		} else {
			return 365;
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Month {
	January,
	Feburary,
	March,
	April,
	May,
	June,
	July,
	August,
	September,
	October,
	November,
	December
}
impl Month {
	pub fn ordinal0(&self) -> u8 {
		match self {
			January => 0,
			Feburary => 1,
			March => 2,
			April => 3,
			May => 4,
			June => 5,
			July => 6,
			August => 7,
			September => 8,
			October => 9,
			November => 10,
			December => 11,
		}
	}
	pub fn ordinal1(&self) -> u8 {
		match self {
			January => 1,
			Feburary => 2,
			March => 3,
			April => 4,
			May => 5,
			June => 6,
			July => 7,
			August => 8,
			September => 9,
			October => 10,
			November => 11,
			December => 12,
		}
	}
	
	pub fn from0(month0: u64) -> Option<Month> {
		match month0 {
			0 => Some(January),
			1 => Some(Feburary),
			2 => Some(March),
			3 => Some(April),
			4 => Some(May),
			5 => Some(June),
			6 => Some(July),
			7 => Some(August),
			8 => Some(September),
			9 => Some(October),
			10 => Some(November),
			11 => Some(December),
			_ => None,
		}
	}
	
	pub fn from1(month1: u64) -> Option<Month> {
		match month1 {
			1 => Some(January),
			2 => Some(Feburary),
			3 => Some(March),
			4 => Some(April),
			5 => Some(May),
			6 => Some(June),
			7 => Some(July),
			8 => Some(August),
			9 => Some(September),
			10 => Some(October),
			11 => Some(November),
			12 => Some(December),
			_ => None,
		}
	}
	
	pub fn num_days(&self, year: Year) -> u8 {
		match self {
			January => 31,
			Feburary if year.is_leap_year() => 29,
			Feburary => 28,
			March => 31,
			April => 30,
			May => 31,
			June => 30,
			July => 31,
			August => 31,
			September => 30,
			October => 31,
			November => 30,
			December => 31,
		}
	}
	
	pub fn all_months() -> impl Iterator<Item=Month> {
		static MONTHS: [Month;  12] = [January, Feburary, March, April, May, June, July, August, September, October, November, December];
		MONTHS.into_iter().cloned()
	}
}

fn day_of_year_to_ymd(doy: DayOfYear, year: Year) -> Date {
	let mut day = doy.0;
	for month in Month::all_months() {
		if day <= month.num_days(year) as u16 {
			return Date::new(year, month, Day(day as u8));
		}
		day -= month.num_days(year) as u16
	}
	Date::new(year, December, Day(31))
}

fn ymd_to_day_of_year(date: Date) -> DayOfYear {
	let mut doy: u16 = 0;
	for month in Month::all_months() {
		if month == date.month {
			doy += date.day.0 as u16;
			return DayOfYear(doy);
		}
		doy += date.month.num_days(date.year).into();
	}
	DayOfYear(date.year.get_num_days())
}
