Preface

TheInformation System Author's and Programmer's Guidedoes the following:Provides information on building SGML-conforming document files into 
a form you can view with the Information Manager.Describes how to create and use style sheets.Describes architectural forms, and explains how to apply them to a DTD so users can
build SGML documents into a form viewable with the Information Manager.

To use the Information Manager successfully, your
SGML source must use the generalized
markup specified by the Standard Generalized Markup Language [SGML (ISO-8879)].
# Who Should Use This Book


This guide is intended for technical writers, administrators, and publishers who
are responsible for creating, validating, and building SGML
documents into on-line information libraries viewable in the
CDE Information Manager. Readers should have an
understanding of SGML and the ability to edit an SGML DTD.How This Book Is OrganizedExplanations of the contents of this book follow:Chapter 1, &ldquo;About the Build Process and Tools,&rdquo; 
provides an overview of the
Information Manager build process
along with a brief description of the documents and
commands associated with that process.Chapter 2, &ldquo;SGML and Information Manager Document Structure&rdquo;
provides a brief overview of SGML concepts and of the
Information Manager on-line information structure.Chapter 3, &ldquo;Preparing to Build&rdquo;
describes the SGML components you need in order to
build an information library.Chapter 4, &ldquo;Building Information Libraries&rdquo;
covers the process of building and updating an
information library using thedtdocbook2infolibcommands.Chapter 5, &ldquo;Administering Bookcases&rdquo;
describes how you can modify an existing information
library by copying, removing, rearranging, renaming,
and listing the bookcases in a library.Chapter 6, &ldquo;Troubleshooting&rdquo;
gives troubleshooting advice and discusses build errors, style sheet errors,
and library administration messages.Chapter 7, &ldquo;Understanding Style Sheets&rdquo;
describes the DtInfo Style Sheet model and provides general guidelines
for developing style sheet structure, setting up element path specifications,
and using the Select feature. It also describes element inheritance.Chapter 8, &ldquo;Creating a Style Sheet&rdquo; describes 
the available style sheet features
and defines their attributes and values. Provides instructions for 
creating a style sheet. Includes examples illustrating
how the features might be used in a style sheet.Chapter 9, &ldquo;Understanding Architectural Forms&rdquo; 
describes Information Manager architectural forms and shows how they
fit in the model for the Information Manager build process and tools.Chapter 10, &ldquo;Using Architectural Forms&rdquo; 
describes Information Manager 
architectural forms and how to apply them in your DTD(s).Chapter 11, &ldquo;Table of Contents Architectural Forms&rdquo; describes 
how to apply architectural forms to the elements in your table of contents
DTD so the Information Manager knows how to treat the table of contents
information.Related Documentation

For additional information about SGML, see:

ISO 8879:1986 Information Processing - Text and Office
Systems - Standard Generalized Markup Language
(SGML)Geneva, 15 October 1986.

The SGML Handbook, C.F. Goldfarb, Clarendon Press,
Oxford University Press, 1990.

Practical SGML, Second Edition, Eric van Herwijnen,
Kluwer Academic Publishers, 1994.

SGML Open Home Page, http://www.sgmlopen.org/sgml/docs/index.htmlWhat DocBook SGML Markup MeansThis book is written in the Structured Generalized Markup
Language (SGML) using the DocBook Document Type Definition (DTD).
The following table describes the DocBook markup used for
various semantic elements.DocBook SGML MarkupMarkup AppearanceSemantic Element(s)ExampleAaBbCc123The names of commands.Use thelscommand to list files.AaBbCc123The names of command options.Usels&minus;ato list all files.AaBbCc123Command-line placeholder:
replace with a real name or value.To delete a file, typermfilename.AaBbCc123The names of files and
directories.Edit your.loginfile.AaBbCc123Book titles, new words or terms, or
words to be emphasized.Read Chapter 6 inUser's
Guide.
These are calledclassoptions.
Youmustbe root to do this.