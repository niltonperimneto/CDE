
# Riferimenti sulla Gestione di applicazioni
riferimenti

Qui di seguito sono riportati gli argomenti di riferimento disponibili per
la Gestione di applicazioni, suddivisi in categorie.
# Riferimenti generali







# Menu della Gestione di applicazioni











# Finestra e riquadri di dialogo della Gestione di applicazioni





















# Riferimenti sull'uso del mouse nella Gestione di &newline; applicazioni
mouse: funzioni dei pulsantiselezione di oggetti
# Pulsante del mouse 1&emdash;Selezione e trascinamento


Il pulsante del mouse 1 (nell'impostazione predefinita, quello di sinistra)
viene usato per selezionare e trascinare gli oggetti.

Per selezionare un oggetto, fare clic sull'icona corrispondente.

Per estendere la selezione a un altro oggetto, premere il tasto Control e
fare clic su un'altra icona.
# Pulsante del mouse 2&emdash;Trascinamento


Modi di trascinamento degli oggetti:

Per spostare un oggetto: premere il pulsante del mouse 1 sull'icona
dell'oggetto e trascinare.

Per copiare un oggetto: premere Control insieme al pulsante del mouse e
trascinare.

Per creare un collegamento simbolico: premere Shift insieme al pulsante
del mouse e trascinare.

Con i mouse a due pulsanti, premere entrambi i pulsanti contemporaneamente.
# Pulsante del mouse 3&emdash;Menu
menu legati al contesto

Il pulsante del mouse 3 (nell'impostazione predefinita, quello di destra)
viene usato per visualizzare i menu a scomparsa.

Puntare su un oggetto o su un'area in cui è disponibile un menu a scomparsa,
quindi premeree tenere premutoil pulsante del mouse 3.
# Nomi dei file nella Gestione di applicazioni
Gestione di applicazioni:nomi dei filenomi dei file:nella Gestione di applicazioniicone di azioni:etichetteetichette per le icone delle azioni

Nella Gestione di file e nella Gestione di applicazioni, i file e le
cartelle vengono rappresentati come icone, solitamente identificate con il
nome dell'oggetto.

Fanno eccezione talvolta le icone delle azioni. Ad esempio, si provi a
visualizzare il menu a scomparsa associato all'icona dell'azione Orologio
digitale nel gruppo di applicazioni Strumenti_desktop. Si noterà che il
nome del file che compare all'inizio del menu a scomparsa non coincide con
il nome che compare sotto l'icona.

&newline;&empty;&newline;

Il nome effettivo del file compare nei riquadri di dialogo Copia di file e
Collegamento di file.
# Posizione delle cartelle della Gestione di applicazioni
Gestione di applicazioni:posizione delle cartelle

La Gestione di applicazioni ha un funzionamento molto simile a quello della
Gestione di file. Infatti, la Gestione di applicazioni non è che una vista
della Gestione di file, che mostra una cartella speciale del sistema usata
per contenere le applicazioni registrate. La Gestione del login crea la
cartella della Gestione di applicazioni ad ogni login dell'utente nel
sistema.

Di norma non è necessario sapere dove si trova la cartella rappresentata
dalla Gestione di applicazioni. Tuttavia, questa informazione può essere
utile per cercare di risolvere alcuni problemi.

La posizione della cartella è la seguente:/var/dt/appconfig/appmanager/`cartella_speciale`

dovecartella_specialeè un nome assegnato dal sistema ed associato in
modo esclusivo al nome del sistema e al nome di login.

Non cercaremaidi modificare direttamente la directorycartella_speciale.dtappgatherRicaricare applicazioni:azione

Dopo avere creato la cartella, la Gestione del login esegue il programmadtappgatherche identifica e raccoglie tutti i gruppi di applicazioni
presenti sul sistema.

dtappgatherpuò essere eseguito anche durante una normale sessione
facendo doppio clic su Ricaricare applicazioni nel gruppo
Strumenti_desktop.
# Barra dei menu della Gestione di applicazioni


La barra dei menu della Gestione di applicazioni contiene i seguenti menu:








# Menu File della Gestione di applicazioni


menu File della Gestione di applicazioni

* **Nuova cartella** 

Richiede un nome da assegnare alla nuova cartella da creare.
* **Nuovo file** 

Richiede un nome da assegnare al nuovo file da creare.
* **livello superiore** 

&newline;Risale di un livello nella gerarchia delle cartelle.
* **Andare a** 

Apre un riquadro di dialogo in cui è possibile digitare il
nome della cartella a cui si desidera passare, oppure
sceglierlodalla lista delle cartelle aperte in precedenza.
* **Ricercare** 

Apre un riquadro di dialogo che permette di ricercare file
e cartelle in base a uno schema di identificazione del
nome o al contenuto del file.
* **Chiudere** 

Chiude la vista corrente della Gestione di applicazioni.

# Menu Selezioni della Gestione di applicazioni


menu Selezioni della Gestione di applicazioni

Molti gruppi di applicazioni possono essere modificati solo
dall'amministratore del sistema. Di conseguenza, è possibile che alcune
delle opzioni descritte non siano disponibili.

* **Spostare in** 

Richiede il nome e il percorso completo del file da
spostare.
* **Copiare in** 

Richiede un nuovo nome da assegnare alla copia del file
selezionato. Il comando Copia è disponibile solo quando è
selezionato un unico file.
* **Copiare come collegamento** 

Richiede il nome e il percorso completo del collegamento
che verrà creato per l'oggetto selezionato.
* **Collocare nello spazio di lavoro** 

&newline;Colloca l'oggetto selezionato nell'angolo superiore destro
dello spazio di lavoro.
* **Spostare nel cestino** 

&newline;Colloca gli oggetti selezionati nel Cestino.
* **Cambiare nome** 

Attiva un campo di editazione per la modifica del nome
dell'oggetto selezionato.
* **Modificare autorizzazioni** 

Apre un riquadro di dialogo che mostra le
autorizzazioni dell'oggetto selezionato.
* **Selezionare tutto** 

Seleziona tutti gli oggetti inclusi nella vista corrente
della Gestione di applicazioni.
* **Annullare tutte le selezioni** 

&newline;Deseleziona tutti gli oggetti della vista corrente della
Gestione di applicazioni.
* **azioni** 

Se all'oggetto selezionato sono associate una o più azioni,
questevengono aggiunte alla fine del menu. Le stesse azioni
compariranno nel menu a scomparsa dell'oggetto.

# Menu Visualizzare della Gestione di applicazioni


menu Visualizzare della Gestione di applicazionivisualizzazione: opzioni nella Gestione di applicazioni

* **Aprire nuova vista** 

Apre un'altra vista della Gestione di applicazioni che
contiene
la cartella corrente.
* **Impostare le opzioni di visualizzazione** 

Apre un riquadro di dialogo con cui è possibile cambiare
l'aspetto e il comportamento della vista corrente della
Gestione di applicazioni.
* **Salvare come opzioni predefinite** 

&newline;Salva le impostazioni correnti, la dimensione della
finestra e la lista dei filtri come opzioni predefinite.
* **Mostrare file nascosti** 

Abilita e disabilita la visualizzazione dei file nascosti.
È possibile specificare i dati da nascondere selezionando
Impostare le opzioni di filtro.
* **Impostare le opzioni di filtro** 

Apre un riquadro di dialogo in cui è possibile specificare
i file che si desidera nascondere (in base al tipo di dati
o al nome).
* **Aggiornare** 

Aggiorna il contenuto della cartella corrente includendo
le ultime modifiche.
L'aggiornamento non includerà le applicazioni aggiunte
dopo il login. Per aggiornare interamente il contenuto
della
Gestione di applicazioni, fare doppio clic su
Ricaricare applicazioni nel gruppo Strumenti_desktop.

# Menu Aiuto della Gestione di applicazioni


menu Aiuto della Gestione di applicazioni

* **Panoramica** 

Presenta un'introduzione generale sulla
Gestione di applicazioni.
* **Attività** 

Presenta istruzioni specifiche sulle procedure da seguire
per l'uso della Gestione di applicazioni.
* **Riferimenti** 

Presenta informazioni sulle finestre, sui menu e sui
riquadri di dialogo della Gestione di applicazioni.
* **Sull'elemento** 

Trasforma il cursore in un punto interrogativo.
Facendo clic su un elemento della finestra della Gestione
di
applicazioni, verrà visualizzato l'aiuto su quell'elemento.
* **Uso dell'aiuto** 

Visualizza informazioni sull'uso delle finestre dell'aiuto.
* **Informazioni sulla Gestione di applicazioni** 

&newline;Mostra il numero di versione e le informazioni di copyright
relative alla Gestione di applicazioni.

# Menu a scomparsa degli oggetti della Gestione di &newline; applicazioni


Gestione di applicazioni:menu a scomparsamenu a scomparsa della Gestione di applicazioniQuasi tutti gli oggetti della Gestione di applicazioni possiedono un
proprio menu a scomparsa.

All'inizio di ciascun menu si trovano due comandi standard:
Collocare nello spazio di lavoro e Spostare nel cestino.

La parte inferiore del menu comprende una serie di azioni inerenti al tipo
di dati dell'oggetto. Sono le stesse opzioni che compaiono nel menu Azioni
quando si seleziona l'icona dell'oggetto.

* **Collocare nello spazio di lavoro** 

Colloca l'oggetto sullo sfondo dello spazio di lavoro
corrente.
La posizione dell'oggetto è determinata dalla risorsaobjectPlacement, il cui valore predefinito è l'angolo
superiore destro dello schermo.
* **Spostare nel cestino** 

Cancella l'oggetto dalla posizione corrente e lo colloca
nel Cestino.
* **Aiuto** 

Visualizza informazioni di aiuto sul menu a scomparsa.
* **Azioni** 

Se all'oggetto sono associate una o più azioni, queste
vengono aggiunte alla fine del menu a scomparsa.

# Finestra della Gestione di applicazioni


Gestione di applicazioni:finestra principaleIl livello più elevato della Gestione di applicazioni contiene le icone dei
gruppi di applicazioni.

&newline;&empty;&newline;

Ogni gruppo di applicazioni è una cartella che contiene una o piùicone
di applicazioni(dette ancheicone di azioni). All'interno del gruppo
possono trovarsi anche altri tipi di file di applicazioni, come i file
"readme" o i file dei modelli.

&newline;&empty;&newline;
# Riquadro di dialogo Copia di file della Gestione di &newline; applicazioni


Gestione di applicazioni:copia di oggetticopia di oggetti&newline;&empty;&newline;

&newline;&empty;&newline;

* **Oggetto selezionato** 

&newline; Mostra l'oggetto che verrà copiato.
* **Cartella di destinazione** 

&newline;Digitare il percorso completo della cartella di
destinazione.
* **Nome per la copia** 

&newline;Digitare il nome del nuovo oggetto.
* **OK** 

Esegue la copia e chiude il riquadro di dialogo.
* **Annullare** 

Annulla il comando di copia e chiude il riquadro
di dialogo.
* **Mostrare icona** 

&newline; Mostra l'icona che verrà usata per il nuovo file.
* **Aiuto** 

Visualizza informazioni di aiuto sul riquadro di dialogo.


La copia dei file o delle cartelle può anche essere eseguita con il mouse.
# Riquadro di dialogo Collegamento di file della &newline; Gestione di applicazioni


&newline;&empty;&newline;

&newline;&empty;&newline;

* **Oggetto selezionato** 

&newline; Mostra l'oggetto che verrà collegato.
* **Cartella di destinazione** 

&newline;Digitare il percorso completo della cartella di
destinazione.
* **Nome per la copia** 

&newline;Digitare il nome del nuovo oggetto.
* **OK** 

Esegue la copia e chiude il riquadro di dialogo.
* **Annullare** 

Annulla il comando di copia e chiude il riquadro di
dialogo.
* **Mostrare icona** 

&newline;Mostra l'icona che verrà usata per il nuovo file.
* **Aiuto** 

Visualizza informazioni di aiuto sul riquadro di dialogo.


Il collegamento dei file o delle cartelle può anche essere eseguito con il
mouse.
# Riquadro di dialogo Spostamento di file della Gestione &newline; di applicazioni


Gestione di applicazioni:spostamento di oggettispostamento di oggetti&newline;&empty;&newline;

&newline;&empty;&newline;

* **Oggetto selezionato** 

&newline; Mostra il file o la cartella che verrà
spostata.
* **Cartella di destinazione** 

&newline;Digitare il percorso completo della cartella di
destinazione.
* **OK** 

Esegue lo spostamento e chiude il riquadro di dialogo.
* **Annullare** 

Annulla il comando di spostamento e chiude il riquadro
di dialogo.
* **Aiuto** 

Visualizza informazioni di aiuto sul cambiamento di nome
a file e cartelle.

# Riquadro di dialogo Nuovo file della Gestione di &newline; applicazioni


&newline;&empty;&newline;

&newline;&empty;&newline;

* **Nome del nuovo file** 

&newline;Digitare il nome del nuovo file o della nuova cartella. Se
il file deve essere creato in una cartella diversa da
quella
corrente, digitare il percorso completo della cartella.
* **OK** 

Crea il file e chiude il riquadro di dialogo.
* **Applicare** 

Crea il file e tiene aperto il riquadro di dialogo,
lasciandolo disponibile per creare un altro file.
* **Annullare** 

Annulla il comando Nuovo file e chiude il riquadro di
dialogo.
* **Mostrare icona** 

Se il nome specificato indica un
tipo di file diverso da quello predefinito, è possibile
che
venga utilizzata un'icona diversa. Per visualizzare
un'anteprima dell'icona, fare clic su Mostrare icona.
Ad esempio, se si digita un nome che termina in.tife
quindi si sceglie Mostrare icona, nel riquadro di dialogo
comparirà l'icona associata al tipo di dati TIFF.
* **Aiuto** 

Visualizza informazioni di aiuto sul riquadro di dialogo.

# Riquadro di dialogo Nuova cartella della Gestione di &newline; applicazioni


&newline;&empty;&newline;

&newline;&empty;&newline;

* **Nome della nuova cartella** 

&newline;Digitare il nome della nuova cartella. Se la cartella deve
essere creata in una directory diversa da quella corrente,
insieme al nome occorrerà digitare anche il percorso
completo.
* **OK** 

Crea la cartella e chiude il riquadro di dialogo.
* **Applicare** 

Crea la cartella e tiene aperto il riquadro di
dialogo, lasciandolo disponibile per creare un'altra cartella.
* **Annullare** 

Annulla il comando di creazione e chiude il riquadro di
dialogo.
* **Mostrare icona** 

Se il nome della nuova cartella indica un tipo di
dati diverso da quello predefinito, è possibile che venga
utilizzata un'icona diversa. Per visualizzare
un'anteprima della nuova icona, fare clic su Mostrare
icona.
* **Aiuto** 

Visualizza informazioni di aiuto sul riquadro di dialogo.

# Riquadro di dialogo Impostazione delle opzioni di filtro &newline; della Gestione di applicazioni


Gestione di applicazioni:filtri per la visualizzazione degli &newline;noggettifiltri per la visualizzazione degli oggetti&newline;&empty;&newline;

&newline;&empty;&newline;

* **Selezionare i tipi di dati da** 

Questo pulsante abilita alternativamente l'impostazione
Nascondere o Visualizzare. Viene mostrato un elenco di tutti
i tipi di dati definiti sul sistema. I tipi di dati
selezionati verranno visualizzati o nascosti nella Gestione di
applicazioni a seconda dell'impostazione del pulsante.
* **Selezionare tutto** 

Seleziona tutti i tipi di dati. Con questa
impostazione,
l'area di visualizzazione della Gestione di applicazioni
apparirà vuota.
* **Annullare tutte le selezioni** 

&newline; Deseleziona tutti i tipi di dati.
* **Stringa di filtro (opzionale)** 

Permette di filtrare le selezioni in base al nome del file.
Ad esempio, digitando*.overranno esclusi dalla
visualizzazione
tutti i file il cui nome termina con.o. Si noti che i
tipi di dati inseriti in questo campo verranno
selezionati nella lista delle icone situata nella parte
superiore del riquadro di dialogo.
* **OK** 

Applica i filtri specificati e chiude
il riquadro di dialogo.
* **Applicare** 

Applica i filtri specificati senza chiudere
il riquadro di dialogo.
* **Impostazioni predefinite** 

Ripristina i filtri di visualizzazione
predefiniti (che includono DOT_FILE,
DOT_FOLDER e CURRENT_FOLDER). Questi filtri verranno
impostati scegliendo Applicare o OK.
* **Annullare** 

Ritorna alle ultime impostazioni applicate e chiude il
riquadro di dialogo.
* **Aiuto** 

Visualizza informazioni di aiuto sui filtri di
visualizzazione degli oggetti.

# Riquadro di dialogo Ricerca di file o cartelle della &newline; Gestione di applicazioni


Il riquadro di dialogo Ricerca di file o cartelle permette di ricercare
all'interno di una cartella e delle sue cartelle secondarie i file che
soddisfano determinati criteri nel nome o nel contenuto.

* **Nome del file o della cartella** 

Digitare il nome del file o della cartella che si desidera
trovare. Si possono usare i caratteri speciali.
* **Contenuto del file** 

&newline; Il testo digitato in questo campo
verrà ricercato all'interno dei file.
* **Cartella di ricerca** 

Digitare il percorso della cartella da cui si desidera
iniziare
la ricerca. L'operazione inizierà da quella cartella e
proseguirà in tutte le sue cartelle secondarie.
* **File trovati** 

Elenca i file e le cartelle trovate con la ricerca.
Facendo doppio clic su un file o su una cartella della lista,
verrà aperta una nuova vista della Gestione di applicazioni
contenente quel file o quella cartella.
* **Aprire nuova vista** 

Se è stato selezionato il nome di un file, apre
una vista della Gestione di applicazioni sulla cartella
contenente quel file. Se è stata
selezionata una cartella, la vista mostrerà il contenuto di
quella cartella.
* **Collocare nello spazio di lavoro** 

&newline; Colloca il file o la cartella
selezionata nello spazio di lavoro corrente.
* **Iniziare** 

Inizia la ricerca.
* **Interrompere** 

Interrompe la ricerca in corso. Si noti che questo
pulsante
può essere usato anche quando è visualizzata la clessidra
di attesa.
* **Annullare** 

Interrompe la ricerca in corso e chiude il riquadro di
dialogo.
* **Aiuto** 

Visualizza informazioni di aiuto sulla ricerca degli
oggetti.

# Riquadro di dialogo Autorizzazioni della Gestione di &newline; applicazioni


Il riquadro di dialogo Autorizzazioni permette di cambiare le
autorizzazioni di lettura, scrittura ed esecuzione dei file e delle
cartelle di proprietà dell'utente. Le autorizzazioni di un oggetto possono
essere cambiate solo dal suo proprietario. Se non si è proprietari del file
o della cartella, il riquadro mostrerà le impostazioni correnti ma non
permetterà di modificarle. Il riquadro di dialogo riporta anche il percorso
completo, la dimensione, la data e l'ora dell'ultima modifica del file o
della cartella.

* **Nome del proprietario** 

Nome dell'utente proprietario dell'oggetto. Il
proprietario
può essere cambiato solo dall'amministratore di sistema
(utente root).
* **Nome del gruppo** 

Nome del gruppo di utenti con le autorizzazioni indicate
nella riga Gruppo. Il proprietario dell'oggetto può impostare
qualsiasi gruppo di cui sia membro. L'utente root può
impostare qualsiasi gruppo.
* **Autorizzazioni** 

Selezionando le caselle appropriate, il proprietario
dell'oggetto può
impostare o modificare le autorizzazioni di
lettura, scrittura ed esecuzione dell'oggetto.
* **OK** 

Applica le impostazioni correnti e chiude il riquadro di
dialogo.
* **Annullare** 

Chiude il riquadro di dialogo senza applicare le
modifiche.
* **Aiuto** 

Visualizza informazioni di aiuto sulla modifica delle
autorizzazioni.

# Riquadro di dialogo Impostazione delle opzioni di &newline; visualizzazione della Gestione di applicazioni


Gestione di applicazioni:opzioni di visualizzazionevisualizzazione: opzioni nella Gestione di applicazioniIl riquadro di dialogo Impostazione delle opzioni di visualizzazione
permette di modificare il modo in cui sono rappresentati i file e le
cartelle nella Gestione di applicazioni.
# Intestazioni


L'area Intestazioni specifica quali righe di intestazione visualizzare
nella finestra della Gestione di applicazioni.

* **Percorso ad icone** 

&newline; Mostra il percorso della cartella corrente
come sequenza di icone.
* **Percorso esplicito** 

Mostra il percorso della cartella corrente in una riga di
testo, posta direttamente sopra l'area di visualizzazione
principale. Per passare a un'altra cartella, fare
clic sul testo ed editare il percorso, oppure fare doppio
clic sul nome della cartella di destinazione.
* **Riga messaggi** 

Mostra il numero totale di file, cartelle e file nascosti
presenti nella cartella corrente.



# Disposizione


L'impostazione di Disposizione indica il modo in cui le icone verranno
disposte nella vista della Gestione di applicazioni.

* **Punto di collocazione** 

Gli oggetti si dispongono e rimangono
esattamente nel punto
in cui vengono collocati dall'utente.
* **Righe e colonne** 

Gli oggetti vengono allineati in righe e colonne
secondo una griglia ideale.



# Mostrare
struttura ad alberovisualizzazione ad albero (Gestione di applicazioni)

* **Per singola cartella** 

&newline;Viene mostrato il contenuto della cartella corrente.
* **Con struttura ad albero** 

Viene mostrato il contenuto della cartella
corrente,
usando una rappresentazione schematica ad albero.
* **Solo cartelle** 

Se è selezionata l'opzione Con struttura ad albero,
vengono
mostrate solo le cartelle. È l'impostazione predefinita.
* **Cartelle e poi file** 

Se è selezionata l'opzione Con struttura ad albero,
facendo clic sul simbolo più (+)
vengono mostrate prima le cartelle e quindi i file.
* **Cartelle e file** 

Se è selezionata l'opzione Con struttura ad albero,
vengono mostrate tutte le cartelle e i file.



# Rappresentazione


* **Solo per nome** 

Gli oggetti vengono rappresentati solo con il nome.
* **Per icone grandi** 

Ogni oggetto viene rappresentato con il nome e un'icona
grande.
(È l'impostazione predefinita.)
* **Per icone piccole** 

Ogni oggetto viene rappresentato con il nome e un'icona
piccola.elenco in forma dettagliata
* **Per nome, data, dimensione...** 

Ogni oggetto viene rappresentato con una serie di
informazioni:
nome, data di modifica, dimensioni, autorizzazioni e
gruppo.



# Ordinamento


Scegliere l'ordine in cui visualizzare file e cartelle:

* **Alfabetico** 

Scegliere Crescente (dalla A alla Z, quindi dalla a alla z)
oppure Decrescente (dalla Z alla A, quindi dalla z alla a).
(L'impostazione predefinita prevede l'ordinamento
alfabetico dalla A alla Z.)
* **Per tipo di file** 

I file vengono raggruppati in base al tipo di dati.
* **Per data** 

Scegliere Crescente (dal più vecchio al più recente) o
Decrescente (dal più recente al più vecchio).
* **Per dimensione** 

Scegliere Crescente (dal più piccolo al più grande)
oppure
Decrescente (dal più grande al più piccolo).

# Direzione


Scegliere la direzione in cui si desidera visualizzare i file e le
cartelle:

* **Crescente** 

Dal più vecchio al più recente, dal più piccolo al più
grande
* **Decrescente** 

Dal più recente al più vecchio, dal più grande al più
piccolo
