pub use rlp::{Encodable, Decodable};
pub use std::clone::Clone;
pub use ethereum_types::H256;
use rlp;
use node::*;
use storage::*;

pub struct MerkleTree<T: Encodable + Decodable + Clone> {
    root : Node<T>,
    storage : Box<Storage>,
}

impl <T: Encodable + Decodable + Clone> MerkleTree<T> {
    pub fn new(root: H256, path : &str) -> MerkleTree<T> {
        let storage = Storage::new(path);
        let mut node : Node<T> = Node::Null;

        if let Some(data) = storage.get_value(&root) {
            node = rlp::decode(&data);
        }
        MerkleTree {
            root : node,
            storage : storage,
        }
    }

    pub fn update(&mut self, key: H256, value: T) {

    }

    pub fn delete(&mut self, key: H256) {

    }

    pub fn get(&self, key: H256) -> Option<T> {
        None
    }
}