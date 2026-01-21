
# Attività di Creare azione

# Creazione ed editazione delle azioni





# Creazione ed editazione dei tipi di dati





# Icone per le azioni e i tipi di dati



# Creare un'azione
azione:creazione

Aprire il gruppo di applicazioni Applicazioni_desktop nella Gestione di applicazioni e fare doppio clic su Creare azione.

Verrà aperta la finestra principale di Creare azione.

&newline;&empty;

&newline;&empty;

Nel campo di testo Nome dell'azione, digitare il nome da
associare come etichetta all'icona dell'azione.

Il nome dell'azione può contenere qualsiasi carattere ad eccezione degli spazi.

Usare i controlli Icone dell'azione per specificare l'icona da
associare all'applicazione. Inizialmente verrà presentata
l'icona predefinita.

Per usare un'icona differente, fare clic su Ricercare icone per
aprire il relativo riquadro di dialogo. Per maggiori informazioni,
vedere.

Per creare una nuova icona, fare clic su Editare icona per avviare
l'Editor delle icone. Vedere.

Nel campo di testo Comando da eseguire al doppio clic sull'icona, digitare
il comando di avvio dell'applicazione.

Per specificare un file come argomento, usare la sintassi$`n`.
Ad esempio:emacs
bitmap $1
diff $1 $2
lp -oraw $1

Se nella riga comandi viene specificato come argomento il nome di
un file ($`n`), l'icona dell'azione potrà essere utilizzata
come area di rilascio per i file.

Se non specificato in modo esplicito, le righe comandi non verranno
inviate a una shell. Ad esempio, le righe seguenti specificano
l'utilizzo di una shell:/bin/sh -c `ps | lp'
/bin/sh -c `spell $1 | more'

Nel campo "Testo di aiuto per l'azione", inserire le
informazioni che dovranno essere visualizzate alla richiesta di aiuto
sull'icona dell'azione.

Il testo inserito andrà a capo automaticamente. Si noti, tuttavia, che i ritorni a capo
che compaiono nel campo di testo non saranno mantenuti nella visualizzazione
online. Se si desidera specificare un ritorno a capo "forzato", usare&bsol;n.

Usare le opzioni del pulsante Tipo di finestra per scegliere il tipo di finestra in cui eseguire l'azione.

* **Grafica (finestra X)** 

L'applicazione creerà una propria finestra.
* **Terminale (chiusura automatica)** 

L'applicazione verrà eseguita in una finestra di terminale che si chiuderà automaticamente all'uscita dall'applicazione.
* **Terminale (chiusura manuale)** 

L'applicazione verrà eseguita in una finestra di terminale che resterà aperta fino a un esplicito comando di chiusura.
* **Nessun output** 

L'applicazione non produrrà alcun output sullo schermo.


Procedere come segue:

Se l'applicazione utilizza file di dati e si desidera creare uno o più
tipi di dati per questi file, vedere.

Se non si desidera creare un tipo di dati:

Salvare l'azione scegliendo il comando Salvare dal menu File.

Provare la nuova azione facendo doppio clic sulla sua icona nella
directory iniziale.
# Creare un tipo di dati
tipo di dati:creazione

Definire l'azione per avviare l'applicazione. Vedere i punti 1-6 della
sezione.

Fare clic sul pulsante Funzioni avanzate per espandere la finestra di Creare azione.

&newline;&empty;

&newline;&empty;

Se si desidera che, facendo doppio clic sull'icona dell'azione,
venga richiesto il nome di un file, digitare il testo della richiesta
nel campo "All'avvio dell'azione, chiedere all'utente".

Questo campo deve essere obbligatoriamente compilato se il comando di avvio
dell'applicazione richiede un argomento.

Il campo deve essere lasciato vuoto se il comando di avvio non prevede
l'uso di un argomento.

Se nel comando di avvio dell'applicazione l'indicazione di un argomento è opzionale, si può scegliere se compilare il campo o meno.
Se si specifica un testo per la richiesta, tale testo comparirà facendo doppio clic sull'icona dell'azione.
Diversamente, l'azione verrà eseguita usando come argomento
una stringa vuota.

Specificare i tipi di file che l'azione può accettare come argomenti:

Se l'azione può accettare qualsiasi tipo di dati, scegliere Tutti i tipi di dati.

Se l'azione può accettare solo i tipi di dati creati per l'applicazione, fare clic su Solo i tipi nella lista.

Inizialmente, la lista dei tipi di dati sarà vuota. Vi verranno inseriti
a mano a mano i tipi di dati creati per l'applicazione.

Fare clic su Aggiungere per
aprire il riquadro di dialogo Aggiunta di un tipo di dati.

&newline;&empty;

&newline;&empty;

Opzionalmente: Se non si desidera usare il nome predefinito per il tipo di dati, digitare un nuovo nome nel campo di testo Nome del tipo di dati. Il nome non potrà contenere spazi vuoti.

Fare clic sul pulsante Editare vicino al riquadro Caratteristiche di identificazione per aprire il riquadro di dialogo corrispondente.

&newline;&empty;

&newline;&empty;

Nel riquadro di dialogo, impostare le caratteristiche che dovranno differenziare il tipo di dati corrente. Si potrà scegliere un unico criterio (ad esempio, Schema del nome) oppure una combinazione di criteri (ad esempio, Schema del nome e Autorizzazioni.

Fare clic su File o Cartelle per specificare se il tipo di dati si riferisce a file o a directory.

&newline;&empty;

&newline;&empty;

Se il tipo di dati dipende dal nome, fare clic sulla casella Schema del nome e digitare lo schema di identificazione appropriato. Si possono usare i caratteri speciali*e?.

&newline;&empty;

&newline;&empty;

* ***** 

corrisponde a una qualunque sequenza di caratteri.
* **?** 

corrisponde a qualsiasi carattere singolo.


Se il tipo di dati dipende dalle autorizzazioni, fare clic sulla casella
Autorizzazioni e selezionare le autorizzazioni richieste per il tipo di dati.

&newline;&empty;

&newline;&empty;

* **Sì** 

Indica che il file deve possedere l'autorizzazione specificata.
* **No** 

Indica che il file non deve possedere l'autorizzazione specificata.
* **Opzionale** 

(Valore predefinito) Indica che la presenza dell'autorizzazione è irrilevante.


Se il tipo di dati dipende dal contenuto, fare clic sulla casella Contenuto
e specificare lo schema di ricerca e il tipo di contenuto. Opzionalmente, si
può specificare anche il byte da cui iniziare la ricerca.

Fare clic su OK per chiudere il riquadro di dialogo.

Le caratteristiche impostate verranno visualizzate nella sezione Caratteristiche di identificazione del riquadro di dialogo Aggiunta di un tipo di dati.

Le autorizzazioni che compaiono nel riquadro Caratteristiche di identificazione hanno i seguenti significati:

* **d** 

Directory
* **r** 

Autorizzazione di lettura
* **w** 

Autorizzazione di scrittura
* **x** 

Autorizzazione di esecuzione
* **!** 

NOT
* **&** 

AND


Nel campo Testo di aiuto per il tipo di dati, digitare il testo da
visualizzare alla richiesta di aiuto sul tipo di dati

Usare i controlli Icone del tipo di dati per specificare l'icona da
usare per l'applicazione.

Inizialmente verranno mostrate le icone predefinite.

Per scegliere un'icona diversa, fare clic su Ricercare icone per
aprire il riquadro di dialogo corrispondente. Per maggiori informazioni,
vedere.

Per creare una nuova icona, fare clic su Editare icona per avviare
l'Editor delle icone.
Vedere.

Opzionalmente: Se l'applicazione dispone di un comando per stampare i file di dati dalla riga comandi, digitarlo nel campo Comando di stampa per il tipo di dati; per specificare un file come argomento, usare la sintassi$`n`.

Fare clic su OK per chiudere il riquadro di dialogo e aggiungere il nuovo tipo di dati nella finestra di Creare azione.
# Scegliere le icone per le azioni e i tipi di dati


La finestra principale di Creare azione e il riquadro di dialogo Aggiunta di un tipo di dati permettono di specificare le icone da associare alle azioni e ai tipi di dati.

Per usare un'icona esistente, fare clic su Ricercare icone.
Vedere.

Per creare una nuova icona usando l'Editor delle icone, fare clic
su Editare icona. Vedere.
# Usare il riquadro di dialogo Ricerca di un set di icone


Questo riquadro di dialogo viene visualizzato facendo clic su Ricercare icone dalla finestra principale di Creare azione o dal riquadro di dialogo Aggiunta di un tipo di dati.
&newline;&empty;

&newline;&empty;

Il riquadro di dialogo permette di specificare:

Un'icona contenuta in una delle cartelle dell'elenco Cartelle delle icone.
Questo elenco contiene tutte le cartelle incluse nel percorso di ricerca
delle icone.

Un'icona inclusa in un pacchetto di registrazione integrato nel desktop
condtappintegrate.
# Per specificare un'icona contenuta in una cartella


Nell'elenco Cartelle delle icone, fare doppio clic sulla cartella
che contiene l'icona desiderata.

L'elenco File delle icone mostrerà tutte le icone contenute in
quella cartella.

Nell'elenco File delle icone, fare clic sull'icona desiderata.

Fare clic su OK.
# Per specificare un'icona contenuta in un pacchetto di registrazione


Questa procedura riguarda principalmente i programmatori o gli amministratori di sistema. Nella creazione di un pacchetto di registrazione, i file delle icone vengono collocati inizialmente in una directory del pacchetto di registrazione:`radice_app`/dt/appconfig/icons/`lingua`

Dopo la registrazione, i file delle icone si troveranno in unacartella delle icone. Per questa ragione, le definizioni delle azioni e dei tipi di dati dovranno usare i nomi base dei file.

Per specificare un nome base per un'icona che non si trovi ancora in una cartella delle icone, procedere come segue:

Nel campo di testo Nome del file dell'icona, digitare ilnome basedel file dell'icona.

Fare clic su OK.

Comparirà un messaggio di errore indicante che il file specificato non è
stato trovato in nessuna cartella delle icone.

Fare clic su OK per far scomparire il messaggio di errore e chiudere il
riquadro di dialogo Ricerca di un set di icone.

Ignorare l'assenza di immagini nel riquadro Icone dell'azione o Icone del
tipo di dati. L'immagine dell'icona verrà identificata non appena
l'applicazazione sarà stata registrata.
# Creazione di una nuova icona


Fare clic su Editare icona nella finestra principale di Creare azione o nel riquadro di dialogo Editazione del tipo di dati.
Verrà avviato l'Editor delle icone.

Usare l'Editor delle icone per disegnare la nuova icona. Vedere:

Attività dell'Editor
delle icone.



Scegliere Salvare dal menu File per salvare l'icona in un file.

Salvare il file dell'icona nella cartella`CartellaIniziale`/.dt/icons.
Vedere.

Per gli amministratori di sistema e i programmatori: Nella creazione di un pacchetto di registrazione, il file dell'icona dovrà essere salvato nella directory`radice_app`/dt/appconfig/icons/`lingua`.
# Dimensioni e nomi delle icone


Le icone del desktop utilizzano le seguenti convenzioni per i nomi:

* **Dimensione(pixel)** 

Nome
* **32 x 32** 

`nomebase`.m.pmo`nomebase`.m.bm
* **16 x 16** 

`nomebase`.s.pmo`nomebase`.s.bm


Per le icone delle azioni, usare come nome base il nome dell'azione.

Per le icone dei tipi di dati, usare come nome base il nome del
tipo di dati.
# Editare un'azione esistente
azione:editazione

L'applicazione Creare azione può essere usata per editare un'azione
esistente quando:

L'azione era stata creata con Creare azione.

e, il file contenente la definizione dell'azione non è
mai stato editato manualmente (usando un editor di testo).

Aprire il gruppo di applicazioni Applicazioni_desktop nella
Gestione di applicazioni e fare doppio clic su Creare azione.

Verrà aperta la finestra principale di Creare azione.

Scegliere Aprire dal menu File. Verrà aperto il riquadro di
dialogo Apertura di un file.

Nell'elenco File, selezionare il file contenente la
definizione dell'azione. Il file avrà il nome`nome_azione`.dt.

Scegliere OK.
# Editare un tipo di dati esistente
tipo di dati:editazione

L'applicazione Creare azione può essere usata per editare un tipo
di dati esistente quando:

Il tipo di dati era stato creato con Creare azione.

e, il file contenente il tipo di dati non è mai stato
editato manualmente (usando un editor di testo).

Aprire il gruppo di applicazioni Applicazioni_desktop nella
Gestione di applicazioni e fare doppio clic su Creare azione.

Verrà aperta la finestra principale di Creare azione.

Scegliere Aprire dal menu File. Verrà aperto il riquadro di
dialogo Apertura di un file.

Nell'elenco File, selezionare il file contenente la definizione
del tipo di dati.

Il nome del file sarà`nome_azione`.dt, dovenome_azioneè
il nome dell'azione che era stata creata insieme al tipo di dati.

Fare clic su Funzioni avanzate.

Nell'elenco Tipi di dati che utilizzano l'azione, selezionare
il tipo di dati da editare.

Fare clic su Editare per aprire il riquadro di dialogo
Editazione del tipo di dati.

Apportare le modifiche desiderate. Al termine, fare clic su OK.

Per salvare la nuova definizione, scegliere Salvare dal menu File.