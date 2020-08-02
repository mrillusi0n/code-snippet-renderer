use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("(int|float|char)")]
    Keyword,

    #[regex("(:|;|,|\\[|\\]|\\(|\\))")]
    Punctuation,

    #[regex("[0-9]+.?[0-9]*")]
    Number,

    #[regex("[ \t\n]+")]
    Whitespace,

    #[regex("[_a-zA-Z][a-zA-Z_0-9]*")]
    Identifier,

    #[error]
    Error,
}

fn main() {
    let source = "int main(int argc, char *argv[])";
    let mut lexer = Token::lexer(source);

    while let Some(t) = lexer.next() {
        println!("{:?}", t);
    }
}
