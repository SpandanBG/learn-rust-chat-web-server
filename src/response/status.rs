pub struct Status {
    pub status_code: String,
}

impl Status {
    pub fn new(status_code: String) -> Status {
        Status { status_code }
    }
}
