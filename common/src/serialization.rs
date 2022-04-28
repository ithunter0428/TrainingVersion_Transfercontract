use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};

pub trait ReadInt {
    fn read_byte(&mut self) -> u8;

    fn read_u64_be(&mut self) -> u64;
    fn read_i64_be(&mut self) -> i64;

    fn read_i32_be(&mut self) -> i32;
    fn read_i32_le(&mut self) -> i32;

    fn read_u32_be(&mut self) -> u32;
    fn read_u32_le(&mut self) -> u32;
}

pub trait WriteInt {
    fn write_byte(&mut self, val: u8) -> std::io::Result<()>;

    fn write_u64_be(&mut self, val: u64) -> std::io::Result<()>;
    fn write_i64_be(&mut self, val: i64) -> std::io::Result<()>;

    fn write_i32_be(&mut self, val: i32) -> std::io::Result<()>;
    fn write_i32_le(&mut self, val: i32) -> std::io::Result<()>;

    fn write_u32_be(&mut self, val: u32) -> std::io::Result<()>;
    fn write_u32_le(&mut self, val: u32) -> std::io::Result<()>;
}

impl<T: Write> WriteInt for T {
    fn write_byte(&mut self, val: u8) -> std::io::Result<()> {
        self.write_all(&[val])
    }

    fn write_u64_be(&mut self, val: u64) -> std::io::Result<()> {
        let buf = u64::to_be_bytes(val);
        self.write_all(&buf)
    }
    fn write_i64_be(&mut self, val: i64) -> std::io::Result<()> {
        let buf = i64::to_be_bytes(val);
        self.write_all(&buf)
    }

    fn write_i32_be(&mut self, val: i32) -> std::io::Result<()> {
        let buf = i32::to_be_bytes(val);
        self.write_all(&buf)
    }

    fn write_i32_le(&mut self, val: i32) -> std::io::Result<()> {
        let buf = i32::to_le_bytes(val);
        self.write_all(&buf)
    }

    fn write_u32_be(&mut self, val: u32) -> std::io::Result<()> {
        let buf = u32::to_be_bytes(val);
        self.write_all(&buf)
    }

    fn write_u32_le(&mut self, val: u32) -> std::io::Result<()> {
        let buf = u32::to_le_bytes(val);
        self.write_all(&buf)
    }
}

impl<T: Read> ReadInt for T {
    fn read_byte(&mut self) -> u8 {
        let mut buf = [0];
        self.read_exact(&mut buf).unwrap();
        buf[0]
    }

    fn read_u64_be(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).unwrap();

        u64::from_be_bytes(buf)
    }

    fn read_i64_be(&mut self) -> i64 {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).unwrap();

        i64::from_be_bytes(buf)
    }

    fn read_i32_be(&mut self) -> i32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).unwrap();

        i32::from_be_bytes(buf)
    }

    fn read_i32_le(&mut self) -> i32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).unwrap();

        i32::from_le_bytes(buf)
    }

    fn read_u32_be(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).unwrap();

        u32::from_be_bytes(buf)
    }

    fn read_u32_le(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf).unwrap();

        u32::from_le_bytes(buf)
    }
}

pub trait ReadWrite: Sized {
    fn read_from<T: Read>(reader: &mut T) -> Self;
    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()>;
}

impl ReadWrite for Vec<u8> {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        let len = reader.read_u32_be() as usize;
        let mut result = Vec::with_capacity(len);
        result.resize(len, 0);
        reader.read_exact(&mut result).unwrap();
        result
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u32_be(self.len() as u32).unwrap();
        writer.write_all(&self)
    }
}

impl<S: ReadWrite> ReadWrite for Vec<S> {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        let len = reader.read_i32_be() as usize;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(S::read_from(reader))
        }
        result
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_i32_be(self.len() as i32).unwrap();
        for item in self {
            item.write_to(writer).unwrap();
        }

        Ok(())
    }
}

impl<S: ReadWrite> ReadWrite for Option<S> {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        let marker = reader.read_byte();
        match marker {
            0 => None,
            _ => Some(S::read_from(reader)),
        }
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        match &self {
            None => writer.write_byte(0),
            Some(value) => {
                writer.write_byte(1).unwrap();
                value.write_to(writer)
            }
        }
    }
}
impl ReadWrite for String {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        let vec: Vec<u8> = Vec::read_from(reader);
        String::from_utf8(vec).unwrap()
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u32_be(self.len() as u32).unwrap();
        writer.write_all(self.as_bytes())
    }
}

impl<K: ReadWrite + Ord, V: ReadWrite> ReadWrite for BTreeMap<K, V> {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        let mut result = BTreeMap::new();
        let len = reader.read_u32_be();

        for _ in 0..len {
            let key = K::read_from(reader);
            let value = V::read_from(reader);
            if result.insert(key, value).is_some() {
                panic!("Duplicate key added");
            }
        }

        result
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u32_be(self.len() as u32)?;
        for (key, value) in self.iter() {
            key.write_to(writer)?;
            value.write_to(writer)?;
        }
        Ok(())
    }
}

impl<V: ReadWrite + Ord + Copy> ReadWrite for BTreeSet<V> {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        let mut result = BTreeSet::new();
        let mut previous = None;

        let len = reader.read_u32_le();
        for _ in 0..len {
            let value = V::read_from(reader);
            let next = Some(value);
            if next <= previous {
                panic!("Unordered or duplicate key added");
            }

            result.insert(value);
            previous = next;
        }

        result
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u32_le(self.len() as u32)?;
        for value in self.iter() {
            value.write_to(writer)?;
        }
        Ok(())
    }
}

impl ReadWrite for u64 {
    fn read_from<T: Read>(reader: &mut T) -> Self {
        reader.read_u64_be()
    }

    fn write_to<T: Write>(&self, writer: &mut T) -> std::io::Result<()> {
        writer.write_u64_be(*self)
    }
}
