const LEFT_PAR : u32 = 1;// (
const RIGHT_PAR : u32 = 2; // )
const a : u32 = 3; // a

//Non terminal tokens begin after terminal tokens
const A : u32 = 4;
const B : u32 = 5;
use llk::*;

//Tests
#[test]
fn test_make_table() {
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
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	assert!( parser.make_table() == Ok(()) );
	let table = parser.get_table();
	let table_to_cmp = vec![
				//  |  EOF    |LEFT_PAR | RIGHT_PAR | a
				vec![u32::MAX , 0       , u32::MAX  , 1],//A
				vec![u32::MAX ,u32::MAX , u32::MAX  , 2]//B
			];
	println!("GOT : {:?}", *table);
	println!("EXPECTED : {:?}", table_to_cmp);
	for ilig in 0..table_to_cmp.len() {
		for icol in 0..table_to_cmp[ilig].len() {
			assert!(table[ilig][icol] == table_to_cmp[ilig][icol]);
		}
	}
}
#[test]
fn test_make_table_error_lf() {
	let mut parser1 =
		LL1Parser::new(vec![
			Rule::new(vec![
				vec![LEFT_PAR, A, RIGHT_PAR],
				vec![LEFT_PAR]
			]),//A
			Rule::new(vec![
				vec![a]
			]),//B
		],
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	if let Err(err_msg) = parser1.make_table() {
		let expected_msg = format!("Left factorisation error for the rule {}", A);
		println!("GOT {}", err_msg);
		println!("EXPECTED {}", err_msg);
		assert!(err_msg == expected_msg);
	} else {
		panic!("Error expected");
	}
	let mut parser2 =
		LL1Parser::new(vec![
			Rule::new(vec![
				vec![LEFT_PAR, A, RIGHT_PAR],
				vec![B]
			]),//A
			Rule::new(vec![
				vec![LEFT_PAR]
			]),//B
		],
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	if let Err(err_msg) = parser2.make_table() {
		let expected_msg = format!("Left factorisation error for the rule {}", A);
		println!("GOT {}", err_msg);
		println!("EXPECTED {}", err_msg);
		assert!(err_msg == expected_msg);
	} else {
		panic!("Error expected");
	}
}
#[test]
fn test_make_table_error_lr() {
	let mut parser1 =
		LL1Parser::new(vec![
			Rule::new(vec![
				vec![A],
				vec![LEFT_PAR, A, RIGHT_PAR]
			]),//A
			Rule::new(vec![
				vec![a]
			]),//B
		],
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	if let Err(err_msg) = parser1.make_table() {
		let expected_msg = format!("Left recursion error for the rule {}\n\tNote : From rule {}", A, A);
		println!("GOT {}", err_msg);
		println!("EXPECTED {}", expected_msg);
		assert!(err_msg == expected_msg);
	} else {
		panic!("Error expected");
	}
	let mut parser2 =
		LL1Parser::new(vec![
			Rule::new(vec![
				vec![B],
				vec![LEFT_PAR, A, RIGHT_PAR],
			]),//A
			Rule::new(vec![
				vec![A]
			]),//B
		],
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	if let Err(err_msg) = parser2.make_table() {
		let expected_msg = format!("Left recursion error for the rule {}\n\tNote : From rule {}\n\tNote : From rule {}", A, B, A);
		println!("GOT {}", err_msg);
		println!("EXPECTED {}", expected_msg);
		assert!(err_msg == expected_msg);
	} else {
		panic!("Error expected");
	}
}
#[test]
fn test_analyse() {
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
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	assert!( parser.make_table() == Ok(()) );
	// ((((a))))
	let l_p = Token::Terminal{id : LEFT_PAR, value : String::from("("), pos : 0};
	let r_p = Token::Terminal{id : RIGHT_PAR, value : String::from(")"), pos : 0};
	let a_ = Token::Terminal{id : a, value : String::from("a"), pos : 0};
	let tokens = vec![&l_p, &l_p, &l_p, &l_p, &a_, &r_p, &r_p, &r_p, &r_p, &default_token::T_EOF];
	let mut ind = 0;
	match parser.analyse_tokens(
		|| {
			if ind < tokens.len() {
				ind += 1;
				return Some(tokens[ind - 1]);
			}
			return None;
		}
	) {
		Ok((_, warn)) => {
			println!("{}", warn);
			assert!(warn == String::from(""));
		}
		Err(e) => { panic!("Unexpected error : {:?}", e); }
	}
}
#[test]
fn test_analyse_warn() {
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
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	assert!( parser.make_table() == Ok(()) );
	// ((((a))))
	let l_p = Token::Terminal{id : LEFT_PAR, value : String::from("("), pos : 0};
	let r_p = Token::Terminal{id : RIGHT_PAR, value : String::from(")"), pos : 0};
	let a_ = Token::Terminal{id : a, value : String::from("a"), pos : 0};
	let tokens = vec![&l_p, &l_p, &l_p, &l_p, &a_, &r_p, &r_p, &r_p, &r_p, &r_p, &default_token::T_EOF];
	let mut ind = 0;
	match parser.analyse_tokens(
		|| {
			if ind < tokens.len() {
				ind += 1;
				return Some(tokens[ind - 1]);
			}
			return None;
		}
	) {
		Ok((_, warn)) => {
			println!("{}", warn);
			assert!(warn == String::from("Warning : Tokens remain unanalysed."));
		}
		Err(e) => { panic!("Unexpected error : {:?}", e); }
	}
}
#[test]
fn test_analyse_error_ueof() {
	let l_p = Token::Terminal{id : LEFT_PAR, value : String::from("("), pos : 0};
	let r_p = Token::Terminal{id : RIGHT_PAR, value : String::from(")"), pos : 0};
	let a_ = Token::Terminal{id : a, value : String::from("a"), pos : 0};
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
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	assert!( parser.make_table() == Ok(()) );
	// ((((a)))
	let expected_msg = String::from("Unexpected EOF");
	let tokens = vec![&l_p, &l_p, &l_p, &l_p, &a_, &r_p, &r_p, &r_p, &default_token::T_EOF];
	let mut ind = 0;
	if let Err(err_msg) = parser.analyse_tokens(
		|| {
			if ind < tokens.len() {
				ind += 1;
				return Some(tokens[ind - 1]);
			}
			return None;
		}
	) {
		println!("GOT {}", err_msg.additional_info);
		println!("EXPECTED {}", expected_msg);
		assert!(err_msg.additional_info == expected_msg);
	} else {
		panic!("Error expected");
	}
}
#[test]
fn test_analyse_error_1() {
	let l_p = Token::Terminal{id : LEFT_PAR, value : String::from("("), pos : 0};
	let r_p = Token::Terminal{id : RIGHT_PAR, value : String::from(")"), pos : 0};
	let a_ = Token::Terminal{id : a, value : String::from("a"), pos : 0};
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
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	assert!( parser.make_table() == Ok(()) );
	// )(((a))))
	let expected_msg = format!(
						"There is no rule which correspond to the derivation : Rule {} doesn't begin by token {}",
						A,
						RIGHT_PAR
						);
	let tokens = vec![&r_p, &l_p, &l_p, &l_p, &a_, &r_p, &r_p, &r_p, &r_p, &default_token::T_EOF];
	let mut ind = 0;
	if let Err(err_msg) = parser.analyse_tokens(
		|| {
			if ind < tokens.len() {
				ind += 1;
				return Some(tokens[ind - 1]);
			}
			return None;
		}
	) {
		println!("GOT {}", err_msg.additional_info);
		println!("EXPECTED {}", expected_msg);
		assert!(err_msg.additional_info == expected_msg);
	} else {
		panic!("Error expected");
	}
}
#[test]
fn test_analyse_error_2() {
	let l_p = Token::Terminal{id : LEFT_PAR, value : String::from("("), pos : 0};
	let r_p = Token::Terminal{id : RIGHT_PAR, value : String::from(")"), pos : 0};
	let a_ = Token::Terminal{id : a, value : String::from("a"), pos : 0};
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
		vec![&|_|{}, &|_|{}, &|_|{}],
		A,
		A
	);
	assert!( parser.make_table() == Ok(()) );
	// ((((aa)))
	let expected_msg = format!("Uncorresponding token : {} expected, found {}", RIGHT_PAR, a);
	let tokens = vec![&l_p, &l_p, &l_p, &l_p, &a_, &a_, &r_p, &r_p, &r_p, &default_token::T_EOF];
	let mut ind = 0;
	if let Err(err_msg) = parser.analyse_tokens(
		|| {
			if ind < tokens.len() {
				ind += 1;
				return Some(tokens[ind - 1]);
			}
			return None;
		}
	) {
		println!("GOT {}", err_msg.additional_info);
		println!("EXPECTED {}", expected_msg);
		assert!(err_msg.additional_info == expected_msg);
	} else {
		panic!("Error expected");
	}
}
