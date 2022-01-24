# LL parser
Here, you will find a "LL" parser.

Currently, I wrote the code in rust. C and python are coming soon.

- FEATURES :
	- A quite fast, efficient both in memory and cpu ressources library
	- A simple and short code (but I think its clarity can still be improved : feel free to contribute if you see some ways to achieve it !)
	- Syntax tree (maybe its memory consumption may be improve : It's the main memory consumer of the code)
	-  Actions on rules
- TODO :
	- Add compiler compiler usage and a more detailed documentation about the API
	- Remove bug which makes execution fails with large files (stack exhausted panic in rust)
	- Add C and python code
	- Finish the compiler compiler
	- Add LL(k) and LL(*) parsers

## Quick start

# Usage (rust)
### Quick start

To install, see installation instructions.\
Then, try this code :
```rust
use llk::*;

//We begin at 2 because 0 and 1 are reserved
const LEFT_PAR : u32 = 2;// (
const RIGHT_PAR : u32 = 3; // )
const a : u32 = 4; // a

//Non terminal tokens begin after terminal tokens
const A : u32 = 5;
const B : u32 = 6;

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
	//Here, we are using a lot of clones because it's simpler, 
	//but keep in mind that that's not a good practice
	let tokens = vec![l_p.clone(),
					  l_p.clone(),
					  l_p.clone(),
					  l_p.clone(),
					  a_.clone(),
					  r_p.clone(),
					  r_p.clone(),
					  r_p.clone(),
					  r_p.clone(),
					  default_token::T_EOF.clone()
					];
	match parser.analyse_tokens(
		|| {
			if ind < tokens.len() {
				ind += 1;
				return Some(tokens[ind - 1].clone());
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
llk = "0.1.1"
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
