pub struct Response {
    bytes: Vec<u8>,
}

const HEADER_SIZE: usize = "HTTP/1.1 ".len()
    + "\r\nContent-Length: ".len()
    + "Content-Type: text/plain; charset=utf-8\r\n".len()
    + "Connection: close\r\n".len()
    + "\r\n\r\n".len();

impl Response {
    pub fn text(text: &str, code: u16) -> Self {
        let content_bytes = {
            let mut bytes = Vec::with_capacity(text.len() + 2);
            bytes.extend_from_slice(text.as_bytes());
            bytes.extend_from_slice(b"\r\n");
            bytes
        };

        let content_length = content_bytes.len();
        let mut response = Vec::with_capacity(content_length + HEADER_SIZE + 8);
        response.extend_from_slice(b"HTTP/1.1 ");
        response.extend_from_slice(code.to_string().as_bytes());
        response.extend_from_slice(b"\r\nContent-Length: ");
        response.extend_from_slice(content_length.to_string().as_bytes());
        response.extend_from_slice(b"\r\nContent-Type: text/plain; charset=utf-8\r\n");
        response.extend_from_slice(b"Connection: close\r\n");
        response.extend_from_slice(b"\r\n\r\n");
        response.extend_from_slice(&content_bytes);

        Self { bytes: response }
    }

    pub fn status(code: u16) -> Self {
        Self::text("", code)
    }
}

impl std::ops::Deref for Response {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
