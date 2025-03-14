mod binary_reader;

pub use binary_reader::BinaryReader;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_u8_and_i8() {
        assert_eq!(
            Cursor::new(u8::MAX.to_be_bytes()).read_u8().unwrap(),
            u8::MAX
        );
        assert_eq!(
            Cursor::new(i8::MIN.to_be_bytes()).read_i8().unwrap(),
            i8::MIN
        );
    }

    #[test]
    fn test_u16() {
        assert_eq!(
            Cursor::new(u16::MAX.to_be_bytes()).read_u16().unwrap(),
            u16::MAX
        );

        assert_eq!(
            Cursor::new(u16::MAX.to_le_bytes()).read_u16_le().unwrap(),
            u16::MAX
        );
    }

    #[test]
    fn test_u32() {
        assert_eq!(
            Cursor::new(u32::MAX.to_be_bytes()).read_u32().unwrap(),
            u32::MAX
        );

        assert_eq!(
            Cursor::new(u32::MAX.to_le_bytes()).read_u32_le().unwrap(),
            u32::MAX
        );
    }

    #[test]
    fn test_u64() {
        assert_eq!(
            Cursor::new(u64::MAX.to_be_bytes()).read_u64().unwrap(),
            u64::MAX
        );

        assert_eq!(
            Cursor::new(u64::MAX.to_le_bytes()).read_u64_le().unwrap(),
            u64::MAX
        );
    }

    #[test]
    fn test_u128() {
        assert_eq!(
            Cursor::new(u128::MAX.to_be_bytes()).read_u128().unwrap(),
            u128::MAX
        );

        assert_eq!(
            Cursor::new(u128::MAX.to_le_bytes()).read_u128_le().unwrap(),
            u128::MAX
        );
    }

    #[test]
    fn test_i16() {
        assert_eq!(
            Cursor::new(i16::MAX.to_be_bytes()).read_i16().unwrap(),
            i16::MAX
        );

        assert_eq!(
            Cursor::new(i16::MAX.to_le_bytes()).read_i16_le().unwrap(),
            i16::MAX
        );
    }

    #[test]
    fn test_i32() {
        assert_eq!(
            Cursor::new(i32::MAX.to_be_bytes()).read_i32().unwrap(),
            i32::MAX
        );

        assert_eq!(
            Cursor::new(i32::MAX.to_le_bytes()).read_i32_le().unwrap(),
            i32::MAX
        );
    }

    #[test]
    fn test_i64() {
        assert_eq!(
            Cursor::new(i64::MAX.to_be_bytes()).read_i64().unwrap(),
            i64::MAX
        );

        assert_eq!(
            Cursor::new(i64::MAX.to_le_bytes()).read_i64_le().unwrap(),
            i64::MAX
        );
    }

    #[test]
    fn test_i128() {
        assert_eq!(
            Cursor::new(i128::MAX.to_be_bytes()).read_i128().unwrap(),
            i128::MAX
        );

        assert_eq!(
            Cursor::new(i128::MAX.to_le_bytes()).read_i128_le().unwrap(),
            i128::MAX
        );
    }

    #[test]
    fn test_f32() {
        assert_eq!(
            Cursor::new(f32::MAX.to_be_bytes()).read_f32().unwrap(),
            f32::MAX
        );

        assert_eq!(
            Cursor::new(f32::MAX.to_le_bytes()).read_f32_le().unwrap(),
            f32::MAX
        );
    }

    #[test]
    fn test_f64() {
        assert_eq!(
            Cursor::new(f64::MAX.to_be_bytes()).read_f64().unwrap(),
            f64::MAX
        );

        assert_eq!(
            Cursor::new(f64::MAX.to_le_bytes()).read_f64_le().unwrap(),
            f64::MAX
        );
    }

    #[test]
    fn test_read_bytes() {
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        let mut cursor = Cursor::new(&bytes);
        let read_bytes = cursor.read_bytes(4).unwrap();
        let x: [u8; 4] = read_bytes.as_slice().try_into().unwrap();
        assert_eq!(u32::from_be_bytes(x), u32::MAX);
    }
}
