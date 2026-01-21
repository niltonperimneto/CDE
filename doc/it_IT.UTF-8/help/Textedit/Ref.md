
# Riferimenti sull'Editor di testo

# Tasti di editazione e di spostamento dell'Editor di testo





# Menu dell'Editor di testo















# Finestre e riquadri di dialogo dell'Editor di testo





















# Riferimenti generali







# Vedere anche



# Combinazioni di accesso rapido per i comandi dell'Editor &newline; di testo
menu: combinazioni di accesso rapido

* **Chiudere** 

Alt+F4
* **Copiare** 

Control+C
* **Tagliare** 

Control+X
* **Cancellare** 

Tasto Delete
* **Ricercare/sostituire** 

Control+F
* **Sovrascrivere** 

Tasto Insert
* **Incollare** 

Control+V
* **Stampare** 

Control+P
* **Selezionare tutto** 

Control+/
* **Annullare** 

Control+Z


Se la tastiera utilizzata non dispone del tasto Alt, chiedere informazioni all'amministratore di sistema sul tasto da utilizzare.
# Tasti di editazione e di spostamento del cursore
editazione, combinazioni di tastiControl, tastoAlt, tasto
# Tasti di editazione


* **Tasto** 

Effetto
* **Backspace** 

Cancella il carattere a sinistra del cursore
* **Delete** 

Cancella il carattere a destra del cursore
* **Control+Delete** 

Cancella tutti i caratteri dal cursore alla fine della riga
* **Control+Backspace** 

Cancella la parola precedente
* **Shift+Backspace** 

Cancella i caratteri dal cursore all'inizio della riga

# Tasti di spostamento del cursore
Control, tasto: uso con i tasti frecciatasti frecciacursore, spostamento con la tastieratastiera, uso per lo spostamento del cursoretasti: uso per lo spostamento del cursore

* **Tasto** 

Spostamento del cursore
* **Freccia su** 

In alto di una riga
* **Freccia giù** 

In basso di una riga
* **Freccia a sinistra** 

A sinistra di un carattere
* **Freccia a destra** 

A destra di un carattere
* **Control+Freccia&sigspace;a&sigspace;destra&sigspace;** 

A destra di una parola
* **Control+Freccia&sigspace;a&sigspace;sinistra** 

A sinistra di una parola
* **Control+Freccia&sigspace;giù&sigspace;** 

All'inizio del paragrafo successivo
* **Control+Freccia&sigspace;su** 

All'inizio del paragrafo precedente
* **Home** 

All'inizio della riga corrente
* **End** 

Alla fine della riga corrente
* **Control+Home** 

All'inizio del documento
* **Control+End** 

Alla fine del documento


Alcune combinazioni di tasti possono variare a seconda del sistema in uso. In questo caso, chiedere all'amministratore di sistema di identificare i tasti da utilizzare.

&sigspace;

&sigspace;

Per usare le associazioni dei tasti di Emacs nell'Editor di testo, vedere.

&sigspace;
# Vedere anche











# Associazioni dei tasti di Unix
Unix, associazioni di tastitasti: associazioni UnixEmacs, associazioni di tasti

Le associazioni dei tasti di Unix permettono di usare nell'Editor di testo numerose combinazioni di Emacs, come Control+F (avanzamento di un carattere) e Control+N (avanzamento alla riga successiva). Per abilitare queste associazioni (che nell'impostazione predefinita sono disabilitate), procedere come segue:

Editare il file.Xdefaultsdella propria directory iniziale aggiungendo la riga seguente:#include "/usr/dt/app-defaults/`lingua`/UNIXbindings"

Sostituire`lingua`con il valore della variabile d'ambiente per la lingua utilizzata sul sistema. Se il file.Xdefaultsnon esiste, crearlo nella propria directory iniziale.

Eseguire il logout dalla sessione corrente.

Eseguire il login e riavviare l'Editor di testo.

&sigspace;

Quando si utilizzano le associazioni dei tasti Unix, l'Editor di testo offre combinazioni di accesso rapido alternative per i seguenti comandi dei menu:

* **Comando** 

Combinazione di accesso alternativa
* **Annullare (Control+Z)** 

Control+_
* **Incollare (Control+V)** 

Shift+Insert
* **Ricercare/Sostituire (Control+F)&sigspace;** 

Control+S
* **Stampare (Control+P)** 

nessuna alternativa


&sigspace;
Se si desidera modificare queste combinazioni di accesso ai menu, copiare il contenuto del file/usr/dt/app-defaults/`lingua`/UNIXbindingsnel proprio file.Xdefaults, e quindi apportare le modifiche desiderate.

Quando vengono abilitate le associazioni dei tasti di Unix, il tasto Delete cancella il carattere a sinistra anziché il carattere a destra del cursore.
# Vedere anche







# Menu File dell'Editor di testo


menu File

* **Nuovo** 

Azzera la finestra dell'Editor di testo. Se il documento contiene modifiche non salvate,
compare un riquadro di dialogo che consente di salvare il lavoro prima di azzerare la finestra.
* **Aprire** 

Presenta un riquadro di dialogo per l'apertura di un
file esistente.
* **Includere** 

Presenta un riquadro di dialogo per specificare il nome
di un file da inserire nel documento corrente.
* **Salvare** 

Salva il documento nel file corrente. Se era stata abilitata
l'opzione A capo automatico, compare il riquadro di dialogo
Salvare. Se il documento corrente
non era stato salvato in precedenza, compare il riquadro di
dialogo Salvataggio come.
* **Salvare come** 

Presenta un riquadro di dialogo per salvare il documento
in un nuovo file.
* **Copiare su file** 

Copia le informazioni del documento corrente in un file separato lasciando aperta la sessione di editazione. In alcuni casi, il comando Salvare come viene sostituito dal comando Copiare su file.
* **Stampare** 

Apre un riquadro di dialogo per la selezione delle opzioni di stampa e l'avvio della stampa del documento.
* **Chiudere** 

Chiude la finestra dell'Editor di testo ed esce dall'applicazione.

# Vedere anche







# Menu Editare dell'Editor di testo


menu Editare

* **Annullare** 

Annulla l'effetto dell'ultima operazione di editazione, di
inclusione o di formattazione eseguita
* **Tagliare** 

Elimina il testo selezionato dal documento e lo memorizza nel blocco appunti
* **Copiare** 

Copia il testo selezionato e lo memorizza nel blocco appunti
* **Incollare** 

Inserisce il testo memorizzato nel blocco appunti con l'ultimo comando Tagliare o Copiare nella posizione corrente del cursore
* **Ripulire** 

Sostituisce il testo selezionato con spazi vuoti
* **Cancellare** 

Cancella il testo selezionato
* **Selezionare tutto** 

Seleziona tutto il testo del documento
* **Ricercare/sostituire** 

Apre un riquadro di dialogo che permette di ricercare,
e sostituire, parole o frasi nel documento corrente
* **Controllare ortografia&sigspace;** 

Esegue la correzione ortografica del documento

# Vedere anche







# Menu Formato dell'Editor di testo


menu Formato

* **Impostazioni** 

Apre un riquadro di dialogo per l'impostazione dei margini
e dell'allineamento dei paragrafi, ed applica le impostazioni di formato al documento
* **Paragrafo** 

Applica le impostazioni al paragrafo in cui si trova il
cursore
* **Tutto** 

Applica le impostazioni all'intero documento

# Vedere anche







# Menu Opzioni dell'Editor di testo


menu Opzioni

* **Sovrascrivere** 

Abilita e disabilita la modalità di inserimento del testo che permette di sovrascrivere i caratteri preesistenti.
* **A capo automatico** 

Abilita e disabilita la modalità di inserimento del testo che prevede il ritorno a capo automatico delle righe in base ai margini della finestra. Se questa opzione non è abilitata, per andare a capo occorre premere Return alla fine di ogni riga.
* **Riga di stato** 

Abilita e disabilita una riga di stato nel bordo inferiore della finestra. Questa riga riporta il numero della riga in cui si trova il cursore, il numero di righe totali del documento, i messaggi dell'applicazione e l'abilitazione della modalità di sovrascrittura. Offre inoltre la possibilità di spostare il cursore su una determinata riga specificandone il numero.

# Vedere anche













# Menu Aiuto dell'Editor di testo


menu Aiuto

* **Panoramica** 

Descrive le procedure per avviare l'Editor di testo.
* **Indice &sigspace;** 

Presenta un elenco degli argomenti di aiuto disponibili per l'Editor di testo.
* **Attività** 

Contiene istruzioni sulle procedure da seguire per le
principali operazioni con l'Editor di testo.
* **Riferimenti** 

Contiene un sommario delle informazioni di riferimento sulle varie funzioni dell'Editor di testo, come menu, riquadri di dialogo, risorse e opzioni dei comandi.
* **Sull'elemento** 

Trasforma il cursore in un punto interrogativo. Facendo clic con questo puntatore su un elemento della finestra o dei riquadri di dialogo dell'Editor di testo, verrà visualizzata una descrizione specifica di quell'elemento.
* **Uso dell'aiuto** 

Fornisce informazioni sull'uso delle finestre dell'Aiuto.
* **Editor di testo** 

Visualizza la versione e le informazioni di copyright per
l'Editor di testo.

# Vedere anche





# Finestra principale dell'Editor di testo


finestra dell'Editor di testo

&sigspace;

&sigspace;

* **Barra dei menu** 

L'Editor di testo dispone di cinque menu: File, Editare,
Formato, Opzioni e Aiuto.
* **Finestra del documento&sigspace;** 

È l'area in cui viene inserito il testo del documento.
* **Riga di stato** 

Riporta il numero della riga in cui si trova il cursore, il numero di righe totali del documento, i messaggi dell'applicazione e l'abilitazione del modo di sovrascrittura. Offre inoltre la possibilità di spostare il cursore su una determinata riga specificandone il numero.

# Vedere anche







# Riquadro di dialogo Apertura file dell'Editor di testo


&sigspace;

&sigspace;

* **Inserire il percorso o
il nome della cartella&sigspace;** 

&newline; Identifica il percorso della cartella corrente.
* **Filtro** 

Specificando un asterisco (*) vengono visualizzati tutti i file. In alternativa, è possibile usare caratteri speciali per visualizzare solo i file il cui nome corrisponde a un determinato schema. Ad esempio, specificando *.doc verranno presentati solo i file che terminano con .doc.
* **File** 

Presenta l'elenco dei file contenuti nella cartella corrente.
* **Cartelle** 

Presenta le cartelle contenute nella cartella corrente. Il file da aprire può trovarsi nella cartella corrente, in una cartella secondaria o in una cartella differente.
* **Inserire il nome del file** 

Mostra il nome del file selezionato nell'elenco File.
Per aprire il file, premere Return o fare clic su OK. Oppure,
digitare direttamente il nome del file che si desidera aprire.
* **OK** 

Apre il file specificato nel campo Inserire il nome del file.
* **Aggiornare** 

Permette di aggiornare l'elenco dei file dopo avere cambiato il filtro di visualizzazione o essere passati ad un'altra cartella.
* **Annullare** 

Annulla l'operazione di apertura.
* **Aiuto** 

Descrive i campi di testo, gli elenchi di selezione e i pulsanti presenti nel riquadro di dialogo.

# Vedere anche







# Riquadro di dialogo Salvataggio come dell'Editor di testo


&sigspace;

&sigspace;

* **Inserire il percorso o
il nome della cartella&sigspace;** 

&newline; Identifica il percorso della cartella corrente.
* **Filtro** 

Specificando un asterisco (*) vengono visualizzati tutti i file. In alternativa, è possibile usare caratteri speciali per visualizzare solo i file il cui nome corrisponde a un determinato schema. Ad esempio, specificando *.doc verranno presentati solo i file che terminano con .doc.
* **File** 

Presenta l'elenco dei file contenuti nella cartella corrente.
* **Cartelle** 

Presenta le cartelle contenute nella cartella corrente.
* **Inserire il nome del file** 

È il campo in cui occorre specificare il nome da assegnare al file del documento.
* **OK** 

Salva il file usando il nome specificato.
* **Aggiornare** 

Permette di aggiornare l'elenco dei file dopo avere cambiato il filtro di visualizzazione o essere passati ad un'altra cartella.
* **Annullare** 

Annulla l'operazione di salvataggio.


Se era stata attivata l'opzione A capo automatico, il riquadro di dialogo presenterà anche le seguenti opzioni:

&sigspace;

&sigspace;

Aggiungere caratteri di ritorno a capo al termine delle righe.

È l'impostazione predefinita. Aggiunge un carattere di ritorno a capo alla fine di ogni riga spezzata in base alla larghezza della finestra e mantiene i ritorni a capo del documento esattamente come compaiono nella finestra corrente.

Non aggiungere caratteri di ritorno a capo. Verranno conservate solo le interruzioni di riga create con Return.

Questa opzione mantiene le interruzioni di riga "soft" inserite dall'opzione A capo automatico. All'apertura successiva del documento, il testo si disporrà in base alla larghezza della nuova finestra.
# Vedere anche







# Riquadro di dialogo Salvare dell'Editor di testo


&sigspace;
Se è stata attivata l'opzione A capo automatico, scegliendo il comando per salvare il documento verrà visualizzata una richiesta di conferma. I ritorni a capo inseriti dall'opzione A capo automatico possono essere gestiti in due modi:

&sigspace;

&sigspace;

Aggiungere caratteri di ritorno a capo al termine delle righe.

È l'impostazione predefinita. Aggiunge un carattere di ritorno a capo alla fine di ogni riga spezzata in base alla larghezza della finestra e mantiene i ritorni a capo del documento esattamente come compaiono nella finestra corrente.

Non aggiungere caratteri di ritorno a capo. Verranno conservate solo le interruzioni di riga create con Return.

Questa opzione mantiene le interruzioni di riga "soft" inserite dall'opzione A capo automatico. All'apertura successiva del documento, il testo si disporrà in base alla larghezza della nuova finestra.

* **Sì** 

Salva il file corrente o visualizza il riquadro di dialogo
Salvataggio come per salvare il documento
* **No** 

Procede con l'operazione senza salvare il file
* **Annullare** 

Annulla l'operazione
* **Aiuto** 

Descrive il riquadro di conferma del salvataggio


Il riquadro di conferma del salvataggio viene visualizzato anche quando si sceglie un comando che può causare la perdita delle modifiche correnti.

&sigspace;

&sigspace;
# Vedere anche







# Riquadro di dialogo Inclusione file dell'Editor di testo


&sigspace;

&sigspace;

* **Inserire il percorso o
il nome della cartella&sigspace;** 

&newline; Identifica il percorso della cartella corrente.
* **Filtro** 

Specificando un asterisco (*) vengono visualizzati tutti i file. In alternativa, è possibile usare caratteri speciali per visualizzare solo i file il cui nome corrisponde a un determinato schema. Ad esempio, specificando *.doc verranno presentati solo i file che terminano con .doc.
* **File** 

Presenta l'elenco dei file contenuti nella cartella corrente.
* **Cartelle** 

Presenta le cartelle contenute nella cartella corrente.

Il file da includere può trovarsi nella cartella corrente, in una cartella secondaria o in una cartella differente.
* **Inserire il nome del file** 

Mostra il nome del file selezionato nell'elenco File.
Per includere il file, premere Return o fare clic su OK. Oppure,
digitare direttamente il nome del file che si desidera includere.
* **OK** 

Inserisce il file specificato nel campo Inserire il nome del file in corrispondenza del cursore.
* **Aggiornare** 

Permette di aggiornare l'elenco dei file dopo avere cambiato il filtro di visualizzazione o essere passati ad un'altra cartella.
* **Annullare** 

Annulla l'operazione di inclusione.
* **Aiuto** 

Descrive i campi di testo, gli elenchi di selezione e i pulsanti presenti nel riquadro di dialogo.

# Vedere anche







# Riquadro di dialogo Controllo ortografico dell'Editor &newline; di testo




&sigspace;

&sigspace;

* **Parole errate** 

Elenca le parole scritte in modo errato trovate nel documento.
* **Sostituire con** 

Digitare in questo campo la parola scritta correttamente.
* **Ricercare** 

Localizza la prima ricorrenza della parola scritta in modo errato. La ricerca ha inizio dalla posizione del cursore.
* **Sostituire** 

Sostituisce la parola evidenziata con quella scritta correttamente.
* **Sostituire tutto** 

Sostituisce tutte le ricorrenze della parola scritta in modo errato.
* **Chiudere** 

Chiude il riquadro di dialogo.


&sigspace;

Il comando Controllare ortografia è disponibile solo per la lingua inglese.
# Vedere anche







# Riquadro di dialogo Ricerca/Sostituzione dell'Editor &newline; di testo




&sigspace;

&sigspace;

* **Ricercare** 

Inserire in questo campo la parola o la frase da ricercare. Si noti che le maiuscole e le minuscole vengono interpretate come caratteri diversi.
* **Sostituire con** 

Inserire in questo campo il nuovo testo da sostituire a quello localizzato.
* **Sostituire** 

Sostituisce la parola evidenziata con il testo specificato.
* **Sostituire tutto** 

Sostituisce tutte le ricorrenze della stringa da ricercare.
* **Chiudere** 

Chiude il riquadro di dialogo.

# Vedere anche







# Riquadro di dialogo Impostazioni di formato dell'Editor &newline; di testo




&sigspace;

&sigspace;

* **Margine sinistro** 

Indica il numero di caratteri di cui il testo stampato deve essere rientrato rispetto al margine sinistro del foglio
* **Margine destro** 

Indica il numero di colonne disponibili per le righe di testo
* **Allineare a sinistra** 

Allinea il testo lungo il margine sinistro
* **Allineare a destra** 

Allinea il testo lungo il margine destro
* **Giustificare** 

Allinea il testo su entrambi i margini
* **Centrare** 

Imposta la centratura del testo rispetto ai bordi del foglio
* **Paragrafo** 

Applica le impostazioni al paragrafo in cui si trova il cursore
* **Tutto** 

Applica le impostazioni all'intero documento
* **Chiudere** 

Chiude il riquadro di dialogo

# Vedere anche







# Riquadro di dialogo Copia su file dell'Editor di testo


L'Editor di testo può essere usato anche da altre applicazioni come strumento di editazione, eventualmente con una restrizione alle modalità di salvataggio delle informazioni. Ad esempio, in alcuni casi l'opzione Salvare come può essere sostituita dal comando Copiare su file, che consente di creare una copia delle informazioni correnti senza spostare la sessione di editazione nel nuovo file.

&sigspace;

&sigspace;

* **Inserire il percorso o
il nome della cartella&sigspace;** 

&newline; Identifica il percorso della cartella corrente.
* **Filtro** 

Specificando un asterisco (*) vengono visualizzati tutti i file. In alternativa, è possibile usare caratteri speciali per visualizzare solo i file il cui nome corrisponde a un determinato schema. Ad esempio, specificando *.doc verranno presentati solo i file che terminano con .doc.
* **File** 

Presenta l'elenco dei file contenuti nella cartella corrente.
* **Cartelle** 

Presenta le cartelle contenute nella cartella corrente.
* **Inserire il nome del file** 

È il campo in cui inserire il nome da assegnare al file del documento.
* **OK** 

Copia le informazioni correnti nel file specificato.
* **Aggiornare** 

Permette di aggiornare l'elenco dei file dopo avere cambiato il filtro di visualizzazione o essere passati ad un'altra cartella.
* **Annullare** 

Annulla l'operazione di salvataggio.


Se è stata attivata l'opzione A capo automatico, il riquadro di dialogo presenterà anche le seguenti opzioni:

&sigspace;

&sigspace;

Aggiungere caratteri di ritorno a capo al termine delle righe.

È l'impostazione predefinita. Aggiunge un carattere di ritorno a capo alla fine di ogni riga spezzata in base alla larghezza della finestra e mantiene i ritorni a capo del documento esattamente come compaiono nella finestra corrente.

Non aggiungere ritorni a capo. Verranno conservate solo le interruzioni di riga create con Return.

Questa opzione mantiene le interruzioni di riga "soft" inserite dall'opzione A capo automatico. All'apertura successiva del documento, il testo si disporrà in base alla larghezza della nuova finestra.
# Vedere anche





# Avvertenza di file esistente dell'Editor di testo


&sigspace;

&sigspace;

Questa avvertenza viene visualizzata quando il nome specificato per il salvataggio di un documento corrisponde a quello di un file già esistente.

Per sovrascrivere il file originale, fare clic su OK.

Per specificare un nome diverso per il file, fare clic su Annullare e scegliere Salvare come dal menu File.
# Vedere anche





# Sintassi e opzioni dei comandi dell'Editor di testo
Editor di testo: avvio in una finestra di terminale

La sintassi del comando per l'avvio dell'Editor di testo è la seguente:dtpad [`opzioni`]

Dove le`opzioni`possono essere una o più delle seguenti:

* **-server** 

Specifica che l'Editor di testo dovrà essere avviato in modalità server senza visualizzare alcuna finestra iniziale. Alle richieste successive di avvio dell'Editor di testo nella modalità predefinita, il server creerà una finestra di editazione separata per ogni richiesta.
* **-standAlone** 

Questa opzione forza l'esecuzione dell'Editor di testo in modalità standalone, cioè in modo indipendente rispetto al server. Può essere utilizzata se si desidera avviare una copia dell'Editor di testo con un ambiente diverso rispetto alle altre finestre dell'applicazione. Ad esempio, si potrà avviare una copia dell'Editor di testo usando una versione locale diversa, oppure colori diversi. L'uso di questa opzione equivale all'impostazione della risorsastandAlonesul valore True.
* **-exitOnLastClose** 

Specifica che il processo server dell'Editor di testo dovrà terminare alla chiusura dell'ultima finestra di editazione sullo schermo. Questa opzione è applicabile solo insieme all'opzione-server. Se quest'ultima non viene specificata, il processo server dell'Editor di testo resterà attivo a tempo indeterminato, anche dopo la chiusura di tutte le finestre di editazione.
* **-noBlocking** 

Specifica che il processo di richiesta dell'Editor di testo verrà ultimato non appena il server avrà comunicato la possibilità di accesso a un file nella cartella specificata. In mancanza di questa opzione il processo di richiesta dell'Editor di testo si bloccherà, e terminerà solo quando il server segnalerà la chiusura della finestra.
* **-statusLine** 

Questa opzione produce la visualizzazione di una riga di stato nel margine inferiore della finestra di editazione. Questa riga riporta il numero della riga in cui si trova il cursore, il numero totale delle righe del documento, i messaggi dell'applicazione e l'attivazione del modo di sovrascrittura. Offre inoltre la possibilità di spostare il cursore su una determinata riga specificandone il numero.
* **-wrapToFit** 

Specifica l'attivazione dell'opzione A capo automatico all'apertura della finestra.

# Vedere anche






Per un elenco e una descrizione completa delle opzioni disponibili per il comando di avvio dell'Editor di testo, vedere la pagina di spiegazionidtpad(1).
# Risorse dell'Editor di testo
applicazioni: risorserisorse delle applicazioni

Le risorse seguenti controllano l'aspetto e il comportamento dell'Editor di testo.

Dtpad*server: [ true | false ]

Specifica che l'Editor di testo dovrà essere avviato in modalità server per elaborare tutte le richieste di editazione successive. L'impostazione di questa risorsa sul valore True equivale a specificare l'opzione-serverdalla riga comandi.

Dtpad*standAlone: [ true | false ]

Specifica se il processo corrente dell'Editor di testo dovrà essere eseguito in modalità standalone, gestendo direttamente le proprie operazioni di editazione, oppure in modalità di richiesta, in cui l'editazione verrà gestita in realtà da un processo server separato. L'impostazione di questa risorsa sul valore True equivale a specificare l'opzione-standAlonedalla riga comandi.

Dtpad*blocking: [ true | false ]

Specifica che quando l'Editor di testo viene eseguito in modalità di richiesta, cioè nella modalità predefinita in cui l'editazione effettiva viene gestita da un processo server separato, il processo di richiesta non dovrà terminare fino a quando non venga chiusa la finestra associata alla richiesta di editazione. L'impostazione di questa risorsa sul valore False equivale a specificare l'opzione-noBlockingdalla riga comandi.

Dtpad*exitOnLastClose: [ true | false ]

Specifica se il processo server dell'Editor di testo dovrà terminare alla chiusura dell'ultima finestra di editazione attiva. Se questa risorsa viene impostata su False, il server dell'Editor di testo resterà attivo e pronto a ricevere i messaggi di editazione. Se la risorsa viene impostata su True, il processo server dell'Editor di testo terminerà alla chiusura dell'ultima finestra di editazione attiva.

Dtpad*statusLine: [ true | false ]

Specifica se l'Editor di testo dovrà visualizzare una riga di stato nel margine inferiore della finestra di editazione. L'impostazione di questa risorsa su True equivale a specificare l'opzione-statusLinedalla riga comandi.

Dtpad*wrapToFit: [ true | false ]

Specifica se l'Editor di testo dovrà attivare (True) o mantenere disattivata (False) l'opzione A capo automatico all'apertura della finestra. L'impostazione di questa risorsa su True equivale a specificare l'opzione-wrapToFitdalla riga comandi.
# Vedere anche






Per un elenco e una descrizione completa delle opzioni disponibili per il comando di avvio dell'Editor di testo, vedere la pagina di spiegazionidtpad(1).
# Insieme di file dell'Editor di testo
Editor di testo: file eseguibileapplicazioni: impostazioni predefiniterisorse delle applicazioni

Il file eseguibile e il file con le impostazioni predefinite dell'Editor di testo sono:

* **File eseguibile** 

/usr/dt/bin/dtpad
* **File con le impostazioni predefinite** 

&newline;/usr/dt/app-defaults/`lingua`/Dtpad

# Vedere anche




