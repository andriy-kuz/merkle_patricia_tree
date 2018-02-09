pub use rlp::{Encodable, Decodable};
use rlp;
use node::*;
use storage::*;

pub struct MerkleTree<T: Encodable + Decodable> {
    root : Node<T>,
    storage : Box<Storage>,
}

impl <T: Encodable + Decodable> MerkleTree<T> {
    pub fn new(root: [u8; 32], path : &str) -> MerkleTree<T> {
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

    pub fn update(&mut self, key: [u8; 32], value: T) {

    }

    pub fn delete(&mut self, key: [u8; 32]) {

    }

    pub fn get(&self, key: [u8; 32]) -> Option<T> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_demo() {
        //let tri = MerkleTree::<u32>::new([0;32], "demo");
    }
}