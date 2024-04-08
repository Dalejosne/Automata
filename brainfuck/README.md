# Brainfucki

An interpreter for brainfuck, in less than 50 lines of code (for the main interpreter part, see eval function in brainfucki.h).

It can be used as a header only library containing a function to interpret brainfuck source file (see brainfucki.h)
or as a standalone interpreter using the brainfucki.c file which is a little implementation of a basic running interpreter.

Nota : an old version of this project, with a known memory leak and a quiet complex design can be found in anc.

## Building

### Under linux

First, do :

```sh
git clone https://github.com/Dalejosne/Automata
cd Automata/brainfuck
```

For linux :
```sh
make linux
```

For linux, with debug and sanitizers :
```sh
make debug
```

For windows :
```sh
make windows
```

For both :
```sh
make all
```

### Under Windows

For now, you can only use [wsl](https://docs.microsoft.com/en-us/windows/wsl/install-win10). If you know how to build it under windows, feel free to contribute !

However, a build executable can be found under the build directory. It was tested under windows 11. In addition, you should be able to compile this
code with vs code, since it's only ansi c, but I didn't test it.

## Documentation

Just run :
```sh
make doc
```
And then open the file docs/build/html/index.html in a web browser

## Running

Once build has been done, you just have to do :

- Linux
```sh
./build/brainfucki my_file.bf
```

- Windows
```sh
./build/brainfucki.exe my_file.bf
```

You can replace my_file.bf by examples/HelloWorld.bf to see a beautifull Syerpinsky triangle displayed thanks to
a program written in brainfuck !
