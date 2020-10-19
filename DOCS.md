<br>

- [Project creation](#create-a-new-project)
- [Compile & Run](#compile-and-run)
  - [Wng API](#use-wng-api-)
  - [Check](#checking)
- [Features](#features)
  - [Reinitialisation](#to-reinitialize-a-project)
  - [Header](#to-create-a-header-file)
  - [Tests](#to-run-tests)
- [Libraries](#libraries)
  - [Install](#to-install-a-library)
  - [Publish](#publish-your-library)

<br>

## Create a new project

Open the command prompt and run :

```
$ wng new <project_name>
$ cd project_name/
```

Three folders have been created, `tests/`, `src/` and `build/`

In `src/`, you'll find file `main.c` that contains a basic hello world program.

<br>


## Compile and Run

```
$ wng build

$ wng run <args>
Hello World
```

*NOTE : `wng build` will build a debug executable, with flags -W -Wall -Werror -Wextra. To disable this, build in release mode with : `wng build --release`*


### Custom build

To build with a custom build, you have to create a `build.py` file with your code to build.

If you want to specify a special python interpreter path, add the section `"pyinterpreter" : "path2python"` to your project.json.

Minimal python version required : 3.5

Then run your script with `wng build --custom`

### Use WNG api !

Wng API provides some useful things to compile your project as you want to.

```py
from wngbuild import * # Import all from wngbuild module

build = BuildProfile(files="src/*.c",output="build/custom/prog.exe" ) # setup a build profile that will compile all files in src/ and place the binary in build/custom/prog.exe
build.cc = "C:\\MinGW\\bin\\gcc.exe" # Setup the compiler (optional, by default "gcc")
build.flags = "-W -Wall -Werror -Wextra" # Setup the flags that the command will be run with (optional)

build.run() # Run the compilation command
build.runOutput() # Run the binary produced by the compilation command (Will raise an error if the compilation command fails)
```

<br>

### Checking

You can just check if there is any errors or warnings in your code without producing any binary with `wng check`


## Features

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

To use functions that are in src/ files, just include the header with `#include "../src/<header>.h`"

Then you can run them with `wng test`

## Libraries

### To install a library

```
$ cd yourproject/
$ wng install <source>:<username>/<repo_name>
```

*Available sources are : `github`,`gitlab` & `bitbucket`*
*NOTE : Repository has to have a `lib/` folder inside or wng will refuse to install it*

### Publish your library

Create a repository on GitHub, BitBucket or GitLab with your project, library files have to be in a `lib/` folder
