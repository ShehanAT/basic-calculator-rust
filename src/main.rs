


fn main() {

    loop {
        println!("Enter input: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        let tokens = tokenize(input);
        let stack = shunt(tokens);
        let res = calculate(stack);
        println!("{}", res);
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Token {
    Number (i64),
    Plus,
    Sub,
    Mul,
    Div,
    LeftParen,
    RightParen,
}

// Tokenizes the input string into a Vec to Tokens
fn tokenize(mut input: String) -> Vec<Token> {
    lazy_static! {
        static ref NUMBER_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
    }
    let mut res = vec![];
    while !(input.trim_left().is_empty()){
        input = input.trim_left().to_string();
        input = if let Some((_, end)) = NUMBER_RE.find(&input) {
            let (num, rest) = input.split_at_mut(end);
            res.push(Token::Number(num.parse::<i64>().unwrap()));
            rest.to_string()
        } else {
            res.push(match input.chars().nth(0) {
                Some('+') => Token::Plus,
                Some('-') => Token::Sub,
                Some('*') => Token::Mul,
                Some('/') => Token::Div,
                Some('(') => Token::LeftParen,
                Some(')') => Token::RightParen,
                _ => panic!("Unknown character!")
            });
            input.trim_left_matches(|c| c == '+' ||
                                        c == '-' ||
                                        c == '*' ||
                                        c == '/' ||
                                        c == '(' ||
                                        c == ')').to_string() 
        }
    }
}



