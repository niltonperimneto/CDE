# dtsearchpath
user cmddtsearchpathset desktop search pathsdtsearchpath-uusername-v-o-csh-ksh
## DESCRIPTION


Thedtsearchpathcommand line utility
sets local and remote locations that desktop components should
search for Application Manager groups, for filetype and action
definitions,
for desktop icons, and for help files.

The/usr/dt/bin/Xsessionstartup script sources
the user's$HOME/.dtprofilescript and any scripts located under/etc/dt/config/Xsession.dand/usr/dt/config/Xsession.d.
After sourcing theXsession.dfiles, theXsessionscript will invoke/usr/dt/bin/dtsearchpathto
set and export the environment variables`DTAPPSEARCHPATH`,`DTDATABASESEARCHPATH`,DTHELPSEARCHPATH,`XMICONSEARCHPATH`, andXMICONBMSEARCHPATH(see the`ENVIRONMENT`heading in this man page for a more detailed description).
### Search Path Syntax


Each of the exported search path variables is supported by, that is,
built from, two comma-separated search path environment variables.
For example:`DTAPPSEARCHPATH`is supported byDTSPSYSAPPHOSTSandDTSPUSERAPPHOSTS.`XMICONSEARCHPATH`is supported byDTSPSYSICONandDTSPUSERICON.

The input search path environment variables fall into two categories -
those that support the host:/path syntax and those that do not.
If the name of
the input environment variable contains the stringHOSTS, it will
support host:/path syntax.
The syntax for search paths that support
host:/path is:search path element
[host`|`pathname`|`hostname+pathname]
host
<hostname>:
pathname
/<absolute path>

The syntax for search paths that do not support host:/path is:search path element
[`pathname`]
pathname
/<absolute path>
### Examples


To set the Icon search path to include the/usr/local/games/iconssubdirectory,
the following line would appear
in a script in the/etc/dt/config/Xsession.d/subdirectory:`DTSPSYSICON=/usr/local/games/icons`To set the Database search path to include host`marlin`,
the following line would appear
in a script in the/etc/dt/config/Xsession.d/subdirectory:`DTSPSYSDATABASEHOSTS=marlin:`To set the Application Manager path to include the applications on host
steelhead under subdirectory/usr/local, and under the default
help location on host`halibut`,
the following line would appear
in a script in the/etc/dt/config/Xsession.d/subdirectory:`DTSPSYSAPPHOSTS=steelhead:/usr/local,halibut:`Thedtsearchpathcommand line utility
parses these path elements and transforms them into a form suitable
for the desktop components.

If a`host`element is included,
then the Tooltalk library's filename mapping capabilities will
control how
a path to files on that host are constructed.
For example, the path to file/tmp/fileon host`goby`may be constructed
as/net/goby/tmp/fileor as/nfs/goby/tmp/file.
### Default Locations


By default,dtsearchpathsets up three search locations for each subsystem.
In search
order, they are:

The user's personal location, under$HOME/.dt.

The system administrator's configuration location, under/etc/dt/appconfig.

The factory default location, under/usr/dt/appconfig.
### Precedence


When searching a path for a particular item, such as an icon file, the
desktop will always be satisfied by the first item found.
For example,
when searching for an icon whose basename is`beeper`,
if that icon exists in both the/etc/dt/appconfig/iconsand the/usr/dt/appconfig/iconssubdirectories,
then the icon under/etc/dt/appconfig/iconswill be found first and used, because that
element appears first in the`XMICONSEARCHPATH`.
The search terminates when the first match occurs.

Each of theDTSPSYSsearch path environment variables has a
correspondingDTSPUSERenvironment variable which will take
precedence over the system setting.
TheDTSPUSERvalues will
be prepended to the`DT`search path whereas theDTSPSYSvalues will come after the$HOME/.dtconfiguration location but before the factory default location.
The relationship between the system administrator's customization and the
default/etc/dtconfiguration location is up to the user of theDTSPSYSvalue.
## OPTIONS


* **`-u user`** 

Causesdtsearchpathto return the search paths for the specified user.
This option is useful
for system administrators who need to understand the search paths for
a particular user.
* **`-v`** 

The verbose option causesdtsearchpathto print to standard output the values of the search environment
formatted for easier human reading. By default, the command runs silently.
* **`-csh`** 

Causesdtsearchpathto return values suitable for
evaluation by a C Shell script. By default, the command returns values suitable for
evaluation by a Bourne Shell script.
* **`-ksh`** 

Causesdtsearchpathto return values suitable fore
evaluation by a Korn Shell script. By default, the command returns values suitable for
evaluation by a Bourne Shell script.

## RETURN VALUES


* **0** 

Command completed successfully.
* **1** 

Command invoked with incorrect usage.

## ENVIRONMENT


The values set bydtsearchpathare passed through to the individual desktop subsystems, such as
the action
database and the help system, by means of environment variables.

TheDTSPSYSAPPHOSTSvariable is the primary environment
variable and drives the remaining search paths for filetypes and
actions, for desktop icons, and for help files.

In the following list, all values are derived from three places: the
user's$HOMEdirectory ($HOME/.dt),
the system configuration directory (/etc/dt/appconfig),
and the factory defaults directory (/usr/dt/appconfig).
The values are listed as transformed for the individual desktop
subsystems.

To verify these values, executedtsearchpathwith the verbose (`-v`) option.`DTAPPSEARCHPATH`,DTSPSYSAPPHOSTS,DTSPUSERAPPHOSTS: Used to discover application hosts and their registry subdirectories.
The default transformed value is:$HOME/.dt/appmanager
/etc/dt/appconfig/appmanager/%L
/etc/dt/appconfig/appmanager/C
/usr/dt/appconfig/appmanager/%L
/usr/dt/appconfig/appmanager/CWhere`%L`is the value of the`LANG`environment variable.

The value of`DTAPPSEARCHPATH`can be altered by either of two
environment variables:DTSPSYSAPPHOSTSand/orDTSPUSERAPPHOSTS.DTSPSYSAPPHOSTSis for use by system administrators.
Values are
specified by adding a ksh script to the/etc/dt/config/Xsession.ddirectory that exports the variable.
export DTSPSYSAPPHOSTS=marlin:,goby:/vipapps,/opt
The expected syntax forDTSPSYSAPPHOSTSis a comma-separated list.DTSPSYSAPPHOSTSaccepts host:, host:/path, or just /path.
The effect
is to insert a searchpath after the configuration location
(/etc/dt/appconfig/appmanager) and before the factory default location
(/usr/dt/appconfig/appmanager).$HOME/.dt/appmanager
/etc/dt/appconfig/appmanager/%L
/etc/dt/appconfig/appmanager/C
/nfs/marlin/etc/dt/appconfig/appmanager/%L
/nfs/marlin/etc/dt/appconfig/appmanager/C
/nfs/goby/vipapps/appconfig/appmanager/%L
/nfs/goby/vipapps/appconfig/appmanager/C
/opt/appconfig/appmanager/%L
/opt/appconfig/appmanager/C
/usr/dt/appconfig/appmanager/%L
/usr/dt/appconfig/appmanager/CIf the system administrator wants the local configuration directory to
appear in a different location within the configurable search paths,
the special host term 'localhost:' can be inserted anywhere into the
environment variable:
export DTSPSYSAPPHOSTS=marlin:,localhost:,goby:/vipapps,/opt
The resulting value for`DTAPPSEARCHPATH`would be:$HOME/.dt/appmanager
/nfs/marlin/etc/dt/appconfig/appmanager/%L
/nfs/marlin/etc/dt/appconfig/appmanager/C
/etc/dt/appconfig/appmanager/%L
/etc/dt/appconfig/appmanager/C
/nfs/goby/vipapps/appconfig/appmanager/%L
/nfs/goby/vipapps/appconfig/appmanager/C
/opt/appconfig/appmanager/%L
/opt/appconfig/appmanager/C
/usr/dt/appconfig/appmanager/%L
/usr/dt/appconfig/appmanager/CIn fact, the value 'localhost:' can be inserted anywhere in theDTSPSYSAPPHOSTSvalue and its order within theDTSPSYSAPPHOSTSwill be reflected in the`DTAPPSEARCHPATH`value.DTSPUSERAPPHOSTSis for use by end users.
Values are specified by exporting
the value in the user's .dtprofile.
export DTSPUSERAPPHOSTS=appsvr:,/myappsDTSPUSERAPPHOSTSalso accepts host:, host:/path, and /path
specifications.
The effect is to insert a searchpath before any other
searchpath./nfs/appsvr/etc/dt/appconfig/appmanager/%L
/nfs/appsvr/etc/dt/appconfig/appmanager/C
/myapps/appmanager/%L
/myapps/appmanager/C
$HOME/.dt/appmanager
/etc/dt/appconfig/appmanager/%L
/etc/dt/appconfig/appmanager/C
/usr/dt/appconfig/appmanager/%L
/usr/dt/appconfig/appmanager/C`DTDATABASESEARCHPATH`,DTSPSYSDATABASEHOSTS,DTSPUSERDATABASEHOSTS: Used to collect filetype and action definitions, as expressed in*.dtfiles.
The default transformed value is:$HOME/.dt/types
/etc/dt/appconfig/types/%L
/etc/dt/appconfig/types/C
/usr/dt/appconfig/types/%L
/usr/dt/appconfig/types/CWhere`%L`is the value of the`LANG`environment variable.

The value of`DTDATABASESEARCHPATH`can be altered by either of two
environment variables:DTSPSYSDATABASEHOSTSand/orDTSPUSERDATABASEHOSTS.DTSPSYSDATABASEHOSTSis for use by system administrators.
Values are
specified by adding a ksh script to the/etc/dt/config/Xsession.ddirectory that exports the variable.
export DTSPSYSDATABASEHOSTS=marlin:,goby:/vipapps,/opt
The expected syntax forDTSPSYSDATABASEHOSTSis a comma-separated
list.DTSPSYSDATABASEHOSTSaccepts host:, host:/path, or just /path.
The effect is to insert a searchpath after the configuration location
(/etc/dt/appconfig/types) and before the factory default location
(/usr/dt/appconfig/types).$HOME/.dt/types
/etc/dt/appconfig/types/%L
/etc/dt/appconfig/types/C
marlin:/etc/dt/appconfig/types/%L
marlin:/etc/dt/appconfig/types/C
goby:/vipapps/appconfig/types/%L
goby:/vipapps/appconfig/types/C
/opt/appconfig/types/%L
/opt/appconfig/types/C
/usr/dt/appconfig/types/%L
/usr/dt/appconfig/types/CAs is the case for`DTAPPSEARCHPATH`, the placement of the local configuration
directory can be affected by the adding special host term 'localhost:'
to theDTSPSYSDATABASEHOSTSenvironment variable.
export DTSPSYSDATABASEHOSTS=marlin:,localhost:,goby:/vipapps
The resulting value for`DTDATABASESEARCHPATH`would be:$HOME/.dt/types
marlin:/etc/dt/appconfig/types/%L
marlin:/etc/dt/appconfig/types/C
/etc/dt/appconfig/types/%L
/etc/dt/appconfig/types/C
goby:/vipapps/appconfig/types/%L
goby:/vipapps/appconfig/types/C
/usr/dt/appconfig/types/%L
/usr/dt/appconfig/types/C

DTSPUSERDATABASEHOSTSis for use by end users.
Values are specified by
exporting the value in the user's .dtprofile.

export DTSPUSERDATABASEHOSTS=dbsvr:,/mytypesDTSPUSERDATABASEHOSTSalso accepts host:, host:/path, and /path
specifications.
The effect is to insert a searchpath before any other
searchpath.dbsvr:/etc/dt/appconfig/types/%L
dbsvr:/etc/dt/appconfig/types/C
/mytypes/types/%L
/mytypes/types/C
$HOME/.dt/types
/etc/dt/appconfig/types/%L
/etc/dt/appconfig/types/C
/usr/dt/appconfig/types/%L
/usr/dt/appconfig/types/C`XMICONSEARCHPATH`,XMICONBMSEARCHPATH,DTSPSYSICON,DTSPUSERICON: Used to locate desktop icons.
The default transformed value is:$HOME/.dt/icons/%B%M.pm
$HOME/.dt/icons/%B%M.bm
$HOME/.dt/icons/%B
/etc/dt/appconfig/icons/%L/%B%M.pm
/etc/dt/appconfig/icons/%L/%B%M.bm
/etc/dt/appconfig/icons/%L/%B
/etc/dt/appconfig/icons/C/%B%M.pm
/etc/dt/appconfig/icons/C/%B%M.bm
/etc/dt/appconfig/icons/C/%B
/usr/dt/appconfig/icons/%L/%B%M.pm
/usr/dt/appconfig/icons/%L/%B%M.bm
/usr/dt/appconfig/icons/%L/%B
/usr/dt/appconfig/icons/C/%B%M.pm
/usr/dt/appconfig/icons/C/%B%M.bm
/usr/dt/appconfig/icons/C/%B

Where`%B`is the basename of the requested icon,`%M`is the magnitude
(size) of the icon, and`%L`is the value of the`LANG`environment variable.

The value of`XMICONSEARCHPATH`can be altered by either of two
environment variables:DTSPSYSICONand/orDTSPUSERICON.DTSPSYSICONis for use by system administrators.
Values are
specified by adding a ksh script to the/etc/dt/config/Xsession.ddirectory that exports the variable.
export DTSPSYSICON=/vipapps/icons

The expected syntax forDTSPSYSICONis a comma-separated list.DTSPSYSICONaccepts the /path format.
The effect
is to insert a searchpath after the configuration location
(/etc/dt/appconfig/icons) and before the factory default location
(/usr/dt/appconfig/icons).$HOME/.dt/icons/%B%M.pm
$HOME/.dt/icons/%B%M.bm
$HOME/.dt/icons/%B%M
/etc/dt/appconfig/icons/%L/%B%M.pm
/etc/dt/appconfig/icons/%L/%B%M.bm
/etc/dt/appconfig/icons/%L/%B%M
/etc/dt/appconfig/icons/C/%B%M.pm
/etc/dt/appconfig/icons/C/%B%M.bm
/etc/dt/appconfig/icons/C/%B%M
/vipapps/icons/%L/%B%M.pm
/vipapps/icons/%L/%B%M.bm
/vipapps/icons/%L/%B%M
/vipapps/icons/C/%B%M.pm
/vipapps/icons/C/%B%M.bm
/vipapps/icons/C/%B%M
usr/dt/appconfig/icons/%L/%B%M.pm
/usr/dt/appconfig/icons/%L/%B%M.bm
/usr/dt/appconfig/icons/%L/%B%M
/usr/dt/appconfig/types/C/%B%M.pm
/usr/dt/appconfig/types/C/%B%M.bm
/usr/dt/appconfig/types/C/%B%M

The placement of the local configuration directory can be affected by
the adding the path term/etc/dt/appconfigto theDTSPSYSICONenvironment variable.
export DTSPSYSICON=/vipapps/icons,/etc/dt/appconfig

The resulting value for`XMICONSEARCHPATH`would be:

$HOME/.dt/icons/%B%M.pm
$HOME/.dt/icons/%B%M.bm
$HOME/.dt/icons/%B%M
/vipapps/icons/%L/%B%M.pm
/vipapps/icons/%L/%B%M.bm
/vipapps/icons/%L/%B%M
/vipapps/icons/C/%B%M.pm
/vipapps/icons/C/%B%M.bm
/vipapps/icons/C/%B%M
/etc/dt/appconfig/icons/%L/%B%M.pm
/etc/dt/appconfig/icons/%L/%B%M.bm
/etc/dt/appconfig/icons/%L/%B%M
/etc/dt/appconfig/icons/C/%B%M.pm
/etc/dt/appconfig/icons/C/%B%M.bm
/etc/dt/appconfig/icons/C/%B%M
usr/dt/appconfig/icons/%L/%B%M.pm
/usr/dt/appconfig/icons/%L/%B%M.bm
/usr/dt/appconfig/icons/%L/%B%M
/usr/dt/appconfig/types/C/%B%M.pm
/usr/dt/appconfig/types/C/%B%M.bm
/usr/dt/appconfig/types/C/%B%MDTSPUSERICONis for use by end users.
Values are specified by
exporting the value in the user's .dtprofile.

export DTSPUSERICON=$HOME/myicons

DTSPUSERICONaccepts /path specifications.
The effect is to insert a
searchpath before any other searchpath.

$HOME/myicons/%B%M.pm
$HOME/myicons/%B%M.bm
$HOME/myicons/%B%M
$HOME/.dt/icons/%B%M.pm
$HOME/.dt/icons/%B%M.bm
$HOME/.dt/icons/%B%M
/etc/dt/appconfig/icons/%L/%B%M.pm
/etc/dt/appconfig/icons/%L/%B%M.bm
/etc/dt/appconfig/icons/%L/%B%M
/etc/dt/appconfig/icons/C/%B%M.pm
/etc/dt/appconfig/icons/C/%B%M.bm
/etc/dt/appconfig/icons/C/%B%M
/usr/dt/appconfig/icons/%L/%B%M.pm
/usr/dt/appconfig/icons/%L/%B%M.bm
/usr/dt/appconfig/icons/%L/%B%M
/usr/dt/appconfig/types/C/%B%M.pm
/usr/dt/appconfig/types/C/%B%M.bm
/usr/dt/appconfig/types/C/%B%MDTHELPSEARCHPATH,DTSPSYSHELP,DTSPUSERHELP:

Used to locate help volumes and families for the desktop help
subsystem.
The default transformed value is:$HOME/.dt/help/<session>/%H
$HOME/.dt/help/<session>/%H.sdl
$HOME/.dt/help/<session>/%H.hv
$HOME/.dt/help/%H
$HOME/.dt/help/%H.sdl
$HOME/.dt/help/%H.hv
/etc/dt/appconfig/help/%L/%H
/etc/dt/appconfig/help/%L/%H.sdl
/etc/dt/appconfig/help/%L/%H.hv
/etc/dt/appconfig/help/C/%H
/etc/dt/appconfig/help/C/%H.sdl
/etc/dt/appconfig/help/C/%H.hv
/usr/dt/appconfig/help/%L/%H
/usr/dt/appconfig/help/%L/%H.sdl
/usr/dt/appconfig/help/%L/%H.hv
/usr/dt/appconfig/help/C/%H
/usr/dt/appconfig/help/C/%H.sdl
/usr/dt/appconfig/help/C/%H.hv

Where`%H`is the basename of the requested help volume, and`%L`is the
value of the`LANG`environment variable.

The value ofDTHELPSEARCHPATHcan be altered by either of two
environment variables:DTSPSYSHELPand/orDTSPUSERHELP.DTSPSYSHELPis for use by system administrators.
Values are
specified by adding a ksh script to the/etc/dt/config/Xsession.ddirectory that exports the variable.

export DTSPSYSHELP=/vipapps/help

The expected syntax forDTSPSYSHELPis a comma-separated list.DTSPSYSHELPaccepts the /path format.
The effect
is to insert a searchpath after the configuration location
(/etc/dt/appconfig/help) and before the factory default location
(/usr/dt/appconfig/help).$HOME/.dt/help/<session>/%H
$HOME/.dt/help/<session>/%H.sdl
$HOME/.dt/help/<session>/%H.hv
$HOME/.dt/help/%H
$HOME/.dt/help/%H.sdl
$HOME/.dt/help/%H.hv
/etc/dt/appconfig/help/%L/%H
/etc/dt/appconfig/help/%L/%H.sdl
/etc/dt/appconfig/help/%L/%H.hv
/etc/dt/appconfig/help/C/%H
/etc/dt/appconfig/help/C/%H.sdl
/etc/dt/appconfig/help/C/%H.hv
/vipapps/help/%L/%H
/vipapps/help/%L/%H.sdl
/vipapps/help/%L/%H.hv
/vipapps/help/C/%H
/vipapps/help/C/%H.sdl
/vipapps/help/C/%H.hv
/usr/dt/appconfig/help/%L/%H
/usr/dt/appconfig/help/%L/%H.sdl
/usr/dt/appconfig/help/%L/%H.hv
/usr/dt/appconfig/help/C/%H
/usr/dt/appconfig/help/C/%H.sdl
/usr/dt/appconfig/help/C/%H.hvThe placement of the local configuration directory can be affected by
the adding the path term/etc/dt/appconfigto theDTSPSYSHELPenvironment variable.

export DTSPSYSHELP=/vipapps/help,/etc/dt/appconfig

The resulting value forDTHELPSEARCHPATHwould be:$HOME/.dt/help/<session>/%H
$HOME/.dt/help/<session>/%H.sdl
$HOME/.dt/help/<session>/%H.hv
$HOME/.dt/help/%H
$HOME/.dt/help/%H.sdl
$HOME/.dt/help/%H.hv
/vipapps/help/%L/%H
/vipapps/help/%L/%H.sdl
/vipapps/help/%L/%H.hv
/vipapps/help/C/%H
/vipapps/help/C/%H.sdl
/vipapps/help/C/%H.hv
/etc/dt/appconfig/help/%L/%H
/etc/dt/appconfig/help/%L/%H.sdl
/etc/dt/appconfig/help/%L/%H.hv
/etc/dt/appconfig/help/C/%H
/etc/dt/appconfig/help/C/%H.sdl
/etc/dt/appconfig/help/C/%H.hv
/usr/dt/appconfig/help/%L/%H
/usr/dt/appconfig/help/%L/%H.sdl
/usr/dt/appconfig/help/%L/%H.hv
/usr/dt/appconfig/help/C/%H
/usr/dt/appconfig/help/C/%H.sdl
/usr/dt/appconfig/help/C/%H.hvDTSPUSERHELPis for use by end users.
Values are specified by
exporting the value in the user's .dtprofile.

export DTSPUSERHELP=$HOME/myhelp

DTSPUSERHELPaccepts /path specifications.
The effect is to insert a
searchpath before any other searchpath.$HOME/myhelp/%H
$HOME/myhelp/%H.sdl
$HOME/myhelp/%H.hv
$HOME/.dt/help/<session>/%H
$HOME/.dt/help/<session>/%H.sdl
$HOME/.dt/help/<session>/%H.hv
$HOME/.dt/help/%H
$HOME/.dt/help/%H.sdl
$HOME/.dt/help/%H.hv
/etc/dt/appconfig/help/%L/%H
/etc/dt/appconfig/help/%L/%H.sdl
/etc/dt/appconfig/help/%L/%H.hv
/etc/dt/appconfig/help/C/%H
/etc/dt/appconfig/help/C/%H.sdl
z.br
/etc/dt/appconfig/help/C/%H.hv
/usr/dt/appconfig/help/%L/%H
/usr/dt/appconfig/help/%L/%H.sdl
/usr/dt/appconfig/help/%L/%H.hv
/usr/dt/appconfig/help/C/%H
/usr/dt/appconfig/help/C/%H.sdl
/usr/dt/appconfig/help/C/%H.hv

(See also the`OPTIMIZATIONS`heading in this man page.)

At the conclusion ofdtsearchpath,DTSPSYSandDTSPUSERvariables are explicitly unset, so that only theDT*SEARCHPATHvalues remain.

MANPATH,DTMANPATH:

In addition to setting the application search paths,dtsearchpathaugments theMANPATHenvironment variable with the path to the
CDE man pages,/usr/dt/man.For example, if the value ofMANPATHprior to execution of:dtsearchpathis/net/manserver/usr/man:/usr/manthen the augmented value will be:/usr/dt/man:/net/manserver/usr/man:/usr/man.

If theMANPATHenvironment variable is not set prior to the
invocation ofdtsearchpath, the system-defined default value ofMANPATHwill be included in the
augmentedMANPATHvalue.
For example, if the system-defined
default value is:/usr/man:/usr/local/manthendtsearchpathwill generate aMANPATHvalue of:/usr/dt/man:/usr/man:/usr/local/man

DTSROCFPATH:DTSROCFPATHis used
by theDtSearchInitfunction to locate the default API
configurationocffile. If theocf_fileargument is NULL,DtSearchInitlooks for anocffile with a base name of eitherdtsearch.ocforaustext.ocfin the directory specified byDTSROCFPATH, in the current working directory,
or in theHOMEdirectory, in that order.

DTINFOLIBSEARCHPATH,DTINFOLIBDEFAULT:DTINFOLIBSEARCHPATHis used
bydtinfoto locate information libraries on local
and remote mounted systems.DTINFOLIBDEFAULTis used bydtinfoto identify the default information library(s)
to load if the-lor-sectoption
is not specified.

DTINFOLIBSEARCHPATHandDTINFOLIBDEFAULTare defined at installation time bydtsearchpath.The default infolib search path includes personal, system-wide, and
built-in locations as follows:System-wide location/etc/dt/infolib/language/%I.dtiBuilt-in location/usr/dt/appconfig/infolib/language/%I.dtiThe default language is C.When a location is added to the application search path, the appropriate
infolib subdirectory is automatically added to the infolib search path.For example, if the application serverhosta:is
added to the application search path, the directoryhosta:/etc/dt/appconfig/infolib/languageis
automatically added to the infolib search path.The infolib search path is assembled from the built-in locations and the
following input variables:DTSPSYSINFOLIBSystem-wide infolib search path input variableDTSPUSERINFOLIBPersonal infolib search path input variableUse these input variables to specify locations outside the application
search path.The assembled database search path is specified by the output variableDTINFOLIBSEARCHPATH.The syntax for theDTSPSYSINFOLIBandDTSPUSERINFOLIBvariables is:VARIABLE =location[,location]wherelocationis the pathname for a directory on the
local (session server) system. Use this syntax to add a local directory.To specify a location on another system, use its network file name.
For example:/nfs/servera/projects/infolib.The value of the infolib search path (DTINFOLIBSEARCHPATH) is created by
assembling the following locations, listed in order of precedence:Locations specified using theDTSPUSERINFOLIBvariableLocations derived from theDTSPUSERAPPHOSTSvariableThe default location:/etc/dt/appconfig/infolib/language/%I.dtiLocations specified using theDTSPSYSINFOLIBvariableLocations derived from theDTSPSYSAPPHOSTSvariable/usr/dt/appconfig/infolib/language/%I.dti
## LOCALES


Each of the search path elements contain a path that references the`LANG`variable using the`%L`construct.
When the user
selects a different language from the greeting screen and logs in, the
search path will already be set up.
At the same time, the
factory defaults are still included, but after the localized elements.

For example, if the user chooses
the German locale
from the greeting screen, then his/her`DTDATABASESEARCHPATH`will already include these elements, as specified by the`LANG`value
set at session startup:$HOME/.dt/types
/etc/dt/appconfig/types/%L
/etc/dt/appconfig/types/C
/usr/dt/appconfig/types/%L
/usr/dt/appconfig/types/C

(See also the`OPTIMIZATIONS`heading in this man page.)

Note that the search path does not use locales under the user's$HOMEdirectory.
Whether the user adds personal icons under$HOME/.dt/iconsor personal applications under$HOME/.dt/appmanager, they will be found regardless of the language
selected at login.
## OPTIMIZATIONS


Before exporting the search paths to the user's environment,dtsearchpathfirst checks to ensure that each subdirectory exists.
If a directory
does not exist at login, then that element will not be added to the
user's search path, to save needless file system accesses by the
desktop components.
## FILES


* **$HOME/.dtprofile** 

Enables setting of the user'sDTSPUSERenvironment variables.
* **Xsession.d/*** 

Enables setting of the systemDTSPSYSenvironment variables.

## NOTES


In order to inject the values fromdtsearchpathinto the user's environment, the command must beeval'd,
as is done by theXsessionlogin script.

eval `/usr/dt/bin/dtsearchpath`
Simply runningdtsearchpathfrom the command line will have no affect on the parent shell.

It is not possible to affect the DT search paths after logging in.
Components such as the Window Manager and File Manager inherit the
values fromdtsearchpathby being invoked from the same shell.
Hence, if the system
administrator creates a new search path element for the end user,
the user will not be able to access it until the next login.
## SEE ALSO


&cdeman.dtappgather;.