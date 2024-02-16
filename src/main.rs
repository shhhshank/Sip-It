use sip_it::{Error, Token};

mod sip_it;

fn main() {
    loop {
        let mut code = String::new();
        std::io::stdin().read_line(&mut code).unwrap();

        let (tokens, err): (Vec<Token>, Option<Error>) = sip_it::run("<stdin>", &*code);

        if err.is_none() {
            println!("{:?}", tokens)
        } else {
            println!("{}", err.unwrap())
        }
    }
}
