fn main() {
    let test_data: [u8; 4] = [0, 2, 3, 4];

    dbg!(f32::from_be_bytes(test_data));
}
