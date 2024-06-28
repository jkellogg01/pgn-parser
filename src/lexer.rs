use std::{cmp::Ordering, fmt::Display};

pub enum Token {
    MoveNumber(String), // the literal will store ellipses vs period info for now
    Ident(String),      // this is for the first half of a tag
    Literal(String),    // this is for the second half, which has double quotes

    // Pieces
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    // pawns do not have a corresponding token in pgn

    // Location Data
    Rank(char),
    File(char),

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
    Castle,
    Dash,
    Star, // i think the star means "this is the end of the file but the game is not over"?
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::MoveNumber(x) => write!(f, "Move {x}"),
            Token::Ident(x) => write!(f, "Ident {x}"),
            Token::Literal(x) => write!(f, "Literal {x}"),
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
            Token::Castle => write!(f, "Castle"),
            Token::Dash => write!(f, "Dash"),
            Token::Star => write!(f, "Asterisk"),
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

    fn advance(&mut self) -> u8 {
        let result = self.peek();
        self.position = self.read_position;
        self.read_position += 1;
        return result;
    }

    fn peek(&self) -> u8 {
        match self.read_position.cmp(&self.input.len()) {
            Ordering::Less => self.input[self.read_position],
            _ => 0,
        }
    }
}
