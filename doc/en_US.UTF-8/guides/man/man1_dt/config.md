# dtconfig
user cmddtconfigdesktop configuration utilitydtconfig-d-e-kill-reset-p
## DESCRIPTION


Desktop configuration utility. Integrates CDE with the
operating system of the underlying platform. System root login privilege is
required to usedtconfig.
## OPTIONS


* **-d** 

Disables desktop auto-start feature. At end of boot cycle, platform's native
text based login mechanism will be used.
* **-e** 

Enable's desktop auto-start feature. Desktop login window will display at end
of platform's boot cycle.
* **-kill** 

Kill desktop (window based) login process and any user sessions associated with
it. Return control to system's native text based console.
* **-reset** 

Tell desktop (window based) login process to reread its configuration files
to incorporate any changes.
* **-p** 

Printer actions for any printer known to platform will be created if such print
actions do not already exist in the platform's actions database. This option
is executed automatically at boot time if desktop auto-start has been enabled.

## RETURN


* **0** 

Successful completion
* **>0** 

Error condition

## FILES


* **/usr/dt/bin/dtconfig** 

location of dtconfig utility

## SEE ALSO


&cdeman.dtlogin;, &cdeman.dtprintinfo;
## NOTES


Thedtconfigscript is an optional utility in the CDE.
It may not be present on all platforms offering this desktop. In such cases,
see platform specific documentation for further information. Alternate
mechanisms may have been supplied. Alldtconfigoptions may not be present on all platforms. Runningdtconfigwithout options will list available options for the platform.