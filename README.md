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
![Github All Releases](https://img.shields.io/github/downloads/Wmanage/wng/total?color=%2300ff00&label=Downloads&style=flat-square)

</div>

---

<br>
<br>

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
    2.10.18/2
    </th>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Working-%2300ff00">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Working-%2300ff00">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Testing-%23ffaa00">
    </td>
    </tr>
    <tr>
    <th align="center" width="100">
    2.10.1
    </th>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Not_Working-%23ff0000">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Working-%2300ff00">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-N/A-%23ffaa00">
    </td>
    </tr>
    <tr>
    <th align="center" width="100">
    2.9.2
    </th>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Not_Working-%23ff0000">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Working-%2300ff00">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-N/A-%23ffaa00">
    </td>
    </td>
    </tr>
    <tr>
    <th align="center" width="100">
    1.0.1
    </th>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-N/A-%23ffaa00">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-Working-%2300ff00">
    </td>
    <td align="center">
        <img src="https://img.shields.io/badge/Status-N/A-%23ffaa00">
    </td>
    </tr>
    </tbody>

</table>

<br>
<br>
<br>

---

<br>
<br>
<br>

# How to use

## Setup

### Prerequisties

Pay attention to have [Git](https://git-scm.com) & [curl](https://curl.haxx.se/) installed on your computer.

### Windows

First download the latest release of wanager, put it in `C:\Program Files` and add `C:\Program Files\` to Path.

### Unix

Download the latest release of wanager and put it in `/bin`

<br>

## Create a new project

Open the command prompt and run :

```
$ wng new <project_name>
$ cd project_name/
```

Two folders have been created, `src/` and `build/`

In `src/`, you'll find file `main.c` that contains a basic hello world program.

<br>

## Compile and Run

```
$ wng build

$ wng run <args>
Hello World
```

NOTE : `wng build` will build a debug executable, with flags -W -Wall -Werror -Wextra. To disable this, build in release mode with : `wng build --release`

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

### To install a library

```
$ cd yourproject/
$ wng install <lib_name>
```

<i>Note: To verify if a library exists, run</i> `wng query <lib_name>`

## Publish your library

To publish a library send me a mail to `wmanager@protonmail.ch`.

The mail has to contain :

- The library's name
- The library's description
- Why it should be published

<br>

WARNING : Library has to be **a single .c file** with **a single .h file**, send them linked to the mail.
