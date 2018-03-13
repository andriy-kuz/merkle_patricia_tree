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
    }

    pub fn get(&mut self, key: &H256) -> Option<T> {
        let key = Self::key_bytes_to_hex(key);
        Self::get_helper(&self.db, &key[..], &mut self.root)
    }

    fn get_helper(db: &Database, key_path: &[u8], node: &mut Node<T>) -> Option<T> {
        match node {
            &mut Node::FullNode {ref mut nibles, ref flags} => {
                if key_path.is_empty() {
                    if let Some(ref mut node) = nibles[16] {
                        return Self::get_helper(db, key_path, node)
                    }
                    return None
                }
                if let Some(ref mut node) = nibles[key_path[0] as usize] {
                    return Self::get_helper(db, &key_path[1..], node)
                }
                return None
            },
            &mut Node::ShortNode {ref key, ref mut node, ref flags} => {
                if key[..] == key_path[..key.len()] {
                    return Self::get_helper(db, &key_path[key.len()..], node)
                }
                return None
            },
            &mut Node::HashNode {ref hash} => {
                //TODO
            },
            &mut Node::ValueNode {ref value} => {
                return Some(value.clone())
            },
            &mut Node::Null => {
                return None
            }
        }
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