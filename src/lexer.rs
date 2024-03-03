/// Wrapper to a generic error encountered during the parsing phase
pub type Result<T> = std::result::Result<T, ParsingError>;

#[derive(Debug, Clone)]
/// Error while the parsing phase
pub struct ParsingError(pub String);

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing Error: {}", self.0)
    }
}

#[derive(Debug, PartialEq)]
/// Token produced by the tokenizer
pub(crate) enum Token {
    OpenParen,
    Symbol(String),
    CloseParen,
    Quote,
    Comma,
    String(String),
    Number(f64),
    // @TODO: support integer.
    End,
}

/// Simple scanner to parse tokens from the source
pub struct Lexer {
    /// Source program to scan
    pub source: String,
    /// List of tokens generated by the lexer
    pub(crate) tokens: Vec<Token>,
    /// The char index at the beginning of the current token parse round
    start: usize,
    /// The index of the char currently parsed in the all `source`
    current: usize,
    /// The actual line in the source code
    line: u32,
}

impl Lexer {
    pub fn init(source: &String) -> Self {
        Lexer {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    /// Parse the source into tokens
    pub fn scan(&mut self) -> Result<()> {
        while !self.is_end() {
            // Make sure to initialize the lexeme start with the current token
            self.start = self.current;
            // Perform the scanning
            self.scan_token()?;
        }
        self.tokens.push(Token::End);
        Ok(())
    }

    /// Scan a token at point
    fn scan_token(&mut self) -> Result<()> {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.tokens.push(Token::OpenParen),
                ')' => self.tokens.push(Token::CloseParen),
                '\'' => self.tokens.push(Token::Quote),
                ',' => self.tokens.push(Token::Comma),
                '"' => self.scan_string()?,
                ';' => self.skip_comment(),
                '0'..='9' => self.scan_number()?,
                // Ignore whitespaces
                ' ' | '\t' | '\r' => (),
                '\n' => self.line += 1,
                // Every else is a symbol
                _ => self.scan_symbol(),
            }
            Ok(())
        } else {
            Err(ParsingError(String::from("Geniric error while parsing")))
        }
    }

    /// Scan a generic symbol
    fn scan_symbol(&mut self) {
        while ([' ', '(', ')', '\n'].iter().all(|&s| s != self.peek())) & (!self.is_end()) {
            // Remember to keep incrementing lines
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        // Extract the substring
        self.tokens.push(Token::Symbol(
            self.source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect(),
        ))
    }

    /// Scan a number
    fn scan_number(&mut self) -> Result<()> {
        while self.peek().is_ascii_digit() {
            self.current += 1;
        }
        if (self.peek() == '.') & (self.peek_next().is_ascii_digit()) {
            self.current += 1;
            while self.peek().is_ascii_digit() {
                self.current += 1;
            }
        }
        let number: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens.push(Token::Number(match number.parse() {
            Ok(it) => it,
            Err(_) => {
                return Err(ParsingError(format!(
                    "{} Error while parsing a number",
                    self.line,
                )))
            }
        }));
        Ok(())
    }

    /// Return the next char in the source file without consuming it
    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    /// Scan a string
    fn scan_string(&mut self) -> Result<()> {
        // Parse until the next "
        while (self.peek() != '"') & (!self.is_end()) {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        if self.is_end() {
            // Error condition, we scanned all the program but no " was found
            return Err(ParsingError(format!(
                "{} Error while parsing a string",
                self.line,
            )));
        }
        // Skip the closing "
        self.current += 1;
        self.tokens.push(Token::String(
            self.source
                .chars()
                .skip(self.start)
                .take(self.current - self.start)
                .collect(),
        ));
        Ok(())
    }

    /// Skip the comment section
    fn skip_comment(&mut self) {
        while (self.peek() != '\n') & (!self.is_end()) {
            self.current += 1;
        }
        // We do not advance the cursor here, since we want the callee to
        // advance the total line number
    }

    /// Check if the scanner is completed, that is, all the chars have been read
    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Advance the scanner, retuting the char at point
    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;
        c
    }

    /// Return the current char the scanner is poining to, without advancing the
    /// iterator
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }
}
