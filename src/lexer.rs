use std::cmp::Ordering;

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
