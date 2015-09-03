mod scanner;
use scanner::Scanner;


fn main() {
    let program = "+ - 34 ; quit a 3 3 - - 1 * ^ 7";
    let mut s = Scanner::new(program);
    s.scan();
    for line in s.output() {
        println!("{}",line);
    }
}
