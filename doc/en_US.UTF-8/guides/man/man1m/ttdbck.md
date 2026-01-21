# ttdbck
user cmdttdbckdisplay, check, or repair ToolTalk databasesttdbckselection optsdiagnosis optsdisplay optsrepair optsdata-base-directory...
## DESCRIPTION


ttdbckis the ToolTalk database maintenance tool.
It allows direct inspection of ToolTalk spec data, detection of
inconsistencies, and repair of problems.
## OPTIONS


* **data-base-directory** 

Names the directory or directories containing the ToolTalk database to be
inspected or repaired. If no directories are named, the current directory is
assumed. If a directory path does not end in ``TT_DB'', ``TT_DB''
is appended.
* **** 

The user running the command must have read access to the files in the
directory to inspect the data and write access to repair the data.
Since ToolTalk databases are typically accessible only to root, this
command is normally run as root.

### Selection options


The selection options determine which specs in the database are displayed
or modified. If no selection options are given, all specs in the database
are displayed. To prevent massive accidental changes to ToolTalk
databases, no repair options except-Iare allowed unless a selection or
diagnosis option is given.

* **-f  filename** 

Restricts the set of specs to be inspected or modified to those which
describe objects in the named file. The file name can contain shell-style
wildcards which must be escaped to prevent the shell from expanding them.
* **-k  objidkey** 

An object id key, specifying a particular spec to be displayed or modified.
The object id key can be obtained from a previous invocation ofttdbck; one might display a set of specs, determine the one that needs repair, and
specify its key here.
* **-t  type** 

Restricts the set of specs to be inspected or modified to those with
otypetype.The type name can contain shell-style wildcards which must be escaped
to prevent the shell from expanding them.

### Diagnosis options


These options check for and report on inconsistencies in the selected specs.
Only specs selected by the selection options are checked. If a diagnosis
option is given, any display or repair option is applied only to specs which
fail the diagnostic check.

* **-b** 

Check for badly formed specs: those which have no file or type or those
which have types not defined in the type database.
* **-x** 

Check for specs which refer to files that no longer exist.

### Display options


These options determine which data is printed for each selected spec.

* **-i** 

Display the object id (including the object id key.)
* **-m** 

Display the mandatory data that must appear in every spec: the otype
of the object described by the spec and the file in which the spec is
stored.
* **-p** 

Display all the properties and values for each selected spec.
* **-a** 

Display all data (equivalent to specifying`-imp`)

### Repair options


* **-I** 

Invoke the NetISAM isrepair() function for all files accessed.
This action is applied before any other inspection or repair
action.
This option should be used when normal operations
return EBADFILE (error code 105).
* **-F  filename** 

Change the file name for the selected specs to the supplied file name.
* **-T  otypeid** 

Change the type of the selected specs to the given otype.
* **-Z** 

Remove the selected specs entirely.

## EXAMPLES


* **** 

ttdbck`-bxi`/home

In the /home/TT_DB directory, finds all badly formed specs and specs that
refer to non-existent files and prints their ids.
* **** 

ttdbck`-f`/home/sample/data`-F`/home/sample/data1/home

In the /home/TT_DB directory, finds all specs that refer to objects in
file /home/sample/data and changes them to refer to /home/sample/data1.
* **** 

ttdbck-t`Sample_Otype_Name``-Z`/export/TT_DB

In the /export/TT_DB directory, finds all specs that refer to objects
of typeSample_Otype_Nameand deletes the specs.

## FILES


* **/path/TT_DB** 

ToolTalk database

## NOTES


Thettdbckcommand should be run on the same machine where the TT_DB files
being inspected and repaired physically exist. That is, don't try to access
the TT_DB files via NFS.