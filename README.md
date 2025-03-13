
CDE - The Common Destop Environment
===

In 2012, CDE was opensourced under the terms of the LGPL V2 license by
the Open Group.

You may reuse and redistribute this code under the terms of this
license. See the COPYING file for details.

# Downloading

git clone https://github.com/niltonperimneto/CDE.git

# Compiling on Arch Linux

Complete build and installation instructions can be found on the CDE
wiki:

http://sourceforge.net/p/cdesktopenv/wiki/Home/

There are a variety of dependencies that must be met, as well as
specific set up steps required to build, especially relating to
localization and locales.

Do not expect to just type 'make' and have it actually work without
meeting the prerequisites and following the correct steps as spelled
out on the wiki.

There are also a lot of other documents and information there that you
might find useful.

Assuming you've met all of the requirements regarding packages needed
for the build, you can follow the standard autoconf method:

```
$ ./autogen.sh
$ ./configure
$ make
$ sudo make install
```

# Support

## Disclaimer

It is a fork of the Commond Desktop Enviroment hosted on SourceForge: 

```

https://sourceforge.net/p/cdesktopenv/code/ci/master/tree/

```

The purpose to this fork on Github is to iterate on the Arch Linux support and build.
Commits are welcomed and they will be submitted to upstream if authorized
It should be the groundwork of making an Arch User Repository (AUR) with the explicit purpose of being Arch Linux first

Maybe the scope of this fork widens in the future if there's time and interest, but for now it will be kept simple.












