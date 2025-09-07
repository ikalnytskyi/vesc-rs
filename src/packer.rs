use crate::Error;

/// A utility for structured serialization into a pre-allocated buffer. It's
/// designed for the VESC communication protocol, which requires big-endian byte
/// order and represents floating-point values as scaled integers. This struct
/// wraps the buffer and write offset to provide a safer abstraction over raw
/// slice manipulation.
pub struct Packer<'a> {
    pub buf: &'a mut [u8],
    pub pos: usize,
}

impl<'a> Packer<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    #[inline]
    pub fn pack_u32(&mut self, value: u32) -> Result<(), Error> {
        self.pack(&value.to_be_bytes())
    }

    #[inline]
    pub fn pack_i32(&mut self, value: i32) -> Result<(), Error> {
        self.pack(&value.to_be_bytes())
    }

    #[inline]
    pub fn pack_u16(&mut self, value: u16) -> Result<(), Error> {
        self.pack(&value.to_be_bytes())
    }

    #[inline]
    pub fn pack_u8(&mut self, value: u8) -> Result<(), Error> {
        self.pack(&value.to_be_bytes())
    }

    #[inline]
    pub fn pack_f32(&mut self, value: f32, scale: f32) -> Result<(), Error> {
        self.pack_i32((value * scale) as i32)
    }

    #[inline]
    fn pack(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let need = bytes.len();
        if self.pos + need > self.buf.len() {
            return Err(Error::BufferTooSmall);
        }
        self.buf[self.pos..self.pos + need].copy_from_slice(bytes);
        self.pos += need;
        Ok(())
    }
}

/// Provides a read-only, forward-only cursor over a byte slice for
/// deserialization. It's designed for the VESC protocol, which uses big-endian
/// byte order and represents floats as scaled integers. By encapsulating the
/// mutable state (the current position) away from the parsing logic, this
/// struct simplifies consumption of VESC data packets.
pub struct Unpacker<'a> {
    pub buf: &'a [u8],
    pub pos: usize,
}

impl<'a> Unpacker<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    #[inline]
    pub fn unpack_u32(&mut self) -> Result<u32, Error> {
        Ok(u32::from_be_bytes(self.consume(4)?.try_into().unwrap()))
    }

    #[inline]
    pub fn unpack_i32(&mut self) -> Result<i32, Error> {
        Ok(i32::from_be_bytes(self.consume(4)?.try_into().unwrap()))
    }

    #[inline]
    pub fn unpack_u16(&mut self) -> Result<u16, Error> {
        Ok(u16::from_be_bytes(self.consume(2)?.try_into().unwrap()))
    }

    #[inline]
    pub fn unpack_i16(&mut self) -> Result<i16, Error> {
        Ok(i16::from_be_bytes(self.consume(2)?.try_into().unwrap()))
    }

    #[inline]
    pub fn unpack_u8(&mut self) -> Result<u8, Error> {
        Ok(u8::from_be_bytes(self.consume(1)?.try_into().unwrap()))
    }

    #[inline]
    pub fn unpack_f32(&mut self, scale: f32) -> Result<f32, Error> {
        Ok(self.unpack_i32()? as f32 / scale)
    }

    #[inline]
    pub fn unpack_f16(&mut self, scale: f32) -> Result<f32, Error> {
        Ok(self.unpack_i16()? as f32 / scale)
    }

    #[inline]
    fn consume(&mut self, amount: usize) -> Result<&[u8], Error> {
        if self.pos + amount > self.buf.len() {
            return Err(Error::IncompleteData);
        }
        self.pos += amount;
        Ok(&self.buf[self.pos - amount..self.pos])
    }
}
