// Copyright Damien Lejosne 2021. See LICENSE for more informations
//TODO File size > 8 MB not handled correctly (too much memory allocated when not using default_mod, if we're using it,
// maximum file size is reported to 100MB)
//! This binary is a very simple compiler compiler : it allows you to create programming languages in a language
//! with a syntax extremly close to the rust one (indeed, it's a superset of the rust syntax).\
//! I use it for my personal projects because I like to use what I make myself, but some compiler compilers are far
//! more complete than this one, I think for example at the [YACC](https://en.wikipedia.org/wiki/Yacc) project (which is using a
//! LALR(1) parser instead of the LL(1) I'm using here), or at the [ANTLR](https://www.antlr.org/) project.
use llk::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

const LEFT_PAR : u32 = 2;// (
const RIGHT_PAR : u32 = 3; // )
const RULE_NAME : u32 = 4; // A word
const ASSIGN : u32 = 5; // =
const END : u32 = 6; // ;
const SPECIAL : u32 = 7; // other

//Non terminal tokens begin after terminal tokens
const CODE : u32 = 8;
const WORD : u32 = 9;
const LIST_WORDS : u32 = 10;
const RULE : u32 = 11;
const RULE_DEF : u32 = 12;
const MAIN : u32 = 13;
const LIST_RULE_DEF : u32 = 14;
const LIST_NAME : u32 = 15;

fn id_to_word(id:u32) -> String {
	match id {
		LEFT_PAR => {return String::from("left par");}
		RIGHT_PAR => {return String::from("right par");}
		RULE_NAME => {return String::from("rule name");}
		ASSIGN => {return String::from("assign");}
		END => {return String::from("end");}
		SPECIAL => {return String::from("special");}

		CODE => {return String::from("code");}
		WORD => {return String::from("word");}
		LIST_WORDS => {return String::from("words list");}
		RULE => {return String::from("rule");}
		RULE_DEF => {return String::from("rule definition");}
		MAIN => {return String::from("main");}
		LIST_RULE_DEF => {return String::from("rules definition's list");}
		LIST_NAME => {return String::from("list name");}

		_ => {return String::from("Unknown");}
	}
}

/*fn default_mod(st : &mut STree) {
	if st.children.len() > 0 {
		*st = st.children[0].clone();
	}
}*/

fn parse_parser(contents : &mut String) {
	let mut ind : usize = 0;
	//let mut token_act = Token::Terminal{id : 0, value : String::from(""), pos : 0};
	let tokenize = || {
		fn is_special_char(letter_act : u8) -> bool {
			return letter_act < 33 || letter_act == 127;
		}
		fn get_char_type(letter_act : char) -> u32 {
			if 0 as char <= letter_act && letter_act < 33 as char || letter_act == 127 as char {
				return SPECIAL;
			} else if letter_act == '=' {
				return ASSIGN;
			} else if letter_act == ';' {
				return END;
			} else if letter_act == '(' {
				return LEFT_PAR;
			} else if letter_act == ')' {
				return RIGHT_PAR;
			} else {
				return RULE_NAME;
			}
		}
		let mut token = Terminal{id : RULE_NAME, value : String::new(), pos : ind as u32};
		if ind == contents.len() {
			ind += 1;
			return Some(default_token::T_EOF.clone());
		} else if ind > contents.len() {
			return None;
		}
		//Suppr white spaces
		let mut letter_act = contents.as_bytes()[ind];
		while is_special_char(letter_act) {
			ind += 1;
			if ind == contents.len() {
				ind += 1;
				return Some(default_token::T_EOF.clone());
			}
			letter_act = contents.as_bytes()[ind];
		}
		//Get a rule name
		let mut type_act = get_char_type(letter_act as char);
		while type_act == RULE_NAME {
			type_act = get_char_type(letter_act as char);
			token.value.push(letter_act as char);
			ind += 1;
			if ind == contents.len() {
				break;
			}
			letter_act = contents.as_bytes()[ind];
		}
		//If it's not a rule name
		if token.value.len() == 0 {
			token.value.push(letter_act as char);
			token.id = type_act;
			ind += 1;
		} else {
			token.value.pop();
			ind -= 1;
		}
		let ret = Token::new_terminal(token.id, token.value, token.pos);
		return Some(ret);
	};
	let mut parser =
		LL1Parser::new(vec![
			Rule::new(vec![
				vec![LEFT_PAR, LIST_WORDS, RIGHT_PAR],//0
			]),//CODE
			Rule::new(vec![
				vec![RULE_NAME],//1
				vec![CODE],//2
				vec![ASSIGN],//3
				vec![SPECIAL],//4
				vec![END],//5
			]),//WORD
			Rule::new(vec![
				vec![WORD, LIST_WORDS],//6
				vec![default_id::NONE]//7
			]),//LIST_WORDS
			Rule::new(vec![
				vec![RULE_NAME, ASSIGN, LIST_RULE_DEF, END]//8
			]),//RULE
			Rule::new(vec![
				vec![LIST_NAME, CODE],//9
			]),//RULE_DEF
			Rule::new(vec![
				vec![RULE, MAIN],//12
				vec![CODE]//13
			]),//MAIN
			Rule::new(vec![
				vec![RULE_DEF, LIST_RULE_DEF],//10
				vec![default_id::NONE]//11
			]),//LIST_RULE_DEF
			Rule::new(vec![
				vec![RULE_NAME, LIST_NAME],//12
				vec![default_id::NONE]//13
			]),//LIST_NAME
		],
		vec![&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{},
			&|_|{}],
		MAIN,
		CODE
	);
	match parser.make_table() {
		Err(msg) => {
			println!("The grammar is buggy : {msg}"); //If so, report this bug to me ;-)
			return;
		}
		Ok(()) => {}
	}
	match parser.analyse_tokens(tokenize) {
		Err(msg) => {
			println!("Syntax error : error \"{}\" with code {} detected.", msg.additional_info, msg.code);
			if let Some(token) = msg.token_concerned {
				match token {
					Token::Terminal{pos, value, id} => {
						println!("\tNote : detected here : [{}:{}] (\'{}\' which is a {})",
							pos,
							pos as usize + value.len(),
							value,
							id_to_word(id)
						);
					}
					Token::NTerminal{id} => {
						println!("\tNote : detected in rule {})", id_to_word(id));
					}
				}
			}
			return;
		}
		Ok((_, warnings)) => {
			//println!("{}", stree);
			println!("{}", warnings);
		}
	}
	println!("Parsing succeeded !");
}

fn main() -> std::io::Result<()> {
	let args: Vec<String> = env::args().collect();
	if args.len() != 2 {
		eprintln!("Error, wrong number of arguments. Got : {}, 1 expected (file name).", args.len() - 1);
		return Ok(());
	}
	let file = File::open(args[1].to_string())?;
	let mut contents = String::new();
	let mut buffer = BufReader::new(file);
	buffer.read_to_string(&mut contents)?;
	drop(buffer);//No more needs of it
	parse_parser(&mut contents);
	Ok(())
}
