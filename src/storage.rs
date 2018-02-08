use exonum_leveldb::database::Database;
use exonum_leveldb::kv::KV;
use exonum_leveldb::options::{Options,WriteOptions,ReadOptions};

pub struct Storage {
    db_handle : Database,
}

impl Storage {
    fn new(path : &str) -> Box<Self> {
        use std::path::Path;
        let mut options = Options::new();
        options.create_if_missing = true;
        Box::new(Storage{
            db_handle : Database::open(Path::new(path), options).unwrap()
        })
    }
}