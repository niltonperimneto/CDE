Nozioni generali sulla Gestione di fileLe sezioni seguenti contengono alcune nozioni generali sul funzionamento
della Gestione di file.File system gerarchicogerarchia di file systemfile system, gerarchiafile: definizioneIl concetto di file system gerarchico può essere poco chiaro
per chi ha scarsa familiarità con i computer. Questa sezione descrive
i componenti base del file system gerarchico.Cos'è un file?Unfileè un contenitore
di informazioni, solitamente dati, organizzati in un formato particolare &emdash;
si può trattare di un documento, di un foglio elettronico, di un diagramma,
ecc. Il formato è il modo specifico in cui i dati sono organizzati
all'interno del file, e può essere definito come il tipo di dati del
file.Quando la Gestione di file è in una delle modalità di
visualizzazione ad icone, il tipo di dati di un file può essere identificato
dall'icona che lo rappresenta.
Ogni tipo di dati è infatti associato a un'icona diversa.La maggior parte delle applicazioni riconosce un numero limitato di
tipi di dati. Ad esempio, un editor di testo non potrà leggere il file
di un foglio elettronico. Il desktop aiuta a distinguere i vari tipi di file
usando un database deitipi di dati. Ogni tipo di dati
identifica i file di un determinato formato e li associa alle applicazioni
appropriate. In genere, facendo doppio clic su un file il desktop avvia automaticamente
l'applicazione appropriata per il tipo di dati contenuto nel file.La lunghezza massima del nome di un file varia da sistema a sistema.
Alcuni sistemi operativi non accettano nomi più lunghi di 14 caratteri.
Per informazioni specifiche, rivolgersi all'amministratore di sistema.Cos'è una cartella?cartella: definizioneUnacartellaè un contenitore di file.
La sua funzione è ben rappresentata dall'icona, che ha la forma di
cassetto d'archivio. Ogni cartella può contenere altre cartelle, denominatecartelle secondarie. Le cartelle e le cartelle secondarie vengono
solitamente organizzate in più livelli, formando una struttura gerarchica.
(In altri contesti, le cartelle vengono spesso chiamate directory.)Se si provasse a disegnare una gerarchia di cartelle con tutte le cartelle
secondarie al di sotto della rispettiva cartella di appartenenza (la cartella
principale o precedente) e a tracciare una linea tra ogni cartella e quella
superiore, l'immagine risultante assomiglierebbe ad un albero capovolto. Per
questa ragione, la gerarchia delle cartelle viene spesso chiamataalberodelle cartelle.All'interno di ogni cartella, ogni file deve avere un nome unico. Questa
regola non si applica ai file contenuti in cartelle differenti.Negli spostamenti da una cartella all'altra, la posizione corrente viene
definita comecartella corrente.Cos'è un percorso?percorso: definizioneLa posizione di un file viene spesso specificata elencando i nomi
delle cartelle e delle cartelle secondarie che portano a quella posizione &emdash;
questo elenco viene chiamatopercorso.
(Vedere.) Nella Gestione di file, il percorso
di un file è indicato in due modi: come sequenza di cartelle nella
rappresentazione ad icone, e in forma di testo nella riga al di sopra dell'area
di visualizzazione.Tipi di percorso e nomi dei percorsiIl percorso di un oggetto specifica la posizione di quell'oggetto nel
file system. Esistono tre modi per specificare tale posizione: il percorso
assoluto, il percorso relativo e il percorso completo.Percorso assolutoUn percorso si diceassolutoquando inizia
dallacartella radice, cioè dalla cartella situata
più in alto nella struttura gerarchica. Se un percorso inizia con una
barra (/), si tratta di un percorso assoluto
specificato a partire dalla cartella radice. Ad esempio, il percorso seguente
è il percorso assoluto del filelettera:/usr/dt/config/letteraPercorso relativoUn percorso si dicerelativoquando descrive
la posizione del file o della cartella in relazione alla cartella corrente.
Se ci si trova in una cartella e si desidera spostarsi a un livello inferiore
della gerarchia, non occorre digitare l'intero percorso. È sufficiente
specificare la parte che inizia con la cartella successiva a quella corrente.
Se un percorso non inizia con una barra, significa che si tratta di un percorso
relativo. Ad esempio, se la cartella corrente è/usr/dte si desidera spostarsi nella cartella ''/usr/dt/config/lettere'',
si potrà utilizzare il seguente percorso relativo:config/lettere.. (livello superiore)Nello specificare i percorsi relativi possono essere utili due nomi
speciali.
La cartella.(detta anche "punto") rappresenta
la cartella corrente. La cartella..(detta
anche "punto-punto") rappresenta la cartellaprecedente&emdash;
cioè la cartella di livello superiore nella gerarchia. Ad esempio,
se la cartella corrente è/usr/dt/config/panels,
il percorso relativo del filesys.dtwmrc,
che si trova nella cartella/usr/dt/config,
un livello più in alto rispetto alla cartella corrente, sarà:../sys.dtwmrcVedere ancheProprietà e sicurezza degli oggettiGli oggetti del desktop sono accessibili a tre gruppi di utenti:proprietario,gruppoealtri.
Si distinguono inoltre tre funzioni di accesso:lettura,scritturaedesecuzione.Chi può accedere agli oggetti?Le tre classi fondamentali di utenti sono:ProprietarioGeneralmente la persona che ha creato il file.GruppoInsieme di utenti raggruppati dall'amministratore di sistema. Ad esempio,
un gruppo può essere formato dai membri di un dipartimento.AltriTutti gli altri utenti del sistema.Quali sono i tipi di accesso?Le autorizzazioni di accesso ad un file specificano il modo in
cui il proprietario, i membri del gruppo e gli altri utenti possono operare
su quel file.Autorizzazione di letturaConcede l'accesso per copiare o visualizzare il contenuto dell'oggetto.