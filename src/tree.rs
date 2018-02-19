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
        let node;
        if let Some(data) = db.get_value(&root) {
            node = rlp::decode(&data);
        } else {
            panic!("Failed to create trie from root");
        }
        MerkleTree {
            root: node,
            hash: root,
            db,
        }
    }

    pub fn update(&mut self, key: H256, value: T) {}

    pub fn delete(&mut self, key: &H256) {
        self.db.delete_value(key);
    }

    pub fn get(&self, key: &H256) -> Option<T> {
        let mut curr_node = self.root.clone();
        let key = Self::decompress_key(key);

        for mut index in 0..key.len() {
            match curr_node {
                Node::Null => {
                    return None;
                }
                Node::Branch { nibles, value } => {
                    if nibles[key[index] as usize].len() == 0 {
                        curr_node = Node::Null;
                        continue;
                    }
                    let hash = H256::from(nibles[key[index] as usize].as_slice());

                    if let Some(node) = self.db.get_node::<T>(&hash) {
                        curr_node = node;
                        continue;
                    }
                    curr_node = Node::Null;
                }
                Node::Leaf { path, value } => {
                    if &key[index..] == &path[..] {
                        return Some(value);
                    }
                    curr_node = Node::Null;
                }
                Node::Extention { path, key } => {
                    if &key[index..index + path.len()] == &path[..] {

                        if let Some(node) = self.db.get_node::<T>(&key) {
                            curr_node = node;
                            index += path.len();
                            continue;
                        }
                    }
                    curr_node = Node::Null;
                }
            }
        }
        None
    }

    fn decompress_key(key: &H256) -> Vec<u8> {
        let mut result = Vec::new();

        for iter in 0..key.len() {
            result.push((key[iter] & 0xF0) >> 4);
            result.push(key[iter] & 0x0F);
        }
        result
    }
}