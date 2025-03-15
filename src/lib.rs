mod binary_reader;
mod binary_writer;

pub use binary_reader::BinaryReader;
pub use binary_writer::BinaryWriter;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_u8_and_i8_read_write() {
        let mut buffer = vec![0u8; 1];
        let mut cursor = Cursor::new(buffer.as_mut_slice());
        // u8
        cursor.write_u8(u8::MAX).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u8().unwrap(), u8::MAX);
        // i8
        cursor.set_position(0);
        cursor.write_i8(i8::MAX).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i8().unwrap(), i8::MAX);
    }

    #[test]
    fn test_u16_and_i16_read_write() {
        let mut buffer = vec![0; 2];
        let mut cursor = Cursor::new(buffer.as_mut_slice());
        // u16 be
        cursor.write_u16_be(u16::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u16_be().unwrap(), u16::MAX - 1);
        // u16 le
        cursor.set_position(0);
        cursor.write_u16_le(u16::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u16_le().unwrap(), u16::MAX - 1);
        // i16 be
        cursor.set_position(0);
        cursor.write_i16_be(i16::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i16_be().unwrap(), i16::MAX - 1);
        // i16 le
        cursor.set_position(0);
        cursor.write_i16_le(i16::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i16_le().unwrap(), i16::MAX - 1);
    }

    #[test]
    fn test_u32_and_i32_read_write() {
        let mut buffer = vec![0; 4];
        let mut cursor = Cursor::new(buffer.as_mut_slice());
        // u32 be
        cursor.write_u32_be(u32::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u32_be().unwrap(), u32::MAX - 1);
        // u32 le
        cursor.set_position(0);
        cursor.write_u32_le(u32::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u32_le().unwrap(), u32::MAX - 1);
        // i32 be
        cursor.set_position(0);
        cursor.write_i32_be(i32::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i32_be().unwrap(), i32::MAX - 1);
        // i32 le
        cursor.set_position(0);
        cursor.write_i32_le(i32::MAX - 1).unwrap();
        cursor.set_position(0);
    }

    #[test]
    fn test_u64_and_i64_read_write() {
        let mut buffer = vec![0; 8];
        let mut cursor = Cursor::new(buffer.as_mut_slice());
        // u64 be
        cursor.write_u64_be(u64::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u64_be().unwrap(), u64::MAX - 1);
        // u64 le
        cursor.set_position(0);
        cursor.write_u64_le(u64::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u64_le().unwrap(), u64::MAX - 1);
        // i64 be
        cursor.set_position(0);
        cursor.write_i64_be(i64::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i64_be().unwrap(), i64::MAX - 1);
        // i64 le
        cursor.set_position(0);
        cursor.write_i64_le(i64::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i64_le().unwrap(), i64::MAX - 1);
    }

    #[test]
    fn test_u128_and_i128_read_write() {
        let mut buffer = vec![0; 16];
        let mut cursor = Cursor::new(buffer.as_mut_slice());
        // u128 be
        cursor.write_u128_be(u128::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u128_be().unwrap(), u128::MAX - 1);
        // u128 le
        cursor.set_position(0);
        cursor.write_u128_le(u128::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_u128_le().unwrap(), u128::MAX - 1);
        // i128 be
        cursor.set_position(0);
        cursor.write_i128_be(i128::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i128_be().unwrap(), i128::MAX - 1);
        // i128 le
        cursor.set_position(0);
        cursor.write_i128_le(i128::MAX - 1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_i128_le().unwrap(), i128::MAX - 1);
    }

    #[test]
    fn test_f32_read_write() {
        let mut buffer = vec![0; 4];
        let mut cursor = Cursor::new(buffer.as_mut_slice());
        // f32 be
        cursor.write_f32_be(f32::MAX - 1.1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_f32_be().unwrap(), f32::MAX - 1.1);
        // f32 le
        cursor.set_position(0);
        cursor.write_f32_le(f32::MAX - 1.1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_f32_le().unwrap(), f32::MAX - 1.1);
    }

    #[test]
    fn test_f64_read_write() {
        let mut buffer = vec![0; 8];
        let mut cursor = Cursor::new(buffer.as_mut_slice());
        // f64 be
        cursor.write_f64_be(f64::MAX - 1.1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_f64_be().unwrap(), f64::MAX - 1.1);
        // f64 le
        cursor.set_position(0);
        cursor.write_f64_le(f64::MAX - 1.1).unwrap();
        cursor.set_position(0);
        assert_eq!(cursor.read_f64_le().unwrap(), f64::MAX - 1.1);
    }

    #[test]
    fn test_read_bytes() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let mut cursor = Cursor::new(&bytes);
        let read_bytes = cursor.read_bytes(4).unwrap();
        let x: [u8; 4] = read_bytes.as_slice().try_into().unwrap();
        assert_eq!(u32::from_be_bytes(x), u32::MAX);
    }

    #[test]
    fn test_tcp_stream() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        let mut client = TcpStream::connect("127.0.0.1:8080").unwrap();
        let (tx, rx) = std::sync::mpsc::channel();
        let tx = Arc::new(Mutex::new(tx));

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut string = String::new();
                loop {
                    let char = stream.read_u8();
                    match char {
                        Ok(c) => string.push(c as char),
                        Err(_) => break,
                    }
                }

                assert_eq!(string, "Hello, World!");
                tx.lock().unwrap().send(()).unwrap();
            }
        });

        std::thread::spawn(move || {
            let message = "Hello, World!".as_bytes();
            for &byte in message {
                client.write_u8(byte).unwrap();
            }
        });

        rx.recv().unwrap();
    }
}
