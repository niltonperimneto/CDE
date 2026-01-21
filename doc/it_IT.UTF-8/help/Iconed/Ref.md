
# Riferimenti sull'Editor delle icone











# Percorsi di ricerca delle icone
percorsi di ricerca delle iconeicone: percorsi di ricerca

Il percorso di ricerca per i file delle immagini è definito da due variabili d'ambiente:

Il sistema utilizza tali variabili, DTUSERAPPSEARCHPATH e DTAPPSEARCHPATH, a meno che non esistano le variabili seguenti:

* **DTUSERICONSEARCHPATH** 

Una variabile personale. Se utilizzata,
deve essere definita in/`DirectoryIniziale`/.dtprofile.
* **DTICONSEARCHPATH** 

Una variabile di sistema.

# Percorso di ricerca predefinito


Il valore predefinito per DTICONSEARCHPATH è:/`DirectoryIniziale`/.dt/icons/etc/dt/appconfig/icons/C/usr/dt/appconfig/icons/C
# Per modificare il percorso di ricerca delle icone personali
aggiunta di directory al percorso di ricerca delle azionidirectory: aggiunta al percorso di ricerca delle azioniazioni: percorso di ricerca, aggiunta di directorypercorso di ricerca delle azioniricerca: percorso di ricerca delle azioniDTACTIONSEARCHPATH, variabile d'ambientevariabile d'ambiente DTACTIONSEARCHPATH

Per aggiungere una directory al percorso di ricerca, aprire con un editor il file/`DirectoryIniziale`/.dtprofilee aggiungere una riga che definisca un valore per la variabile DTUSERICONSEARCHPATH:DTUSERICONSEARCHPATH=`percorso`[,`percorso`...]
# Per modificare il percorso di ricerca delle icone di sistema


Le variabili che definiscono il percorso di ricerca a livello di sistema sono specificate in appositi file inclusi nella directory/etc/dt/Xsession.d.

Se la variabile DTICONSEARCHPATH è già definita in un file di questa
directory, editare il valore esistente aggiungendo la nuova directory
al percorso di ricerca.

Se DTICONSEARCHPATH non è ancora definita in questa directory, definire
la variabile includendo il percorso di ricerca predefinito e gli altri
percorsi che si desidera aggiungere. (Il percorso di ricerca predefinito
è specificato in una riga commentata nel file/usr/dt/bin/dtsearchpath.)
# Strumenti dell'Editor delle icone
strumenti dell'Editor delle iconeEditor delle icone: strumentistrumenti di disegno

Gli strumenti dell'Editor delle icone, una volta selezionati, restano attivi fino a quando si seleziona un altro strumento. Qui di seguito sono elencati e descritti gli strumenti disponibili:

Matita&newline;Per disegnare linee e punti a mano libera.

Linea&newline;Per disegnare linee rette.

Rettangolo&newline;Per disegnare rettangoli, vuoti o pieni.

Cerchio&newline;Per disegnare cerchi, vuoti o pieni.

Gomma&newline;Per cancellare parti dell'immagine.

Riempimento&newline;Per riempire un'area con il colore selezionato.

Poligono aperto&newline;Per disegnare linee rette concatenate.

Poligono chiuso&newline;Per disegnare linee rette concatenate unendo la prima e l'ultima in modo da formare un poligono chiuso.

Ellisse&newline;Per disegnare ellissi, vuoti o pieni.

Selezione&newline;Per effettuare una selezione primaria, richiesta per eseguire diversi comandi del menu Editare.
# Riempimento


Selezionando la casella Riempimento, i rettangoli, i cerchi, i poligoni e gli ellissi vengono disegnati con l'area piena anziché solo con il perimetro, come indicato dalle caselle dei relativi strumenti.
(Vedere anche).
# Menu dell'Editor delle icone











# Menu File dell'Editor delle icone




* **Nuovo** 

Richiede una dimensione (larghezza e altezza, in pixel), e
crea un'area di disegno vuota della dimensione specificata.
(Vedere.)
* **Aprire** 

Apre un file bitmap o pixmap esistente.
(Vedere.)
* **Salvare** 

Salva l'icona corrente usando il nome già definito. Se
l'icona non ha ancora un nome, viene richiesto di
specificarne uno. (Vedere.)
* **Salvare come** 

Richiede un nuovo nome e salva l'icona corrente con il
nome specificato. (Vedere.)
* **Chiudere** 

Chiude l'Editor delle icone. Se non tutte le modifiche
sono state salvate, compare un messaggio per avvertire
che l'uscita causerà la perdita di queste modifiche.
Per salvarle prima di uscire, scegliere Annullare e
quindi salvare l'icona.

# Menu Editare dell'Editor delle icone




* **Annullare** 

Annulla l'ultima operazione eseguita e ripristina l'icona
allo stato precedente. È possibile annullare la maggior
parte delle operazioni.
(Vedere.)
* **Tagliare area** 

Taglia l'area selezionata dell'icona. L'area vuota restante
viene riempita con il colore trasparente, mentre l'area
tagliata viene salvata nel blocco appunti dell'Editor delle
icone. (Vedere.)
* **Copiare area** 

Copia l'area selezionata nel blocco appunti dell'Editor
delle icone. (Vedere.)
* **Incollare area** 

Incolla il contenuto del blocco appunti nell'icona.
(Vedereo.)
* **Ruotare area** 

Ruota l'area selezionata di 90 gradi verso sinistra o
verso destra. La direzione può essere scelta dal menu
secondario. (Vedere.)
* **Capovolgere area** 

Crea un'immagine speculare dell'area selezionata
capovolgendola verticalmente o orizzontalmente. La direzione
può essere scelta dal menu secondario.
(Vedere.)
* **Riprodurre in scala area** 

Permette di ridimensionare (o modificare la "scala")
dell'area selezionata. (Vedere.)
* **Ridimensionare icona** 

Richiede una nuova dimensione per l'icona corrente.
(Vedere.)
* **Aggiungere punto focale** 

Permette di specificare un determinato pixel dell'icona
corrente come "punto focale". Dopo avere selezionato
Aggiungere punto focale, fare clic sul pixel da designare
come punto focale.

I punti focali vengono usati nel creare le immagini per
i puntatori del mouse -- il punto focale rappresenta la
posizioneeffettivadel puntatore.
(Vedere.)
* **Cancellare punto focale** 

&newline;Cancella il punto focale dall'icona corrente.
* **Catturare immagine dello schermo** 

Cattura un'area dello schermo e la carica nell'area di
disegno.
(Vedere.)
L'indicatore X-Y situato sopra l'area di lavoro
mostra la dimensione in pixel dell'area catturata.
* **Cancellare icona** 

Cancella il contenuto dell'area di disegno corrente.
(Vedere.)

# Menu Opzioni dell'Editor delle icone




* **Griglia visibile** 

Abilita e disabilita la griglia nell'area di
disegno. L'impostazione predefinita è con la
griglia attivata.
* **Formato di uscita** 

Determina il formato in cui verrà salvata l'icona:

XBM per il formato X bitmap a due colori. I file
bitmap terminano solitamente con il suffisso.bm.

XPM per il formato X pixmap a colori (impostazione
predefinita). I file pixmap terminano solitamente
con il suffisso.pm.

Vedere.
* **Ingrandimento** 

Cambia la dimensione di visualizzazione dell'immagine
nell'area di disegno. Si può scegliere un fattore
di ingrandimento compreso tra 2x (due volte la
dimensione
normale) e 12x (dodici volte la dimensione normale).

# Menu Aiuto dell'Editor delle icone




* **Panoramica** 

Presenta una descrizione introduttiva dell'Editor delle icone
* **Attività** 

Visualizza un sommario delle principali operazioni
eseguibili con l'Editor delle icone
* **Riferimenti** 

Mostra un sommario di riferimenti a varie caratteristiche
dell'Editor delle icone, come finestre, riquadri di
dialogo, menu e risorse
* **Sull'elemento** 

Presenta una descrizione dell'elemento selezionato
nella finestra dell'Editor delle icone
* **Uso dell'aiuto** 

Contiene informazioni sull'utilizzo delle finestre dell'aiuto
* **Informazioni sull'Editor delle icone** 

&newline;Mostra la versione e le informazioni di copyright
per l'Editor delle icone

# Finestre e riquadri di dialogo dell'Editor delle icone

# Subtopics







# Finestra principale dell'Editor delle icone


La finestra principale dell'Editor delle icone contiene cinque aree importanti:

Lariga di statoposta direttamente sotto della barra
dei menu indica lo strumento correntemente selezionato e le coordinate
del pixel su cui si trova il puntatore in quel momento.

L'area di disegnoè l'area in cui viene disegnata
l'immagine dell'icona.

Ilpannello degli strumentiraccoglie gli strumenti
di disegno disponibili, inclusi una gomma e uno strumento di selezione.

Latavolozza dei coloripresenta i colori disponibili
per il disegno dell'icona: otto colori statici, otto grigi statici
e sei colori dinamici.

L'area di visualizzazione in formato realemostra
in che modo l'icona comparirà effettivamente sullo schermo. Vengono
presentate sia la versione a colori che quella monocromatica.
# Tavolozze di colori dell'Editor delle icone




* **Colori statici** 

Otto colori standard; nero,
bianco, i tre colori primari
e i tre colori secondari
* **Grigi statici** 

Otto gradazioni di grigio
(dal 10% al 90%)
* **Colori dinamici** 

Sei colori dinamici che cambiano
in base alle impostazioni dei
colori nella Gestione degli stili


Per maggiori informazioni, consultare le sezioni seguenti dell'aiuto sulla Gestione degli stili:

Selezionare una tavolozza

Cambiare il numero dei colori usati dal desktop.
# Riquadri di dialogo Apertura file e Salvataggio come &newline; dell'Editor delle icone


* **Inserire il percorso o il nome della cartella:** 

&newline; Digitare il percorso completo della
cartella che contiene l'icona da aprire, o di quella in
cui si desidera salvare l'icona.
* **Cartelle** 

Mostra le cartelle contenute nella directory che compare in
"Inserire il percorso o il nome della cartella".
Facendo doppio clic su una cartella, gli elenchi Cartelle e File mostreranno
gli elementi contenuti in quella cartella. In alternativa, selezionare una
cartella dall'elenco e quindi fare clic sul pulsante Aggiornare.
* **File** 

Mostra i file contenuti nella directory che compare in
"Inserire il percorso o il nome della cartella". Se in questo campo di testo si specifica il nome di un'altra directory, per visualizzare il relativo contenuto occorrerà fare clic sul pulsante Aggiornare.
* **Inserire il nome del file:** 

Mostra il nome dell'icona che verrà caricata o salvata.
Il modo più semplice per specificare un'icona è quello di fare doppio clic sul suo nome nell'elenco File. In alternativa, digitare il nome dell'icona desiderata e quindi fare clic sul pulsante Aprire.

Si noti che
il formato corretto per i nomi delle icone è nome.dimensione.formato. Per il funzionamento corretto di un'icona, è necessario che le informazioni sulle dimensioni e sul formato siano incluse nel nome. L'Editor delle icone dovrebbe inserire automaticamente i valori corretti in base alla dimensione e al formato di uscita selezionati dalla barra dei menu.
* **Aprire o Salvare** 

&newline; Apre o salva il file e chiude il riquadro di dialogo.
* **Aggiornare** 

Aggiorna gli elenchi Cartelle e File in base al contenuto
della directory specificata in Inserire il percorso o il nome della
cartella. È possibile digitare nel campo di testo il nome della cartella
da visualizzare e quindi fare clic sul pulsante Aggiornare. Oppure, se la
directory compare nell'elenco Cartelle, si potrà fare doppio clic sul suo
nome.
* **Annullare** 

Annulla l'operazione di apertura o di salvataggio e chiude
il riquadro di dialogo.
* **Aiuto** 

Visualizza informazioni di aiuto sul riquadro di dialogo.

# Riquadro di dialogo Salvataggio come dell'Editor &newline; delle icone




* **Inserire il percorso o il nome della cartella:** 

&newline; Digitare il percorso
completo della cartella in cui si desidera salvare l'icona.
* **Cartelle** 

Mostra le cartelle contenute nella directory che compare in
"Inserire il percorso o il nome della cartella". Facendo
doppio clic su una cartella, gli elenchi Cartelle e File
mostreranno gli elementi contenuti in quella cartella. In
alternativa, selezionare una cartella dall'elenco e quindi
fare clic sul pulsante Aggiornare.
* **File** 

Mostra i file contenuti nella directory che compare in "Inserire
il percorso o il nome della cartella". Se in questo campo di testo si
specifica il nome di un'altra directory, per visualizzare il relativo
contenuto occorrerà fare clic sul pulsante Aggiornare.
* **Inserire il nome del file:** 

Digitare il nome dell'icona da salvare.
Si noti che il formato corretto per i nomi delle icone è
nome.dimensione.formato. Per il funzionamento corretto di un'icona, è
necessario che le informazioni sulle dimensioni e sul formato siano
incluse nel nome. L'Editor delle icone dovrebbe inserire automaticamente
i valori corretti in base alla dimensione e al formato di uscita
selezionati dalla barra dei menu.
* **Salvare** 

Salva il file e chiude il riquadro di dialogo.
* **Aggiornare** 

Aggiorna gli elenchi Cartelle e File in base al contenuto
della directory specificata in "Inserire il percorso o il nome della
cartella". È possibile digitare nel campo di testo il nome della cartella
da visualizzare e quindi fare clic sul pulsante Aggiornare. Oppure, se la
directory compare nell'elenco Cartelle, si potrà fare doppio clic sul suo
nome.
* **Annullare** 

Annulla l'operazione di salvataggio e chiude il riquadro di dialogo.
* **Aiuto** 

Visualizza informazioni di aiuto sul riquadro di dialogo.

# Riquadro di dialogo di conferma dell'Editor delle icone


Il riquadro di dialogo di conferma chiede all'utente di confermare l'esecuzione del comando richiesto (ad esempio l'uscita dall'Editor delle icone senza salvare) per impedire la perdita accidentale dei dati.

Fare clic su OK per continuare, oppure su Annullare per abbandonare l'operazione.