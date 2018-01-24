enum Node<T> {
    Null,
    Branch { nibles: [[u8; 64]; 16], value: T },
    Leaf { path: Vec<u8>, value: T },
    Extention { path: Vec<u8>, key: [u8; 64] },
}
