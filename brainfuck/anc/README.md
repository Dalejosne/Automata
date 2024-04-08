# Brainfucki

An interpreter for brainfuck.

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

For windows :
```sh
make windows
```

For both :
```sh
make all
```

### Under Windows

For now, you can only use the [wsl](https://docs.microsoft.com/en-us/windows/wsl/install-win10). If you know how to build it under windows, feel free to contribute !

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
