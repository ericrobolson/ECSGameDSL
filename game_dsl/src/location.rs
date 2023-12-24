#[derive(Clone, PartialEq, Debug)]
pub enum Location {
    Text {
        line: usize,
        column: usize,
    },
    File {
        path: std::path::PathBuf,
        line: usize,
        column: usize,
    },
    SystemDefined,
}
impl Location {
    pub fn print(start: &Location, end: &Location) -> String {
        let file = {
            if start.is_file() && !end.is_file() {
                panic!("Cannot print location of text and file")
            }

            let start_file = start.get_file();
            let end_file = end.get_file();

            if start_file != end_file {
                panic!(
                    "Start file and end file differ! {:?} != {:?}",
                    start_file, end_file
                )
            }

            match start_file {
                Some(f) => format!("{} ", f.display()),
                None => String::new(),
            }
        };
        format!(
            "Source location {}{}-{}",
            file,
            start.pretty_print_line_column(),
            end.pretty_print_line_column()
        )
    }

    pub fn pretty_print_line_column(&self) -> String {
        match self {
            Location::Text { line, column } => format!("{}:{}", line, column),
            Location::File { path, line, column } => {
                format!("{}:{}", line, column)
            }
            Location::SystemDefined => "System defined".to_string(),
        }
    }

    pub fn is_text(&self) -> bool {
        match self {
            Location::Text { line: _, column: _ } => true,
            Location::File {
                path: _,
                line: _,
                column: _,
            } => false,
            Location::SystemDefined => false,
        }
    }

    pub fn get_file(&self) -> Option<&std::path::PathBuf> {
        match self {
            Location::Text { line: _, column: _ } => None,
            Location::File {
                path,
                line: _,
                column: _,
            } => Some(path),
            Location::SystemDefined => None,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            Location::Text { line: _, column: _ } => false,
            Location::File {
                path: _,
                line: _,
                column: _,
            } => true,
            Location::SystemDefined => false,
        }
    }

    pub fn increment_line(&mut self) {
        match self {
            Location::Text { line, column } => {
                *line += 1;
                *column = 0;
            }
            Location::File {
                path: _,
                line,
                column,
            } => {
                *line += 1;
                *column = 0;
            }
            Location::SystemDefined => {}
        }
    }

    pub fn increment_column(&mut self) {
        match self {
            Location::Text { line: _, column } => {
                *column += 1;
            }
            Location::File {
                path: _,
                line: _,
                column,
            } => {
                *column += 1;
            }
            Location::SystemDefined => {}
        }
    }

    pub fn subtract_column(&mut self) {
        match self {
            Location::Text { line: _, column } => {
                *column -= 1;
            }
            Location::File {
                path: _,
                line: _,
                column,
            } => {
                *column -= 1;
            }
            Location::SystemDefined => {}
        }
    }

    pub fn line(&self) -> usize {
        match self {
            Location::Text { line, column: _ } => *line,
            Location::File {
                path: _,
                line,
                column: _,
            } => *line,
            Location::SystemDefined => 0,
        }
    }

    pub fn column(&self) -> usize {
        match self {
            Location::Text { line: _, column } => *column,
            Location::File {
                path: _,
                line: _,
                column,
            } => *column,
            Location::SystemDefined => 0,
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location::Text { line: 0, column: 0 }
    }
}
impl From<(usize, usize)> for Location {
    fn from((line, column): (usize, usize)) -> Self {
        Location::Text { line, column }
    }
}
