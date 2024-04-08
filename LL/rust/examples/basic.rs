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
	//To simplify the example, we use a lot of clone. It should not be necessary in real usage.
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
