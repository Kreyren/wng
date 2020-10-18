<h1 align="center">Wanager</h1>
<br>
<br>
<div align="center">
<img width=200 src="assets/wng.png"/>
</div>
<br>
<div align="center">

![GitHub pull requests](https://img.shields.io/github/issues-pr/Wmanage/wng?label=Pull%20requests&style=flat-square)
![GitHub issues](https://img.shields.io/github/issues/Wmanage/wng?label=Issues&style=flat-square)
![GitHub stars](https://img.shields.io/github/stars/Wmanage/wng?color=%23aa1111&label=Stars&style=flat-square)
![GitHub](https://img.shields.io/github/license/Wmanage/wng?color=%23ffaa00&label=License&style=flat-square)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/Wmanage/wng?color=%23888800&label=Latest%20release&style=flat-square)
![Github All Releases](https://img.shields.io/github/downloads/Wmanage/wng/total?color=%2300aa00&label=Downloads&style=flat-square)

</div>

---

<br>
<br>
<h1 align="center">Continuous Integration</h1>
<table border="1" align="center">
    <thead>
    <tr>
    <th>
    </th>
    <th>
                <center><img alt="Linux" src="https://www.screenconnect.com/Images/LogoLinux.png" align="center" height="30" width="30" /></center><br>
                <center>GNU/Linux</center>
    </th>
    <th>
                <center><img alt="Windows" src="https://upload.wikimedia.org/wikipedia/commons/thumb/7/76/Windows_logo_-_2012_%28dark_blue%2C_lines_thinner%29.svg/414px-Windows_logo_-_2012_%28dark_blue%2C_lines_thinner%29.svg.png" align="center" height="30" width="30" /></center><br>
                <center>Windows</center>
    </th>
    <th>
                <center><img alt="MacOS" src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/fa/Apple_logo_black.svg/245px-Apple_logo_black.svg.png" align="center" height="30"width="25" /></center><br>
                 <center>MacOS</center>
    </th>
    </tr>
    </thead>
    <tbody>
    <tr>
    <th align="center" width="100">
    Status
    </th>
    <td align="center">
        <img src="https://img.shields.io/github/workflow/status/Wmanage/wng/RustUnix?label=Status&style=flat-square&color=success">
    </td>
    <td align="center">
        <img src="https://img.shields.io/github/workflow/status/Wmanage/wng/Rust?label=Status&style=flat-square&color=success">
    </td>
    <td align="center">
        <img src="https://img.shields.io/github/workflow/status/Wmanage/wng/RustUnix?label=Status&style=flat-square&color=success">
    </td>
    </tr>
    </tbody>

</table>

<br>
<br>
<br>

---

**Wanager** (aka wng) is a package manager & build tool (like [cargo](https://doc.rust-lang.org/cargo/) for Rustlang) for the C programming language written in [Rust](https://rust-lang.org). It allow you to create projects, headers, to install libraries & to compile and run fast and easily. It is different from CMake by its hability to manage libraries, packages and projects. The objective of this tool is to definitely give up Makefiles.

<br>

- [Setup](#setup)
  - [Prerequisties](#prerequisties)
  - [Windows](#windows)
  - [Unix](#unix)
- [Project creation](#create-a-new-project)
- [Compile & Run](#compile-and-run)
  - [Wng API](#use-wng-api-)
- [Features](#features)
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

Make sure to have [Git](https://git-scm.com) & [gcc](https://gcc.gnu.org/) installed on your computer.

### Windows

First download the latest release of wanager, put it in `C:\Program Files` and add `C:\Program Files\` to Path. [?](https://stackoverflow.com/questions/44272416/how-to-add-a-folder-to-path-environment-variable-in-windows-10-with-screensho)

### Unix

Download and run **as super user** [install.sh](https://github.com/Wmanage/wng/tree/master/install.sh)

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

<i>NOTE : `wng build` will build a debug executable, with flags -W -Wall -Werror -Wextra. To disable this, build in release mode with : `wng build --release`</i>

<h3>Custom build</h3>

To build with a custom build, you have to create a `build.py` file with your code to build.

If you want to specify a special python interpreter path, add the section `"pyinterpreter" : "path2python"` to your project.json.

Minimal python version required : 3.5

Then run your script with `wng build --custom`


### Use WNG api !

Wng API provides some useful things to compile your project as you want to.

```py
from wngbuild import * # Import all from wngbuild module

build = BuildProfile(files="src/*.c",output="build/custom/prog.exe" ) # setup a build profile that will compile all files in src/ and place the binary in build/custom/prog.exe
build.cc = "C:\MinGW\bin\gcc.exe" # Setup the compiler (optional, by default "gcc")
build.flags = "-W -Wall -Werror -Wextra" # Setup the flags that the command will be run with (optional)

build.run() # Run the compilation command
build.runOutput() # Run the binary produced by the compilation command (Will raise an error if the compilation command fails)
```

<<<<<<< HEAD
=======
<h3>Custom build</h3>

To build with a custom build, you have to create a `build.py` file with your code to build.

If you want to specify a special python interpreter path, add the section `"pyinterpreter" : "path2python"` to your project.json.

Minimal python version required : 3.5

Then run your script with `wng build --custom`
>>>>>>> dev

<br>

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

<h2>Libraries</h2>

### To install a library

```
$ cd yourproject/
$ wng install <source>:<username>/<repo_name>
```

<i>Available sources are : `github`,`gitlab` & `bitbucket`</i>
<i>NOTE : Repository has to have a `lib/` folder inside or wng will refuse to install it</i>

### Publish your library

Create a repository on GitHub, BitBucket or GitLab with your project, library files have to be in a `lib/` folder
