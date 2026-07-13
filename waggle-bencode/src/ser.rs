use serde::Serialize;
use serde::ser;
use serde::ser::{Impossible, SerializeSeq, SerializeTuple, SerializeTupleStruct};
use std::io::Write;

pub fn to_bytes<T>(value: &T) -> crate::error::Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut serializer = Serializer { writer: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.writer)
}

struct Serializer<W> {
    writer: W,
}

struct ListSerializer<'s, W> {
    serializer: &'s mut Serializer<W>,
}

impl<'s, W: Write> ser::Serializer for &'s mut Serializer<W> {
    type Ok = ();
    type Error = crate::error::Error;
    type SerializeSeq = ListSerializer<'s, W>;
    type SerializeTuple = ListSerializer<'s, W>;
    type SerializeTupleStruct = ListSerializer<'s, W>;
    type SerializeTupleVariant = Impossible<(), Self::Error>;
    type SerializeMap = Impossible<(), Self::Error>;
    type SerializeStruct = Impossible<(), Self::Error>;
    type SerializeStructVariant = Impossible<(), Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupported("bool"))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.write_number(v)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupported("f32"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupported("f64"))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buffer = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buffer))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.serialize_bytes(v.as_bytes())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.write_bytes(v)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupported("none"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupported("unit"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(Self::Error::NotSupported("unit_struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let mut list_serializer = ListSerializer { serializer: self };
        list_serializer.start()?;
        Ok(list_serializer)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<W: Write> SerializeSeq for ListSerializer<'_, W> {
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

impl<W: Write> SerializeTuple for ListSerializer<'_, W> {
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

impl<W: Write> SerializeTupleStruct for ListSerializer<'_, W> {
    type Ok = ();
    type Error = crate::error::Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end()
    }
}

impl<W: Write> Serializer<W> {
    fn write_number<I: itoa::Integer>(&mut self, number: I) -> crate::error::Result<()> {
        self.write_number_delimiter()?;
        let mut buffer = itoa::Buffer::new();
        self.writer.write_all(buffer.format(number).as_bytes())?;
        self.write_end()?;
        Ok(())
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> crate::error::Result<()> {
        let mut buffer = itoa::Buffer::new();
        self.writer
            .write_all(buffer.format(bytes.len()).as_bytes())?;
        self.writer.write_all(b":")?;
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn write_number_delimiter(&mut self) -> crate::error::Result<()> {
        self.writer.write_all(b"i")?;
        Ok(())
    }

    fn write_list_delimiter(&mut self) -> crate::error::Result<()> {
        self.writer.write_all(b"l")?;
        Ok(())
    }

    fn write_end(&mut self) -> crate::error::Result<()> {
        self.writer.write_all(b"e")?;
        Ok(())
    }
}

impl<W: Write> ListSerializer<'_, W> {
    fn start(&mut self) -> Result<(), crate::error::Error> {
        self.serializer.write_list_delimiter()
    }
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), crate::error::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.serializer)
    }

    fn end(self) -> Result<(), crate::error::Error> {
        self.serializer.write_end()
    }
}
