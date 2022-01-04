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
