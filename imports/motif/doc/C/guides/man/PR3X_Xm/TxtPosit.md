# XmTextPosition
library callXmTextPositionData type for a character position within a text stringXmTextPositiondata typesXmTextPosition&npzwc;#include &lt;Xm/Xm.h>
## DESCRIPTION


XmTextPositionis an integer data type that holds a character's position within
a text string for
Text and TextField.

AnXmTextPositionvalue conceptually points to the gap
between two characters.
For example, consider a text string consisting of`N`characters.
A value of 0 refers to the position immediately prior to the first character.
A value of 1 refers to the position in between the first and second characters.
A value of`N`refers to the position immediately following the last
character. Therefore, the text string of`N`characters
actually contains`N + 1`positions.
## RELATED


&cdeman.XmText;.