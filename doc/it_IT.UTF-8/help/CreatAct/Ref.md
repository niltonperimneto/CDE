
# Riferimenti su Creare azione

# Informazioni generali su Creare azione





# Finestre di Creare azione









# File creati da Creare azione
Creare azione:file creati conazione: nome

L'output di Creare azione può essere:

La definizione per un'azione che esegue un comando. In questo caso viene
creato un file`CartellaIniziale`/.dt/types/`nome_azione`.dtcontenente la definizione.

azione: icona

L'icona dell'azione. L'icona viene collocata nella
directory iniziale dell'utente, ed avvia l'azione facendovi doppio clic.
Opzionalmente, l'icona può essere creata come area di rilascio specificando
un tipo di dati che supporti le operazioni di trascinamento e rilascio.

Utilizzando le funzioni avanzate è anche possibile creare:

Uno o più tipi di dati per i file creati con l'applicazione.

Azioni per l'apertura e la stampa dei tipi di dati creati con l'applicazione

I tipi di dati e le azioni di apertura e di stampa vengono scritti nel file`CartellaIniziale`/.dt/types/`nome_azione`.dt.
# Limitazioni di Creare azione

# Limitazioni alla creazione delle azioni
Creare azione:limitazioni alla creazione delle azioni

L'applicazione Creare azione non può essere usata per creare un'azione quando:

La riga comandi richiede un parametro diverso da un file. Ad esempio,
Creare azione non può essere usato per creare un'azione per il comando:lp -d`unità``nomefile`

dove l'utente deve specificare l'unità ad ogni esecuzione del comando.
(Si può tuttavia creare manualmente un'azione che esegua questa operazione.)

Si desidera visualizzare sotto l'icona un'etichetta diversa dal nome
dell'azione. Ad esempio, Creare azione non può essere usato per
specificare una versione locale di un'azione esistente.

L'azione richiede una o più delle funzioni avanzate incluse nel
database delle azioni. È il caso delle azioni che:

Avviano un comando su un sistema remoto rispetto a quello
in cui risiede la definizione dell'azione.

Richiamano altre azioni.

Devono essere eseguite con un accesso diverso da quello
normale dell'utente (ad esempio come superutente).

Fanno ampio uso della funzione di "mappatura".

Hanno comportamenti molto diversi a seconda del numero
di file specificati come argomenti.
# Limitazioni alla creazione dei tipi di dati
Creare azione:limitazioni alla creazione dei tipi di dati

L'applicazione Creare azione non può essere usata per creare un
tipo di dati quando:

Al tipo di dati devono essere associate altre azioni oltre a quelle
di apertura e di stampa.

L'azione di apertura per il tipo di dati non corrisponde al comando
dell'azione. Ad esempio, Creare azione non permette di creare un tipo di
dati associato a una singola icona per una directory che rappresenta il
gruppo di applicazioni di un'applicazione.
# Finestra principale di Creare azione
Creare azione:finestra principale

Per informazioni sulle procedure da seguire per creare le azioni e i tipi di dati, vedere.

&newline;&empty;

&newline;&empty;

* **Nome dell'azione** 

Digitare il nome da assegnare all'azione. Si noti che le maiuscole e le minuscole vengono interpretate come caratteri diversi, e che il nome non può contenere spazi vuoti.
* **Icone dell'azione** 

Mostra l'immagine che verrà utilizzata per le icone dell'azione.
* **Ricercare icone** 

Apre un riquadro di dialogo da cui è possibile selezionare un file a mappa di bit o a mappa di pixel da usare per l'icona.
* **Editare icona** 

Apre l'Editor delle icone, che consente di creare una nuova icona o di editarne una esistente.
* **Comando da eseguire al
doppio clic sull'icona** 

Digitare il comando. Per specificare un file come argomento, usare la sintassi$`n`.
* **Testo di aiuto per l'azione** 

Digitare il testo di aiuto da associare all'icona dell'azione.
* **Tipo di finestra** 

Selezionare il tipo di finestra che l'azione dovrà creare:

Grafica: L'applicazione verrà avviata in una finestra separata

Terminale (chiusura automatica): L'azione avvierà una finestra
di terminale che verrà chiusa automaticamente al termine dell'azione

Terminale (chiusura manuale): L'azione avvierà una finestra di
terminale che dovrà poi essere chiusa manualmente

Nessun output: L'applicazione non richiede alcuna finestra

# Funzioni avanzate


Usare le funzioni avanzate solo se il comando specificato in "Comando da eseguire al doppio clic sull'icona" include un file come argomento.

&newline;&empty;

&newline;&empty;

* **All'avvio dell'azione,
chiedere all'utente** 

Digitare la stringa di richiesta del file che dovrà essere visualizzata facendo doppio clic sull'icona dell'azione.
* **Tipi di dati che
utilizzano l'azione** 

Mostra i tipi di dati che sono stati creati per l'azione corrente. Questo elenco è di sola lettura; le voci vengono aggiunte usando il riquadro di dialogo Aggiunta di un tipo di dati.
* **Aggiungere** 

Apre un riquadro di dialogo da cui è possibile creare un nuovo tipo di dati.
* **Cancellare** 

Cancella il tipo di dati selezionato nell'elenco "Tipi di dati che utilizzano l'azione".
* **Editare** 

Permette di editare il tipo di dati selezionato nella lista "Tipi di dati che utilizzano l'azione".
* **Tipi di dati accettati** 

Scegliere se l'icona dell'azione dovrà accettare qualsiasi tipo di file di dati, oppure solo quelli specificati nell'elenco "Tipi di dati che utilizzano l'azione".

# Riquadro di dialogo Aggiunta di un tipo di dati
Aggiunta di un tipo di dati, riquadro di dialogo

Per informazioni specifiche sulle procedure da seguire, vedere.

* **Nome del tipo di dati** 

In questo campo viene inserito automaticamente un tipo di dati, che tuttavia può essere modificato. Le maiuscole e le minuscole vengono considerate caratteri diversi e il nome non può contenere spazi vuoti.
* **Caratteristiche di
identificazione** 

È un elenco dei criteri che definiscono il tipo di file. Scegliendo Editare sarà possibile impostare e modificare le caratteristiche.
* **Editare** 

Apre il riquadro di dialogo Caratteristiche di identificazione.
* **Testo di aiuto per
il tipo di dati** 

Digitare il testo che dovrà essere visualizzato alla richiesta di aiuto sul tipo di dati.
* **Icone del tipo di dati** 

Visualizza l'immagine che verrà utilizzata per le icone del tipo di dati.
* **Ricercare icone** 

Apre un riquadro di dialogo da cui è possibile selezionare un file a mappa di bit o a mappa di pixel da usare per l'icona.
* **Editare icona** 

Avvia l'Editor delle icone, con cui è possibile creare una nuova icona o editarne una esistente.
* **Comando di apertura per
il tipo di dati** 

Visualizza il comando che dovrà essere eseguito facendo doppio clic sull'icona del tipo di dati. È lo stesso comando che compare nel campo Comando da eseguire al doppio clic sull'icona.
* **Comando di stampa per
il tipo di dati** 

Digitare il comando che l'applicazione dovrà eseguire per stampare il tipo di dati definito.
* **OK** 

Crea le informazioni per il tipo di dati, aggiunge il tipo di dati alla lista "Tipi di dati che utilizzano l'azione" e chiude il riquadro di dialogo.
* **Applicare** 

Crea le informazioni per il tipo di dati e aggiunge il tipo di dati alla lista "Tipi di dati che utilizzano l'azione". Il riquadro di dialogo rimane aperto.
* **Annullare** 

Chiude il riquadro di dialogo senza creare alcun tipo di dati.
* **Aiuto** 

Visualizza l'aiuto online.

# Riquadro di dialogo Caratteristiche di identificazione
Caratteristiche di identificazione, riquadro di dialogo

Per istruzioni specifiche sulle procedure da seguire, vedere.

&newline;&empty;

&newline;&empty;

* **Includere** 

File: Selezionare questo pulsante se il tipo di dati è
applicabile solo ai file.

Cartelle: Selezionare questo pulsante se il tipo di dati è
applicabile solo alle directory.
* **Schema del nome** 

Selezionare la casella e digitare uno schema di identificazione, ad esempio un'estensione, per il nome dei file o delle cartelle. Si possono utilizzare i seguenti caratteri speciali:

*: Corrisponde a una qualunque sequenza di caratteri.

?: Corrisponde a un qualsiasi carattere singolo.
* **Autorizzazioni** 

Selezionare la casella e quindi i pulsanti di scelta appropriati. Se la presenza o meno di un'autorizzazione non è rilevante, scegliere Opzionale.
* **Contenuto** 

Selezionare la casella e digitare i dati che il file deve contenere.
* **Tipo** 

Selezionare il tipo di dati. Per scegliere un file di testo (ASCII), selezionare Stringa.
* **Byte di inizio** 

Indicare il punto del file da cui si desidera avviare la ricerca dei dati. Per specificare l'inizio del file, usare il valore1.
* **OK** 

Applica i valori specificati come Caratteristiche di identificazione e chiude il riquadro di dialogo.
* **Annullare** 

Chiude il riquadro di dialogo senza applicare le modifiche.
* **Ripristinare** 

Ripristina le impostazioni predefinite.
* **Aiuto** 

Visualizza l'aiuto online.

# Riquadro di dialogo Ricerca di un set di icone
Ricerca di icone, riquadro di dialogo

Questo riquadro di dialogo permette di specificare l'immagine da utilizzare per l'icona dell'azione o del tipo di dati. Per istruzioni specifiche sulle procedure da seguire, vedere.

&newline;&empty;

&newline;&empty;

* **Cartelle delle icone** 

Mostra le cartelle incluse nel percorso di ricerca delle icone. Facendo doppio clic su una cartella vengono mostrate le icone contenute in quella cartella.
* **File delle icone** 

Mostra le icone contenute nella directory. Facendo clic su un file per selezionarlo, il suo nome verrà visualizzato, con il percorso completo, nel campo di testo Nome del file dell'icona.
* **Nome del file dell'icona** 

Questo campo di testo viene usato per inserire ilnome basedel file dell'icona. Il suo contenuto cambia facendo clic su un elemento nella lista File delle icone.
* **OK** 

Accetta l'icona specificata nel campo Nome del file dell'icona e chiude il riquadro di dialogo.
* **Annullare** 

Chiude il riquadro di dialogo senza cambiare l'icona.
* **Aiuto** 

Visualizza l'aiuto online.

# Riquadro di dialogo Apertura di un file
Creare azione:apertura di un file di azioneazione:aperturaazione:editazione

* **Inserire il percorso o
il nome della cartella &sigspace;** 

&newline;
Identifica il percorso della cartella corrente.
* **Filtro** 

Inserendo un asterisco (*) vengono visualizzati tutti i file.
È possibile anche utilizzare i caratteri speciali per visualizzare
solo i file con un determinato suffisso. Ad esempio,
specificando lo schema *.doc verranno visualizzati solo i file
che terminano con .doc.
* **File** 

Mostra i file contenuti nella cartella corrente.
* **Cartelle** 

Mostra le cartelle contenute nella cartella corrente.

È possibile aprire qualsiasi file residente nella cartella corrente, in una cartella secondaria o in un'altra cartella.
* **Inserire il nome del file** 

Mostra il nome del file selezionato
nell'elenco File.
Per aprire il file, premere Return o fare clic su OK.
Oppure, digitare direttamente il nome del file che si
desidera aprire.
* **OK** 

Apre il file specificato nel campo Inserire il nome del file.
* **Aggiornare** 

Aggiorna l'elenco dei file dopo avere modificato il filtro
per il nome o dopo essere passati a una cartella differente.
* **Annullare** 

Annulla l'operazione di apertura.
* **Aiuto** 

Descrive i campi di testo, gli elenchi di selezione e
i pulsanti del riquadro di dialogo.
