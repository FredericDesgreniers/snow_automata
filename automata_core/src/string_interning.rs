use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Mutex;
use std::fmt;

lazy_static! {
	static ref GLOBAL_INTERNER: Interner = {
		Interner::new()
	};
}

/// Intern a string, giving back a InternedString
pub fn intern<T: AsRef<str>>(string_to_intern: T) -> InternedString {
	GLOBAL_INTERNER.intern(string_to_intern)
}

/// get the String for an InternedString
pub fn intern_get_str(interned_str: InternedString) -> Option<String> {
	GLOBAL_INTERNER.get(interned_str)
}

/// Interns strings, transforming them from String's to InternedStrings and back the other way around
struct Interner {
	intern_map: Mutex<RefCell<HashMap<String, InternedString>>>,
	strings: Mutex<RefCell<Vec<String>>>,
}

/// An Interned String
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct InternedString{value:usize}


impl fmt::Debug for InternedString {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", intern_get_str(*self))
	}
}

impl Interner {
	/// Create a new interned String
	fn new() -> Self {
		Interner {
			intern_map: Mutex::new(RefCell::new(HashMap::new())),
			strings: Mutex::new(RefCell::new(Vec::new()))
		}
	}

	/// Intern a String / str, giving back an InternedString
	fn intern<T: AsRef<str>>(&self, raw_str: T) -> InternedString {
		let raw_str = raw_str.as_ref().to_string();

		let intern_map_lock = self.intern_map.lock().unwrap();
		let mut intern_map = intern_map_lock.borrow_mut();


		if let Some(&interned_string) = intern_map.get(&raw_str) {
			return interned_string;
		}

		let strings_lock = self.strings.lock().unwrap();
		let mut strings = strings_lock.borrow_mut();

		let interned_string = InternedString{value: strings.len()};

		intern_map.insert(raw_str.clone(),  interned_string);

		strings.push(raw_str);

		interned_string
	}

	/// Get a String back from an InternedString
	fn get(&self, interned_string: InternedString) -> Option<String> {
		let strings_lock = self.strings.lock().unwrap();
		let strings = strings_lock.borrow();

		if let Some(string) = strings.get(interned_string.value) {
			return Some(string.clone());
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn intern_test() {

		let str1 = intern("test1");
		let str2 = intern("test12");
		let str1_copy = intern("test1");

		assert_eq!(str1, str1_copy);
		assert_ne!(str1, str2);
		assert_ne!(str2, str1_copy);

		assert_eq!(intern_get_str(str1).unwrap(), "test1")
	}


}
