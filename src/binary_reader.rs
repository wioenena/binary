pub trait BinaryReader {
    fn read_u8(&mut self) -> std::io::Result<u8>;
    fn read_u16_be(&mut self) -> std::io::Result<u16>;
    fn read_u16_le(&mut self) -> std::io::Result<u16>;
    fn read_u32_be(&mut self) -> std::io::Result<u32>;
    fn read_u32_le(&mut self) -> std::io::Result<u32>;
    fn read_u64_be(&mut self) -> std::io::Result<u64>;
    fn read_u64_le(&mut self) -> std::io::Result<u64>;
    fn read_u128_be(&mut self) -> std::io::Result<u128>;
    fn read_u128_le(&mut self) -> std::io::Result<u128>;
    fn read_i8(&mut self) -> std::io::Result<i8>;
    fn read_i16_be(&mut self) -> std::io::Result<i16>;
    fn read_i16_le(&mut self) -> std::io::Result<i16>;
    fn read_i32_be(&mut self) -> std::io::Result<i32>;
    fn read_i32_le(&mut self) -> std::io::Result<i32>;
    fn read_i64_be(&mut self) -> std::io::Result<i64>;
    fn read_i64_le(&mut self) -> std::io::Result<i64>;
    fn read_i128_be(&mut self) -> std::io::Result<i128>;
    fn read_i128_le(&mut self) -> std::io::Result<i128>;
    fn read_f32_be(&mut self) -> std::io::Result<f32>;
    fn read_f32_le(&mut self) -> std::io::Result<f32>;
    fn read_f64_be(&mut self) -> std::io::Result<f64>;
    fn read_f64_le(&mut self) -> std::io::Result<f64>;
    fn read_bytes(&mut self, len: usize) -> std::io::Result<Vec<u8>>;
}

impl<T: std::io::Read> BinaryReader for T {
    fn read_u8(&mut self) -> std::io::Result<u8> {
        let mut buffer = 0u8;
        self.read_exact(std::slice::from_mut(&mut buffer))?;
        Ok(buffer)
    }

    fn read_u16_be(&mut self) -> std::io::Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)
            .map(|_| u16::from_be_bytes(buffer))
    }

    fn read_u16_le(&mut self) -> std::io::Result<u16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)
            .map(|_| u16::from_le_bytes(buffer))
    }

    fn read_u32_be(&mut self) -> std::io::Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)
            .map(|_| u32::from_be_bytes(buffer))
    }

    fn read_u32_le(&mut self) -> std::io::Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)
            .map(|_| u32::from_le_bytes(buffer))
    }

    fn read_u64_be(&mut self) -> std::io::Result<u64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)
            .map(|_| u64::from_be_bytes(buffer))
    }

    fn read_u64_le(&mut self) -> std::io::Result<u64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)
            .map(|_| u64::from_le_bytes(buffer))
    }

    fn read_u128_be(&mut self) -> std::io::Result<u128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)
            .map(|_| u128::from_be_bytes(buffer))
    }

    fn read_u128_le(&mut self) -> std::io::Result<u128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)
            .map(|_| u128::from_le_bytes(buffer))
    }

    fn read_i8(&mut self) -> std::io::Result<i8> {
        let mut buffer = 0u8;
        self.read_exact(std::slice::from_mut(&mut buffer))?;
        Ok(buffer as i8)
    }

    fn read_i16_be(&mut self) -> std::io::Result<i16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)
            .map(|_| i16::from_be_bytes(buffer))
    }

    fn read_i16_le(&mut self) -> std::io::Result<i16> {
        let mut buffer = [0; 2];
        self.read_exact(&mut buffer)
            .map(|_| i16::from_le_bytes(buffer))
    }

    fn read_i32_be(&mut self) -> std::io::Result<i32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)
            .map(|_| i32::from_be_bytes(buffer))
    }

    fn read_i32_le(&mut self) -> std::io::Result<i32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)
            .map(|_| i32::from_le_bytes(buffer))
    }

    fn read_i64_be(&mut self) -> std::io::Result<i64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)
            .map(|_| i64::from_be_bytes(buffer))
    }

    fn read_i64_le(&mut self) -> std::io::Result<i64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)
            .map(|_| i64::from_le_bytes(buffer))
    }

    fn read_i128_be(&mut self) -> std::io::Result<i128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)
            .map(|_| i128::from_be_bytes(buffer))
    }

    fn read_i128_le(&mut self) -> std::io::Result<i128> {
        let mut buffer = [0; 16];
        self.read_exact(&mut buffer)
            .map(|_| i128::from_le_bytes(buffer))
    }

    fn read_f32_be(&mut self) -> std::io::Result<f32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)
            .map(|_| f32::from_be_bytes(buffer))
    }

    fn read_f32_le(&mut self) -> std::io::Result<f32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)
            .map(|_| f32::from_le_bytes(buffer))
    }

    fn read_f64_be(&mut self) -> std::io::Result<f64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)
            .map(|_| f64::from_be_bytes(buffer))
    }

    fn read_f64_le(&mut self) -> std::io::Result<f64> {
        let mut buffer = [0; 8];
        self.read_exact(&mut buffer)
            .map(|_| f64::from_le_bytes(buffer))
    }

    fn read_bytes(&mut self, len: usize) -> std::io::Result<Vec<u8>> {
        let mut buffer = vec![0; len];
        self.read_exact(buffer.as_mut_slice())?;
        Ok(buffer)
    }
}
