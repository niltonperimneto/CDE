# XmTranslateKey
library call`XmTranslateKey`The default keycode-to-keysym translatorXmTranslateKey#include <Xm/Xm.h>void`XmTranslateKey`Display *displayKeyCodekeycodeModifiersmodifiersModifiers *modifiers_returnKeySym *keysym_return
## DESCRIPTION


`XmTranslateKey`is the default`XtKeyProc`translation
procedure for Motif applications. The function takes a keycode
and modifiers and returns the corresponding keysym.

`XmTranslateKey`serves two main purposes: to enable new translators
with expanded functionality to get the default
Motif keycode-to-keysym translation in addition to whatever they add,
and to reinstall the default translator. This function enables
keysyms defined by the Motif virtual bindings to be used when an
application requires its own XtKeyProc to be installed.

* **`display`** 

Specifies the display that the keycode is from
* **`keycode`** 

Specifies the keycode to translate
* **`modifiers`** 

Specifies the modifier keys to be applied to the keycode
* **`modifiers_return`** 

Specifies a mask of the modifier keys actually used to generate
the keysym (an AND of`modifiers`and any default modifiers
applied by the currently registered translator)
* **`keysym_return`** 

Specifies a pointer to the resulting keysym

## RELATED


&cdeman.VirtualBindings;.