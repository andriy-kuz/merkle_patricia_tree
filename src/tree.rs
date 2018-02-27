pub use rlp::{Encodable, Decodable};
pub use std::clone::Clone;
pub use ethereum_types::H256;
use rlp;
use node::*;
use db::*;

//TODO: Tests for MerkleTree, Doc, Results system
pub struct MerkleTree<T: Encodable + Decodable + Clone> {
    root: Node<T>,
    hash: H256,
    db: Box<Database>,
}

impl<T: Encodable + Decodable + Clone> MerkleTree<T> {
    pub fn new(root: H256, db: Box<Database>) -> MerkleTree<T> {
        MerkleTree {
            root: Node::Null,
            hash: root,
            db,
        }
    }

    pub fn update(&mut self, key: H256, value: T) {}

    pub fn delete(&mut self, key: &H256) {
        self.db.delete_value(key);
    }

    pub fn get(&self, key: &H256) -> Option<T> {
        None
    }

    fn key_bytes_to_hex(key: &H256) -> Vec<u8> {
        let mut result = Vec::new();

        for iter in 0..key.len() {
            result.push((key[iter] & 0xF0) >> 4);
            result.push(key[iter] & 0x0F);
        }
        result
    }
}