# dthelp_htag2
user cmddthelp_htag2second pass for parse of HelpTag sourcedthelp_htag2  -cdhotfileoutfile
## DESCRIPTION


Thedthelp_htag2executable
is invoked by
&cdeman.dthelptag; as the second
pass of translating HelpTag markup into SDL.
Thedthelp_htag2executable accepts the output of the first pass of
dthelptag (either
&cdeman.dthelp.htag1; or
&cdeman.dthelp.ctag1;) and computes the byte
offsets of virtual pages to be used in the runtime display of the SDL
volume.
If only one file is specified on the command line, the output file
will overwrite the input file.
If two files are specified, the output
will be placed in the second.
If the input file has no .sdl extension, one will be added.
If a
second file name is specified for output, it will be used without
modification.
The options todthelp_ctag2are:

* **-c:** 

compress the SDL document on a per virtual page basis - if
the document is already compressed, this command will result
in no change to the document.
* **-d:** 

decompress an SDL document - if the document is already
compressed, this command will result in no change to the
document; however, the byte offsets of virtual pages will be
recomputed.
* **-h:** 

print a help message and exit.
* **-o:** 

perform peephole optimization of the SDL document.
In
particular,`<FORM>`elements containing only a single`<BLOCK>`or`<FORM>`without a user supplied identifier are replaced by that
single`<BLOCK>`or`<FORM>`with the identifier of the original
(outer)`<FORM>`.
* **-t:** 

eliminate any`<TOSS>`sub-elements that are not used in this SDL
document.

## SEE ALSO


&cdeman.dthelptag;, &cdeman.dthelp.htag1;, &cdeman.dthelp.ctag1;, &cdeman.dtsdldtd;.