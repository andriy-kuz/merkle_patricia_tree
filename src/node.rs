extern crate rlp;

use rlp::{Encodable, Decodable, RlpStream};

pub enum Node<T: Encodable + Decodable> {
    Null,
    Branch { nibles: [Vec<u8>; 16], value: Option<T> },
    Leaf { is_even: bool, path: Vec<u8>, value: T },
    Extention { is_even: bool, path: Vec<u8>, key: [u8; 64] },
}

impl <T: Encodable + Decodable> Encodable for Node<T> {
    fn rlp_append(&self, s: &mut RlpStream) {
        match self {
            &Node::Null => {
                s.begin_list(0);
            },
            &Node::Branch { ref nibles, ref value }=> {
                let mut count = 16;
                if let &Some(_) = value {
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
            &Node::Leaf{ ref is_even, ref path, ref value} => {
                
                s.begin_list(2)
                .append(path)
                .append(value);
            },
            &Node::Extention{ ref is_even, ref path, ref key} => {
                s.begin_list(2)
                .append(path)
                .append(&key.to_vec());
            },
        };
    }
}

impl <T: Encodable + Decodable> Decodable for Node<T> {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {

    }
}
