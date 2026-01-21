
# Introduzione generale all'Editor delle icone







# Funzioni dell'Editor delle icone
uso dell'Editor delle iconeEditor delle icone: usoeditazione di iconeicone: editazione

L'Editor delle icone permette di creare ed editare le immagini in due formati:

X pixmap (formato XPM)-- Immagini a colori statici
e dinamici. I file pixmap sono normalmente identificati
dall'estensione.pm.

X bitmap (formato XBM)-- Immagini monocromatiche. I file
bitmap sono normalmente identificati dall'estensione.bm.

Per creare un'immagine è necessario selezionare uno strumento e un colore e quindi eseguire il disegno vero e proprio nell'area di lavoro. Durante questa operazione, l'Editor delle icone visualizza una copia dell'icona in entrambi i formati. L'immagine, anche se creata per sistemi a colori, dovrebbe sempre risultare leggibile anche nella versione monocromatica; infatti, se non sono disponibili colori sufficienti per visualizzare la versione completa, l'icona viene convertita nel formato bitmap.
# Consigli per il disegno delle icone


È buona norma usare un motivo ricorrente o altre forme di analogia per accomunare le icone correlate tra loro. Ad esempio, nel disegnare le icone per una determinata applicazione, si può creare una somiglianza tra l'icona dell'applicazione e le icone dei file di dati associati a quell'applicazione.

Nel creare un'icona a colori, accertarsi che anche la versione in bianco e nero appaia in modo accettabile. Infatti, se l'icona viene visualizzata su un monitor monocromatico o a scala di grigi (o non sono disponibili colori sufficienti per visualizzarla nella forma originale), l'icona viene mostrata automaticamente in versione monocromatica.




# Uso dei colori
colori: uso nelle iconeicone: uso dei colori

Le icone del desktop usano una tavolozza di 22 colori:

Otto grigi statici

Otto colori statici: rosso, blu, verde, azzurro, magenta, giallo, nero
e bianco

Sei colori dinamici: colore del primo piano, colore dello sfondo,
ombreggiatura superiore, ombreggiatura inferiore, selezione e
trasparente

Questa tavolozza permette di creare icone di effetto e di
facile identificazione lasciando risorse di colore sufficienti
a disposizione delle altre applicazioni.
Le icone predefinite del desktop utilizzano in prevalenza
i toni del grigio con alcuni accenti di colore.

I colori dinamici permettono alle icone di cambiare colore quando
si seleziona una diversa tavolozza nella Gestione degli stili.

Il colore trasparente permette di creare icone apparentemente
non rettangolari, lasciando trasparire il colore sottostante
l'icona.
# Dimensioni consigliate per le icone
icone: dimensioni consigliatedimensioni delle icone

Qui di seguito sono indicate le dimensioni consigliate -- in pixel, larghezza&times;altezza -- per la creazione delle icone.

Gestione di file (grande):

Alta risoluzione: 32&times;32&newline;Media risoluzione:32&times;32&newline;Bassa risoluzione: 32&times;32

Gestione di file (piccola):

Alta risoluzione: 16&times;16&newline;Media risoluzione:16&times;16&newline;Bassa risoluzione: 16&times;16

Gestione di applicazioni (grande):

Alta risoluzione: 32&times;32&newline;Media risoluzione:32&times;32&newline;Bassa risoluzione: 32&times;32

Gestione di applicazioni (piccola):

Alta risoluzione: 16&times;16&newline;Media risoluzione:16&times;16&newline;Bassa risoluzione: 16&times;16

Panello principale:

Alta risoluzione: 48&times;48&newline;Media risoluzione:48&times;48&newline;Bassa risoluzione: 32&times;32

Pannelli secondari del pannello principale:

Alta risoluzione: 32&times;32&newline;Media risoluzione:32&times;32&newline;Bassa risoluzione: 16&times;16

Finestre ridotte a icona:

Alta risoluzione: 48&times;48&newline;Media risoluzione:48&times;48&newline;Bassa risoluzione: 32&times;32

Desktop:

Alta risoluzione: 32&times;32&newline;Media risoluzione:32&times;32&newline;Bassa risoluzione: 32&times;32

Le immagini per lo sfondo possono avere qualsiasi dimensione. Il motivo viene ripetuto fino a riempire l'intero spazio di lavoro.
# Vedere anche



# Convenzioni per i nomi delle icone
icone: ricerca dei filefile di immagini: vedere iconeicone: convenzioni per i nomiestensione.pmestensione.bmmappe di bit: ricercamappe di pixel: ricercaconvenzioni per i nomi dei file grafici

Le icone e le immagini per lo sfondo vengono memorizzate singolarmente in file separati. Per indicare un'icona si utilizza generalmente solo la parte principale del nome del file. Ad esempio, si potrà utilizzare il nome semplificatomailal posto del nome effettivo:/usr/dt/appconfig/icons/`lingua`/mail.l.pm

La convenzione di denominare i file con l'uso di suffissi consente di raggruppare le icone per tipo e dimensioni. Per i componenti del desktop si utilizzano in genere i seguenti formati:`nomebase`.`dimensione`.`formato``nomebase`.`formato`

dove`nomebase`è il nome dell'immagine di riferimento per l'icona,`dimensione`è un unico carattere indicante la dimensione dell'icona, e`formato`èpmper le icone in formato X pixmap obmper quelle in formato X bitmap.

Le dimensioni utilizzabili per le icone sono:

* **Nome** 

Dim. Suffisso
* **Molto piccola** 

16&times;16`t`
* **Piccola** 

24&times;24`s`
* **Media** 

32&times;32`m`
* **Grande** 

48&times;48`l`


Ad esempio, si supponga di voler utilizzare un'icona di nomemailper un nuovo tipo di file. Se si dispone di un monitor a colori e la Gestione di file è stata impostata per l'utilizzo di icone piccole, il nome assunto per l'icona saràmail.s.pm(lasrappresenta la dimensione piccola, mentrepmindica il formato`pixmap`, cioè il formato delle icone a colori).
# Percorsi di ricerca


Per determinare la directory in cui è memorizzata un'icona occorre ricercare il file in una serie di directory. Questo insieme di directory viene definito da un "percorso di ricerca" specifico per le icone.

Se si utilizza un monitor a colori, il desktop ricerca inizialmente i file che terminano in.pm. Diversamente, vengono ricercati i file che terminano con il suffisso.bm.

Se insieme al nome del file viene specificato il percorso completo, il percorso di ricerca non viene utilizzato.
# Vedere anche


per informazioni specifiche sui percorsi di ricerca.
# Memorizzazione delle icone
memorizzazione dei file delle iconefile delle icone: memorizzazione

Se non vengono fornite informazioni specifiche, i componenti del desktop cercano i file delle icone nelle directory seguenti:

Icone personali:/`DirectoryIniziale`/.dt/icons

Icone di sistema:/etc/dt/appconfig/icons/`lingua`

Icone predefinite:/usr/dt/appconfig/icons/`lingua`

Per cercare le icone della versione inglese, al posto di`lingua`usareC.
# Memorizzazione delle immagini per lo sfondo
sfondo: memorizzazione dei file grafici per lo sfondofile grafici: memorizzazione dei file per lo sfondo

Ognuno degli sfondi elencati nel riquadro di dialogo Sfondo schermo della Gestione degli stili rappresenta un file di un'immagine. Se non vengono fornite informazioni specifiche, le immagini per lo sfondo vengono cercate nelle directory seguenti:

Immagini di sistema:/etc/dt/appconfig/backdrops/`lingua`

Immagini predefinite:/usr/dt/backdrops/`lingua`

Se si crea una nuova immagine per lo sfondo dello schermo e si desidera renderla accessibile a tutti gli utenti del sistema, memorizzare l'immagine nella directory/etc/dt/appconfig/backdrops/`lingua`.
Per limitare l'accesso all'immagine a determinati utenti, memorizzarla in una directory differente ed aggiungere tale directory alla risorsa*backdropDirectoriesper quegli utenti.