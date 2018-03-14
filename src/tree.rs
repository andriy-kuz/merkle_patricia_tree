pub use rlp::{Encodable, Decodable};
pub use std::clone::Clone;
pub use ethereum_types::H256;
use rlp;
use node::*;
use db::*;

//TODO: Results system, Tests for MerkleTree, Doc
pub struct MerkleTree<T: Encodable + Decodable + Clone> {
    root: Box<Node<T>>,
    hash: H256,
    db: Box<Database>,
}

impl<T: Encodable + Decodable + Clone> MerkleTree<T> {
    pub fn new(hash: H256, db: Box<Database>) -> MerkleTree<T> {
        let root;
        if let Some(data) = db.get_value(&hash) {
            let node_value = decode_node::<T>(&hash, &data[..]).unwrap();
            root = Box::new(node_value);
        }
        else {
            root = Box::new(Node::Null)
        }
        MerkleTree {
            root,
            hash,
            db,
        }
    }

    pub fn update(&mut self, key: &H256, value: Option<T>) {
        let key_path = Self::key_bytes_to_hex(key);

        if let Some(value) = value {
            Self::insert_helper(&self.db, &key_path[..], &mut self.root, Box::new(Node::ValueNode{value}));
        }
        else {
            Self::delete_helper(&self.db, &key_path[..], &mut self.root);
        }
        //Recalculate hash
    }

    pub fn get(&mut self, key: &H256) -> Option<T> {
        let key_path = Self::key_bytes_to_hex(key);
        Self::get_helper(&self.db, &key_path[..], &mut self.root)
    }

    fn get_helper(db: &Database, key_path: &[u8], node: &mut Box<Node<T>>) -> Option<T> {
        let mut decoded_node;

        match node.as_mut() {
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
                if key_path.len() >= key.len() && key[..] == key_path[..key.len()] {
                    return Self::get_helper(db, &key_path[key.len()..], node)
                }
                return None
            },
            &mut Node::HashNode {ref hash} => {
                if let Some(data) = db.get_value(hash) {
                    let node_value = decode_node::<T>(hash, &data[..]).unwrap();
                    decoded_node = Box::new(node_value);
                }
                else {
                    decoded_node = Box::new(Node::Null)
                }
            },
            &mut Node::ValueNode {ref value} => {
                if key_path.is_empty() {
                    return Some(value.clone())
                }
                return None
            },
            &mut Node::Null => {
                return None
            }
        }
        *node = decoded_node;
        Self::get_helper(db, &key_path[..], node)
    }

    fn insert_helper(db: &Database, key_path: &[u8], node: &mut Box<Node<T>>, value_node: Box<Node<T>>) {

    }

    fn delete_helper(db: &Database, key_path: &[u8], node: &mut Box<Node<T>>) {

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