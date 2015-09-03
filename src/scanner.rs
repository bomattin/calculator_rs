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
    output: Vec<String>,
    state: ScannerState,
}

enum ScannerState {
    Idle,
    CharMode,
    IntMode,
    QuitMode,
    Done,
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
        for character in self.input.chars() {
            match character {
                i @ '0'...'9'   => self.output.push(format!("Integer digit: {}", i)),
                '+'             => self.output.push("Addition operator: +".to_string()),
                '-'             => self.output.push("Subtraction operator: -".to_string()),
                '*'             => self.output.push("Multiplication operator: *".to_string()),
                '/'             => self.output.push("Division operator: /".to_string()),
                '%'             => self.output.push("Modulus operator: %".to_string()),
                '^'             => self.output.push("Exponent operator: ^".to_string()),
                '='             => self.output.push("Assignment operator: =".to_string()),
                ';'             => self.output.push("Statement terminator: ;".to_string()),
                c @ 'a'...'z'| c @ 'A'...'Z'
                                => self.output.push(format!("Variable name: {}", c)),
                ' '             => self.output.push("Space, ignoring.".to_string()),
                z @ _           => self.output.push(format!("Unrecognized token: {}", z))
            }
        }
    }

    pub fn output(&self) -> &Vec<String> {
        &self.output
    }
}
