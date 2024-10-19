use crate::tools::filesystem::FileSystem;

#[derive(Debug)]
pub struct Response<'a> {
    filesystem: &'a FileSystem,
    pub status_line: String,
    pub contents: String,
    pub response_data: String
}

impl<'a> Response<'a> {
    pub fn new(filesystem: &'a FileSystem) -> Self {
        let empty_string = String::new();
        Self {
            filesystem: filesystem,
            status_line: empty_string.clone(),
            contents: empty_string.clone(),
            response_data: empty_string
        }
    }

    pub fn format_file(&mut self, string_path: String) {
        self.status_line = String::from("HTTP/1.1 200 OK");
        match self.filesystem.get_template(string_path) {
            Some(contents) => {
                self.contents = contents;
                self.response_data = Self::format_response(self);
            },
            None => { 
                self.response_data = Self::format_error(self);
            }
        };
    }

    pub fn format_error(&mut self) -> String {
        self.status_line = String::from("HTTP/1.1 404 NOT FOUND");
        self.contents = String::from("NOT FOUND");
        Self::format_response(&self)
    }

    fn format_response(&self) -> String {
        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line,
            self.contents.len(),
            self.contents
        )
    }
}