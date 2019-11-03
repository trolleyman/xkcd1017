
#[macro_use]
extern crate derive_more;

use chrono::prelude::*;

use date::{Date, Day, Month, Year, DayOfYear};
use db::DB;
use util::CommaWrapper;

mod db;
mod date;
mod util;


pub struct Progress {
	now: Date,
	pub progress: f64,
	pub date: Date,
	pub title: String,
	pub info: String,
}
impl Progress {
	pub fn new() -> Progress {
		Progress::new_with_date(Local::today().naive_local())
	}
	pub fn new_with_date<D: Into<Date>>(date: D) -> Progress {
		let date = date.into();
		let mut prog = Progress {
			progress: 0.0,
			now: date,
			date: date,
			title: String::new(),
			info: String::new(),
		};
		prog.set_progress(0.0);
		prog
	}
	
	pub fn set_progress(&mut self, progress: f64) {
		self.progress = progress;
		
		let subtract_total_years: f64 = (20.3444 * progress.powi(3) + 3.0).exp() - std::f64::consts::E.powi(3);
		
		let start_prog_year = self.now.get_day_of_year().get_progress_through_year(self.now.year);
		
		let sub_y: f64 = subtract_total_years - start_prog_year;
		let date = if sub_y > 0.0 {
			// Previous year
			let y = Year(self.now.year.0 - sub_y.ceil() as i64);
			let doy = DayOfYear::from_progress_through_year(y, 1.0 - sub_y.fract());
			doy.to_ymd(y)
		} else {
			// Current year
			let y = self.now.year;
			let doy = DayOfYear::from_progress_through_year(y, self.now.get_day_of_year().get_progress_through_year(self.now.year) - subtract_total_years);
			doy.to_ymd(y)
		};
		
		// Set correct date
		self.date = date;
		
		// Update title
		if self.date.year.0 <= -1_000_000_000 {
			self.title = format!("{:.1} billion years ago", CommaWrapper(-self.date.year.0 as f64 / 1_000_000_000.0));
		} else if self.date.year.0 <= -1_000_000 {
			self.title = format!("{:.0} million years ago", CommaWrapper(-self.date.year.0 as f64 / 1_000_000.0));
		} else if self.date.year.0 <= -10_000 {
			self.title = format!("{} years ago", CommaWrapper(-self.date.year.0));
		} else if self.date.year.0 < 0 {
			self.title = format!("{} BC", -self.date.year.0);
		} else if self.date.year.0 == 0 {
			self.title = format!("1 AD"); // Special case
		} else if self.date.year.0 <= 1000 {
			self.title = format!("{} AD", self.date.year.0);
		} else if self.date.year.0 <= 1900 {
			self.title = format!("{}", self.date.year.0);
		} else if self.date.year.0 <= 2000 {
			self.title = format!("{:?} {}", self.date.month, self.date.year.0);
		} else if self.date == self.now {
			self.title = format!("Now");
		} else {
			self.title = format!("{} {:?} {}", self.date.day.0, self.date.month, self.date.year.0);
		}

		// Update info string
		self.info = db::get_events(self.now, self.date)
			.into_iter().map(|e| e.info).collect::<Vec<_>>().join(", ");
	}
	
	pub fn get_progress(&self) -> f64 {
		self.progress
	}
}
