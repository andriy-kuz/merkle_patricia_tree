use exonum_leveldb::database::Database;
use exonum_leveldb::kv::KV;
use exonum_leveldb::options::{Options,WriteOptions,ReadOptions};

pub struct Storage {
    db_handle : Database,
}

impl Storage {
    pub fn new(path : &str) -> Box<Self> {
        use std::path::Path;
        let mut options = Options::new();
        options.create_if_missing = true;
        Box::new(Storage{
            db_handle : Database::open(Path::new(path), options).unwrap(),
        })
    }

    pub fn get_value(&self, key : &[u8; 32]) -> Option<Vec<u8>> {
        match self.db_handle.get(ReadOptions::new(), key) {
            Ok(data) => data,
            Err(e) => { panic!("failed reading data: {:?}", e) },
        }
    }

    pub fn set_value(&mut self, key : &[u8; 32], value: &Vec<u8> ) {
        match self.db_handle.put(WriteOptions::new(), key, value) {
            Ok(_) => {},
            Err(e) => { panic!("failed to write to database: {:?}", e) }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    fn run_test<T>(test: T) -> ()
        where T: FnOnce() -> () + panic::UnwindSafe
    {
        use std::path::Path;
        use std::fs;

        fs::remove_dir_all(Path::new("storage_test"));

        let result = panic::catch_unwind(|| {
            test()
         });

         fs::remove_dir_all(Path::new("storage_test"));

         assert!(result.is_ok())
    }

    #[test]
    fn basic_storage_test() {
        run_test( || {
            let mut storage = Storage::new("storage_test");
            storage.set_value(&[0; 32], &vec![0x01, 0x02, 0x03, 0x04, 0x05]);

            if let Some(value) = storage.get_value(&[0; 32]) {
                assert_eq!(value, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
            }
            else {
                assert!(false);
            }
        })
    }


}