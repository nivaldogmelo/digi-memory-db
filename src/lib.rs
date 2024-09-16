#![cfg_attr(not(feature = "std"), no_std)]

mod domain;
mod services;

use services::Database;

// #[cfg(not(feature = "std"))]
// fn init_clean_db() -> Database {
//     Database::new()
// }

#[cfg(test)]
#[cfg(feature = "std")]
mod tests {
    use domain::{Key, Value};

    use super::*;
    use crate::domain::DataStore;

    #[test]
    fn insert_key() {
	let mut db = Database::new();
	let key = Key::parse("key").unwrap();
	let value = Value::parse("value").unwrap();

	let _ = db.set(key.clone(), value.clone());

	assert_eq!(db.get(&key).unwrap(), value);
    }
}
