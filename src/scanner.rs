/// The scanner portion of CalcLang compiler.
/// Detects the following tokens:
///
/// Integer constant: one or more decimal digits
/// Arithmetic operators: +, -, *, /, %, ^
/// Assignment operator: =
/// Semicolon: ;
/// Variable name: a single letter (ignore case)
/// The word "quit" (ignore case)


pub struct Scanner {
    input: String,
    output: Vec<Token>,
    state: ScannerState,
}

#[derive(Debug, Eq, PartialEq)]
enum ScannerState {
    Idle,
    CharMode,
    IntMode,
    QuitMode,
    Done,
}

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

impl Scanner {
    pub fn new(input: &str) -> Scanner {
        Scanner{
            input: input.to_string(),
            output: Vec::new(),
            state: ScannerState::Idle,
        }
    }

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
                ' ' => None,
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
                'a'...'z' => Some(Token::Variable(c)),
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
            if tok.is_some() {
                self.output.push(tok.unwrap());
            }
        }
    }

    pub fn output(&self) -> &Vec<Token> {
        &self.output
    }
}
