<br>

- [Setup](#setup)
  - [Prerequisties](#prerequisties)
  - [Windows](#windows)
  - [Unix](#unix)
- [Project creation](#create-a-new-project)
- [Compile & Run](#compile-and-run)
  - [Wng API](#use-wng-api-)
  - [Check](#checking)
- [Features](#features)
  - [Archive](#archive)
  - [Reinitialisation](#to-reinitialize-a-project)
  - [Header](#to-create-a-header-file)
  - [Tests](#to-run-tests)
- [Libraries](#libraries)
  - [Install](#to-install-a-library)
  - [Publish](#publish-your-library)

<br>

# How to use

## Setup

### Prerequisties

Make sure to have [Git](https://git-scm.com), [tar](https://www.gnu.org/software/tar/) & [gcc](https://gcc.gnu.org/) installed on your computer.

### Installation

To install wng you can either :
- Download the latest binary in the releases
- Download updater.sh (*Nix only and needs cargo)
	- *Note : you can setup a cron to keep an up to date wng version 😉*
- Run `cargo install wng`
	- In /bin/ if you are on *nix
	- In C:\Program Files\ if you are on Windows

## Create a new project

Open the command prompt and run :

```
$ wng new <project_name> [-cpp]
$ cd project_name/
```

Three folders have been created, `tests/`, `src/` and `build/`

In `src/`, you'll find file `main.c[pp]` that contains a basic hello world program.

<br>

## Compile and Run

```
$ wng build

$ wng run <args>
Hello World
```

_NOTE : `wng build` will build a debug executable, with flags -W -Wall -Werror -Wextra. To disable this, build in release mode with : `wng build --release`_

### Custom build

To build with a custom build, you have to create a `build.py` or `build.rb` file with your code to build.

If you want to specify a special python / ruby interpreter path, add the section `"pyinterpreter" : "path2python"` or `"rbinterpreter" : "path2ruby"` to your project.json.

Minimal python version required : 3.5
Minimal ruby version required : 2.3

Then run your script with `wng build --custom`

### Wngbuild library

Wngbuild library provides some useful features to compile your project

It is available in Ruby and Python

*Note : If both build.rb & build.py files exists, build.py will be used*

```py
from wngbuild import * # Import all from wngbuild module

build = BuildProfile(files="src/*.c",output="build/custom/prog.exe" ) # setup a build profile that will compile all files in src/ and place the binary in build/custom/prog.exe
build.cc = "C:\\MinGW\\bin\\gcc.exe" # Setup the compiler (optional, by default "gcc")
build.flags = "-W -Wall -Werror -Wextra" # Setup the flags that the command will be run with (optional)

build.run() # Run the compilation command
build.runOutput() # Run the binary produced by the compilation command (Will raise an error if the compilation command fails)
```

```rb
require 'wngbuild'

build=BuildProfile.new("src/*.c", "build/custom/prog") * Setup a build profile
builc.cc="C:\\Program Files\\clang\\bin\\clang.exe"

build.run() * Run compilation
build.runOutput() * Run produced file (Will raise an error if compilation failed)
```

<br>

### Checking

You can just check if there is any errors or warnings in your code without producing any binary with `wng check`

## Features

### Archive

To create a gunzip archive of your project files, just run `wng archive` and a file called `project.tar.gz` will be created

### To reinitialize a project

```
$ wng reinit
Really want to reinit ? Y/N : Y
Project renitialized !
```

### To create a header file

```
$ wng header foo
$ cat foo.h
#ifndef _FOO_H_
#define _FOO_H_


#endif /*_FOO_H*/
```

### To run tests

Tests have to be in tests/tests.c

To use functions that are in src/ files, just include the header with `#include "../src/<header>.h"`

Then you can run them with `wng test`

## Libraries

### To install a library

```
$ cd yourproject/
$ wng install <source>:<username>/<repo_name>
```

_Available sources are : `github`,`gitlab` & `bitbucket`_
_NOTE : Repository has to have a `lib/` folder inside or wng will refuse to install it_

### Publish your library

Create a repository on GitHub, BitBucket or GitLab with your project, library files have to be in a `lib/` folder

## Contributing

See our [contribution guidelines](https://github.com/wmanage/wng/blob/master/CONTRIBUTING.md).