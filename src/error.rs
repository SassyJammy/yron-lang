use crate::position::Position;

pub struct Error {
    pub error_name: String,
    pub details: String,
    pub start_position: Position,
    pub end_position: Position,
}

impl Error {
    pub fn new_illegal_char(start_position: Position, end_position: Position, details: String) -> Self {
        Error {
            error_name: "Unknown character".to_string(),
            details,
            start_position,
            end_position
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}: {} in {}({}, {}) ",
            &self.error_name, 
            &self.details, 
            &self.start_position.file_name,
            &self.start_position.line,
            &self.start_position.column
        )
    }
}
