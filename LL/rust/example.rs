Terminal (RULE_NAME LEFT_PAR RIGHT_PAR SPECIAL ASSIGN END)
CODE = LEFT_PAR LIST_WORDS RIGHT_PAR ();
WORD = RULE_NAME ()
	   CODE ()
	   ASSIGN ()
	   SPECIAL ()
	   END ();
LIST_WORDS = WORD OPT_SPECIAL LIST_WORDS ()
			 NONE ();
RULE = RULE_NAME OPT_SPECIAL ASSIGN OPT_SPECIAL LIST_RULE_DEF END ();
RULE_DEF = LIST_NAME CODE ();
MAIN = RULE OPT_SPECIAL MAIN()
	   CODE ();
LIST_RULE_DEF = RULE_DEF OPT_SPECIAL LIST_RULE_DEF ()
				NONE ();
LIST_NAME = RULE_NAME OPT_SPECIAL LIST_NAME ()
			NONE ();
OPT_SPECIAL = SPECIAL ()
			  NONE ();
()
