//TODO NONE explicit actions description and handling
// Copyright Damien Lejosne 2021. See LICENSE for more informations
//! This crate allow you to create LL(1) (for now, LL(k) are coming soon) parsers.\
//! You should have a look to the followings links to learn more about these parsers :\
//! [LL parsers : Wikipédia](https://en.wikipedia.org/wiki/LL_parser) and [LL parsers : INSA](http://coursenligne.insa-rouen.fr/UNIT-CoursDeCompilation/Theme02/Support_Papier_Theme02.pdf)

///Error codes
pub mod err_code {
	///Unexpected EOF
	pub const UEOF : u32 = 0;
	///Unexpected teminal
	pub const U_TERM : u32 = 1;
	///Unknown token
	pub const U_TOKEN : u32 = 2;
	///Uncorresponding token
	pub const M_TOKEN : u32 = 3;
}


use err_code::*;
#[derive(Debug)]
///Structure which allow you to create an understand error messages
pub struct SyntaxError {
	///The code of the error
	pub code : u32,
	///A copy of the concerned token (it can be a non terminal)
	pub token_concerned : Option<Token>,
	//A string which you can diplay to add some info
	pub additional_info : String
}
impl SyntaxError {
	///Create a new syntax error
	fn new(code : u32, t : Option<Token>, info : String) -> SyntaxError {
		SyntaxError{code : code, token_concerned : t, additional_info : info}
	}
}
#[derive(Debug, Clone)]
pub struct STree
{
	pub value : Token,
	pub children : Vec<STree>
}
impl std::fmt::Display for STree {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		fn aux(this : &STree, lvl : u32) -> String {
			let mut ident = String::new();
			for _ in 0..lvl {
				ident = format!("{ident}\t");
			}
			let mut ret = format!("{ident}STree : value = {}\n{ident}\tchildrens :\n", this.value);
			for child in &this.children {
				ret = format!("{ret}{}\n", aux(child, lvl + 1));
			}
			ret
		}
		write!(f, "{}", aux(self, 0))
	}
}
#[derive(Debug, Clone)]
///Enum which give you the ability to
pub enum Token {
	///A terminal
	Terminal {
		///Its id
		id : u32,
		//Its pos in the file
		pos : u32,
		///Its value e.g. the string got by the lexer
		value : String
	},
	//A not terminal token
	NTerminal {
		///Its id
		id : u32
	}
}
impl Token {
	pub fn new_terminal(id : u32, value : String, pos : u32) -> Token {
		return Token::Terminal{id : id, value : value, pos : pos};
	}
	pub fn new_nb(id : u32) -> Token {
		return Token::NTerminal{id : id};
	}
}
impl std::fmt::Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Terminal{id, pos, value} => {
				return write!(f, "T : {value} is {}, begin at {} and finish at {}", *id, *pos, *pos as usize + value.len());
			}
			Token::NTerminal{id} => {
				return write!(f, "NT : {}", *id);
			}
		}
	}
}
//Internal, just here to simplified treatments
#[derive(Debug)]
pub struct Terminal {
	pub id : u32,
	pub pos : u32,
	pub value : String
}

///Ids of tokens reserved by the library.
pub mod default_id {
	///EOF Id
	pub const EOF : u32 = 0;
	///Default token : Equivalent to the empty token
	pub const NONE : u32 = 1;
}
///Tokens reserved by the library.
pub mod default_token {
	use super::*;
	///EOF token. Should always be present at the end of list of tokens.
	pub static T_EOF : Token = Token::Terminal{id : default_id::EOF, pos : 0, value : String::new()};
}

///## Class to create a syntaxic rule.
///Each terminal and non terminal tokens are represented by u32 index.\
///### Example :
///'a' => 0 (terminal)\
///'b' => 1 (terminal)\
///A => 2 (non terminal)\
///B => 3 (non terminal)\
///NB : Each terminal should follow the next one, and non terminal should follow terminal (as in the example above).\
///Then, you can create the rule :\
///A ::= 'a' B 'b'\
///With :
///```
///llk::Rule::new(vec![vec![0, 3, 1]]);
///```
pub struct Rule {
	derivations : Vec<Vec<u32>>
}

impl Rule {
	pub fn new(d : Vec<Vec<u32>>) -> Rule {
		Rule{derivations : d}
	}
}

///Class to create a LL(1) parser.
pub struct LL1Parser<'a>
{
	rules : Vec<Rule>,
	//Begining of non terminal (id)
	nt_begin : u32,
	axiom : u32,

	derivations : Vec<Vec<u32>>,
	actions : Vec<&'a dyn Fn(&mut STree)>,
	derivations_begin : Vec<u32>,
	table : Vec<Vec<u32>>
}

impl<'a> LL1Parser<'a> {
	///Function to create a new LL1 parser.\
	///Usage :
	///```
	/// //Terminal token (begin at 2 because of the 2 token reserved by the library)
	///const B : u32 = 2;
	///const LEFT_PAR : u32 = 3;
	///const RIGHT_PAR : u32 = 4;
	/// //Non terminal token begin
	///const A : u32 = 5;
	///let mut my_parser =
	///    llk::LL1Parser::new(
	///        //Rules
	///        vec![
	///        llk::Rule::new(vec![
	///            vec![LEFT_PAR, B, RIGHT_PAR],
	///        ]),//A
	///    ],
	///    //Actions linked to the rules
	///    vec![&|stree|{println!("{}", *stree)}],
	///    A,//axiom (first rule to be evaluated)
	///    A //Non terminal tokens begin
	///);
	///```
	///NB : Your parser is not usable immediatly after you've called this function.\
	///You should have a look to the following functions before ;)
	pub fn new(rules : Vec<Rule>, actions : Vec<&'a dyn Fn(&mut STree)>, axiom : u32, nt_begin : u32) -> LL1Parser {
		let nb_rules = rules.len();
		let mut this = LL1Parser{
			rules: rules,
			nt_begin: nt_begin,
			axiom : axiom,

			derivations : Vec::new(),
			actions : actions,
			derivations_begin : Vec::with_capacity(nb_rules),
			table: Vec::new()
		};
		//Count rules
		let mut nb_derivations : usize = 0;
		for rule in &this.rules {
			this.derivations_begin.push(nb_derivations as u32);
			nb_derivations += rule.derivations.len();
		}
		this.derivations = Vec::with_capacity(nb_derivations);
		for rule in &this.rules {
			for derivation in &rule.derivations {
				this.derivations.push(Vec::with_capacity(derivation.len()));
				let i = this.derivations.len() - 1;
				for sub_d in derivation {
					this.derivations[i].push(*sub_d);
				}
			}
		}
		this.table = vec![vec![u32::MAX; nt_begin as usize]; nb_rules];
		this
	}
	///Function which will create a table.
	///Usage :
	///```
	/// //Terminal token (begin at 2 because of the 2 token reserved by the library)
	///const B : u32 = 2;
	///const LEFT_PAR : u32 = 3;
	///const RIGHT_PAR : u32 = 4;
	/// //Non terminal token begin
	///const A : u32 = 5;
	///let mut my_parser =
	///    llk::LL1Parser::new(
	///        //Rules
	///        vec![
	///        llk::Rule::new(vec![
	///            vec![LEFT_PAR, B, RIGHT_PAR],
	///        ]),//A
	///    ],
	///    //Actions linked to the rules
	///    vec![&|stree|{println!("{}", *stree)}],
	///    A,//axiom (first rule to be evaluated)
	///    A //Non terminal tokens begin
	///);
	///my_parser.make_table();
	///```
	///You should call this function before calling the "analyse" function.\
	///This function will return an error message if an error occured. For example :\
	///```
	/// //Terminal token (begin at 2 because of the 2 token reserved by the library)
	///const B : u32 = 2;
	///const LEFT_PAR : u32 = 3;
	///const RIGHT_PAR : u32 = 4;
	/// //Non terminal token begin
	///const A : u32 = 5;
	///let mut my_parser =
	///    llk::LL1Parser::new(
	///        //Rules
	///        vec![
	///        llk::Rule::new(vec![
	///            vec![LEFT_PAR, B, RIGHT_PAR],
	///        ]),//A
	///    ],
	///    //Actions linked to the rules
	///    vec![&|stree|{println!("{}", *stree)}],
	///    A,//axiom (first rule to be evaluated)
	///    A //Non terminal tokens begin
	///);
	///match my_parser.make_table() {
	///    Err(msg) => {
	///        println!("Error while creating parsing table : {}", msg);//There was an error : print it.
	///    }
	///    Ok(()) => {} //Nothing to be done
	///}
	///```
	///### Notes about errors :
	///- There are two kinds of errors :\
	///    - Left recursion errors : A ::= A. Can often be corrected by changing the grammar.
	///    - Left factorisation errors : A ::= 'a' | 'a' B. Usually these grammars aren't LL(1).
	///- There is a backtrace on left recursion errors, but not on left factorisation ones.
	///- The index of the error is return (for exemple : "Left recursion error for the rule %i", where %i is the index of the rule).
	///- The ordre of the rules is important for error messages :
	///    - A ::= A | 'a' B 'c' will return a left recursion error.
	///    - A ::= 'a' B 'c' | A will return a left factorisation error.
	pub fn make_table(&mut self) -> Result<(), String> {
		//Rec fn
		fn create_rule(
			this : &mut LL1Parser,
			r_i : usize,
			term0 : &mut Vec<Vec<u32>>,
			is_in_progress : &mut Vec<bool>
		) -> Result<(), String> {
			if is_in_progress[r_i] {
				return Err(format!("Left recursion error for the rule {}", r_i + this.nt_begin as usize));
			}
			for d_i in 0..this.rules[r_i].derivations.len() {
				let derivation = &this.rules[r_i].derivations[d_i];
				let t_act = derivation[0] as usize;
				//derivation[0] is a terminal ?
				if t_act < this.nt_begin as usize {
					term0[r_i].push(t_act as u32);
					if this.table[r_i][t_act] != u32::MAX && t_act as u32 != default_id::NONE {
						return Err(
							format!(
								"Left factorisation error for the {r_i}th rule (derivation {}).",
								d_i + this.derivations_begin[r_i] as usize
							)
						);
					}
					this.table[r_i][t_act] = this.derivations_begin[r_i] + d_i as u32;
					continue;
				} //else
				is_in_progress[r_i] = true;
				//The rule is a nt, thus to access to the right index in the tables we have to substract it
				//the begin index of nt.
				let t_acti = t_act - this.nt_begin as usize;
				if term0[t_acti].len() == 0 {
					match create_rule(this, t_acti, term0, is_in_progress) {
						Err(err) => {
							return Err(format!("{err}\n\tNote : From rule {}", r_i + this.nt_begin as usize));
						}
						Ok(()) => {}
					}
				}
				for t_i in 0..term0[t_acti].len() {
					let terminal = term0[t_acti][t_i];
					term0[r_i].push(terminal);
					if this.table[r_i][terminal as usize] != u32::MAX {
						return Err(format!("Left factorisation error for the rule {}", r_i + this.nt_begin as usize));
					}
					this.table[r_i][terminal as usize] = this.derivations_begin[r_i] + d_i as u32
				}
				is_in_progress[r_i] = false;
			}
			Ok(())
		}

		//main
		let nb_rules = self.rules.len();
		let nb_terminals = self.nt_begin;
		let mut term0 = vec![Vec::with_capacity(nb_terminals as usize); nb_rules];
		let mut is_in_progress = vec![false; nb_rules];

		for r_i in 0..self.rules.len() {
			if term0[r_i].len() > 0 {
				continue;
			}
			match create_rule(self, r_i, &mut term0, &mut is_in_progress) {
				Err(err) => {return Err(err);}
				Ok(()) => {}
			}
		}

		Ok(())
	}
	///## Function which will analyse some tokens.
	///The only argument which you have to give to this function is a function "get_next_token":\
	///Each time "get_next_token" will be called, it should return Some(token) where token is the next token to be analysed,
	///or None if every tokens were analysed. The last token returned should alway the "T_EOF" token.\
	///- If there was no error, a tuple which contains a syntax tree and a warning string
	/// is return (for more informations about the stree, see STree struct).
	/// The only warning which can be emmit says that the parser recognize the tokens list, but that it did not reach the
	/// end of the file. If the warning string is empty, it means that there was no warning (warnings == String::from("")).
	///- Else, one of the following error is return :
	///    - "Unexpected EOF" : The end of the file was reached unexpectedly.
	///    - "Uncorresponding token : {} expected, found {}" or
	///"There is no rule which correspond to the derivation : Rule {} doesn't begin by token {}" :
	///Syntax error.
	///    - "Unknown token" : One the token in the list given in input is > to the begining of non terminal tokens.
	///
	///### Example :
	///```
	///const B : u32 = 2;
	///const LEFT_PAR : u32 = 3;
	///const RIGHT_PAR : u32 = 4;
	/// //Non terminal token begin
	///const A : u32 = 5;
	/// //First of all, create the parser
	///let mut my_parser =
	///    llk::LL1Parser::new(
	///        //Rules
	///        vec![
	///        llk::Rule::new(vec![
	///            vec![LEFT_PAR, B, RIGHT_PAR],
	///        ]),//A
	///    ],
	///    //Actions linked to the rules
	///    vec![&|stree|{println!("{}", *stree)}],
	///    A,//axiom (first rule to be evaluated)
	///    A //Non terminal tokens begin
	///);
	/// //Then, make the corresponding tables as we saw in the function 'make_table()'.
	///my_parser.make_table();
	/// //Lets create a little list of tokens (we need to clone them here. To a more advanced example of lexer,
	/// //you should have a look to the 'tokenize' closure in the main.rs file. I think it should not be too hard
	/// //to understand).
	///let T_L = llk::Token::Terminal{id : LEFT_PAR, value : String::from("("), pos : 0};
	///let T_R = llk::Token::Terminal{id : RIGHT_PAR, value : String::from(")"), pos : 0};
	///let T_B = llk::Token::Terminal{id : B, value : String::from("b"), pos : 0};
	/// // (b)
	/// //Here, everythings OK
	///let tokens = [T_L.clone(), T_B.clone(), T_R.clone(), llk::default_token::T_EOF.clone()];
	///let mut ind = 0;
	///match my_parser.analyse_tokens(|| {
	///        ind += 1;
	///        if ind == tokens.len() {
	///            return None;
	///        }
	///        return Some(tokens[ind - 1].clone());
	///    }
	///){
	///    Ok((stree, warnings)) => {
	///        //Do smthg with the stree
	///        if warnings != String::from("") {
	///            println!("{}", warnings);
	///            panic!("Unexpected warning");
	///        }
	///    }
	///    Err(e) => {
	///        println!("{:?}", e);
	///    }
	///}
	/// // (b))
	/// //Here, we should got a warning : tokens remain unanalysed.
	///let tokens1 = [T_L.clone(), T_B.clone(), T_R.clone(), T_B.clone(), llk::default_token::T_EOF.clone()];
	///let mut ind = 0;
	///match my_parser.analyse_tokens(|| {
	///        ind += 1;
	///        if ind == tokens1.len() {
	///            return None;
	///        }
	///        return Some(tokens1[ind - 1].clone());
	///    }
	///){
	///    Ok((stree, warnings)) => {
	///        //Do smthg with the stree
	///        if warnings != String::from("") {
	///            println!("{}", warnings);
	///            assert!(warnings == String::from("Warning : Tokens remain unanalysed."));
	///        }
	///    }
	///    Err(e) => {
	///        println!("{:?}", e);
	///    }
	///}
	/// // (b
	/// //Here, we should got an error : unexpected EOF.
	///let tokens2 = [T_L.clone(), T_B.clone(), llk::default_token::T_EOF.clone()];
	///let mut ind = 0;
	///match my_parser.analyse_tokens(|| {
	///        ind += 1;
	///        if ind == tokens2.len() {
	///            return None;
	///        }
	///        return Some(tokens2[ind - 1].clone());
	///    }
	///){
	///    Ok((stree, warnings)) => {
	///        //Do smthg with the stree
	///        if warnings != String::from("") {
	///            println!("{}", warnings);
	///        }
	///    }
	///    Err(e) => {
	///        //TODO assert!(warnings == String::from("Warning : Tokens remain unanalysed."));
	///        println!("{:?}", e);
	///    }
	///}
	///```
	pub fn analyse_tokens<F>(&mut self, mut get_token : F)
		-> Result<(STree, String), SyntaxError> //(stree, warnings), error
		where F : FnMut() -> Option<Token>
	{
		//INITIALISATIONS
		let mut warnings = String::from("");
		//Contains id of the tokens to be analysed
		let mut stack : Vec<u32> = Vec::new();
		//The first rule to be analysed.
		stack.push(self.axiom);
		//Contains (node_act, nb_children_remaining_to_push_to_node_act)
		let mut tree_stack : Vec<(STree, u32, u32)> = Vec::new();
		//The first token to be analysed
		let mut token_act = match get_token() {
			Some(Token::Terminal{id, value, pos}) => { Terminal{id : id, value : value.clone(), pos : pos} }
			Some(Token::NTerminal{id}) => {
				return Err(SyntaxError::new(
					U_TERM,
					Some(Token::NTerminal{id : id}),
					format!("Tokens expected in input, got a non terminal")
					)
				);
			}
			None => {
				return Err(SyntaxError::new(
					UEOF,
					None,
					String::from("Unexpected EOF")
					)
				);
			}
		};

		//ANALYSE
		'main_loop : while let Some(top) = stack.pop() {
			//Construct the stree
			//TODO improve because it's ugly
			let mut i_top = tree_stack.len();
			//> 1 because we are popping and then pushing on the new top (NB : A i_top == 1 should be a bug)
			//println!("{:?}", tree_stack);
			if i_top > 1 {
				//println!("top = {}\nnb_to_push{}\nd_i={}",tree_stack[i_top - 1].0, tree_stack[i_top - 1].1, tree_stack[i_top - 1].2);
				let mut nb_node_to_push = tree_stack[i_top - 1].1;
				while nb_node_to_push == 0 && i_top > 1 {
					i_top -= 1;
					let last_vertex = match tree_stack.pop() {
						Some((mut vertex, _, i_action)) => {
							println!("{i_action}");
							self.actions[i_action as usize](&mut vertex);
							vertex
						}
						None => { continue; }
					};
					tree_stack[i_top - 1].0.children.push(last_vertex);
					tree_stack[i_top - 1].1 -= 1;
					nb_node_to_push = tree_stack[i_top - 1].1;
				}
			}

			if token_act.id > self.nt_begin { //We are facing an unknown token
				return Err(SyntaxError::new(
					U_TOKEN,
					Some(Token::Terminal{id : token_act.id, value : token_act.value, pos : token_act.pos}),
					String::from("Unknown token")
					)
				);
			}

			//Analyse if it is a terminal
			if top < self.nt_begin {
				if top == token_act.id {
					//Push it into the stree at the top of the stack
					let i_top = tree_stack.len() - 1;
					tree_stack[i_top].0.children.push(
						STree {
							children : Vec::new(),
							value : Token::Terminal{id : token_act.id, value : token_act.value.clone(), pos : token_act.pos}
						}
					);
					tree_stack[i_top].1 -= 1;
				} else {
					if token_act.id == default_id::EOF {
						return Err(SyntaxError::new(
							UEOF,
							None,
							String::from("Unexpected EOF")
							)
						);
					}
					return Err(SyntaxError::new(
						M_TOKEN,
						Some(Token::Terminal{id : token_act.id, value : token_act.value, pos : token_act.pos}),
						format!("Uncorresponding token : {top} expected, found {}", token_act.id)
						)
					);
				}
				//Get the next token
				let next_t = match get_token() {
					Some(Token::Terminal{id, value, pos}) => { Terminal{id : id, value : value.to_string(), pos : pos} }
					Some(Token::NTerminal{id}) => {
						return Err(SyntaxError::new(
							U_TERM,
							Some(Token::NTerminal{id : id}),
							format!("Tokens expected in input, got a non terminal")
							)
						);
					}
					None => {
						return Err(SyntaxError::new(
							UEOF,
							None,
							String::from("Unexpected EOF")
							)
						);
					}
				};
				drop(token_act);
				token_act = next_t;
				continue 'main_loop;
			}

			//Analyse if it is a non terminal
			//We handle UEOF after because of NONE handling TODO explain why
			let d_i = self.table[(top - self.nt_begin) as usize][token_act.id as usize] as usize;
			if d_i == u32::MAX as usize {
				if self.table[(top - self.nt_begin) as usize][default_id::NONE as usize] != u32::MAX {
					let i_top = tree_stack.len();
					tree_stack[i_top - 1].1 -= 1;
					continue 'main_loop;
				} else {
					//Here, we handle UEOF
					if token_act.id == default_id::EOF {
						return Err(SyntaxError::new(
							UEOF,
							None,
							String::from("Unexpected EOF")
							)
						);
					}
					//ELSE
					return Err(
						SyntaxError::new(
							0,
							Some(Token::Terminal{id : token_act.id, value : token_act.value, pos : token_act.pos}),
							format!(
						"There is no rule which correspond to the derivation : No derivation of the rule {} begins by token {}",
								top - self.nt_begin,
								token_act.id,
							)
						)
					);
				}
			}
			let len = self.derivations[d_i].len();
			println!("di : {d_i} = [{}][{}]", top - self.nt_begin, token_act.id);
			tree_stack.push(
				(
				STree {children : Vec::with_capacity(len), value : Token::NTerminal{id : top}},
				len as u32,
				d_i as u32 //The index of the action to execute is the same as the corresponding derivation index
				)
			);
			for t_i in 1..len + 1 {
				let t_to_push_i = len - t_i;
				stack.push(self.derivations[d_i][t_to_push_i] as u32);
			}
		}

		//If it remains more than one subtree from the analyse, it's because the last rule was a kind of list.
		//Then, push every subtree until it remains one syntax tree : the main one (which also represent the axiom of the grammar).
		let mut i_top = tree_stack.len();
		let mut nb_node_to_push = tree_stack[i_top - 1].1;//If it panics, that's an internal error. Please report it.
		while i_top > 1 {
			i_top -= 1;
			if nb_node_to_push != 0 {
				panic!("Internal parser error : look at line 479 : more than one node remaining to push while not in the main loop");
			}
			let last_vertex = match tree_stack.pop() {
				Some((mut vertex, _, i_action)) => {
					self.actions[i_action as usize](&mut vertex);
					vertex
				}
				None => { continue; } //Will never append since we stop at i_top = 1.
			};
			tree_stack[i_top - 1].0.children.push(last_vertex);
			tree_stack[i_top - 1].1 -= 1;
			nb_node_to_push = tree_stack[i_top - 1].1;
		}

		//RETURNS
		if let Some(_) = get_token() {
			warnings = String::from("Warning : Tokens remain unanalysed.");
		}
		match tree_stack.pop() {
			Some((stree, _, _)) => { return Ok((stree, warnings)); }
			None => { panic!("Internal parser error : No syntax tree to be return"); }
		}
	}

	fn get_table(&self) -> &Vec<Vec<u32>> {
		&self.table
	}
}
