use heapless::String;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Key(String<16>);

impl Key {
    pub fn parse(s: &str) -> Result<Key, DataStoreError> {
        if s.len() > 16 {
            return Err(DataStoreError::KeyTooLong);
        }
        let mut key: String<16> = String::new();

        let result = key.push_str(s);
        if result.is_err() {
            return Err(DataStoreError::UnknownError);
        }

        Ok(Key(key))
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Value(String<32>);

impl Value {
    pub fn parse(s: &str) -> Result<Value, DataStoreError> {
        if s.len() > 32 {
            return Err(DataStoreError::ValueTooLong);
        }
        let mut value: String<32> = String::new();

        let result = value.push_str(s);
        if result.is_err() {
            return Err(DataStoreError::UnknownError);
        }

        Ok(Value(value))
    }
}

pub trait DataStore {
    // fn init(&self, length: usize) -> Result<Database, DataStoreError>;
    fn get(&self, key: &Key) -> Result<Value, DataStoreError>;
    fn set(&mut self, key: Key, value: Value) -> Result<(), DataStoreError>;
    fn delete(&mut self, key: &Key) -> Result<(), DataStoreError>;
}

#[derive(Debug)]
pub enum DataStoreError {
    NotFound,
    WriteError,
    EraseError,
    KeyTooLong,
    ValueTooLong,
    UnknownError,
}
