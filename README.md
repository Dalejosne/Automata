# Automata
Here, you will find some funny automata.

For now, it includes :
- a ***brainfuck*** interpreter (written in C)
- a ***game of life*** library (writen in rust)
- a ***LL parser*** library (for now in rust, C and python are comming soon)

I will add some more automata in the future, and if there are some that you absolutely want to add, tell me so I'll include them.\
I'll also try to add some libraries utilities to create easily automata. If you want to see a "real" language inplementation in pure
C, have a look to [this project](https://github.com/Dalejosne/yafl).

Since I'm quite busy at school, I'm not always very active, but if you have some questions you can also send me an email at
***damlejo6445@gmail.com***

I tried to make the code as simple as I could, but if you see some ways of improving these code, feel free to do so.

## Dependencies

- Get and updates :
	- git
- Building :
	- gcc / mingw
	- rustup
	- make
- Testing
	- cunit
- Documentation :
	- python3
	- python3-pip
	- sphinx (pip install -U Sphinx)
	- hawkmoth (pip install hawkmoth)
		- clang
	- sphinx_rtd_theme (pip install sphinx-rtd-theme)
	- myst-parser (pip install myst-parser)

## Navigation

In this repository, you'll find some classic files (LICENSE), and every automata is located in subdirectory.

## Contributing

If you see some ways to improve code, tests or documentation, please send a pull request.

### Coding conventions

#### C
Caml case for struct, enum, etc, and snake for var, functions, etc. Define are in maj, and namespaces are in maj following by an
underscore.\
Firstly come the namespace, then the struct name, and then the function name.

Examples:
- With namespaces :
	- struct : MYNAMESPACE_MyStruct
	- fn : MYNAMESPACE_my_struct_my_fn
- Without :
	- struct : MyStruct
	- fn : my_struct_my_fn
***Important*** : There is an exception for new and delete : in these cases, you have to call your function new_my_struct or
delete_my_struct.

#### Rust
Coding conventions recommended by rust.

***RQ*** : There is no automated tests yet, but I tested the programs with the examples code. Since the codes are (yet) very
simple, it's not too bad but if you want to add some, it would be cool ! I like the cunit framework for C testing, or basically
using assert (std).
