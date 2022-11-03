pub struct Position {
    pub index: i64,
    pub line: i64,
    pub column: i64,
    pub file_name: String,
    pub file_text: String
}

impl Position {
    pub fn new(index: i64, line: i64, column: i64, file_name: String, file_text: String) -> Self {
        Position {
            index, line, column, file_name, file_text
        }
    }

    pub fn advance(&mut self, current_char: char) -> &mut Self{
        self.index += 1;
        self.column += 1;

        if current_char == '\n' {
            self.line += 1;
            self.column = 0;
        }

        self
    }

    pub fn copy(&self) -> Self {
        Position::new(self.index, self.line, self.column, self.file_name.clone(), self.file_text.clone())
    }
}