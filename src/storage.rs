use exonum_leveldb::database::Database;
use exonum_leveldb::kv::KV;
use exonum_leveldb::options::{Options,WriteOptions,ReadOptions};

pub struct Storage<'a> {
    db_handle : Database,
    clear : bool,
    path : &'a str,
}

impl <'a> Storage <'a> {
    fn new(path : &'a str, clear : bool) -> Box<Self> {
        use std::path::Path;
        let mut options = Options::new();
        options.create_if_missing = true;
        Box::new(Storage{
            db_handle : Database::open(Path::new(path), options).unwrap(),
            clear,
            path : path,
        })
    }

    fn get_value(&self, key : &[u8; 32]) -> Option<Vec<u8>> {
        match self.db_handle.get(ReadOptions::new(), key) {
            Ok(data) => data,
            Err(e) => { panic!("failed reading data: {:?}", e) },
        }
    }

    fn set_value(&mut self, key : &[u8; 32], value: &Vec<u8> ) {
        match self.db_handle.put(WriteOptions::new(), key, value) {
            Ok(_) => {},
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
    }
}

impl <'a> Drop for Storage<'a> {
    fn drop(&mut self) {
        use std::fs;
        use std::path::Path;

        if self.clear {
            fs::remove_dir_all(Path::new(self.path));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn basic_storage_test() {
        let mut storage = Storage::new("storage_test", true);
        storage.set_value(&[0; 32], &vec![0x01, 0x02, 0x03, 0x04, 0x05]);

        if let Some(value) = storage.get_value(&[0; 32]) {
            assert_eq!(value, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        }
        else {
            assert!(false);
        }
    }


}