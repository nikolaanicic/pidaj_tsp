use std::{hash::Hash, fmt::Display};


pub struct Location {
	pub city:String,
	pub country:String,
}	

impl PartialEq for Location {
	fn eq(&self, other: &Self) -> bool {
		self.city == other.city && self.country == other.country
	}
}

impl Eq for Location {}


impl Hash for Location {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.city.hash(state);
		self.country.hash(state);
	}
}

impl Display for Location {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} ({})", self.city, self.country)
	}
}
