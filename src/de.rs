use super::{
    Error,
    Result,
};

pub struct Deserializer<'buffer> {
    buffer: &'buffer [u8],
}

impl<'buffer> Deserializer<'buffer> {
    fn new(buffer: &'buffer [u8]) -> Self {
        Self {
            buffer,
        }
    }

    fn parse_bool(&mut self) -> Result<bool> {
        self.buffer.iter().next().map_or(Err(Error::ValueTruncated), |byte| {
            self.buffer = &self.buffer[1..];
            Ok(*byte != 0)
        })
    }

    #[allow(clippy::cast_possible_wrap)]
    fn parse_i8(&mut self) -> Result<i8> {
        self.buffer.iter().next().map_or(Err(Error::ValueTruncated), |byte| {
            self.buffer = &self.buffer[1..];
            Ok(*byte as i8)
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn parse_i16(&mut self) -> Result<i16> {
        Ok(self.parse_i64(Some(i64::from(std::i16::MAX)))? as i16)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn parse_i32(&mut self) -> Result<i32> {
        Ok(self.parse_i64(Some(i64::from(std::i32::MAX)))? as i32)
    }

    #[allow(clippy::cast_lossless)]
    fn parse_i64(
        &mut self,
        max: Option<i64>,
    ) -> Result<i64> {
        let mut it = self.buffer.iter();
        let first = it.next().ok_or(Error::ValueTruncated)?;
        self.buffer = &self.buffer[1..];
        let mut more = (first & 0x80) != 0;
        let negative = (first & 0x40) != 0;
        let mut value = (first & 0x3F) as i64;
        while more {
            let next = it.next().ok_or(Error::ValueTruncated)?;
            self.buffer = &self.buffer[1..];
            let lsb = next & 0x7F;
            more = (next & 0x80) != 0;
            // Special case: the negative of exactly one value, `i64::MIN`,
            // won't fit in 64 bits, so we detect it here and return it
            // early before we would overflow.
            if !more
                && negative
                && lsb == 0
                && value == 0x0100_0000_0000_0000_i64
            {
                return Ok(i64::MIN);
            }
            value = value.checked_mul(128).ok_or(Error::IntegerOverflow)?;
            value += lsb as i64;
            if let Some(max) = max {
                if (negative && value > (max + 1)) || (!negative && value > max)
                {
                    return Err(Error::IntegerOverflow);
                }
            }
        }
        Ok(if negative {
            -value
        } else {
            value
        })
    }

    fn parse_u8(&mut self) -> Result<u8> {
        self.buffer.iter().next().map_or(Err(Error::ValueTruncated), |byte| {
            self.buffer = &self.buffer[1..];
            Ok(*byte)
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn parse_u16(&mut self) -> Result<u16> {
        Ok(self.parse_u64(Some(u64::from(std::u16::MAX)))? as u16)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn parse_u32(&mut self) -> Result<u32> {
        Ok(self.parse_u64(Some(u64::from(std::u32::MAX)))? as u32)
    }

    #[allow(clippy::cast_lossless)]
    fn parse_u64(
        &mut self,
        max: Option<u64>,
    ) -> Result<u64> {
        let mut it = self.buffer.iter();
        let first = it.next().ok_or(Error::ValueTruncated)?;
        self.buffer = &self.buffer[1..];
        let mut more = (first & 0x80) != 0;
        let mut value = (first & 0x7F) as u64;
        while more {
            let next = it.next().ok_or(Error::ValueTruncated)?;
            self.buffer = &self.buffer[1..];
            let lsb = next & 0x7F;
            more = (next & 0x80) != 0;
            value = value.checked_mul(128).ok_or(Error::IntegerOverflow)?;
            value += lsb as u64;
            if let Some(max) = max {
                if value > max {
                    return Err(Error::IntegerOverflow);
                }
            }
        }
        Ok(value)
    }

    #[allow(clippy::cast_lossless)]
    fn parse_f32(&mut self) -> Result<f32> {
        if self.buffer.len() < 4 {
            return Err(Error::ValueTruncated);
        }
        let mut value = 0_u32;
        for _ in 0..4 {
            value <<= 8;
            value += self.buffer[0] as u32;
            self.buffer = &self.buffer[1..];
        }
        unsafe {
            let value = *(&value as *const u32 as *const f32);
            Ok(value)
        }
    }

    #[allow(clippy::cast_lossless)]
    fn parse_f64(&mut self) -> Result<f64> {
        if self.buffer.len() < 8 {
            return Err(Error::ValueTruncated);
        }
        let mut value = 0_u64;
        for _ in 0..8 {
            value <<= 8;
            value += self.buffer[0] as u64;
            self.buffer = &self.buffer[1..];
        }
        unsafe {
            let value = *(&value as *const u64 as *const f64);
            Ok(value)
        }
    }

    fn parse_char(&mut self) -> Result<char> {
        let mut it = self.buffer.iter();
        let byte1 = it.next().ok_or(Error::ValueTruncated)?;
        let n = match byte1 {
            0x00..=0x7F => 1,
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => return Err(Error::InvalidUtf8),
        };
        if self.buffer.len() < n {
            return Err(Error::ValueTruncated);
        }
        let ch = std::str::from_utf8(&self.buffer[0..n])
            .map_err(|_| Error::InvalidUtf8)?
            .chars()
            .next()
            .ok_or(Error::InvalidUtf8)?;
        self.buffer = &self.buffer[n..];
        Ok(ch)
    }

    #[allow(clippy::cast_possible_truncation)]
    fn parse_str(&mut self) -> Result<&'buffer str> {
        let len = self.parse_u64(None)? as usize;
        if self.buffer.len() < len {
            Err(Error::ValueTruncated)
        } else {
            let value = std::str::from_utf8(&self.buffer[0..len])
                .map_err(|_| Error::InvalidUtf8)?;
            self.buffer = &self.buffer[len..];
            Ok(value)
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn parse_bytes(&mut self) -> Result<&'buffer [u8]> {
        let len = self.parse_u64(None)? as usize;
        if self.buffer.len() < len {
            Err(Error::ValueTruncated)
        } else {
            let value = &self.buffer[0..len];
            self.buffer = &self.buffer[len..];
            Ok(value)
        }
    }
}

impl<'de, 'buffer> serde::Deserializer<'buffer>
    for &'de mut Deserializer<'buffer>
{
    type Error = Error;

    fn deserialize_any<V>(
        self,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        Err(Error::TypeUnknown)
    }

    fn deserialize_bool<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_i8(self.parse_i8()?)
    }

    fn deserialize_i16<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_i16(self.parse_i16()?)
    }

    fn deserialize_i32<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_i32(self.parse_i32()?)
    }

    fn deserialize_i64<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_i64(self.parse_i64(None)?)
    }

    fn deserialize_u8<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_u16(self.parse_u16()?)
    }

    fn deserialize_u32<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_u64(self.parse_u64(None)?)
    }

    fn deserialize_f32<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_f32(self.parse_f32()?)
    }

    fn deserialize_f64<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_f64(self.parse_f64()?)
    }

    fn deserialize_char<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_char(self.parse_char()?)
    }

    fn deserialize_str<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_string<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    fn deserialize_bytes<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_byte_buf<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    fn deserialize_option<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_unit<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_seq<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_map<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(
        self,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'buffer>,
    {
        todo!()
    }
}

pub fn from_bytes<'buffer, T>(bytes: &'buffer [u8]) -> Result<T>
where
    T: serde::Deserialize<'buffer>,
{
    let mut deserializer = Deserializer::new(bytes);
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
#[allow(clippy::non_ascii_literal)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_bool() {
        for (expected, value) in &[(false, &[0][..]), (true, &[1][..])] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_i8() {
        for (expected, value) in &[(42_i8, &[42][..]), (-42_i8, &[0xD6][..])] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_i16() {
        for (expected, value) in &[
            (42_i16, &[0x2A][..]),
            (-42_i16, &[0x6A][..]),
            (4000_i16, &[0x9F, 0x20][..]),
            (-4000_i16, &[0xDF, 0x20][..]),
            (9001_i16, &[0x80, 0xC6, 0x29][..]),
            (-9001_i16, &[0xC0, 0xC6, 0x29][..]),
            (32767_i16, &[0x81, 0xFF, 0x7F][..]),
            (-32768_i16, &[0xC2, 0x80, 0x00][..]),
        ] {
            let serialization = from_bytes(value);
            assert!(serialization.is_ok());
            let serialization = serialization.unwrap();
            assert_eq!(*expected, serialization);
        }
        let deserialization: Result<i16> = from_bytes(&[0xA0, 0xC6, 0xC9][..]);
        assert!(matches!(deserialization, Err(Error::IntegerOverflow)));
    }

    #[test]
    fn deserialize_i32() {
        for (expected, value) in &[
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
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_i64() {
        for (expected, value) in &[
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
            (
                i64::MAX,
                &[0x80, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F][..],
            ),
            (
                i64::MIN,
                &[0xC1, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00][..],
            ),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
        let deserialization: Result<i64> = from_bytes(
            &[0xC1, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01][..],
        );
        assert!(matches!(deserialization, Err(Error::IntegerOverflow)));
    }

    #[test]
    fn deserialize_u8() {
        for (expected, value) in &[(42_u8, &[42][..]), (255_u8, &[0xFF][..])] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_u16() {
        for (expected, value) in &[
            (42_u16, &[42][..]),
            (255_u16, &[0x81, 0x7F][..]),
            (9001_u16, &[0xC6, 0x29][..]),
            (40000_u16, &[0x82, 0xB8, 0x40][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
        let deserialization: Result<u16> = from_bytes(&[0xA0, 0xC6, 0xC9][..]);
        assert!(matches!(deserialization, Err(Error::IntegerOverflow)));
    }

    #[test]
    fn deserialize_u32() {
        for (expected, value) in &[
            (42_u32, &[42][..]),
            (255_u32, &[0x81, 0x7F][..]),
            (9001_u32, &[0xC6, 0x29][..]),
            (40000_u32, &[0x82, 0xB8, 0x40][..]),
            (30_000_000_u32, &[0x8E, 0xA7, 0x87, 0x00][..]),
            (4_000_000_000_u32, &[0x8E, 0xF3, 0xAC, 0xD0, 0x00][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_u64() {
        for (expected, value) in &[
            (42_u64, &[42][..]),
            (255_u64, &[0x81, 0x7F][..]),
            (9001_u64, &[0xC6, 0x29][..]),
            (40000_u64, &[0x82, 0xB8, 0x40][..]),
            (30_000_000_u64, &[0x8E, 0xA7, 0x87, 0x00][..]),
            (4_000_000_000_u64, &[0x8E, 0xF3, 0xAC, 0xD0, 0x00][..]),
            (
                u64::MAX,
                &[0x81, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F][..],
            ),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
        let deserialization: Result<u64> = from_bytes(
            &[0x82, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00][..],
        );
        assert!(matches!(deserialization, Err(Error::IntegerOverflow)));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn deserialize_f32() {
        for (expected, value) in &[
            (3.141_592_5_f32, &[0x40, 0x49, 0x0F, 0xDA][..]),
            (-10_f32, &[0xC1, 0x20, 0x00, 0x00][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn deserialize_f64() {
        for (expected, value) in &[
            #[allow(clippy::approx_constant)]
            (
                3.141_592_653_5_f64,
                &[0x40, 0x09, 0x21, 0xFB, 0x54, 0x41, 0x17, 0x44][..],
            ),
            (-10_f64, &[0xC0, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_char() {
        for (expected, value) in &[
            ('X', &[88][..]),
            ('£', &[0xC2, 0xA3][..]),
            ('€', &[0xE2, 0x82, 0xAC][..]),
            ('💩', &[0xF0, 0x9F, 0x92, 0xA9][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_str() {
        for (expected, value) in &[
            (
                "Hello, World!",
                &[
                    0x0D, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f,
                    0x72, 0x6c, 0x64, 0x21,
                ][..],
            ),
            ("A≢Α.", &[0x07, 0x41, 0xe2, 0x89, 0xa2, 0xce, 0x91, 0x2e][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization: &str = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_string() {
        for (expected, value) in &[
            (
                "Hello, World!",
                &[
                    0x0D, 0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f,
                    0x72, 0x6c, 0x64, 0x21,
                ][..],
            ),
            ("A≢Α.", &[0x07, 0x41, 0xe2, 0x89, 0xa2, 0xce, 0x91, 0x2e][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization: String = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_bytes() {
        for (expected, value) in &[
            (&[][..], &[0x00][..]),
            (&[0x00_u8][..], &[0x01, 0x00][..]),
            (&[0x12, 0x34, 0x56][..], &[0x03, 0x12, 0x34, 0x56][..]),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization: &[u8] = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }

    #[test]
    fn deserialize_byte_buf() {
        use std::ffi::CString;
        for (expected, value) in &[
            (CString::new(&[][..]).unwrap(), &[0x00][..]),
            (CString::new(&[0x12_u8][..]).unwrap(), &[0x01, 0x12][..]),
            (
                CString::new(&[0x12, 0x34, 0x56][..]).unwrap(),
                &[0x03, 0x12, 0x34, 0x56][..],
            ),
        ] {
            let deserialization = from_bytes(value);
            assert!(deserialization.is_ok());
            let deserialization: std::ffi::CString = deserialization.unwrap();
            assert_eq!(*expected, deserialization);
        }
    }
}
