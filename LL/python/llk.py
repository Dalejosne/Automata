"""
A little library for creating LL(K) and LL(*) parser (which for now only contains a LL(1) parser)
Performance : 6MB/min with the grammar you can find in main, with around 120 byte / byte => 720 MB memory consumption
with a 6MB file (due to the Syntax tree creation. Not creating it should improve a lot memory consumption)
"""
MAX_INT = 2**32

class STree :
	"""
	TODO doc
	"""
	def __init__ (self, value, children) -> None :
		"""
		TODO doc
		"""
		self.value = value
		self.children = children

	def __repr__ (self) -> str :
		def aux (self, ident) -> str :
			tab = ""
			for _ in range(ident) :
				tab += '\t'
			children_str = ""
			if self.children :
				for child in self.children :
					children_str += f"\n{tab}{aux(child, ident + 1)}"
			return f"STree : {self.value}, children : {children_str}"
		return aux(self, 1)

	def push_child (self, child) -> None :
		self.children.append(child)


class Token :
	"""
	TODO doc
	"""
	def __init__ (self, id, value, pos) -> None :
		"""
		TODO doc
		"""
		self.id = id
		self.value = value
		self.pos = pos
	def is_terminal (self) -> bool :
		"""
		TODO doc
		"""
		return self.value != None

	def __repr__ (self) -> str :
		return f"Token : {self.value} of type {self.id} at pos {self.pos}"

class Rule :
	"""
	TODO doc
	"""
	def __init__ (self, list_derivations) -> None :
		"""
		TODO doc
		"""
		self.derivations = list_derivations

	def __repr__ (self) -> str :
		"""
		TODO doc
		"""
		return str(self.derivations)+"\n"

class LL1Parser :
	"""
	TODO doc
	"""
	def __init__ (self, rules, actions, axiom, nt_begin) -> None :
		"""
		TODO doc
		"""
		self.rules = rules
		self.nt_begin = nt_begin
		self.axiom = axiom
		self.actions = actions
		self.derivations = []
		for rule in rules :
			self.derivations += [derivation for derivation in rule.derivations]
		d_end = 0
		self.derivations_begin = []
		for rule in rules :
			self.derivations_begin.append(d_end)
			d_end += len(rule.derivations)
		self.table = [[MAX_INT]*nt_begin for _ in range(len(rules))]

	def make_table (self) -> str :
		"""
		TODO : test and doc
		"""
		begin_terminal = [[] for _ in range(len(self.rules))]
		marked_rule = [False]*len(self.rules)
		def aux (self, act, begin_terminal, marked_rule) :
			error = None
			if marked_rule[act] :
				return f"Left recursion error for rule {act}"
			marked_rule[act] = True
			for i in range(len(self.rules[act].derivations)) :
				derivation = self.rules[act].derivations[i]
				t_act = derivation[0]
				derivation_id = self.derivations_begin[act] + i
				#It's a terminal token
				if t_act < self.nt_begin :
					if self.table[act][t_act] != MAX_INT and t_act != NONE :
						return f"Left factorisation error for rule {act} in derivation {i} (token {t_act} concerned)"
					self.table[act][t_act] = derivation_id
					begin_terminal[act].append(t_act)
				#It's a non terminal token
				else :
					t_act -= self.nt_begin
					if len(begin_terminal[t_act]) == 0 :
						error = aux (self, t_act, begin_terminal, marked_rule)
					if error :
						return error + f"\n\tNote : from here : Rule {act} derivation {i}"
					for terminal in begin_terminal[t_act] :
						if self.table[act][terminal] != MAX_INT :
							return f"Left factorisation error for rule {act} in derivation {i} (token {terminal} concerned)"
						self.table[act][terminal] = derivation_id
						begin_terminal[act].append(terminal)
				marked_rule[act] = False
			return error

		for i in range(len(self.rules)) :
			if len(begin_terminal[i]) == 0 :
				error = aux(self, i, begin_terminal, marked_rule)
				if error :
					return error
		for derivation in self.derivations :
			derivation.reverse() #since we will need to push it into a reversed stack for parsing
		return None

	def analyse (self, get_token) -> (str, STree) :
		"""
		TODO : doc and tests
		"""
		stack = [self.axiom]
		#contains (stree, nb_node_to_push, action)
		tree_stack = []
		token_act = get_token()
		while stack :
			top = stack.pop()
			while len(tree_stack) > 1 and tree_stack[-1][1] == 0 :
				(tree_to_push, _, todo) = tree_stack.pop()
				todo(tree_to_push)
				tree_stack[-1][1] -= 1
				tree_stack[-1][0].push_child(tree_to_push)
			#top is a terminal
			if top < self.nt_begin :
				if not token_act :
					return f"Error, parser unexpectedly reached end of file"
				if token_act.id > self.nt_begin :
					return f"Error, unknown token : Got a non terminal in input"
				if token_act.id != top :
					return f"Error, uncorresponding token : Expected {top}, got {token_act.id}"
				tree_stack[-1][0].push_child(STree(token_act, None))
				tree_stack[-1][1] -= 1
				token_act = get_token()
				continue
			#else, top is a non terminal
			if not token_act :
				return f"Error, parser unexpectedly reached end of file"
			r_i = top - self.nt_begin # do the action on rule at index r_i
			d_i = self.table[r_i][token_act.id]
			if d_i == MAX_INT :
				if self.table[r_i][NONE] != MAX_INT :
					tree_stack[-1][1] -= 1
					continue
				return f"Error, none of the derivation of the rule {r_i} begin by a {token_act.id}"
			#OK
			tree_stack.append([STree(top, []), len(self.derivations[d_i]), self.actions[d_i]])
			stack += self.derivations[d_i]
		while len(tree_stack) > 1 and tree_stack[-1][1] == 0 :
			(tree_to_push, _, todo) = tree_stack.pop()
			todo(tree_to_push)
			tree_stack[-1][1] -= 1
			tree_stack[-1][0].push_child(tree_to_push)
		warnings = None
		if token_act :
			warnings = f"Warning, tokens remain unanalysed (from token {token_act})"
		res = (warnings, tree_stack.pop()[0])
		return res

	def __repr__ (self) -> str :
		return f"LL1Parser : table : {self.table}\n\taxiom : {self.axiom}\n\tNon terminal begin at {self.nt_begin}"

global t_eof, t_none, eof, NONE
(NONE,) = (i for i in range(1))
t_none = Token(NONE, '', 0)

global ueof, unknown_token, uncorresponding_token
(ueof, unknown_token, uncorresponding_token) = (i for i in range(3))

def main() :
	def delete(st) :
		del st
		st = None
	da = lambda st : delete(st)
	(LEFT_PAR, RIGHT_PAR, RULE_NAME, ASSIGN, SPECIAL, END) = (i+1 for i in range(NONE, 6+NONE))
	(CODE, WORD, LIST_WORDS, RULE, RULE_DEF, MAIN, LIST_RULE_DEF, LIST_NAME, OPT_SPECIAL) = (i+1 for i in range(END, END+9))
	my_parser = LL1Parser([
							Rule([[LEFT_PAR, LIST_WORDS, RIGHT_PAR]]),#rule CODE
							Rule([
								[RULE_NAME],
								[CODE],
								[ASSIGN],
								[SPECIAL],
								[END]
							]),#rule WORD
							Rule([
								[WORD, LIST_WORDS],
								[NONE]
							]),#rule LIST_WORDS
							Rule([
								[RULE_NAME, OPT_SPECIAL, ASSIGN, OPT_SPECIAL, LIST_RULE_DEF, END]
							]),#rule RULE
							Rule([
								[LIST_NAME, CODE]
							]),#rule RULE_DEF
							Rule([
								[RULE, OPT_SPECIAL, MAIN],
								[CODE]
							]),#rule MAIN
							Rule([
								[RULE_DEF, OPT_SPECIAL, LIST_RULE_DEF],
								[NONE]
							]),#rule LIST_RULE_DEF
							Rule([
								[RULE_NAME, OPT_SPECIAL, LIST_NAME],
								[NONE]
							]),#rule LIST_NAME
							Rule([
								[SPECIAL],
								[NONE]
							])#rule OPT_SPECIAL
						  ],
						  [da, da, da, da, da, da, da, da, da, da, da, da, da, da, da, da, da, da],
						  MAIN,
						  CODE
						)
	error = my_parser.make_table()
	if error :
		print("An error occured while making the table : The grammar is buggy : ", error)
		return 1

	with open("test.py", "r") as f :
		code = f.read()
	code = list(code[::-1]) #code.reverse()
	code_len = len(code)
	def get(code):
		t_act = ""
		pos_act = code_len-len(code)
		letter_act = code.pop()
		if not code :
			return None
		while code and ord(letter_act) < 33 :
			t_act += letter_act
			letter_act = code.pop()
		if len(t_act) > 0 :
			code.append(letter_act)
			return Token(SPECIAL, t_act, pos_act)
		if letter_act == '=' :
			return Token(ASSIGN, '=', pos_act)
		elif letter_act == ';' :
			return Token(END, ';', pos_act)
		elif letter_act == '(' :
			return Token(LEFT_PAR, '(', pos_act)
		elif letter_act == ')' :
			return Token(RIGHT_PAR, ')', pos_act)
		while not (ord(letter_act) < 33 or letter_act == ';' or letter_act == '=') and code :
			t_act += letter_act
			letter_act = code.pop()
		#push the last letter, which we want to analyse next time.
		code += letter_act
		return Token(RULE_NAME, t_act, pos_act)

	get_token = lambda : get(code)
	result = my_parser.analyse(get_token)
	if isinstance(result, str) :
		print("An error occured while parsing : ", result)
		return 2
	(warnings, stree) = result
	if warnings :
		print(f"Warnings were produced : {warnings}")
	print(f"Parsing succeed.")
	return 0

if __name__ == "__main__" :
	main()
