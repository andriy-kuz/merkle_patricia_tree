use rlp::{Encodable, Decodable, RlpStream, UntrustedRlp, DecoderError};

pub enum Node<T: Encodable + Decodable> {
    Null,
    Branch { nibles: [Vec<u8>; 16], value: Option<T> },
    Leaf { path: Vec<u8>, value: T },
    Extention { path: Vec<u8>, key: [u8; 64] },
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
            &Node::Leaf{ ref path, ref value} => {
                let mut path = path.clone();
                // add terminating(leaf) node flag
                path.push(16);
                let path = compact_encode(path);
                s.begin_list(2)
                .append(&path)
                .append(value);
            },
            &Node::Extention{ ref path, ref key} => {
                let path = compact_encode(path.clone());
                s.begin_list(2)
                .append(&path)
                .append(&key.to_vec());
            },
        };
    }
}

impl <T: Encodable + Decodable> Decodable for Node<T> {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {

    }
}

fn compact_encode(hex_array : Vec<u8>) -> Vec<u8> {
    let term = if *hex_array.last().unwrap() == 16 as u8 {1} else {0};

    if term == 1 {
        hex_array.pop();
    }
    let odd_len = hex_array.len() % 2;
    let flags = 2 * term + odd_len as u8;
    
    let hex_array = if odd_len == 1 {
        let array = Vec::new();
        array.push(flags);
        array.append(&mut hex_array);
        array
    } else {
        let array = Vec::new();
        array.push(flags);
        array.push(0);
        array.append(&mut hex_array);
        array
    };
    let result = Vec::with_capacity(hex_array.len()/2);

    for iter in (0..hex_array.len()).step_by(2) {
        result.push(16*hex_array[iter] + hex_array[iter + 1]);
    }
    result
}

fn compact_decode(hex_array : Vec<u8>) -> Vec<u8> {
    Vec::new()
}
