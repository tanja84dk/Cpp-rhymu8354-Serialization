use super::{
    Error,
    Result,
};

pub struct Serializer<'buffer> {
    buffer: &'buffer mut Vec<u8>,
}

impl<'buffer> Serializer<'buffer> {
    fn new(buffer: &'buffer mut Vec<u8>) -> Self {
        Self {
            buffer,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_usize(
        &mut self,
        mut v: usize,
    ) {
        let mut stack = Vec::new();
        stack.reserve(8);
        while v & !0x7F != 0 {
            stack.push((v & 0x7F) as u8);
            v >>= 7;
            if v == 0 {
                break;
            }
        }
        let more = if stack.is_empty() {
            0x00
        } else {
            0x80
        };
        self.buffer.push((v as u8) | more);
        while !stack.is_empty() {
            let mut next = stack.pop().unwrap();
            if !stack.is_empty() {
                next |= 0x80;
            }
            self.buffer.push(next);
        }
    }
}

impl<'buffer> serde::Serializer for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();
    type SerializeMap = Self;
    type SerializeSeq = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;

    fn serialize_bool(
        self,
        v: bool,
    ) -> Result<Self::Ok> {
        self.buffer.push(if v {
            1
        } else {
            0
        });
        Ok(())
    }

    fn serialize_i8(
        self,
        v: i8,
    ) -> Result<Self::Ok> {
        #[allow(clippy::cast_sign_loss)]
        self.buffer.push(v as u8);
        Ok(())
    }

    fn serialize_i16(
        self,
        v: i16,
    ) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(
        self,
        v: i32,
    ) -> Result<Self::Ok> {
        self.serialize_i64(i64::from(v))
    }

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn serialize_i64(
        self,
        v: i64,
    ) -> Result<Self::Ok> {
        let (sign, mut abs) = if v >= 0 {
            (0x00_u8, v as u64)
        } else {
            (0x40_u8, (-v) as u64)
        };
        let mut stack = Vec::new();
        stack.reserve(8);
        while abs & !(0x3F_u64) != 0 {
            stack.push((abs & 0x7F) as u8);
            abs >>= 7;
            if abs == 0 {
                break;
            }
        }
        let more = if stack.is_empty() {
            0x00
        } else {
            0x80
        };
        self.buffer.push((abs as u8) | sign | more);
        while !stack.is_empty() {
            let mut next = stack.pop().unwrap();
            if !stack.is_empty() {
                next |= 0x80;
            }
            self.buffer.push(next);
        }
        Ok(())
    }

    fn serialize_u8(
        self,
        v: u8,
    ) -> Result<Self::Ok> {
        self.buffer.push(v as u8);
        Ok(())
    }

    fn serialize_u16(
        self,
        v: u16,
    ) -> Result<Self::Ok> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(
        self,
        v: u32,
    ) -> Result<Self::Ok> {
        self.serialize_u64(u64::from(v))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_u64(
        mut self,
        v: u64,
    ) -> Result<Self::Ok> {
        self.serialize_usize(v as usize);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_f32(
        self,
        v: f32,
    ) -> Result<Self::Ok> {
        let v = unsafe { *(&v as *const f32 as *const u32) };
        for i in (0..4).rev() {
            self.buffer.push(((v >> (i * 8)) & 0xFF) as u8);
        }
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn serialize_f64(
        self,
        v: f64,
    ) -> Result<Self::Ok> {
        let v = unsafe { *(&v as *const f64 as *const u64) };
        for i in (0..8).rev() {
            self.buffer.push(((v >> (i * 8)) & 0xFF) as u8);
        }
        Ok(())
    }

    fn serialize_char(
        self,
        v: char,
    ) -> Result<Self::Ok> {
        let mut bytes = [0; 4];
        let slice = v.encode_utf8(&mut bytes);
        self.buffer.extend(slice.as_bytes());
        Ok(())
    }

    fn serialize_str(
        mut self,
        v: &str,
    ) -> Result<Self::Ok> {
        let bytes = v.as_bytes();
        self.serialize_usize(v.len());
        self.buffer.extend(bytes);
        Ok(())
    }

    fn serialize_bytes(
        self,
        v: &[u8],
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(
        self,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_struct(
        self,
        name: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(
        self,
        len: usize,
    ) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        todo!()
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        todo!()
    }
}

impl<'buffer> serde::ser::SerializeMap for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();

    fn serialize_key<T: ?Sized>(
        &mut self,
        key: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'buffer> serde::ser::SerializeSeq for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'buffer> serde::ser::SerializeStruct for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'buffer> serde::ser::SerializeStructVariant for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'buffer> serde::ser::SerializeTuple for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();

    fn serialize_element<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'buffer> serde::ser::SerializeTupleStruct for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'buffer> serde::ser::SerializeTupleVariant for Serializer<'buffer> {
    type Error = Error;
    type Ok = ();

    fn serialize_field<T: ?Sized>(
        &mut self,
        value: &T,
    ) -> Result<()>
    where
        T: serde::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

fn serialize<T>(value: &T) -> Result<Vec<u8>>
where
    T: serde::Serialize,
{
    let mut buffer = Vec::new();
    serde::Serialize::serialize(value, Serializer::new(&mut buffer));
    Ok(buffer)
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use super::*;

    #[test]
    fn serialize_bool() {
        for (value, expected) in &[(false, &[0][..]), (true, &[1][..])] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_i8() {
        for (value, expected) in &[(42_i8, &[42][..]), (-42_i8, &[0xD6][..])] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_i16() {
        for (value, expected) in &[
            (42_i16, &[0x2A][..]),
            (-42_i16, &[0x6A][..]),
            (4000_i16, &[0x9F, 0x20][..]),
            (-4000_i16, &[0xDF, 0x20][..]),
            (9001_i16, &[0x80, 0xC6, 0x29][..]),
            (-9001_i16, &[0xC0, 0xC6, 0x29][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_i32() {
        for (value, expected) in &[
            (42_i32, &[0x2A][..]),
            (-42_i32, &[0x6A][..]),
            (4000_i32, &[0x9F, 0x20][..]),
            (-4000_i32, &[0xDF, 0x20][..]),
            (9001_i32, &[0x80, 0xC6, 0x29][..]),
            (-9001_i32, &[0xC0, 0xC6, 0x29][..]),
            (70_000_i32, &[0x84, 0xA2, 0x70][..]),
            (-70_000_i32, &[0xC4, 0xA2, 0x70][..]),
            (2_000_000_i32, &[0x80, 0xFA, 0x89, 0x00][..]),
            (-2_000_000_i32, &[0xC0, 0xFA, 0x89, 0x00][..]),
            (2_000_000_000_i32, &[0x87, 0xB9, 0xD6, 0xA8, 0x00][..]),
            (-2_000_000_000_i32, &[0xC7, 0xB9, 0xD6, 0xA8, 0x00][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_i64() {
        for (value, expected) in &[
            (42_i64, &[0x2A][..]),
            (-42_i64, &[0x6A][..]),
            (4000_i64, &[0x9F, 0x20][..]),
            (-4000_i64, &[0xDF, 0x20][..]),
            (9001_i64, &[0x80, 0xC6, 0x29][..]),
            (-9001_i64, &[0xC0, 0xC6, 0x29][..]),
            (70_000_i64, &[0x84, 0xA2, 0x70][..]),
            (-70_000_i64, &[0xC4, 0xA2, 0x70][..]),
            (2_000_000_i64, &[0x80, 0xFA, 0x89, 0x00][..]),
            (-2_000_000_i64, &[0xC0, 0xFA, 0x89, 0x00][..]),
            (2_000_000_000_i64, &[0x87, 0xB9, 0xD6, 0xA8, 0x00][..]),
            (-2_000_000_000_i64, &[0xC7, 0xB9, 0xD6, 0xA8, 0x00][..]),
            (2_000_000_000_000_i64, &[0xBA, 0x9A, 0xCA, 0xA8, 0xC0, 0x00][..]),
            (-2_000_000_000_000_i64, &[0xFA, 0x9A, 0xCA, 0xA8, 0xC0, 0x00][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_u8() {
        for (value, expected) in &[(42_u8, &[42][..]), (255_u8, &[0xFF][..])] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_u16() {
        for (value, expected) in &[
            (42_u16, &[42][..]),
            (255_u16, &[0x81, 0x7F][..]),
            (9001_u16, &[0xC6, 0x29][..]),
            (40000_u16, &[0x82, 0xB8, 0x40][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_u32() {
        for (value, expected) in &[
            (42_u32, &[42][..]),
            (255_u32, &[0x81, 0x7F][..]),
            (9001_u32, &[0xC6, 0x29][..]),
            (40000_u32, &[0x82, 0xB8, 0x40][..]),
            (30_000_000_u32, &[0x8E, 0xA7, 0x87, 0x00][..]),
            (4_000_000_000_u32, &[0x8E, 0xF3, 0xAC, 0xD0, 0x00][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_u64() {
        for (value, expected) in &[
            (42_u64, &[42][..]),
            (255_u64, &[0x81, 0x7F][..]),
            (9001_u64, &[0xC6, 0x29][..]),
            (40000_u64, &[0x82, 0xB8, 0x40][..]),
            (30_000_000_u64, &[0x8E, 0xA7, 0x87, 0x00][..]),
            (4_000_000_000_u64, &[0x8E, 0xF3, 0xAC, 0xD0, 0x00][..]),
            (
                90_000_000_000_000_u64,
                &[0x94, 0xBB, 0xAC, 0x90, 0x9E, 0xC0, 0x00][..],
            ),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_f32() {
        for (value, expected) in &[
            (3.141_592_5_f32, &[0x40, 0x49, 0x0F, 0xDA][..]),
            (-10_f32, &[0xC1, 0x20, 0x00, 0x00][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_f64() {
        for (value, expected) in &[
            #[allow(clippy::approx_constant)]
            (
                3.141_592_653_5_f64,
                &[0x40, 0x09, 0x21, 0xFB, 0x54, 0x41, 0x17, 0x44][..],
            ),
            (-10_f64, &[0xC0, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_char() {
        for (value, expected) in &[
            ('X', &[88][..]),
            ('£', &[0xC2, 0xA3][..]),
            ('€', &[0xE2, 0x82, 0xAC][..]),
            ('💩', &[0xF0, 0x9F, 0x92, 0xA9][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }

    #[test]
    fn serialize_str() {
        for (value, expected) in &[
            (
                "Hello, World!",
                &[
                    0x0D, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f,
                    0x72, 0x6c, 0x64, 0x21,
                ][..],
            ),
            ("A≢Α.", &[0x07, 0x41, 0xe2, 0x89, 0xa2, 0xce, 0x91, 0x2e][..]),
        ] {
            let serialization = serialize(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
    }
}
