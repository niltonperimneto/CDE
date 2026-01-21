
# Finestra della Gestione dell'aiuto con le famiglie di aiuto


Ogni famiglia comprende uno o più volumi di aiuto. È possibile selezionare e visualizzare i singoli volumi dalla Gestione dell'aiuto.

La finestra della Gestione dell'aiuto contiene un pulsante di spostamento denominato Livello più elevato, che permette di tornare allo schermo principale dopo avere visualizzato altri volumi di aiuto.
# Vedere anche



# Ricerca con espressioni regolari


* **Carattere** 

Significato
* **&sigspace;. (punto)** 

Corrisponde a qualsiasi carattere
* **&sigspace; * (asterisco)** 

Corrisponde a 0 o più ricorrenze del carattere precedente
* **&sigspace; ? (punto interrogativo)&sigspace;** 

Corrisponde a 0 o 1 ricorrenze del carattere precedente
* **&sigspace; | (barra verticale)** 

Specifica due schemi di ricerca e corrisponde ad entrambi (OR logico)
* **&sigspace; () (parentesi)** 

Racchiude un'espressione utilizzata come schema


Per ricercare un carattere che ha un significato speciale nelle espressioni regolari, far precedere il carattere da una barra rovesciata (&newline;).

&sigspace;
# Esempi


L'espressione seguente ricerca le voci dell'indice che contengono il termine "mouse" seguito da un numero qualsiasi di caratteri e quindi da "clic".mouse.*clic

L'espressione seguente ricerca le voci dell'indice che contengono il termine "mouse" o "clic".mouse | clic

L'esempio seguente ricerca le voci dell'indice che contengono il termine "Gestione delle sessioni" o "Gestione degli stili".gestione*.(delle sessioni | degli stili)
# Vedere anche




Per maggiori informazioni sulle espressioni regolari, vedere la pagina di spiegazioni relativa aregexp(5).