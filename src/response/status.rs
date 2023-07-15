use crate::response::constants::HTTP_VERSION;

pub struct Status {
    status_code: String,
}

impl Status {
    pub fn new(status_code: String) -> Status {
        Status { status_code }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        format!("{} {}\r\n", HTTP_VERSION, self.status_code)
            .as_bytes()
            .to_owned()
    }
}
