
use std::fmt;


#[derive(Deref, DerefMut)]
pub(crate) struct CommaWrapper<T: CommaWrap>(pub T);
impl<T: CommaWrap> CommaWrapper<T> {
	pub fn new(inner: T) -> CommaWrapper<T> {
		CommaWrapper(inner)
	}
}
impl<T: CommaWrap> fmt::Display for CommaWrapper<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.comma_fmt(f)
	}
}

trait CommaWrap {
	fn comma_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl CommaWrap for i64 {
	fn comma_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO
		unimplemented!()
	}
}
impl CommaWrap for f64 {
	fn comma_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// TODO
		unimplemented!()
	}
}
