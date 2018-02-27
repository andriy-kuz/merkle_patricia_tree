use ethereum_types::H256;
use rlp::{Encodable, Decodable, RlpStream, UntrustedRlp, DecoderError};
use std::clone::Clone;
use std::str::FromStr;
use std::fmt::Debug;

pub enum Node<T: Decodable> {
    FullNode {nibles: [Option<Box<Node<T>>>; 17], flags: NodeFlag},
    ShortNode {key: Vec<u8>, node: Box<Node<T>>, flags: NodeFlag},
    HashNode {hash: H256},
    ValueNode {value: T},
    Null,
}

pub struct NodeFlag {
    hash: H256,
    dirty: bool,
}

pub fn decode_node<T: Decodable>(hash: &H256, data: &[u8]) -> Result<Node<T>, &'static str> {
    if data.is_empty() {
        return Err("Empty data buffer")
    }
    let rlp = UntrustedRlp::new(data);
    // This is full node
    if rlp.val_at::<Vec<u8>>(16).is_ok() {
        return decode_full(hash, rlp);
    }
    // This is short node
    return decode_short(hash, rlp)
}

pub fn decode_short<T: Decodable>(hash: &H256, rlp: UntrustedRlp) -> Result<Node<T>, &'static str> {
    let key = compact_decode(rlp.val_at::<Vec<u8>>(0).unwrap());
    let flags = NodeFlag{hash: hash.clone(), dirty: false};

    // Is term node
    if *key.last().unwrap() == 0x10 {
        return Ok(
            Node::ShortNode {
                key,
                node: Box::new(Node::ValueNode {
                    value: rlp.val_at::<T>(1).unwrap()
                }),
                flags,
            }
        )
    }
    // This is hash node
    let data = rlp.val_at::<Vec<u8>>(1).unwrap();
    let rlp = UntrustedRlp::new(&data[..]);
    let node : Node<T> = decode_ref(rlp)?;

    return Ok(
        Node::ShortNode {
            key,
            node: Box::new(node),
            flags,
        }
    )
}

pub fn decode_full<T: Decodable>(hash: &H256, rlp: UntrustedRlp) -> Result<Node<T>, &'static str> {
    let flags = NodeFlag{hash:hash.clone(), dirty: false};
    let mut node : Node<T> = Node::FullNode {nibles: [None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None], flags};
    //
    for index in 0..16 {
        let data = rlp.val_at::<Vec<u8>>(index).unwrap();

        if let Ok(node_ref) = decode_ref::<T>(UntrustedRlp::new(&data[..])) {

            if let &mut Node::FullNode {ref mut nibles, ref flags} = &mut node {
                nibles[index] = Some(Box::new(node_ref));
            }
        }
    }
    if let Ok(value) = rlp.val_at::<T>(16) {
        if let &mut Node::FullNode {ref mut nibles, ref flags} = &mut node {
            nibles[16] = Some(Box::new(Node::ValueNode{value}));
        }
    }
    Ok(node)
}

pub fn decode_ref<T: Decodable>(rlp: UntrustedRlp) -> Result<Node<T>, &'static str> {
    if let Ok(info) = rlp.payload_info() {
        return decode_node(&H256::zero(), rlp.as_raw())
    } 
    else if rlp.as_raw().len() == 32 {
        return Ok(
            Node::HashNode{
                hash: H256::from_slice(rlp.as_raw())
            }
        )
    }
    else if rlp.as_raw().len() == 0 {
        return Ok(
            Node::Null
            )
    }
    return Err("Invalid RLP")
}

fn compact_encode(mut hex_array : Vec<u8>) -> Vec<u8> {
    let term = if *hex_array.last().unwrap() == 0x10 {1} else {0};

    if term == 1 {
        hex_array.pop();
    }
    let odd_len = hex_array.len() % 2;
    let flags = 2 * term + odd_len as u8;

    let hex_array = if odd_len == 1 {
        let mut array = Vec::new();
        array.push(flags);
        array.append(&mut hex_array);
        array
    } else {
        let mut array = Vec::new();
        array.push(flags);
        array.push(0);
        array.append(&mut hex_array);
        array
    };
    let mut result = Vec::with_capacity(hex_array.len()/2);

    for iter in (0..hex_array.len()).step_by(2) {
        result.push(16*hex_array[iter] + hex_array[iter + 1]);
    }
    result
}

fn compact_decode(hex_array : Vec<u8>) -> Vec<u8> {
    let odd_len = (hex_array[0] & 0x10) == 0x10;
    let term = (hex_array[0] & 0x20) == 0x20;
    //allocate vector with accurate capacity value
    let mut result = Vec::with_capacity((hex_array.len()- !odd_len as usize)*2 - odd_len as usize + term as usize);

    if odd_len {
        result.push(hex_array[0] & 0x0F);
    }

    for iter in 1..hex_array.len() {
        result.push((hex_array[iter] & 0xF0) >> 4);
        result.push(hex_array[iter] & 0x0F);
    }
    
    if term {
        result.push(0x10);
    }
    result
}

#[cfg(test)]
mod tests {
    extern crate rlp;
    use super::*;

    #[test]
    fn compact_encode_test() {
        // [ 0x01, 0x02, 0x03, 0x04, 0x05 ] -> [ 0x11, 0x23, 0x45 ]
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        let result = compact_encode(data);
        assert_eq!(result.len(), 3);
        assert_eq!(result, vec![0x11, 0x23, 0x45]);
        // [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05 ] -> [ 0x00, 0x01, 0x23, 0x45 ]
        let data = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05];
        let result = compact_encode(data);
        assert_eq!(result.len(), 4);
        assert_eq!(result, vec![0x00, 0x01, 0x23, 0x45]);
        // [ 0x00, 0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10 ] -> [ 0x20, 0x0f, 0x1c, 0xb8 ]
        let data = vec![0x00, 0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10];
        let result = compact_encode(data);
        assert_eq!(result.len(), 4);
        assert_eq!(result, vec![0x20, 0x0f, 0x1c, 0xb8]);
        // [ 0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10 ] -> [ 0x3f, 0x1c, 0xb8 ]
        let data = vec![0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10];
        let result = compact_encode(data);
        assert_eq!(result.len(), 3);
        assert_eq!(result, vec![0x3f, 0x1c, 0xb8]);
    }

    #[test]
    fn compact_decode_test() {
        // [ 0x11, 0x23, 0x45 ] -> [ 0x01, 0x02, 0x03, 0x04, 0x05 ]
        let data = vec![0x11, 0x23, 0x45];
        let result = compact_decode(data);
        assert_eq!(result.len(), 5);
        assert_eq!(result, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
        // [ 0x00, 0x01, 0x23, 0x45 ] -> [ 0x00, 0x01, 0x02, 0x03, 0x04, 0x05 ]
        let data = vec![0x00, 0x01, 0x23, 0x45];
        let result = compact_decode(data);
        assert_eq!(result.len(), 6);
        assert_eq!(result, vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05]);
        // [ 0x20, 0x0f, 0x1c, 0xb8 ] -> [ 0x00, 0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10 ]
        let data = vec![0x20, 0x0f, 0x1c, 0xb8];
        let result = compact_decode(data);
        assert_eq!(result.len(), 7);
        assert_eq!(result, vec![0x00, 0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10]);
        // [ 0x3f, 0x1c, 0xb8 ] -> [ 0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10 ]
        let data = vec![0x3f, 0x1c, 0xb8];
        let result = compact_decode(data);
        assert_eq!(result.len(), 6);
        assert_eq!(result, vec![0x0f, 0x01, 0x0c, 0x0b, 0x08, 0x10]);
    }

    fn check_value_node<T: Decodable + PartialEq + Debug>(node_val: Box<Node<T>>, test_value: T) {
        match *node_val {
            Node::ValueNode{value} => {
                assert_eq!(test_value, value)
            },
            Node::ShortNode{node, .. } => {
                check_value_node::<T>(node, test_value)
            },
            _ => {assert!(false)}
        }
    }

    fn check_hash_node<T: Decodable>(node: Box<Node<T>>, test_hash: &H256) {
        match *node {
            Node::HashNode{hash} => {
                    assert_eq!(*test_hash, hash)
                },
                _ => {assert!(false)}
        }
    }

    #[test]
    fn value_node_test() {
        let data;
        let test_value: u64 = 77;
        // create test rlp data
        {
            let mut rlp_s = RlpStream::new_list(2);
            rlp_s.append(&vec![ 0x20, 0x0f, 0x1c, 0xb8 ]).append(&test_value);
            data = rlp_s.out()
        }
        //
        let node = decode_node::<u64>(&H256::zero(), &data[..]);
        assert!(node.is_ok());
        let node = node.unwrap();

        match node {
            Node::ShortNode{key, node, flags} => {
                check_value_node(node, test_value);
                
            },
            _ => {assert!(false)}
        }
    }

    #[test]
    fn hash_node_test() {
        let data;
        let test_hash = H256::from_str("fb7a44857f2faf8167c8b24bf91563335bc8a6459e055029907d1edb9dd143d8").unwrap();
        // create test rlp data
        {
            let mut rlp_s = RlpStream::new_list(2);
            rlp_s.append(&vec![ 0x11, 0x23, 0x45 ]).append(&test_hash[..].to_vec());
            data = rlp_s.out()
        }
        //
        let node = decode_node::<u64>(&H256::zero(), &data[..]);
        assert!(node.is_ok());
        let node = node.unwrap();

        match node {
            Node::ShortNode{key, node, flags} => {
                check_hash_node::<u64>(node, &test_hash);
                
            },
            _ => {assert!(false)}
        }
    }

    #[test]
    fn recursive_short_node_test() {
        let mut data;
        let test_value: u64 = 77;
        // create test rlp data
        {
            let mut rlp_s = RlpStream::new_list(2);
            rlp_s.append(&vec![ 0x20, 0x0f, 0x1c, 0xb8 ]).append(&test_value);
            data = rlp_s.out();

            let mut rlp_s = RlpStream::new_list(2);
            rlp_s.append(&vec![ 0x11, 0x23, 0x45 ]).append(&data);
            data = rlp_s.out();
        }
        let node = decode_node::<u64>(&H256::zero(), &data[..]);
        assert!(node.is_ok());
        let node = node.unwrap();
        match node {
            Node::ShortNode{node, .. } => {
                check_value_node(node, test_value);
                
            },
            _ => {assert!(false)}
        }
    }
}
