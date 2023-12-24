pub struct OutputBuilder {
    output: String,
    indent_symbol: String,
    indent_level: usize,
    comment_symbol: String,
}
impl OutputBuilder {
    /// Creates a new output builder.
    pub fn new(lang_name: &str, indent_symbol: &str, comment_symbol: &str) -> Self {
        let mut s = Self {
            indent_symbol: indent_symbol.to_string(),
            comment_symbol: comment_symbol.to_string(),
            output: String::new(),
            indent_level: 0,
        };

        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");
        let authors = env!("CARGO_PKG_AUTHORS");

        s.add_multiline_section(&vec![
            "Output Metadata".to_string(),
            format!("Target: {}", lang_name),
            format!("Compiler: {} v{}", name, version),
            format!("{}", authors),
        ]);

        s.add_line();
        s
    }

    /// Adds a comment to the code.
    pub fn add_comment(&mut self, comment: &str) {
        self.push_line(&format!("{} {}", self.comment_symbol, comment));
    }

    /// Adds multiple comments to the code.
    pub fn add_comments(&mut self, comments: &Vec<String>) {
        for comment in comments {
            self.add_comment(comment);
        }
    }

    pub fn add_multiline_section(&mut self, sections: &Vec<String>) {
        let terminal_width = 80;

        // Build section break
        let section_break = self
            .comment_symbol
            .repeat(terminal_width)
            .chars()
            .take(terminal_width)
            .collect::<String>();

        self.push_line(&section_break);

        // Build content
        for section in sections {
            let content = format!(" {} ", section);
            let prefix_length = section_break.len() / 2 - content.len() / 2;
            let content = format!(
                "{}{}{}",
                section_break
                    .chars()
                    .take(prefix_length)
                    .collect::<String>(),
                content,
                section_break
                    .chars()
                    .skip(prefix_length + content.len())
                    .collect::<String>(),
            );

            self.push_line(&content);
        }

        self.push_line(&section_break);
        self.add_line();
    }

    /// Adds a new section to the code.
    pub fn add_section(&mut self, section: &str) {
        self.add_multiline_section(&vec![section.to_string()]);
    }

    /// Adds an indentation level.
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    /// Removes an indentation level.
    pub fn unindent(&mut self) {
        self.indent_level -= 1;
    }

    /// Adds indentation to the output.
    pub fn add_indentation(&mut self) {
        for _ in 0..self.indent_level {
            self.output.push_str(&self.indent_symbol);
        }
    }

    /// Pushes the string to the output.
    pub fn push(&mut self, s: &str) {
        self.output.push_str(s);
    }

    /// Pushes the string to the output and adds a new line.
    pub fn push_line(&mut self, line: &str) {
        self.add_indentation();
        self.output.push_str(line);
        self.add_line();
    }

    /// Adds a new line to the output.
    pub fn add_line(&mut self) {
        self.output.push('\n');
    }

    /// Consumes the builder and returns the final output.
    pub fn build(self) -> String {
        self.output
    }
}
