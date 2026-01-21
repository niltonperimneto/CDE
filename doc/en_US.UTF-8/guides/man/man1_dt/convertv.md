# dtconvertvf
special filedtconvertvfconvert VUE 3.0 action/filetype files to DT syntaxdtconvertvf
## DESCRIPTION


The`dtconvertvf`utility is a filter script for converting VUE 3.0 action and filetype
databases to the new CDE 1.0 datatypes syntax.
If the input is a VUE 3.0
filetype file (*.vf) then its output will be converted to a CDE 1.0 datatypes
file.
It is up to the user to name the output file accordingly.
To be
recognized by CDE 1.0 it must have the.dtsuffix.
## EXAMPLE


To convert a single action/filetype file to CDE 1.0 syntax:
sp 1`dtconvertvf``<``file.vf``>``file.dt`To convert all action/filetype files in current directory to CDE 1.0 syntax:
for vffile in *.vf; do`dtfile=${vffile%.vf}.dt``dtconvertvf``<``$vffile``>``$dtfile`done