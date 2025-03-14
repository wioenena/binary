use std::io::{Read, Result};

pub trait BinaryReader {
    fn read_u8(&mut self) -> Result<u8>;
    fn read_u16(&mut self) -> Result<u16>;
    fn read_u16_le(&mut self) -> Result<u16>;
    fn read_u32(&mut self) -> Result<u32>;
    fn read_u32_le(&mut self) -> Result<u32>;
    fn read_u64(&mut self) -> Result<u64>;
    fn read_u64_le(&mut self) -> Result<u64>;
    fn read_u128(&mut self) -> Result<u128>;
    fn read_u128_le(&mut self) -> Result<u128>;
    fn read_i8(&mut self) -> Result<i8>;
    fn read_i16(&mut self) -> Result<i16>;
    fn read_i16_le(&mut self) -> Result<i16>;
    fn read_i32(&mut self) -> Result<i32>;
    fn read_i32_le(&mut self) -> Result<i32>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_i64_le(&mut self) -> Result<i64>;
    fn read_i128(&mut self) -> Result<i128>;
    fn read_i128_le(&mut self) -> Result<i128>;
    fn read_f32(&mut self) -> Result<f32>;
    fn read_f32_le(&mut self) -> Result<f32>;
    fn read_f64(&mut self) -> Result<f64>;
    fn read_f64_le(&mut self) -> Result<f64>;
    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>>;
}

impl<T: Read> BinaryReader for T {
    fn read_u8(&mut self) -> Result<u8> {
        let mut buffer = 0u8;
        self.read_exact(std::slice::from_mut(&mut buffer))?;
        Ok(buffer)
    }

    fn read_u16(&mut self) -> Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }

    fn read_u16_le(&mut self) -> Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(u16::from_le_bytes(buffer))
    }

    fn read_u32(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_be_bytes(buffer))
    }

    fn read_u32_le(&mut self) -> Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(u32::from_le_bytes(buffer))
    }

    fn read_u64(&mut self) -> Result<u64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_be_bytes(buffer))
    }

    fn read_u64_le(&mut self) -> Result<u64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(u64::from_le_bytes(buffer))
    }

    fn read_u128(&mut self) -> Result<u128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)?;
        Ok(u128::from_be_bytes(buffer))
    }

    fn read_u128_le(&mut self) -> Result<u128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)?;
        Ok(u128::from_le_bytes(buffer))
    }

    fn read_i8(&mut self) -> Result<i8> {
        let mut buffer = 0u8;
        self.read_exact(std::slice::from_mut(&mut buffer))?;
        Ok(buffer as i8)
    }

    fn read_i16(&mut self) -> Result<i16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(i16::from_be_bytes(buffer))
    }

    fn read_i16_le(&mut self) -> Result<i16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)?;
        Ok(i16::from_le_bytes(buffer))
    }

    fn read_i32(&mut self) -> Result<i32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(i32::from_be_bytes(buffer))
    }

    fn read_i32_le(&mut self) -> Result<i32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(i32::from_le_bytes(buffer))
    }

    fn read_i64(&mut self) -> Result<i64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(i64::from_be_bytes(buffer))
    }

    fn read_i64_le(&mut self) -> Result<i64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(i64::from_le_bytes(buffer))
    }

    fn read_i128(&mut self) -> Result<i128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)?;
        Ok(i128::from_be_bytes(buffer))
    }

    fn read_i128_le(&mut self) -> Result<i128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)?;
        Ok(i128::from_le_bytes(buffer))
    }

    fn read_f32(&mut self) -> Result<f32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(f32::from_be_bytes(buffer))
    }

    fn read_f32_le(&mut self) -> Result<f32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;
        Ok(f32::from_le_bytes(buffer))
    }

    fn read_f64(&mut self) -> Result<f64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(f64::from_be_bytes(buffer))
    }

    fn read_f64_le(&mut self) -> Result<f64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)?;
        Ok(f64::from_le_bytes(buffer))
    }

    fn read_bytes(&mut self, len: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; len];
        self.read_exact(buffer.as_mut_slice())?;
        Ok(buffer)
    }
}
