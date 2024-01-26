/// All types of token splax can handle.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // single-character tokens.
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Slash,      // /
    Star,       // *
    Mod,        // %

    // one or two character tokens.
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // literals.
    Identifier, // variables, function names, class names.
    String,     // Strings.
    Number,     // numbers : integers, floats.

    // keywords
    True,   // true
    False,  // false
    Null,   // null
    And,    // and
    Or,     // or
    Let,    // let
    Class,  // class
    Fn,     // fn
    Return, // return
    Super,  // super
    This,   // this
    If,     // if
    Else,   // else
    While,  // while
    For,    // for
    Print,  // print

    Eof, // end of file.
}

// using debug macro to generate display fmt and then use that as its actual implementation.
impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
