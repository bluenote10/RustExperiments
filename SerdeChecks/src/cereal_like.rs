#![allow(dead_code)]

use std::io::Empty;
use std::io::Read;
use std::io::Result;
use std::io::Sink;
use std::io::Write;

/*

// Draft v1

trait Serializer {
    fn write(&self, buf: &[u8]);
    fn read(&self, num_bytes: usize) -> Vec<u8>;
}

struct Foo {
    a: i32,
    b: i32,
}

trait Serialize {
    fn serialize<S: Serializer>(&mut self, serializer: &mut S, mode: Mode);
}

impl Serialize for Foo {
    fn serialize<S: Serializer>(&mut self, serializer: &mut S, mode: Mode) {
        self.a.handle(serializer, mode);
        self.b.handle(serializer, mode);
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Mode {
    Serialize,
    Deserialize,
}

trait Handle {
    fn handle<S: Serializer>(&mut self, handler: &mut S, mode: Mode) {
        if mode == Mode::Serialize {
            self.encode(handler);
        } else {
            self.decode(handler);
        }
    }

    fn encode<S: Serializer>(&self, handler: &mut S);
    fn decode<S: Serializer>(&mut self, handler: &mut S);
}

impl Handle for i32 {
    fn encode<S: Serializer>(&self, handler: &mut S) {
        handler.write(&self.to_le_bytes());
    }

    fn decode<S: Serializer>(&mut self, handler: &mut S) {
        let buf = handler.read(4);
        *self = i32::from_le_bytes(buf.try_into().unwrap());
    }
}

*/

// Draft v2

trait Serializer {
    fn write_i32(&mut self, x: i32) -> Result<()>;
    // ...
}

trait Deserializer {
    fn read_i32(&mut self) -> Result<i32>;
    // ...
}

enum Handler<S: Serializer, D: Deserializer> {
    Serializer(S),
    Deserializer(D),
}

impl<S: Serializer, D: Deserializer> Handler<S, D> {
    fn get_mode(self) -> Mode {
        match self {
            Handler::Serializer(_) => Mode::Serialize,
            Handler::Deserializer(_) => Mode::Deserialize,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Mode {
    Serialize,
    Deserialize,
}

trait Serializable {
    fn handle<S, D>(&mut self, handler: &mut Handler<S, D>) -> Result<()>
    where
        S: Serializer,
        D: Deserializer;
}

impl Serializable for i32 {
    fn handle<S, D>(&mut self, handler: &mut Handler<S, D>) -> Result<()>
    where
        S: Serializer,
        D: Deserializer,
    {
        match handler {
            Handler::Serializer(s) => s.write_i32(*self)?,
            Handler::Deserializer(d) => *self = d.read_i32()?,
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Foo {
    a: i32,
    b: i32,
}

impl Serializable for Foo {
    fn handle<S, D>(&mut self, handler: &mut Handler<S, D>) -> Result<()>
    where
        S: Serializer,
        D: Deserializer,
    {
        self.a.handle(handler)?;
        self.b.handle(handler)?;
        Ok(())
    }
}

// Example Serializer / Deserializer

struct BinarySerializer<W: Write> {
    writer: W,
}

impl<W: Write> Serializer for BinarySerializer<W> {
    fn write_i32(&mut self, x: i32) -> Result<()> {
        self.writer.write_all(&x.to_le_bytes())
    }
}

struct BinaryDeserializer<R: Read> {
    reader: R,
}

impl<R: Read> Deserializer for BinaryDeserializer<R> {
    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0; 4];
        self.reader.read_exact(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
}

type BinarySerializerHandler<W> = Handler<BinarySerializer<W>, BinaryDeserializer<Empty>>;
type BinaryDeserializerHandler<R> = Handler<BinarySerializer<Sink>, BinaryDeserializer<R>>;

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_example_serializer() {
        let mut buf = Vec::new();

        let serializer = BinarySerializer { writer: &mut buf };
        let mut handler = BinarySerializerHandler::Serializer(serializer);

        let mut foo = Foo { a: 1, b: 2 };
        foo.handle(&mut handler).unwrap(); // major design flaw: serializing requires mut.

        assert_eq!(buf, vec![1, 0, 0, 0, 2, 0, 0, 0]);

        let deserializer = BinaryDeserializer {
            reader: Cursor::new(buf),
        };
        let mut handler = BinaryDeserializerHandler::Deserializer(deserializer);

        let mut foo = Foo { a: -1, b: -2 };
        foo.handle(&mut handler).unwrap(); // major design flaw: deserializing is in-place / mutable.

        assert_eq!(foo, Foo { a: 1, b: 2 });
    }
}
