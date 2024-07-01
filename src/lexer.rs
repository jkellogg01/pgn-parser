use std::fmt::Display;

pub enum Token {
    Illegal(String),
    EOF,

    Turn(String), // the literal will store ellipses vs period info for now
    TagPair(String, String),

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
    // having tokens for long & short castles requires like four characters of lookahead, which I
    // want to avoid so we'll make a token for O and a token for a dash and let the parser turn
    // that into a move
    LongCastle,
    ShortCastle,
    Star, // i think the star means "this is the end of the file but the game is not over"?
    Lcurly,
    Rcurly,
    Lparen,
    Rparen,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal(x) => write!(f, "Illegal: {x}"),
            Token::EOF => write!(f, "EOF"),
            Token::Turn(x) => write!(f, "Move {x}"),
            Token::TagPair(x, y) => write!(f, "Tag Pair: {x}-{y}"),
            Token::King => write!(f, "King"),
            Token::Queen => write!(f, "Queen"),
            Token::Rook => write!(f, "Rook"),
            Token::Bishop => write!(f, "Bishop"),
            Token::Knight => write!(f, "Knight"),
            Token::Rank(x) => write!(f, "Rank {x}"),
            Token::File(x) => write!(f, "File {x}"),
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
            Token::Lcurly => write!(f, "Left brace"),
            Token::Rcurly => write!(f, "Right brace"),
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
        let mut lex = Lexer {
            position: 0,
            read_position: 0,
            input: input.into_bytes(),
        };
        lex.advance();
        return lex;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = self.advance();
        match c {
            b'0'..=b'9' => self.process_number(c),
            b'[' => self.process_tag_pair(),
            0 => Token::EOF,
            _ => Token::Illegal(String::from("unrecognized character")),
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

    fn seek(&mut self, c: u8) -> Option<Token> {
        while self.peek() != c {
            if self.peek() == 0 {
                return Some(Token::Illegal(String::from(
                    "Reached end of file before seek termination",
                )));
            }
            self.advance();
        }
        None
    }

    fn consume(&mut self, s: &str, msg: &str, t: Token) -> Token {
        if s.bytes().fold(false, |a, v| a || self.advance() != v) {
            Token::Illegal(String::from(msg))
        } else {
            t
        }
    }

    fn process_number(&mut self, c: u8) -> Token {
        match (c, self.advance()) {
            (b'1', b'/') => self.consume("2-1/2", "malformed draw", Token::Draw),
            (b'1', b'-') => self.consume("0", "malformed white victory", Token::WhiteWin),
            (b'0', b'-') => self.consume("1", "malformed black victory", Token::BlackWin),
            (x, b'.') => {
                let mut literal = vec![x, b'.'];
                while self.peek() == b'.' {
                    literal.push(self.advance());
                }
                Token::Turn(String::from_utf8_lossy(literal.as_slice()).to_string())
            }
            (x, y) if y.is_ascii_digit() => {
                let mut literal = vec![x, y];
                if let None = self.seek(b'.') {
                    while self.peek() == b'.' {
                        literal.push(self.advance());
                    }
                    Token::Turn(String::from_utf8_lossy(literal.as_slice()).to_string())
                } else {
                    Token::Illegal(String::from(
                        "multi-digit formation did not terminate before eof",
                    ))
                }
            }
            (x, _) => Token::Rank(x),
        }
    }

    fn process_tag_pair(&mut self) -> Token {
        let mut key_literal: Vec<u8> = vec![];
        loop {
            let c = self.advance();
            if c == b']' {
                return Token::Illegal(String::from("malformed tag pair"));
            }
            if c.is_ascii_whitespace() {
                break;
            }
            key_literal.push(c);
        }
        if self.advance() != b'"' {
            return Token::Illegal(String::from("malformed tag pair"));
        }
        let mut val_literal: Vec<u8> = vec![];
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
            val_literal.push(c);
        }
        Token::TagPair(
            String::from_utf8_lossy(key_literal.as_slice()).to_string(),
            String::from_utf8_lossy(val_literal.as_slice()).to_string(),
        )
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }
}
