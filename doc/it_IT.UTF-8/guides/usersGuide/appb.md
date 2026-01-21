Uso di altre lingue nelle sessioniL'interfaccia utente del desktop pu&ograve; essere utilizzata in molte
lingue diverse. Ogni utente pu&ograve; modificare vari elementi come l'aspetto
degli schermi, la lingua predefinita, i font, il metodo di input e le icone.
&Egrave; inoltre possibile personalizzare e visualizzare in lingue diverse
i menu, l'aiuto online e i messaggi di errore.linguauso nelle sessionisessioneassociazione a una linguaLingua predefinita al loginI messaggi e i menu dello schermo di login compaiono nella lingua predefinita,
che inizialmente &egrave; una versione locale generica in lingua inglese
(&ldquo;C&rdquo;). Il menu Opzioni dello schermo di login consente di scegliere
una lingua diversa da quella predefinita.Login con scelta della lingua per la sessionelinguascelta al loginloginscelta della lingua per la sessioneIl cambiamento della lingua al login &egrave; molto semplice. Tuttavia,
per utilizzare senza difficolt&agrave; una sessione in un'altra lingua pu&ograve;
essere richiesta una speciale configurazione hardware, ad esempio per la
tastiera e la stampante. Tali requisiti dipendono dalla lingua, dal set di
caratteri e dal paese. Il software e i font possono migliorare ulteriormente
l'adattamento del sistema alla lingua locale. Per effettuare il login in
una sessione utilizzando la lingua locale:Usare il menu Opzioni dello schermo di login per selezionare
la lingua.L'elenco delle opzioni comprender&agrave; tutte le lingue supportate.Effettuare il login normalmente, inserendo il nome utente
e la parola chiave.Creazione o editazione di file in una lingua specificalinguaassociazione ai datiGli utenti possono creare, editare e stampare i file in qualunque lingua.
Anche i nomi dei file possono essere nella lingua locale; tuttavia, per quanto
riguarda i file di amministrazione condivisi in una rete, i nomi dovrebbero
contenere solo caratteri ASCII. I diversi sistemi di una rete possono usare
lingue differenti.Se al login &egrave; stata scelta una lingua specifica, tutte le applicazioni
avviate nella sessione utilizzeranno quella lingua. Sar&agrave; sempre possibile,
tuttavia, avviare un'applicazione usando un'altra lingua.Se ad esempio si desidera creare un file usando una lingua diversa,
aprire una nuova finestra dell'Editor di testo specificando la lingua desiderata.Creazione o editazione di file in una lingua specificacreatingfile con caratteri di una
lingua specificaeditazionedi file con caratteri di una lingua specificafilecon caratteri di una lingua specifica
[filelanguage-specificcaratteri in filePer creare o editare un file in una lingua specifica &egrave; possibile
avviare direttamente l'Editor di testo nella lingua desiderata oppure si pu&ograve;
impostare la variabile d'ambienteLANGprima di avviare l'Editor.Per avviare l'Editor di testo direttamente nella lingua desiderata,
invocare il comandodtpadcon la lingua specificata-xnllanguageper l'opzione. Ad esempio:/usr/dt/bin/dtpad -xnllanguageversione_locale_italianaPer impostareLANGprima
di invocare l'Editor, eseguire le operazioni seguenti:In una finestra di Terminale, impostare la variabile d'ambienteLANGin base alla lingua desiderata. Ad esempio,
per impostare l'italiano:Terminaleimpostazione della lingua tramiteLANG variabile d'ambienteLANG=versione_locale_italianadoveversione_locale_italianaspecifica
il set di caratteri italiano. Per conoscere il valore diversione_locale_italiana, vedere la documentazione relativa alla
piattaforma.Nella stessa finestra, invocare l'Editor di testo (dtpad) nella lingua desiderata digitando:Editor di testoavvio nella lingua specificastartingEditor di testo
nella lingua specificalingua
specificaEditor di testo/usr/dt/bin/dtpad &Se i file per la versione locale italiana sono installati sul sistema,
sar&agrave; possibile inserire il nuovo testo con i caratteri appropriati.
L'Editor di testo pu&ograve; anche essere utilizzato per editare un file creato
in precedenza in italiano.Per maggiori informazioni sull'impostazione di un set di caratteri,
vedere.Uso di un emulatore di terminale in una lingua
specificaL'esempio seguente si riferisce all'avvio di un emulatore di terminale
(dtterm) in lingua italiana. Si presume che la lingua predefinita
non sia l'italiano, che si utilizzi una Korn shell e che i file per la versione
locale siano installati sul sistema.emulatore di terminalelingua specificalingua specificaemulatore di terminaleavvioemulatore di terminale in lingua specificaDa una riga comandi di una finestra di Terminale in Korn shell,
digitare:LANG=versione_locale_italianadttermdoveversione_locale_italianaspecifica
il set di caratteri italiano. Per conoscere il valore diversione_locale_italianavedere la documentazione relativa alla piattaforma.Impostazione dei fontfontinternazionalizzazione<$startrange>internazionalizzazione
e fontIn genere, per modificare i font si utilizza la Gestione degli stili,
che a sua volta riavvia la Gestione dello spazio di lavoro per rileggere
le impostazioni del desktop. &Egrave; anche possibile, tuttavia, cambiare
i font dalla riga comandi o nei file di risorse. Se si utilizza un ambiente
internazionalizzato, si dovr&agrave; usare una definizione del font che sia
indipendente dalla tabella codici. Questo &egrave; necessario perch&eacute;
la definizione potr&agrave; essere usata in varie versioni locali con tabelle
codici diverse rispetto al set di caratteri (charset) del font. Per questo motivo, tutti gli elenchi di font dovrebbero
essere specificati con un set di font.Definizione di un fontfontdefinizioneLadefinizione di un fontpu&ograve; essere
un nome XLFD (X Logical Function Description) o un alias per il nomeXLFDXLFD. Qui di seguito sono riportate due
definizioni valide per un font a 14 punti:-dt-interface system-medium-r-normal-serif-*-*-*-*-p-*-iso8859-1Oppure,-*-r-*-14-*iso8859-1Definizione di un set di fontset di font, definizioneLadefinizione di un set di font&egrave; un elenco di nomi XLFD o
dei rispettivi alias (detto ancheelenco dei nomi base).
I nomi sono separati da punti e virgola; gli spazi vuoti prima o dopo il
punto e virgola vengono ignorati. Per agevolare l'indicazione dei nomi XLFD
si possono usare i caratteri speciali convenzionali.elenco dei nome base dei fontfontelenco dei nomi baseLa definizione di un set di font dipende dalla versione locale utilizzata.
Ad esempio, la versione locale giapponese definisce tre font (set di caratteri)
per visualizzare tutti i suoi caratteri. L'esempio seguente si riferisce
al set di font Mincho.Esempio di un elenco di nomi con set di caratteri:-dt-interface system-medium-r-normal-serif-*-*-*-*-p-*-14;
-dt-mincho-medium-r-normal--14-*-*-m-*-jisx0201.1976-0;
-dt-mincho-medium-r-normal--28-*-*-*-m-*-jisx0208.1983-0:Esempio di un singolo nome senza set di caratteri:-dt-*-medium-*-24-*-m-*:Per utilizzare i due esempi precedenti in una versione locale giapponese,
&egrave; necessario che i font indicati nell'elenco siano installati sul
sistema.Cambiamento dei fontI font didttermpossono essere cambiati
usando uno dei metodi seguenti:Specificando i font dalla riga comandiSpecificando i font in un file di risorseSpecificare i font dalla riga comandifontimpostazione dalla riga
comandi<$startrange>Per cambiare il font dei menu dalla riga comandi, digitare:dtterm -xrm '*fontList:set_di_font'doveset_di_font'&egrave; la definizione
di un set di font. Quest'ultima pu&ograve; essere specificata con un elenco
di nomi XLDF (X Logical Font Description), con un semplice schema XLFD o
con un alias. Si noti che la definizione del set di font dipende dalla versione
locale utilizzata.EsempiPer usare un font pi&ugrave; grande per il contenuto della finestra,
ma non per i menu, digitare:dtterm -xrm '*fontList:-dt-interface user-medium-r-normal-l*-*-*-*:'Per usare un font pi&ugrave; piccolo per il contenuto della finestra,
ma non per i menu, digitare:dtterm -xrm '*fontList:-dt-interface user-medium-r-normal-s*-*-*-*:'Queste definizioni funzioneranno per tutte le versioni locali.fontimpostazione dalla riga comandi<$endrange>Specificare i font in un file di risorsePur essendo possibile impostare i font editando i file di risorse dell'applicazionenella
directory/usr/dt/app-defaults/lingua, ci&ograve; non &egrave; consigliabile dato che tali file
vengono automaticamente sovrascritti ad ogni installazione. Piuttosto, si
consiglia di impostare i font aggiungendo le risorse al proprio file personale/.XdefaultsDirectoryIniziale.Scelta del metodo di input e della tastieraOgni versione locale &egrave; associata ad un metodo di input predefinito,
che viene selezionato automaticamente se l'utente non specifica un'impostazione
diversa. Le sezioni seguenti descrivono il modo in cui viene selezionato
il metodo di input tra quelli installati sul sistema.Oltre alle risorse utilizzate per impostare il metodo e lo stile di
input per la pre-editazione, &egrave; possibile utilizzare il controllo Gestione
degli StiliInternazionalizzazione