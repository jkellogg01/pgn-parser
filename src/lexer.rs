use std::fmt::Display;

pub enum Token {
    Illegal(String),
    EOF,

    Turn(String), // the literal will store ellipses vs period info for now
    TagPair(String, String),
    Comment(String),

    // Pieces
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    // pawns do not have a corresponding token in pgn

    // Location Data
    Rank(u8),
    File(u8),

    // Game results
    WhiteWin,
    BlackWin,
    Draw,

    // Symbols
    Takes,
    Check,
    Mate,
    Promote,
    LongCastle,
    ShortCastle,
    Star, // i think the star means "this is the end of the file but the game is not over"?
    Lparen,
    Rparen,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal(x) => write!(f, "Illegal: {x}"),
            Token::EOF => write!(f, "EOF"),
            Token::Turn(x) => write!(f, "Move: {x}"),
            Token::TagPair(x, y) => write!(f, "Tag Pair: [{x} \"{y}\"]"),
            Token::Comment(x) => write!(f, "Comment: {x}"),
            Token::King => write!(f, "King"),
            Token::Queen => write!(f, "Queen"),
            Token::Rook => write!(f, "Rook"),
            Token::Bishop => write!(f, "Bishop"),
            Token::Knight => write!(f, "Knight"),
            Token::Rank(x) => write!(f, "Rank: {}", *x as char),
            Token::File(x) => write!(f, "File: {}", *x as char),
            Token::WhiteWin => write!(f, "White Wins"),
            Token::BlackWin => write!(f, "Black Wins"),
            Token::Draw => write!(f, "Draw"),
            Token::Takes => write!(f, "Takes"),
            Token::Check => write!(f, "Check"),
            Token::Mate => write!(f, "Checkmate"),
            Token::Promote => write!(f, "Promote"),
            Token::LongCastle => write!(f, "Castle Queenside"),
            Token::ShortCastle => write!(f, "Castle Kingside"),
            Token::Star => write!(f, "Asterisk"),
            Token::Lparen => write!(f, "Left paren"),
            Token::Rparen => write!(f, "Right paren"),
        }
    }
}

pub struct Lexer {
    position: usize,
    read_position: usize,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let lex = Lexer {
            position: 0,
            read_position: 0,
            input: input.into_bytes(),
        };
        return lex;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = self.advance();
        match c {
            b'K' => Token::King,
            b'Q' => Token::Queen,
            b'R' => Token::Rook,
            b'B' => Token::Bishop,
            b'N' => Token::Knight,
            b'a'..=b'h' => Token::File(c),
            b'x' => Token::Takes,
            b'+' => Token::Check,
            b'#' => Token::Mate,
            b'=' => Token::Promote,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'*' => Token::Star,
            b'0'..=b'9' => self.process_number(c),
            b'[' => self.process_tag_pair(),
            b'{' => self.process_comment(),
            b'O' => self.process_castle(),
            0 => Token::EOF,
            x => Token::Illegal(format!("unrecognized character {}", x as char)),
        }
    }

    fn advance(&mut self) -> u8 {
        let result = self.peek();
        self.position = self.read_position;
        self.read_position += 1;
        return result;
    }

    fn peek(&self) -> u8 {
        match self.read_position.cmp(&self.input.len()) {
            std::cmp::Ordering::Less => self.input[self.read_position],
            _ => 0,
        }
    }

    fn consume(&mut self, s: &str, msg: &str, t: Token) -> Token {
        for c in s.bytes() {
            if self.peek() != c {
                return Token::Illegal(String::from(msg));
            }
            self.advance();
        }
        t
    }

    fn process_number(&mut self, c: u8) -> Token {
        match (c, self.advance()) {
            (b'1', b'/') => self.consume("2-1/2", "malformed draw", Token::Draw),
            (b'1', b'-') => self.consume("0", "malformed white victory", Token::WhiteWin),
            (b'0', b'-') => self.consume("1", "malformed black victory", Token::BlackWin),
            (x, b'.') => {
                let mut literal = String::new();
                literal.push(x as char);
                literal.push('.');
                while self.peek() == b'.' {
                    literal.push(self.advance() as char);
                }
                Token::Turn(literal)
            }
            (x, y) if y.is_ascii_digit() => {
                let mut literal = String::new();
                literal.push(x as char);
                literal.push(y as char);
                loop {
                    let c = self.advance();
                    if c == 0 {
                        return Token::Illegal(String::from(
                            "multi-digit formation did not terminate before eof",
                        ));
                    }
                    literal.push(c as char);
                    if c == b'.' {
                        break;
                    }
                }
                while self.peek() == b'.' {
                    literal.push(self.advance() as char)
                }
                Token::Turn(literal)
            }
            (x, _) => Token::Rank(x),
        }
    }

    fn process_tag_pair(&mut self) -> Token {
        let mut key_literal = String::new();
        loop {
            let c = self.advance();
            if c == b']' {
                return Token::Illegal(String::from("malformed tag pair"));
            }
            if c.is_ascii_whitespace() {
                break;
            }
            key_literal.push(c as char);
        }
        if self.advance() != b'"' {
            return Token::Illegal(String::from("malformed tag pair"));
        }
        let mut val_literal = String::new();
        loop {
            let c = self.advance();
            if c == 0 {
                return Token::Illegal(String::from(
                    "tag pair contains unterminated string literal",
                ));
            }
            if c == b'"' {
                break;
            }
            val_literal.push(c as char);
        }
        if self.advance() != b']' {
            return Token::Illegal(String::from("malformed tag pair"));
        }
        Token::TagPair(key_literal, val_literal)
    }

    fn process_comment(&mut self) -> Token {
        let mut literal = String::new();
        loop {
            match self.advance() {
                0 => return Token::Illegal(String::from("unterminated comment")),
                b'}' => break,
                x => literal.push(x as char),
            }
        }
        Token::Comment(literal)
    }

    fn process_castle(&mut self) -> Token {
        if let Token::ShortCastle = self.consume("-O", "malformed short castle", Token::ShortCastle)
        {
            if let Token::Illegal(_) =
                self.consume("-O", "malformed long castle", Token::LongCastle)
            {
                Token::ShortCastle
            } else {
                Token::LongCastle
            }
        } else {
            Token::Illegal(String::from("malformed castle"))
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }
}
