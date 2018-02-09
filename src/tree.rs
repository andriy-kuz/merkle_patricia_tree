pub use rlp::{Encodable, Decodable};
use node::*;
use storage::*;

pub struct MerkleTree<T: Encodable + Decodable> {
    _root : Node<T>,
}