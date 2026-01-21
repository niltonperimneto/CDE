PrefaceTheCommon Desktop Environment: Internationalization Programmer's
Guideprovides information for internationalizating the desktop,
enabling applications to support various languages and cultural conventions
in a consistent user interface.Specifically, this guide:Provides guidelines and hints
for developers on how to write applications for worldwide distribution.Provides an overall view of internationalization
topics that span different layers within the desktop.Provides pointers to reference and more detailed
documentation. In some cases, standard documentation is referenced.This guide is not intended to duplicate the existing reference or conceptual
documentation but rather to provide guidelines and conventions on specific
internationalization topics. This document focuses on internationalization
topics and not on any specific component or layer in an open software environment.Who Should Use This BookThis book provides various levels of information for the application
programmer and developer and related fields.How This Book Is OrganizedExplanations of the contents of this book follow:Chapter 1, &ldquo;Introduction to Internationalization,&rdquo;provides an overview of internationalization and localizing within
the desktop, including locales, fonts, drawing, inputting, interclient communication,
and extracting user visual text. Information on the significance of internationalization
standards is also provided.Chapter 2, &ldquo;Internationalization and
the Common Desktop Environment,&rdquo;covers the set of topics
that developers commonly need to consider when internationalizing their applications,
including locale management, localized resources, font management, localized
text tasks, interclient communication for localized text, and internationalized
functions.Chapter 3, &ldquo;Internationalization and
Distributed Networks,&rdquo;discusses topics related to handling
encoded characters in distributed networks. Basic principles and examples
for interclient interoperability are provided to guide developers in internationalized
distributed environments.Chapter 4, &ldquo;Xt, Xlib, and Motif Dependencies,&rdquo;topics include internationalized applications, locale management,
localized text, international User Interface Language (UIL), and localized
applications.Appendix A,&ldquo;Message Guidelines,&rdquo;is a set of guidelines for writing messages.Related PublicationsSee the following documentation for additional information on topics
presented in this book:ISO C: ISO/IEC 9899:1990,Programming Languages --- C(technically identical to ANS X3.159-1989,
Programming Language C).ISO/IEC 9945-1: 1990, (IEEE Standard 1003.1)Information Technology - Portable Operating System Interface (POSIX) - Part
1: System Application Program Interface (API) [C Language].ISO/IEC DIS 9945-2: 1992, (IEEE Standard 1003.2-Draft)Information Technology - Portable Operating System Interface (POSIX) - Part
2: Shell and Utilities.Motif:Motif Programmer's
Reference,Revision 1.2, Open Software Foundation,
Prentice Hall, 1992, ISBN: 0-13-643115-1.Scheifler, W. R.,X Window System, The
Complete Reference to Xlib, Xprotocol, ICCCM, XLFD- X Version
11, Release 5, Digital Press, 1992, ISBN: 1-55558- 088-2.X/Open:X/Open CAE Specification System
Interface Definition, Issue 4, X/Open Company Ltd., 1992, ISBN:
1-872630-46-4.X/Open:X/Open CAE Specification Commands
and Utilities, Issue 4, X/Open Company Ltd., 1992, ISBN: 1-872630-48-0.X/Open:X/Open CAE Specification System
Interface and Headers, Issue 4, X/Open Company Ltd., 1992, ISBN:
1-872630-47-2.X/Open:X/Open Internationalization Guide, X/Open Company Ltd., 1992, ISBN: 1-872630-20-0.ISO/IEC 10646-1:1993 (E):Information
Technology - Universal Multi-Octet Coded Character Set (UCS). Part 1: Architecture
and Basic Multilingual Plane.What DocBook SGML Markup MeansThis book is written in the Structured Generalized Markup Language (SGML)
using the DocBook Document Type Definition (DTD). The following table
describes the DocBook markup used for various semantic elements.DocBook SGML MarkupMarkup AppearanceSemantic Element(s)ExampleAaBbCc123The names of commands.Use thelsto list files.AaBbCc123The names of command options.Usels&minus;ato list all files.AaBbCc123Command-line placeholder: replace with a real name or value.To delete a file, typermfilename.AaBbCc123The names of files and directories.Edit your.loginfile.AaBbCc123Book titles, new words or terms, or words to be emphasized.Read Chapter 6 inUser's Guide. These are calledclassoptions. Youmustbe root to do this.