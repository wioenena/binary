use std::io::Write;

pub trait BinaryWriter {
    fn write_u8(&mut self, value: u8) -> std::io::Result<usize>;
    fn write_u16_be(&mut self, value: u16) -> std::io::Result<usize>;
    fn write_u16_le(&mut self, value: u16) -> std::io::Result<usize>;
    fn write_u32_be(&mut self, value: u32) -> std::io::Result<usize>;
    fn write_u32_le(&mut self, value: u32) -> std::io::Result<usize>;
    fn write_u64_be(&mut self, value: u64) -> std::io::Result<usize>;
    fn write_u64_le(&mut self, value: u64) -> std::io::Result<usize>;
    fn write_u128_be(&mut self, value: u128) -> std::io::Result<usize>;
    fn write_u128_le(&mut self, value: u128) -> std::io::Result<usize>;
    fn write_i8(&mut self, value: i8) -> std::io::Result<usize>;
    fn write_i16_be(&mut self, value: i16) -> std::io::Result<usize>;
    fn write_i16_le(&mut self, value: i16) -> std::io::Result<usize>;
    fn write_i32_be(&mut self, value: i32) -> std::io::Result<usize>;
    fn write_i32_le(&mut self, value: i32) -> std::io::Result<usize>;
    fn write_i64_be(&mut self, value: i64) -> std::io::Result<usize>;
    fn write_i64_le(&mut self, value: i64) -> std::io::Result<usize>;
    fn write_i128_be(&mut self, value: i128) -> std::io::Result<usize>;
    fn write_i128_le(&mut self, value: i128) -> std::io::Result<usize>;
    fn write_f32_be(&mut self, value: f32) -> std::io::Result<usize>;
    fn write_f32_le(&mut self, value: f32) -> std::io::Result<usize>;
    fn write_f64_be(&mut self, value: f64) -> std::io::Result<usize>;
    fn write_f64_le(&mut self, value: f64) -> std::io::Result<usize>;
}

impl<T: Write> BinaryWriter for T {
    fn write_u8(&mut self, value: u8) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_u16_be(&mut self, value: u16) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_u16_le(&mut self, value: u16) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_u32_be(&mut self, value: u32) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_u32_le(&mut self, value: u32) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_u64_be(&mut self, value: u64) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_u64_le(&mut self, value: u64) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_u128_be(&mut self, value: u128) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_u128_le(&mut self, value: u128) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_i8(&mut self, value: i8) -> std::io::Result<usize> {
        self.write(&[value as u8])
    }

    fn write_i16_be(&mut self, value: i16) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_i16_le(&mut self, value: i16) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_i32_be(&mut self, value: i32) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_i32_le(&mut self, value: i32) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_i64_be(&mut self, value: i64) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_i64_le(&mut self, value: i64) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_i128_be(&mut self, value: i128) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_i128_le(&mut self, value: i128) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_f32_be(&mut self, value: f32) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_f32_le(&mut self, value: f32) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }

    fn write_f64_be(&mut self, value: f64) -> std::io::Result<usize> {
        self.write(value.to_be_bytes().as_slice())
    }

    fn write_f64_le(&mut self, value: f64) -> std::io::Result<usize> {
        self.write(value.to_le_bytes().as_slice())
    }
}
