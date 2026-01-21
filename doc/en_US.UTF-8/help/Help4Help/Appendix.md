
# Help Manager Window with Help Families


A help family includes one or more help volumes. You can select and view
individual help volumes from Help Manager.

The Help Manager window contains an additional navigation button
called Top Level. After browsing different help volumes,
you can use Top Level to return to the Help Manager's main screen.
# See Also



# Regular Expression Pattern Search


* **Character** 

Meaning
* **&sigspace;. (period)** 

Matches any character
* **&sigspace;* (asterisk)** 

Matches 0 or more of the preceding character
* **&sigspace;? (question mark)&sigspace;** 

Matches 0 or 1 of the preceding character
* **&sigspace;| (vertical bar)** 

Specifies two search patterns and matches
either pattern (logical OR)
* **&sigspace;() (parantheses)** 

Encloses a pattern expression


To search for a character that has special meaning in a regular
expression precede the character with a &newline; (backslash).

&sigspace;
# Examples


This expression finds index entries that contain the word "mouse"
followed by any number of characters followed by "clicking".mouse.*clicking

This expression finds index entries that contain the
word "mouse" or "click".mouse | click

This example finds index entries that contain "Session Manager" or "Style Manager".(session | style).*manager
# See Also




For more information about regular expressions, refer to theregexp(5)man page