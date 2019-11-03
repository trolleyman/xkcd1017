
use lazy_static::lazy_static;

use crate::date::{Date, Year, Month, Day, Month::*};


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
	pub date: Date,
	pub info: String,
}
impl Event {
	pub fn new<D: Into<Date>, S: Into<String>>(date: D, info: S) -> Event {
		Event{ date: date.into(), info: info.into() }
	}
}

lazy_static!(
	pub static ref DB: Vec<Event> = {
		let mut v = Vec::new();
		v.push(Event::new((-44, March, 15), "Julius Caesar assasinated"));
		v.push(Event::new((2001, September, 11), "9/11"));
		v.push(Event::new((1941, December, 7), "Pearl Harbour attacked"));
		v.sort();
		v
	};
);

pub fn get_events<D1: Into<Date>, D2: Into<Date>>(now: D1, date: D2) -> Vec<Event> {
	let date = date.into();
	let events: &Vec<Event> = &*DB;
	let base_idx = match events.binary_search_by_key(&date, |e| e.date) {
		Ok(i) => i,
		Err(i) => i,
	};
	
	if base_idx >= events.len() {
		base_idx = events.len() - 1;
	}
	let mut ret = Vec::new();
	ret.push(events[base_idx]);
	for i in 1..3 {
		let idx = base_idx + i;
		if idx < events.len() {
			// TODO
		}
	}
	
	ret
}
