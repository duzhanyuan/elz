#[derive(Clone, Debug, PartialEq)]
pub enum TkType {
    EOF,
    /// e.g. a, ab, foo
    Ident,
    /// return
    Return,
    /// type
    Type,
    /// import
    Import,
    /// contract
    Contract,
    /// impl
    Impl,
    /// for
    For,
    /// e.g. 1, 10, 34
    Num,
    /// "string_literal"
    String,
    /// +
    Plus,
    /// *
    Star,
    /// ,
    Comma,
    /// =
    Assign,
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LBrace,
    /// }
    RBrace,
    /// :
    Colon,
    /// ::
    Accessor,
    /// ;
    Semicolon,
    /// '
    Prime,
    /// |
    VerticalLine,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token((u32, u32), TkType, String);

impl Token {
    pub fn location(&self) -> (u32, u32) {
        self.0
    }
    pub fn tk_type(&self) -> &TkType {
        &self.1
    }
    pub fn value(&self) -> String {
        self.2.clone()
    }
}

enum State {
    Fn(fn(&mut Lexer) -> State),
    EOF,
}

struct Lexer {
    code: Vec<char>,
    tokens: Vec<Token>,
    state_fn: State,
    start: usize,
    offset: usize,
    // (line, pos) represent the position for user
    pos: u32,
    line: u32,
}

impl Lexer {
    fn new(code: String) -> Lexer {
        Lexer {
            code: code.chars().collect(),
            tokens: vec![],
            state_fn: State::Fn(whitespace),
            start: 0,
            offset: 0,
            pos: 0,
            line: 1,
        }
    }

    fn ignore(&mut self) {
        self.pos += (self.offset - self.start) as u32;
        self.start = self.offset;
    }
    fn peek(&self) -> Option<char> {
        match self.code.get(self.offset) {
            Some(c) => Some(*c),
            None => None,
        }
    }
    fn next(&mut self) -> Option<char> {
        self.offset += 1;
        let c = self.peek();
        match c {
            Some('\n') => {
                self.pos = 0;
                self.line += 1;
                c
            }
            _ => c,
        }
    }
    fn new_token(&mut self, token_type: TkType, value: String) -> Token {
        Token((self.line, self.pos), token_type, value)
    }
    fn emit(&mut self, token_type: TkType) {
        let s: String = self.code[self.start..self.offset].into_iter().collect();
        let tok = match s.as_str() {
            "return" => self.new_token(TkType::Return, s),
            "type" => self.new_token(TkType::Type, s),
            "import" => self.new_token(TkType::Import, s),
            "contract" => self.new_token(TkType::Contract, s),
            "impl" => self.new_token(TkType::Impl, s),
            "for" => self.new_token(TkType::For, s),
            _ => self.new_token(token_type, s),
        };
        self.tokens.push(tok);
        self.ignore();
    }
}

fn whitespace(lexer: &mut Lexer) -> State {
    while let Some(c) = lexer.peek() {
        if c == ' ' || c == '\n' {
            lexer.next();
        } else {
            break;
        }
    }
    lexer.ignore();

    match lexer.peek() {
        Some(_c @ '0'..='9') => State::Fn(number),
        Some('=') => {
            lexer.next();
            lexer.emit(TkType::Assign);
            State::Fn(whitespace)
        }
        Some(',') => {
            lexer.next();
            lexer.emit(TkType::Comma);
            State::Fn(whitespace)
        }
        Some('+') => {
            lexer.next();
            lexer.emit(TkType::Plus);
            State::Fn(whitespace)
        }
        Some('*') => {
            lexer.next();
            lexer.emit(TkType::Star);
            State::Fn(whitespace)
        }
        Some('(') => {
            lexer.next();
            lexer.emit(TkType::LParen);
            State::Fn(whitespace)
        }
        Some(')') => {
            lexer.next();
            lexer.emit(TkType::RParen);
            State::Fn(whitespace)
        }
        Some('{') => {
            lexer.next();
            lexer.emit(TkType::LBrace);
            State::Fn(whitespace)
        }
        Some('}') => {
            lexer.next();
            lexer.emit(TkType::RBrace);
            State::Fn(whitespace)
        }
        Some(':') => {
            lexer.next();
            if lexer.peek() == Some(':') {
                lexer.next();
                lexer.emit(TkType::Accessor);
                State::Fn(whitespace)
            } else {
                lexer.emit(TkType::Colon);
                State::Fn(whitespace)
            }
        }
        Some(';') => {
            lexer.next();
            lexer.emit(TkType::Semicolon);
            State::Fn(whitespace)
        }
        Some('\'') => {
            lexer.next();
            lexer.emit(TkType::Prime);
            State::Fn(whitespace)
        }
        Some('|') => {
            lexer.next();
            lexer.emit(TkType::VerticalLine);
            State::Fn(whitespace)
        }
        Some('"') => State::Fn(string),
        Some(c) => {
            println!("char {:?}", c);
            if identifier_set(c) {
                State::Fn(ident)
            } else {
                panic!("Not implemented for {} yet", c);
            }
        }
        None => State::EOF,
    }
}

fn identifier_set(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn ident(lexer: &mut Lexer) -> State {
    while let Some(c) = lexer.next() {
        if !identifier_set(c) {
            break;
        }
    }
    lexer.emit(TkType::Ident);
    State::Fn(whitespace)
}

fn string(lexer: &mut Lexer) -> State {
    lexer.next();
    while let Some(c) = lexer.next() {
        if c == '"' {
            break;
        }
    }
    lexer.next();
    lexer.emit(TkType::String);
    State::Fn(whitespace)
}

fn number(lexer: &mut Lexer) -> State {
    while let Some(c) = lexer.next() {
        if !c.is_digit(10) {
            break;
        }
    }
    lexer.emit(TkType::Num);
    State::Fn(whitespace)
}

pub fn lex(source: String) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    while let State::Fn(f) = lexer.state_fn {
        lexer.state_fn = f(&mut lexer);
    }
    lexer.emit(TkType::EOF);
    lexer.tokens
}

#[cfg(test)]
mod tests {
    use self::TkType::*;
    use super::*;

    #[test]
    fn test_import() {
        assert_eq!(
            lex("import foo::bar".to_string()),
            vec![
                Token((1, 0), Import, "import".to_string()),
                Token((1, 7), Ident, "foo".to_string()),
                Token((1, 10), Accessor, "::".to_string()),
                Token((1, 12), Ident, "bar".to_string()),
                Token((1, 15), EOF, "".to_string()),
            ]
        )
    }

    #[test]
    fn test_unicode_identifier_a() {
        let code = "測試: int = 1";

        assert_eq!(
            lex(code.to_string()),
            vec![
                Token((1, 0), Ident, "測試".to_string()),
                Token((1, 2), Colon, ":".to_string()),
                Token((1, 4), Ident, "int".to_string()),
                Token((1, 8), Assign, "=".to_string()),
                Token((1, 10), Num, "1".to_string()),
                Token((1, 11), EOF, "".to_string()),
            ]
        );
    }

    #[test]
    fn get_number_tokens() {
        let ts = lex("10 30".to_string());
        assert_eq!(
            ts,
            vec![
                Token((1, 0), Num, "10".to_string()),
                Token((1, 3), Num, "30".to_string()),
                Token((1, 5), EOF, "".to_string()),
            ]
        );
    }

    #[test]
    fn get_ident_tokens() {
        let ts = lex(" abc6".to_string());
        assert_eq!(
            ts,
            vec![
                Token((1, 1), Ident, "abc6".to_string()),
                Token((1, 5), EOF, "".to_string()),
            ]
        )
    }
}
