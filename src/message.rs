pub struct Message {
    pub data: Vec<u8>,
}

impl Into<Message> for String {
    fn into(self) -> Message {
        return Message {
            data: self.into_bytes(),
        };
    }
}

impl Into<Message> for &str {
    fn into(self) -> Message {
        return Message {
            data: self.as_bytes().to_vec(),
        };
    }
}

impl Into<Message> for &[u8] {
    fn into(self) -> Message {
        return Message {
            data: self.to_vec(),
        };
    }
}

impl Into<Message> for Vec<u8> {
    fn into(self) -> Message {
        return Message { data: self };
    }
}
