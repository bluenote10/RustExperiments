#![allow(dead_code)]

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
