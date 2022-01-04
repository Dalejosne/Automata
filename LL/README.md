# LL parser
Here, you will find a "LL" parser.

Currently, I wrote the code in rust, C and python will come soon

## Quickstart

# Usage (rust)
### Quickstart

To install, see installation instructions.\
Then, try this code :
```rust
use llk::*;

const LEFT_PAR : u32 = 0;// (
const RIGHT_PAR : u32 = 1; // )
const a : u32 = 2; // a

//Non terminal tokens begin after terminal tokens
const A : u32 = 3;
const B : u32 = 4;

fn main() {
	let mut parser =
		LL1Parser::new(vec![
			Rule::new(vec![
				vec![LEFT_PAR, A, RIGHT_PAR],
				vec![B]
			]),//A
			Rule::new(vec![
				vec![a]
			]),//B
		],
		vec![&|stree|{println!("{}", *stree)}, &|_|{}, &|_|{}],
		A,
		A
	);
	match parser.make_table() {
		Err(msg) => {
			println!("The grammar is buggy : {}", msg); //If so, report this bug to me ;-)
			return;
		}
		Ok(()) => {}
	}
	// ((((a))))
	let l_p = Token::Terminal{id : LEFT_PAR, value : String::from("("), pos : 0};
	let r_p = Token::Terminal{id : RIGHT_PAR, value : String::from(")"), pos : 0};
	let a_ = Token::Terminal{id : a, value : String::from("a"), pos : 0};
	let mut ind = 0;
	let tokens = vec![&l_p, &l_p, &l_p, &l_p, &a_, &r_p, &r_p, &r_p, &r_p, &default_token::T_EOF];
	match parser.analyse_tokens(
		|| {
			if ind < tokens.len() {
				ind += 1;
				return Some(tokens[ind - 1]);
			}
			return None;
		}
	){
		Err(msg) => {
			println!("Syntax error : {:?}", msg); //If so, report this bug to me ;-)
			return;
		}
		Ok((stree, warnings)) => {
			println!("{}", stree);
			println!("{}", warnings);
		}
	}
	println!("Parsing succeeded !");
}
```

### Install
To install it as a dependency, just add this line to your Cargo.toml :
```toml
[dependencies]
llk = "0.1.0"
```
If you want to install it locally, do :
```sh
git clone https://github.com/Dalejosne/Automata
```
And every files you need will be on the subdirectory Automata/game_of_life.

### Doc
After having installed it, do :
```
cargo doc
```
And then open the file index.html on the directory target/doc/game_of_life

### Examples

See the "examples" repository. To run them, do :
```sh
cargo run --example basic
```
