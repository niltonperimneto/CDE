PrefaceTheDesktop KornShell User's Guideprovides the
information you need to create Motif applications with KornShell (kshell)
scripts. In addition to the basic information you'll need to get started,
several example scripts of increasing complexity are described. Throughout
this guide the termdtkshmeans the Desktop KornShell.Who Should Use This GuideThis guide is intended for programmers who want a quick and easy means
of creating Motif applications, but don't have the time, knowledge, or inclination
to use the C programming language. A good understanding of kshell programming,
Motif, the Xt Intrinsics, and, to a lesser extent, Xlib is needed. An understanding
of C would also be helpful.How This Guide Is OrganizedChapter 1, &ldquo;Introduction to Desktop KornShell,&rdquo;describes the basic information you need to begin writing Motif
applications indtkshscripts.Chapter 2, &ldquo;A Sample Script,&rdquo;describes two simpledtkshscripts. The first
script creates a push button widget within a bulletin board widget. The
second script expands the first by adding a callback for the push button.Chapter 3, &ldquo;Advanced Topics,&rdquo;describes more advanced topics pertaining todtkshscripts.Chapter 4, &ldquo;A Complex Script,&rdquo;describes a much more complex script than either of the ones
described in Chapter 2. This script creates a graphic interface to thefindcommand.Appendix A, &ldquo;dtksh Commands,&rdquo;lists all thedtkshcommands.Appendix B, &ldquo;dtksh Convenience Functions,&rdquo;contains man pages for commands or functions that are not documented
elsewhere.Appendix C, &ldquo;Listing for script_find,&rdquo;contains the complete listing of the complex script described
in Chapter 4.Related BooksThe following books provide information on kshell programming, Motif,
the Xt Intrinsics, and Xlib:Desktop KornShell
Graphical Programming For the Common Desktop Environment Version 1.0,by J. Stephen Pendergrast, Jr., published by Addison-Wesley, Reading, MA
01867.The New KornShell Command and Programming
Language, by Morris I. Bolsky and David G. Korn, published by
Prentice-Hall, Englewood Cliffs, NJ 07632.KornShell Programming Tutorial,
by Barry Rosenberg, published by Addison-Wesley, Reading, MA 01867.Motif Programmer's Guide,
Open Software Foundation, 11 Cambridge Center, Cambridge, MA 02142, published
by Prentice-Hall, Englewood Cliffs, NJ 07632.Motif Programmer's Reference,
Open Software Foundation, 11 Cambridge Center, Cambridge, MA 02142, published
by Prentice-Hall, Englewood Cliffs, NJ 07632.Motif Reference Guide,
by Douglas A. Young, published by Prentice-Hall, Englewood Cliffs, NJ 07632.Mastering Motif Widgets(Second
Edition), by Donald L. McMinds, published by Addison-Wesley, Reading, MA
01867The X Window System Programming and Applications
with Xt Motif Edition, by Douglas A. Young, published by Prentice-Hall,
Englewood Cliffs, NJ 07632.The Definitive Guides to the X Window
System, Volume 1: Xlib Programming Manual, by Adrian Nye, published
by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 2: Xlib Reference Manual, edited by Adrian Nye,
published by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 3: X Window System User's Guide, by Valerie Quercia
and Tim O'Reilly, published by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 4: X Toolkit Intrinsics Programming Manual, by
Adrian Nye and Tim O'Reilly, published by O'Reilly and Associates, Sebastopol,
CA 95472.The Definitive Guides to the X Window
System, Volume 5: X Toolkit Intrinsics Reference Manual,edited
by Tim O'Reilly, published by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 6: Motif Programming Manual, by Dan Heller, published
by O'Reilly and Associates, Sebastopol, CA 95472.What DocBook SGML Markup MeansThis book is written in the Structured Generalized Markup
Language (SGML) using the DocBook Document Type Definition (DTD).
The following table describes the DocBook markup used for
various semantic elements.DocBook SGML MarkupMarkup AppearanceSemantic Element(s)ExampleAaBbCc123The names of commands.Use thelscommand to list files.AaBbCc123The names of command options.Usels&minus;ato list all files.AaBbCc123Command-line placeholder:
replace with a real name or value.To delete a file, typermfilename.AaBbCc123The names of files and
directories.Edit your.loginfile.AaBbCc123Book titles, new words or terms, or
words to be emphasized.Read Chapter 6 inUser's
Guide.
These are calledclassoptions.
Youmustbe root to do this.Shell Prompt CharactersThe following table shows shell prompt characters
used in this book.Shell Prompt Characters