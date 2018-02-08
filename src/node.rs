use rlp::{Encodable, Decodable, RlpStream, UntrustedRlp, DecoderError};

pub enum Node<T: Encodable + Decodable> {
    Null,
    Branch { nibles: [Vec<u8>; 16], value: Option<T> },
    Leaf { path: Vec<u8>, value: T },
    Extention { path: Vec<u8>, key: Vec<u8> },
}

impl <T: Encodable + Decodable> Encodable for Node<T> {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            &Node::Null => {
                s.begin_list(0);
            },
            &Node::Branch { ref nibles, ref value }=> {
                let mut count = 16;

                if value.is_some() {
                    count = 17;
                }
                let mut list = s.begin_list(count);
                list.append(&nibles[0])
                .append(&nibles[1])
                .append(&nibles[2])
                .append(&nibles[3])
                .append(&nibles[4])
                .append(&nibles[5])
                .append(&nibles[6])
                .append(&nibles[7])
                .append(&nibles[8])
                .append(&nibles[9])
                .append(&nibles[10])
                .append(&nibles[11])
                .append(&nibles[12])
                .append(&nibles[13])
                .append(&nibles[14])
                .append(&nibles[15]);

                if let &Some(ref value) = value {
                    list.append(value);
                }
            },
            &Node::Leaf{ ref path, ref value} => {
                let mut path = path.clone();
                // add terminating(leaf) node flag
                path.push(0x10);
                let path = compact_encode(path);
                s.begin_list(2)
                .append(&path)
                .append(value);
            },
            &Node::Extention{ ref path, ref key} => {
                let path = compact_encode(path.clone());
                s.begin_list(2)
                .append(&path)
                .append(key);
            },
        };
    }
}

impl <T: Encodable + Decodable> Decodable for Node<T> {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        // Is null node
        if !rlp.val_at::<Vec<u8>>(0).is_ok() {
            return Ok(Node::Null);
        }
        // Is branch node
        if rlp.val_at::<Vec<u8>>(15).is_ok() {
            return  
            Ok(
                Node::Branch {
                    nibles: [
                        rlp.val_at::<Vec<u8>>(0)?,
                        rlp.val_at::<Vec<u8>>(1)?,
                        rlp.val_at::<Vec<u8>>(2)?,
                        rlp.val_at::<Vec<u8>>(3)?,
                        rlp.val_at::<Vec<u8>>(4)?,
                        rlp.val_at::<Vec<u8>>(5)?,
                        rlp.val_at::<Vec<u8>>(6)?,
                        rlp.val_at::<Vec<u8>>(7)?,
                        rlp.val_at::<Vec<u8>>(8)?,
                        rlp.val_at::<Vec<u8>>(9)?,
                        rlp.val_at::<Vec<u8>>(10)?,
                        rlp.val_at::<Vec<u8>>(11)?,
                        rlp.val_at::<Vec<u8>>(12)?,
                        rlp.val_at::<Vec<u8>>(13)?,
                        rlp.val_at::<Vec<u8>>(14)?,
                        rlp.val_at::<Vec<u8>>(15)?,
                    ],
                    //TODO: CHECK ENCODE AND DECODE OF TYPE T
                    value: rlp.val_at::<T>(16).ok()
                }
            );
        }
        // This is extension or terminating node
        let mut path = compact_decode( rlp.val_at::<Vec<u8>>(0)? );
        // Is terminating node
        if *path.last().unwrap() == 0x10 {
            path.pop();
            return Ok (
                Node::Leaf {
                    path,
                    value : rlp.val_at::<T>(1)?
                }
            )
        }
        // This is extension node
        Ok(
            Node::Extention {
                path,
                key : rlp.val_at::<Vec<u8>>(1)?
            }
        )
    }
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

    #[test]
    fn null_node_test() {
        // Null node
        let node : Node<u32> = Node::Null;
        let data = rlp::encode(&node).into_vec();
        assert_eq!(data[0], 0xc0);
        let node : Node<u32> = rlp::decode(&data);

        match node {
            Node::Null => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn branch_node_test() {
        branch_node_with_value(true);
        branch_node_with_value(false);
    }

    fn branch_node_with_value(some_value: bool) {

        let node : Node<u32> = Node::Branch {
            nibles : [
                vec![],
                vec![0x01, 0x02, 0x03, 0x04, 0x05], //index 1
                vec![],
                vec![],
                vec![0x01, 0x02, 0x03, 0x04, 0x05], //index 4
                vec![],
                vec![],
                vec![],
                vec![],
                vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 
                0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 
                0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 
                0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04], // index 9
                vec![],
                vec![],
                vec![],
                vec![0x01, 0x02, 0x03, 0x04, 0x05], // index 13
                vec![],
                vec![0x01, 0x02, 0x03, 0x04, 0x05], // index 15
            ],
            value : if some_value {Some(77)} else {None},
        };
        let data = rlp::encode(&node).into_vec();
        let node : Node<u32> = rlp::decode(&data);

        match node {
            Node::Branch{ nibles, value } =>{
                // Check empty nibles
                assert!(nibles[0].is_empty());
                assert!(nibles[2].is_empty());
                assert!(nibles[3].is_empty());
                assert!(nibles[5].is_empty());
                assert!(nibles[6].is_empty());
                assert!(nibles[7].is_empty());
                assert!(nibles[8].is_empty());
                assert!(nibles[10].is_empty());
                assert!(nibles[11].is_empty());
                assert!(nibles[12].is_empty());
                assert!(nibles[14].is_empty());
                // Check nibles data
                assert_eq!(nibles[1], vec![0x01, 0x02, 0x03, 0x04, 0x05]);
                assert_eq!(nibles[4], vec![0x01, 0x02, 0x03, 0x04, 0x05]);

                assert_eq!(nibles[9], 
                vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 
                0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 
                0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 
                0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x01, 0x02, 0x03, 0x04]);

                assert_eq!(nibles[13], vec![0x01, 0x02, 0x03, 0x04, 0x05]);
                assert_eq!(nibles[15], vec![0x01, 0x02, 0x03, 0x04, 0x05]);
                // Check value
                assert_eq!(value.is_some(), some_value);

                if let Some(value) = value {
                    assert_eq!(value, 77);
                }
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn extention_node_test() {
        let node : Node<u32> = Node::Extention { 
            path: vec![0x01, 0x02, 0x03, 0x04, 0x05], 
            key: vec![0x01, 0x02, 0x03, 0x04, 0x05], 
        };
        let data = rlp::encode(&node).into_vec();
        let node : Node<u32> = rlp::decode(&data);

        match node {
            Node::Extention {path, key} => {
                assert_eq!(path, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
                assert_eq!(key, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn leaf_node_test() {
        let node : Node<u32> = Node::Leaf { 
            path: vec![0x01, 0x02, 0x03, 0x04, 0x05], 
            value: 77, 
        };
        let data = rlp::encode(&node).into_vec();
        let node : Node<u32> = rlp::decode(&data);

        match node {
            Node::Leaf {path, value} => {
                assert_eq!(path, vec![0x01, 0x02, 0x03, 0x04, 0x05]);
                assert_eq!(value, 77);
            }
            _ => assert!(false),
        }

    }
}
