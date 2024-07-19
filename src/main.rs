use leb128::read::signed;
use std::io::Cursor;
fn main() {
    let prefix = "F601D403B205";

    let prefix_bytes = hex::decode(prefix).unwrap();
    let mut reader = Cursor::new(prefix_bytes);

    // Decode varints for Height, Size, and Version
    let height = signed(&mut reader).unwrap();
    let size = signed(&mut reader).unwrap();
    let version = signed(&mut reader).unwrap();

    println!("Height: {}, Size: {}, Version: {}", height, size, version);

    assert!(height == 123);
    assert!(size == 234);
    assert!(version == 345);
}
