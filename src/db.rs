use exonum_leveldb::database;
use exonum_leveldb::kv::KV;
use exonum_leveldb::options::{Options, WriteOptions, ReadOptions};
use ethereum_types::H256;
use rlp::{Encodable, Decodable};
use rlp;
use node::*;
use std::clone::Clone;


pub struct Database {
    db_impl: database::Database,
}

impl Database {
    pub fn new(path: &str) -> Box<Self> {
        use std::path::Path;
        let mut options = Options::new();
        options.create_if_missing = true;
        Box::new(Database {
            db_impl: database::Database::open(Path::new(path), options).unwrap(),
        })
    }

    pub fn get_value(&self, key: &H256) -> Option<Vec<u8>> {
        match self.db_impl.get(ReadOptions::new(), key) {
            Ok(data) => data,
            Err(e) => panic!("failed reading data: {:?}", e),
        }
    }

    pub fn set_value(&mut self, key: &H256, value: &Vec<u8>) {
        match self.db_impl.put(WriteOptions::new(), key, value) {
            Ok(_) => {}
            Err(e) => panic!("failed to write to database: {:?}", e),
        };
    }

    pub fn delete_value(&mut self, key: &H256) {
        match self.db_impl.delete(WriteOptions::new(), key) {
            Ok(_) => {}
            Err(e) => panic!("failed to delete from database: {:?}", e),
        }
    }

    pub fn get_node<T: Encodable + Decodable + Clone>(&self, key: &H256) -> Option<Node<T>> {
        if let Some(data) = self.get_value(key) {
            return Some(rlp::decode(&data));
        }
        None
    }

    pub fn set_node<T: Encodable + Decodable + Clone>(&mut self, key: &H256, value: &Node<T>) {
        let data = rlp::encode(value).into_vec();
        self.set_value(key, &data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    fn run_test<T>(test: T) -> ()
    where
        T: FnOnce() -> () + panic::UnwindSafe,
    {
        use std::path::Path;
        use std::fs;

        fs::remove_dir_all(Path::new("storage_test"));

        let result = panic::catch_unwind(|| test());

        fs::remove_dir_all(Path::new("storage_test"));

        assert!(result.is_ok())
    }

    #[test]
    fn basic_database_test() {
        run_test(|| {
            let mut db = Database::new("storage_test");
            db.set_value(&H256::from(1 as u64), &vec![0x01, 0x02, 0x03, 0x04, 0x05]);

            if let Some(value) = db.get_value(&H256::from(1 as u64)) {
                assert_eq!(value, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
            } else {
                assert!(false);
            }
            db.delete_value(&H256::from(1 as u64));

            if let Some(value) = db.get_value(&H256::from(1 as u64)) {
                assert!(false);
            }
        })
    }


}