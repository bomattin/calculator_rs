/// The scanner portion of CalcLang compiler.
/// Detects the following tokens:
///
/// Integer constant: one or more decimal digits
/// Arithmetic operators: +, -, *, /, %, ^
/// Assignment operator: =
/// Semicolon: ;
/// Variable name: a single letter (ignore case)
/// The word "quit" (ignore case)

// Rust doesn't have classes, but it does have structs
// Vec is known as a 'vector', Rust's growable array type.
// ScannerState is an enum type we'll define below.
pub struct Scanner {
    input: String,
    output: Vec<Token>,
    state: ScannerState,
}

/// Rust has an extremely powerful trait system that acts like a combination of Java's
/// interfaces and generics, and then some.  However, implementing every trait by hand can be tedious.
/// Luckily, we have an Attribute (like a compiler directive) that tells the compiler to implement
/// these trivial traits for us. In this case we implement:
/// * Eq + PartialEq : For use of the == operator
/// * Debug : For printing in formatted strings
#[derive(Debug, Eq, PartialEq)]
enum ScannerState {
    Idle,
    CharMode,
    IntMode,
    QuitMode,
    Done,
}

/// Enums in Rust are...well, they're pretty freakin' awesome. As you can see here, enum variants
/// can have optional data associated with them!
#[derive(Debug, Eq, PartialEq)]
enum Token {
    Integer(i64),
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    Exponent,
    Assignment,
    Terminator,
    Quit,
    Variable(char),
    Unknown(char),
}

/// `impl` is short for implement, and is how we define both methods and associated functions (or
/// static methods) in Rust. In this case we provide 2:
/// * An associated function `new()` which creates a new `Scanner` with a given input string. This
/// acts like a constructor.
/// * A method `scan()`, which iterates over the input and builds its token vector.
impl Scanner {
    pub fn new(input: &str) -> Scanner {
        Scanner{
            input: input.to_string(),
            output: Vec::new(),
            state: ScannerState::Idle,
        }
    }
/// Rust has "implicit returns", meaning the last statement of a block is automatically returned.
/// Therefore even code blocks in Rust have types. Notice how we have no return statement, even
/// though the function is defined as returning a `Scanner` (by the `->` token).

/// Here's where the fun begins.
/// Rust includes pattern matching as an integral part of the language. Pattern matching acts like
/// a switch statement with a little (or a lot) more power. In this case, we get an iterator over
/// the characters in the input stream, and for each character, we match it against certain patterns.
/// `let` is Rust's way of defining a variable binding
/// `let mut` is Rust's way of defining a _mutable_ variable binding. Bindings in Rust are
/// immutable by default.
/// But why `Some()`? `tok` is defined as an `Option<Token>`, which is exactly what is sounds like:
/// the contents may be a Token (expressed by `Some(Token)`) or they may be empty (None). We use
/// this for ignoring whitespace while still being able to catch unknown tokens.
    pub fn scan(&mut self) {
        self.state = ScannerState::CharMode;
        let mut chars = self.input.chars().peekable();
        while let Some(c) = chars.next() {
            let tok: Option<Token> = match c {
                '+' => Some(Token::Addition),
                '-' => Some(Token::Subtraction),
                '*' => Some(Token::Multiplication),
                '/' => Some(Token::Division),
                '%' => Some(Token::Modulus),
                '^' => Some(Token::Exponent),
                '=' => Some(Token::Assignment),
                ';' => Some(Token::Terminator),
                ' '|'\r'|'\n'|'\t'
                    => None,

                /// The case for q gets a little insane, because it peeks ahead up to 3 characters
                /// to see if it matches the word 'quit.' This isn't a fault of Rust, this is a
                /// fault of me being a dummy and not matching on whole words until it was too late.
                'q' => {
                    let mut proceed = match chars.peek() {
                        Some(&next) if next == 'u' => true,
                        _   => false
                    };
                    if !proceed {
                        Some(Token::Variable(c))
                    } else {
                        chars.next();
                        proceed = match chars.peek() {
                            Some(&next) if next == 'i' => true,
                            _   => false
                        };
                        if !proceed {
                            Some(Token::Variable(c))
                        } else {
                            chars.next();
                            proceed = match chars.peek() {
                                Some(&next) if next == 't' => true,
                                _   => false
                            };
                            if !proceed {Some(Token::Variable(c))}
                            else {
                                chars.next();
                                Some(Token::Quit)
                            }
                        }
                    }
                },
                /// Patterns can be ranges.
                'a'...'z' => Some(Token::Variable(c)),
                /// The number case does some fun math trickery and peeking ahead.
                '0'...'9' => {
                    let mut number = 0;
                    let mut current = c;
                    loop {
                        number *= 10;
                        number += ((current as u8) - ('0' as u8)) as i64;
                        match chars.peek() {
                            Some(&next) if '0' <= next && next <= '9' => {
                                current = next;
                                chars.next();
                            }
                            _ => break,
                        };
                    }
                    Some(Token::Integer(number))
                },
                _ => Some(Token::Unknown(c))
            };

            // Finally, after all this, if there is a token, push it to the output vector.
            // `unwrap()` means "Assume there's a result, give me the value, otherwise panic."
            // It's safe to do so since we just checked it the line before.
            if tok.is_some() {
                self.output.push(tok.unwrap());
            }
        }
    }

    /// A simple getter for the output `Vec` that returns a read-only reference (in Rust, an
    /// immutable borrow) to the output `Vec`
    pub fn output(&self) -> &Vec<Token> {
        &self.output
    }
}
