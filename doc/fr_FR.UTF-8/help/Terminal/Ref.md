Terminal - RéférencePour obtenir de l'aide sur un sujet particulier, cliquez dessus.Page de manuel dtterm (1X)Voir aussiX(1)resize(1)xset(1)xterm(1)SYNOPSISdtterm[-options]DESCRIPTIONLe clientdttermpeut exécuter
les applications existantes écrites pour des terminaux conformes aux
normes ANSI X3.64-1979 et ISO 6429:1992(E) (le terminal DEC VT220, par exemple).OPTIONSL'émulateur de terminaldttermaccepte toutes les options standard entrées sur la ligne de commande
X Toolkit ainsi que celles répertoriées ci-après. Les
options précédées d'un `+' (au lieu d'un `-') recouvrent
leur valeur par défaut:-132La séquence d'échappement DECCOLM, qui permet
d'alterner entre le mode 80 colonnes et le mode 132 colonnes, est désactivée
par défaut. Si vous choisissez l'option-132,
la séquence DECCOLM est activée et la fenêtredttermest redimensionnée en conséquence.c132Ressource associ&eacute;e:c132.+132Cette option désactive la séquence d'échappement
DECCOLM (mode de fonctionnement par défaut).c132Ressource associ&eacute;e:c132.-awCette option active le retour ligne automatique. Lorsque
ce dernier est actif, le curseur revient automatiquement au début de
la ligne suivante à la fin de chaque ligne. Il s'agit du mode de fonctionnement
par défaut.Ressource associ&eacute;e:autoWrap.autoWrap+awCette option désactive le retour ligne automatique.Ressource associ&eacute;e:autoWrap.autoWrap-backgroundcouleurCette option indique la couleur d'arrière-plan
de la fenêtre de terminal et celle utilisée par défaut
pour la barre de défilement et le pointeur X11. Dans le Bureau CDE,
la couleur d'arrière-plan s'aligne par défaut sur le pixel de
sélection des couleurs primaires ou sur le pixel d'arrière-plan
(voir-bs).
En dehors du Bureau, cette option prend par défaut la valeur*background/*Backgroundqui équivaut, en l'absence
de toute autre indication, à la couleur noir.couleurspécifie
la couleur d'arrière-plan à utiliser.Arrière-planRessource
associ&eacute;e:background-bdcouleurCette option indique la couleur du cadre des fenêtres.
Le cadre du widget de shell peut ne pas être visible si vous utilisez
des gestionnaires de fenêtres apparentés commedtwm(1X)etmwm(1X). La
couleur par défaut est le noir.couleurspécifie
la couleur à utiliser pour le cadre.borderColorRessource
associ&eacute;e:borderColor.-bgcouleurCette option est identique à-background.couleurspécifie
la couleur d'arrière-plan à utiliser.backgroundRessource
associ&eacute;e:background.-bordercolorcouleurCette option est identique à -bd (voir ci-dessus).couleurspécifie la couleur
à appliquer au cadre.borderColorRessource
associ&eacute;e:borderColor.-borderwidthlargeur_cadreCette option indique la largeur du cadre de la fenêtre
du widget de shell. Cette valeur peut être ignorée dans un gestionnaire
de fenêtres apparenté (dtwm(1X)etmwm(1X), par exemple). La valeur par défaut
est 0.largeur_cadrespécifie
la largeur, en pixels, du cadre de la fenêtre.borderwidthRessource
associ&eacute;e:borderWidth.-bsCette option indique que la fenêtre de terminal
doit utiliser la couleur de sélection de l'environnement Motif au lieu
de sa couleur d'arrière-plan habituelle. Cette option est sélectionnée
par défaut.backgroundIsSelectRessource
associ&eacute;e:backgroundIsSelect.+bsCette option indique que la fenêtre du terminal
ne doit pas utiliser la couleur de sélection de l'environnement Motif
au lieu de sa couleur d'arrière-plan habituelle.Ressource associ&eacute;e:backgroundIsSelect.-bwlargeur_cadreCette option est identique à-borderwidth(voir ci-dessus).Ressource associ&eacute;e:borderWidth.-CCette option indique que l'affichage doit être dirigé
vers la fenêtre de terminal et non vers/dev/console. Elle permet d'éviter que le contenu de l'écran
ITE ne vienne écraser celui du serveur X. Elle n'a pas pour vocation
d'être utilisée de façon systématique pour rediriger
l'affichage de la console/dev/consoled'un
système choisi arbitrairement vers un serveur X choisi lui aussi arbitrairement.
Cette option ne fonctionne que si vous êtes propriétaire de/dev/consoleet que vous y avez accès en lecture-écriture.-displaynom_écranCette option indique le serveur d'affichage X11 quedttermdoit utiliser.
Par défaut, sa valeur est donnée par la variable d'environnement
$DISPLAY.nom_écranindique
le nom du serveur X11 auquel se connecter.display-eargument_programme ...Cette option spécifie le nom du programme exécutable
à appeler en tant que processus secondaire lors du lancement dedtterm. Cette option doit figurer en dernière position
sur la ligne de commande.argument_programme ...spécifie le programme et les arguments de ligne de commande à
exécuter.-fbpoliceCette option spécifie le jeu de polices (variable
XFontSet) dans lequel le texte en gras doit être affiché sur
le terminal. Ce jeu doit être choisi dans la liste des polices Motif
(variable XmFontList). Seuls les caractères et les polices à
chasse fixe sont pris en charge. Si vous utilisez des polices proportionnelles,
vous obtiendrez des résultats imprévisibles. La police générée
par défaut en caractères gras est fonction de la désignation
XLFD de la police utilisateur (variable userFont). Si cette police n'est pas
disponible, la mise en gras sera obtenue par impression en double (avec un
décalage d'un pixel) de la police utilisateur.policespécifie le jeu de
polices (variable XFontSet) que doit utiliser le terminal pour l'affichage
du texte en gras.Ressource associ&eacute;e:userFont.-fgcouleurCette option indique la couleur de premier-plan de la
fenêtre de terminal ainsi que celle utilisée par défaut
pour la barre de défilement et le pointeur X11. Dans le Bureau CDE,
la couleur de premier plan s'aligne par défaut sur le pixel de premier
plan des couleurs primaires. En dehors du Bureau, cette option prend par défaut
la valeur*foregroundou*Foregroundqui équivaut, en l'absence de toute autre
indication, à la couleur blanc.couleurspécifie
la couleur de premier-plan à utiliser.foregroundRessource
associ&eacute;e:foreground.-fnpoliceCette option spécifie le jeu de polices (variable
XFontSet) dans lequel le texte doit être affiché sur le terminal.
Ce jeu doit être choisi dans la liste des polices Motif (variable XmFontList).
Seuls les caractères et les polices à chasse fixe sont pris
en charge. Si vous utilisez des polices proportionnelles, vous obtiendrez
des résultats imprévisibles. Les éléments de texte
n'apparaissant pas à proprement parler dans la fenêtre de terminal
(barre de menus, menus instantanés, boîtes de dialogue, etc.)
n'utilisent pas cette police. Par défaut, la valeur de la variable
XmNtextFontList associée au tableau d'affichage père (voir XmBulletinBoard(3X))
est utilisée de la même façon que le widget XmText.policeindique le jeu de polices
(variable XFontSet) à utiliser sur le terminal.Ressource associ&eacute;e:userFont.-fontpoliceCete option est identique à-fn(voir ci-dessus).policespécifie le jeu de
polices (variable XFontSet) à utiliser sur le terminal.Ressource associ&eacute;e:userFont.-foregroundpremier planCette option est identique à-fg(voir ci-dessus).premier planindique
la couleur de premier plan à utiliser.foregroundRessource
associ&eacute;e:foreground.-geometrygéométrieCette option indique les dimensions et la position privilégiées
de la fenêtre de terminal. Par défaut, les dimensions de la fenêtre
sont de 24 lignes de 80 caractères. En revanche, aucune position par
défaut n'est définie.géométriespécifie la géométrie à appliquer au terminal.geometryRessource
associ&eacute;e:geometry.-helpAffiche un message d'aide sur l'utilisation dedtterm.help-iconicLorsque cette option est activée, l'émulateur
de terminal apparaît initialement à l'écran sous forme
réduite.iconicRessource
associ&eacute;e:iconic.+iconicLorsque cette option est activée, l'émulateur
de terminal apparaît comme une fenêtre normale à l'écran.
Il s'agit de l'option par défaut.Ressource associ&eacute;e:iconic.-jCette option active le défilement discontinu. Dans
ce cas, il est possible de faire défiler l'écran de plusieurs
lignes à la fois et, par conséquent, d'accélérer
la vitesse de régénération des écrans lorsque
plusieurs lignes de texte sont envoyées au terminal. Le nombre de lignes
qu'il est possible de faire défiler est limité au nombre de
lignes contenues dans la fenêtre de terminal. En tout état de
cause, les lignes sont toujours affichées dans leur totalité.
Il s'agit de l'option par défaut.jumpScrollRessource
associ&eacute;e:jumpScroll.+jCette option désactive le défilement discontinu.
Pour plus de détails sur cette fonction, reportez-vous à l'option-jdécrite plus haut.Ressource associ&eacute;e:jumpScroll.-kshModeCette option active le mode ksh. Dans ce mode, la frappe
d'une touche utilisant le jeu étendu à bit de modification génère
un caractère d'échappement suivi du caractère généré
par la frappe de la touche non étendue. Cette option est destinée
à être utilisée avec emacs et avec l'éditeur emacs
lancé sur la ligne de commande, en modeksh(1)ouied(1). Elle n'est pas compatible avec
l'utilisation normale de la métatouche pour obtenir des caractères
étendus codés sur un octet et les caractères codés
sur plusieurs octets (pays asiatiques).kshModeRessource associ&eacute;e:kshMode.+kshModeCette option désactive le mode ksh (configuration
par défaut).Ressource associ&eacute;e:kshMode.-lCette option active la consignation de données
: toutes les données reçues du processus secondaire sont consignées
soit dans un fichier, soit dans un pipeline de commandes (voir description
de l'option-lfplus bas). Les données
sont consignées directement à partir du processus secondaire
: elles comprennent par conséquent les caractères d'échappement
et les combinaisons retour chariot/avance ligne transmis, selon le mode de
gestion des lignes du terminal. L'activation et la désactivation de
l'affichage s'effectuent via les séquences d'échappement.loggingRessource
associ&eacute;e:logging.+lCette option désactive la consignation des données.
Pour plus de détails, reportez-vous à la description de l'option-lplus haut. Par défaut, les données ne sont
pas consignées.loggingRessource
associ&eacute;e:logging.-lfnom_fichierCette option indique le nom du fichier de consignation.
Sinom_fichiercommence par le symbole|, le restant de la chaîne est traité comme
une commande à exécuter une fois l'extrémité du
tube atteinte. Par défaut, le fichier s'appelleDttermLogXXXXX(oùXXXXXdésigne l'ID de processus dedtterm) et réside dans le répertoire à
partir duqueldtterma été
lancé. Si les cinq derniers caractères sont "XXXXX," ils sont
remplacés par l'ID du processus.nom_fichierspécifie
le nom du fichier journal à utiliser.logFileRessource
associ&eacute;e:logFile.-lsCette option indique que le shell lancé doit être
un shell de connexion. Le premier caractère de l'expressionargv[0]est par conséquent un tiret, ce qui signale
au shell qu'il doit lire le fichierprofiledu système ainsi que le fichier$HOME/.profilede l'utilisateur (pourkshetsh), ou bien le fichiercsh.logindu système et le fichier$HOME.loginde l'utilisateur (pourcsh).loginShellRessource
associ&eacute;e:loginShell.+lsCette option indique que le shell à lancer doit
être un shell ordinaire (par opposition à un shell de connexion),
ce qui correspond à la configuration par défaut.loginShellRessource
associ&eacute;e:loginShell.-mapCette option indique quedttermne doit plus, s'il y a lieu, s'afficher sous forme réduite lors de
l'affichage des résultats du processus secondaire. La ressourcemapOnOutputDelaypermet de définir un délai entre l'affichage des résultats
et la restauration à l'état de fenêtre.mapOnOutputRessource
associ&eacute;e:mapOnOutput.+mapCette option indique qu'aucune restauration/réduction
spécifique ne doit intervenir. Par défaut, elle est activée.mapOnOutputRessource
associ&eacute;e:mapOnOutput.-mbLorsque cette option est activée,dttermémet un signal sonore lorsque la marge droite
est sur le point d'être atteinte. La distance réelle à
laquelle ce signal est émis est indiquée par l'option-nb.marginBellRessource
associ&eacute;e:marginBell.+mbCette option désactive l'avertissement de marge,
ce qui correspond à la configuration par défaut.marginBellRessource
associ&eacute;e:marginBell.-mscouleur_pointeurCette option indique la couleur de premier plan à
utiliser pour le pointeur X11 de la fenêtre de terminal. Par défaut,
la couleur de premier plan du pointeur est alignée sur celle de la
fenêtre de terminal. Pour plus de détails, voir la description
de la ressourceforegroundplus haut.couleur_pointeurspécifie
la couleur de premier plan à appliquer au pointeur.Ressource associ&eacute;e:pointerColor.-namenom_programmeCette option indique le nom X11 de la fenêtredtterm.nom_programmeest le nom à
utiliser.name-nbnombreCette option indique le nombre de caractères précédant
la marge droite à partir duquel l'avertissement de marge sera déclenché,
si l'option correspondante a été activé. Par défaut,
l'avertissement de marge est déclenché à dix caractères
de la marge droite.nMarginBellRessource
associ&eacute;e:nMarginBell.-rLorsque cette option est activée, les couleurs
de premier et d'arrière-plan de la fenêtredttermsont inversées. Elle est équivalente aux options-rvet-reverse.+rLorsque cette option est activée, la fenêtredttermest affichée dans ses couleurs de premier et
d'arrière-plan habituelles. Cette option, qui est activée par
défaut, est équivalente à+rv.-reverseLorsque cette option est activée, les couleurs
de premier et d'arrière-plan de la fenêtredttermsont inversées. Elle est équivalente aux options-ret-rv.-rvLorsque cette option est activée, les couleurs
de premier et d'arrière-plan de la fenêtredttermsont inversées. Elle permet d'obtenir le même
résultat que lorsque vous choisissez Options générales
dans le menu Options, et que vous affectez la valeur "Inverse" au menu d'options
"Arrière-plan fenêtre". Si vous spécifiez cette option
à l'ouverture d'une fenêtredtterm,
le menu d'options "Arrière-plan fenêtre" aura pour valeur "Inverse".
Reportez-vous à la section"Options
générales".+rvLorsque cette option est activée, la fenêtredtterms'affiche dans ses couleurs de premier et d'arrière-plan
habituelles. Il s'agit de l'option par défaut.-rwLa sélection de cette option entraîne l'activation
du bouclage inverse.reverseWrapRessource
associ&eacute;e:reverseWrap.+rwLa sélection de cette option entraîne la
désactivation du bouclage inverse. Il s'agit de l'option par défaut.reverseWrapRessource
associ&eacute;e:reverseWrap.-SccnCette option indique que l'émulateur de terminal
doit être exécuté sur une unité STREAMS (lecteur
de bande) ou un pty pré-ouvert. Cette option n'est à utiliser
que si le nom d'esclave de l'unité STREAMS ou du pty a la syntaxetty??(exactement deux caractères aprèstty). Cette option est destinée à être
utilisée lorsquedttermest appelé par une instruction d'une autre application.ccdésigne les deux derniers
caractères du nom d'esclave de l'unité
STREAMS ou du pty (le nom complet ayant la syntaxetty??).
Ce paramètre n'est pas pris en compte mais doit être composé
de
deux caractères exactement.ndésigne le numéro
affecté au descripteur de fichier, qui correspond à l'extrémité
maître pré-ouverte de l'unité STREAMS ou du pty.-Sc.nCette option est identique à l'option-Sccndécrite ci-dessus, mais est destinée
plus spécifiquement aux systèmes utilisant un nom de pty plus
long.cdésigne le dernier caractère
du nom d'esclave du pty. Ce paramètre n'étant pas pris en compte,
il n'est pas nécessaire de le spécifier.ndésigne le numéro
affecté au descripteur de fichier, qui correspond à l'extrémité
maître pré-ouvert du pty.-sbCette option active l'affichage d'une barre de défilement.
Il s'agit de l'option par défaut.scrollBarRessource
associ&eacute;e:scrollBar.+sbSi vous sélectionnez cette option,aucunebarre de défilement ne sera affichée.scrollBarRessource
associ&eacute;e:scrollBar.-sfCette option indique que les touches de fonction doivent
utiliser les codes d'échappement Sun au lieu des séquences d'échappement
VT220 standard.sunFunctionKeysRessource
associ&eacute;e:sunFunctionKeys.+sfCette option indique que les touches de fonction doivent
utiliser les séquences d'échappement standard au lieu des codes
d'échappement Sun. Par défaut, elle est activée.sunFunctionKeysRessource
associ&eacute;e:sunFunctionKeys.-slscreens[s|l]Cette option indique de combien de lignes la mémoire
tampon du terminal dépasse la longueur de fenêtre. Cette option
se compose d'une valeur numérique suivie, parfois, d'un suffixe. S'il
n'y a pas de suffixe, ou que le suffixe est la lettre "l", la taille totale
de la mémoire tampon du terminal sera égale à:nombre d'écrans+ longueur de la fenêtre
de terminal. Si le suffixe est la lettre "s", la taille totale de la mémoire
tampon du terminal sera égale aunombre d'écrans+ 1 multiplié par la longueur de la fenêtre de terminal.
Lorsque la fenêtre est agrandie,dtterms'efforce de conserver le même ratio mémoire tampon-fenêtre.
La valeur par défaut de cette option est "4s."screensindique le nombre
d'écrans ou de lignes à sauvegarder.saveLinesRessource
associ&eacute;e:saveLines.-tiID_termCette option indique le nom à spécifier
lorsque le système vous invite à préciser l'ID du terminal.
Les valeurs autorisées sontvt100, vt101, vt102etvt220. La valeur par défaut estvt220.ID_termindique l'ID de terminal
à utiliser.-titlechaîneCette option indique le titre de la fenêtre. Si
elle est accompagnée du paramètre-e,
le titre par défaut de la fenêtre correspond au dernier élément
du chemin d'accès du programme. Dans le cas contraire, c'est le dernier
élément de l'argument servant à exécuterdtterm(à savoirargv[0])
qui est utilisé.chaîneindique le titre à
utiliser.titleRessource
associ&eacute;e:title.-tmmodesCette option est constituée d'une chaîne
qui indique les mots clés de paramétrage du terminal et les
caractères avec lesquels ils peuvent être associés. Les
mots clés admis sont:intr,quit,erase,kill,eof,eol,swtch,start,stop,brk,susp,dsusp,rprnt,flush,werasetlnext. Les mots clés qui ne s'appliquent pas à
une architecture spécifique sont analysés correctement, mais
pas pris en compte. Les caractères de contrôle peuvent être
spécifiés sous la forme^+
caractère (^cou^u, par exemple) tandis que la combinaison^?indique une suppression. Ceci permet de remplacer les
paramètres par défaut du terminal sans avoir à effectuer
unstty(1)chaque fois qu'un processus de
terminal est lancé. La valeur par défaut est NULL.modesest une chaîne qui indique
le mode du terminal.ttyModesRessource
associ&eacute;e:ttyModes.-tnnomCette option indique la valeur (nom) qui doit être
associée à la variable d'environnement$TERM. Par défaut, cette valeur est"vt220".nomindique le nom de terminal à
utiliser.termNameRessource
associ&eacute;e:termName.-usageAffiche un message relatif à l'utilisation du terminal.-vbCette option permet d'activer un signal visible plutôt
qu'un signal sonore: chaque fois qu'elle reçoit la séquence
d'échappement Ctrl+G, la fenêtre clignote au lieu d'émettre
un signal sonore.visualBellRessource
associ&eacute;e:visualBell.+vbCette option permet d'activer un signal sonore plutôt
qu'un signal visible (configuration par défaut).visualBellRessource
associ&eacute;e:visualBell.-wlargeur_cadreCette option est identique à-borderwidth(voir plus
haut).largeur_cadreindique
la largeur, en pixels, du cadre de la fenêtre.-xrmchaîne_ressourceCette option permet de définir à partir
de la ligne de commande
les ressources du terminal X11 de type Gestionnaire de ressources.chaîne_ressourceest une chaîne
correspondant à une ressource X11.RESSOURCESallowSendEventsCette ressource indique que l'émulateur de terminal
doit pouvoir accepter
des événements synthétiques (c'est-à-dire, générés
et envoyés par une
application externe). L'activation de cette option peut ouvrir une brèche
dans le dispositif de sécurité. Par défaut, cette option
a pour valeur
False.appCursorDefaultSelon que cette option a la valeur True ou False, les
touches de déplacement
du curseur fonctionnent initialement en mode application ou en mode curseur.
La valeur par défaut est False.appKeypadDefaultSelon que cette option a la valeur True ou False, le clavier
fonctionne initialement en mode application ou en mode numérique.
La valeur par défaut est False.autoWrapCette ressource a pour valeur True ou False selon que
la fonction de retour
ligne est activée ou non par défaut. La valeur par défaut
est True.backgroundCette ressource indique la couleur d'arrière-plan
de la fenêtre de terminal
et celle utilisée par défaut pour la barre de défilement.
Dans le Bureau CDE,
cette ressource s'aligne sur le pixel de sélection ou sur
le pixel d'arrière-plan du jeu de couleurs primaires
(voirbackgroundIsSelect). Par défaut,
la couleur d'arrière-plan
de la fenêtre de terminal s'aligne sur le pixel d'arrière-plan.
En dehors du Bureau, la couleur d'arrière-plan est le noir.backgroundIsSelectCette ressource a pour valeur True ou False selon que
la fenêtre de terminal
doit utiliser ou non la couleur de sélection de l'environnement Motif
à la place de sa couleur d'arrière-plan habituelle.
La valeur par défaut est False.blinkRateCette ressource indique le nombre de millisecondes durant
lesquelles le curseur
clignotant est activé et désactivé. Une valeur de 250
(valeur par défaut)
signifie que le curseur clignote deux fois par seconde, tandis que
la valeur 0 désactive le clignotement.borderColorCette ressource définit la couleur du cadre de
fenêtre. Ce cadre
peut ne pas être visible si vous utilisez des gestionnaires de fenêtres
apparentés commedtwmetmwm. La couleur par défaut est le noir.borderWidthCette ressource indique la largeur du cadre de la fenêtre
du widget
de shell. Cette valeur peut être ignorée dans les gestionnaires
de fenêtres apparentés (dtwmetmwm, par exemple). La valeur
par défaut est 0.c132Cette ressource a pour valeur True ou False selon que
la séquence
d'échappement DECCOLM, qui permet d'alterner entre les modes d'affichage
80 et 132 colonnes, doit être activée ou non. La valeur par défaut
est False.charCursorStyleCette ressource indique la forme du curseur. La valeurchar_cursor_box(valeur par défaut)
désigne un curseur dont
la largeur et la hauteur sont calquées sur le rectangle qui entoure
la police de base.
La valeurchar_cursor_bardésigne,
pour sa part, un curseur
dont la largeur est déterminée par le rectangle entourant la
police
de base, haut de deux pixels et dont le haut touche la ligne de base.consoleModeCette ressource indique que l'affichage doit être
dirigé vers
la fenêtre de terminal et non vers/dev/console.
Elle permet
d'éviter que le contenu de l'écran ITE ne vienne écraser
celui
du serveur X. Cette ressource n'a pas pour vocation d'être utilisée
de
façon systématique pour rediriger l'affichage de la console/dev/consoled'un système choisi arbitrairement
vers un serveur X
choisi lui aussi arbitrairement. Cette ressource ne fonctionne que si vous
êtes propriétaire de/dev/consoleet que vous y avez accès en
lecture-écriture. La valeur par défaut est False.foregroundCette ressource indique la couleur de premier plan de
la fenêtre de terminal
ainsi que celle utilisée par défaut pour la barre de défilement
et le
pointeur. Dans le Bureau CDE, il s'agit par défaut de la couleur de
premier plan du
jeu de couleurs primaires. En dehors du Bureau, la couleur par défaut
est le blanc.geometryCette ressource indique les dimensions et la position
privilégiées de la
fenêtre de terminal. Par défaut, les dimensions de la fenêtre
sont
de 24 lignes de 80 colonnes. En revanche, aucune position par défaut
n'est définie.iconGeometryCette ressource indique la position privilégiée
de l'icône de l'émulateur de
terminal. Les gestionnaires de fenêtres peuvent passer outre
ce paramètre. Aucune valeur par défaut n'est définie.iconicCette ressource a pour valeur True ou False selon que
l'émulateur de terminal
doit ou non apparaître à l'écran sous forme réduite
au démarrage.
Les gestionnaires de fenêtres (y comprisdtwmetmwm) peuvent passer
outre ce paramètre. La valeur par défaut est False.iconicNameCette ressource indique le nom de l'icône. Selon
qu'elle est associée ou non
à l'option-e, le nom par défaut
est le dernier élément du
chemin d'accès du programme ou le nom de base de l'argument servant
à exécuterdtterm(à
savoir,argv[0]).jumpScrollCette ressource a pour valeur True (valeur par défaut)
ou False selon que
le défilement discontinu doit être activé ou non. Lorsqu'il
est activé,
il est possible de faire défiler l'écran de plusieurs lignes
à la fois et,
par conséquent, d'accélérer la vitesse de régénération
des écrans lorsque
plusieurs lignes de texte sont envoyées au terminal.
Le nombre de lignes qu'il est possible de faire défiler est limité
au nombre
de lignes contenues dans la fenêtre. En tout état de cause, les
lignes sont
toujours affichées dans leur totalité.kshModeCette ressource a pour valeur True ou False (valeur par
défaut)
selon que le mode ksh est
activé ou non. Dans ce mode, la frappe d'une touche utilisant le jeu
de bits de modification étendu génère un caractère
d'échappement suivi du
caractère généré par la frappe de la touche en
mode non étendu.
Cette option est destinée à être utilisée avec
emacs et avec l'éditeur
emacs lancé sur la ligne de commande (en modeksh(1)ouied(1)).
Elle n'est pas compatible avec l'utilisation normale de la métatouche
pour obtenir des caractères étendus codés sur un octet
et des caractères
codés sur plusieurs octets (pays asiatiques).logFileCette ressource indique le nom du fichier dans lequel
les données affichées à consigner décrites plus
bas doivent être écrites. Si le nom du fichier commence par le
symbole|, le restant de la chaîne
est traité comme une commande à exécuter une fois l'extrémité
du tube atteinte. Par défaut, le fichier s'appelleDttermLogXXXXX(oùXXXXXest une chaîne de caractères univoque)
et réside dans le répertoire à partir duquel le processus
secondaire a été lancé. Si les cinq derniers caractères
sont "XXXXX," ils sont remplacés par une chaîne de caractères
univoque.loggingCette ressource a pour valeur True ou False (valeur par
défaut) selon que la consignation des données affichées
est activée ou non. Lorsque la fonction de consignation est activée,
toutes les données reçues du processus secondaire sont consignées
soit dans un fichier, soit dans un pipeline de commandes (en fonction de la
valeur de l'optionlogFiledécrite
plus haut). Les données sont consignées directement à
partir du processus secondaire: elles comprennent donc les caractères
d'échappement et les combinaisons retour chariot/avance ligne transmis
en application du mode de gestion des lignes sur le terminal. L'activation
et la désactivation de l'affichage s'effectuent via les séquences
d'échappement.logInhibitCette ressource a pour valeur True ou False selon que
la consignation dans un fichier ou sur une unité doit être désactivée
ou non. La valeur par défaut est False.loginShellCette ressource a pour valeur True ou False (valeur par
défaut) selon que le shell lancé doit être un shell de
connexion ou non. Si oui, le premier caractère deargv[0]est un tiret, ce qui signale au shell qu'il doit
lire le fichierprofiledu système
ainsi que le fichier$HOME/.profilede l'utilisateur
(pourkshetsh)
ou bien le fichiercsh.loginet le fichier$HOME/.loginde l'utilisateur (pourcsh).mapOnOutputCette ressource a pour valeur True ou False (valeur par
défaut)
selon que l'émulateur de terminal doit se mapper (ne plus s'afficher
sous forme réduite) lors de l'affichage des résultats du processus
secondaire. La ressourcemapOnOutputDelay(voir ci-dessous) permet
de définir un délai entre l'affichage et le mappage.mapOnOutputDelayCette ressource indique le nombre de secondes qui s'écoule
entre
le démarrage dedttermet l'activation
de la ressourcemapOnOutput.
(voir plus haut). Ce délai permet d'afficher des données sur
le terminal
(des invites de shell, par exemple) sans que cela entraîne automatiquement
le mappage de la fenêtre. La valeur par défaut est0(pas de délai).marginBellCette ressource a pour valeur True ou False (valeur par
défaut)
selon que l'avertissement de marge est activé ou non.menuBarCette ressource a pour valeur True (valeur par défaut)
ou False
selon qu'un menu déroulant doit être affiché ou non.menuPopupCette ressource a pour valeur True (valeur par défaut)
ou False selon
qu'un menu instantané doit être activé ou non.nMarginBellCette ressource indique le nombre de caractères
précédant la marge droite
à partir duquel l'avertissement de marge doit être déclenché,
s'il y a lieu.
La distance est de 10 caractères par défaut.pointerBlankCette ressource a pour valeur True ou False selon que
le pointeur doit passer
en mode masquage ou non. Lorsque ce mode est actif, le curseur disparaît
au bout d'un délai spécifié (en secondes) ou après
une entrée au clavier.
Il réapparaît lorsque vous déplacez la souris.
La ressourcepointerBlankDelaypermet de
définir le délai avant masquage.
La valeur par défaut est False.pointerBlankDelayCette ressource indique le nombre de secondes qui s'écoulent
entre
la fin du déplacement du pointeur et le masquage du curseur. Si la
valeur est
de 0, la fonction de masquage n'est activée que lorsque l'utilisateur
entre des données au clavier. Par défaut, le délai est
de 2 secondes.pointerColorCette ressource indique la couleur de premier plan à
appliquer
au pointeur X11 de la fenêtre de terminal. Par défaut, il s'agit
de la même
couleur que la fenêtre. Pour plus de détails, reportez-vous à
la description
de la ressourceforeground.pointerColorBackgroundCette ressource indique la couleur d'arrière-plan
à appliquer au pointeur X11
de la fenêtre de terminal. Par défaut, il s'agit de la même
couleur que la
fenêtre. Pour plus de détails, reportez-vous à la description
de
la ressourcebackground.pointerShapeCette ressource indique la police à utiliser pour
le caractère représentant le curseur. Il doit s'agir de l'une
des chaînes indiquées dans le fichier<X11/cursorfont.h>, sans les caractères de débutXC_. La valeur par défaut estxterm.reverseVideoCette ressource a pour valeur True ou False selon que
la vidéo inverse doit être activée ou non. La valeur par
défaut est False.reverseWrapCette ressource a pour valeur True ou False selon que
la boucle inverse doit être activée ou non. La valeur par défaut
est False.saveLinesCette ressource indique de combien de lignes la mémoire
tampon du terminal dépasse la longueur de la fenêtre. Cette ressource
se compose d'une valeur numérique suivie, s'il y a lieu, d'un suffixe.
S'il n'y a pas de suffixe, ou que le suffixe est la lettre "l", la taille
totale de la mémoire tampon du terminal sera égale à
:nombre d'écrans+ longueur de la
fenêtre de terminal. Si le suffixe est la lettre "s", la taille totale
de la mémoire tampon du terminal sera égale ànombre d'écrans+ 1 multiplié par la
longueur de la fenêtre de terminal. Lorsque la fenêtre est agrandie,dtterms'efforce de conserver le même ratio mémoire
tampon-fenêtre. La valeur par défaut de cette option est "4s."scrollBarCette ressource a pour valeur True ou False selon que
la barre de défilement doit être visible ou non. La valeur par
défaut est True.sunFunctionKeysCette ressource a pour valeur True ou False selon que
les touches de fonction doivent utiliser ou non les codes d'échappement
Sun au lieu des séquences d'échappement VT220 standard. La valeur
par défaut est False.termIdCette ressource indique le nom à spécifier
lorsque le système vous invite à préciser l'ID du terminal.
Les valeurs autorisées sontvt100,vt101,vt102etvt220. La valeur par défaut estvt220.termNameCette ressource indique la valeur (nom) qui doit être
associée à la variable d'environnement$TERM. Par défaut, cette valeur est"vt220".titleCette ressource indique le titre de la fenêtre.
Si elle est accompagnée du paramètre-e,
le titre par défaut de la fenêtre correspond au dernier élément
du chemin d'accès du programme. Dans le cas contraire, c'est le dernier
élément de l'argument servant à exécuterdtterm(à savoirargv[0])
qui est utilisé.ttyModesCette ressource est constituée d'une chaîne
qui indique les mots clés de paramétrage du terminal et les
caractères auxquels ils peuvent être associés. Les mots
clés admis sont:intr,quit,erase,kill,eof,eol,swtch,start,stop,brk,susp,dsusp,rprnt,flush,werasetlnext. Les mots clés qui ne s'appliquent pas à
une architecture spécifique sont analysés correctement, mais
pas pris en compte. Les caractères de contrôle peuvent être
spécifiés sous la forme^+
caractère (^cou^u, par exemple) tandis que la combinaison^?dénote une suppression. Ceci permet de remplacer
les paramètres par défaut du terminal sans avoir à effectuer
unstty(1)chaque fois qu'un processus de
terminal est lancé. La valeur par défaut est NULL.userBoldFontCette ressource spécifie le jeu de polices (variable
XFontSet) dans lequel le texte en gras doit être affiché sur
le terminal. Ce jeu doit être choisi dans la liste des polices Motif
(variable XmFontList). Seuls les caractères et les polices à
chasse fixe sont pris en charge. Si vous utilisez des polices proportionnelles,
vous obtiendrez des résultats imprévisibles. La police générée
par défaut en caractères gras est fonction de la désignation
XLFD de la police utilisateur (variable userFont). Si cette police n'est pas
disponible, la mise en gras sera obtenue par impression en double (avec un
décalage d'un pixel) de la police utilisateur.userFontCette ressource spécifie le jeu de polices (variable
XFontSet) dans lequel le texte doit être affiché sur le terminal.
Ce jeu doit être choisi dans la liste des polices Motif (variable XmFontList).
Seuls les caractères et les polices à chasse fixe sont pris
en charge. Si vous utilisez des polices proportionnelles, vous obtiendrez
des résultats imprévisibles. Les éléments de texte
n'apparaissant pas à proprement parler dans la fenêtre de terminal
(barre de menus, menus instantanés, boîtes de dialogue, etc.)
n'utilisent pas cette police. Par défaut, la valeur de la variable
XmNtextFontList associée au tableau d'affichage parent (voir XmBulletinBoard(3X))
est utilisée de la même façon que le widget XmText.visualBellCette ressource a pour valeur True ou False selon qu'un
signal visible doit ou non être activé de préférence
à un signal sonore: chaque fois qu'elle reçoit la séquence
d'échappement Ctrl+G, la fenêtre clignote au lieu d'émettre
un signal sonore. La valeur par défaut est False.UTILISATION DU POINTEURdttermpermet de sélectionner
des zones de texte. Le mode de sélection est déterminé
par le modèle décrit dans le manuelInter-Client
Communication Conventions Manual(ICCCM).dttermne reconnaît que la sélection primaire. La
fonction de transfert primaire permet de copier ou coller du texte sélectionné.
Toute entrée est traitée comme une entrée au clavier
et insérée à l'emplacement du curseur. Les opérations
de sélection/insertion et les boutons de la souris qui leur sont associés
par défaut sont décrits ci-dessous.SélectionLe bouton gauche sert à sélectionner le texte
à copier. Amenez le pointeur au début du texte à copier,
appuyez sur
le bouton gauche de la souris et, tout en le maintenant enfoncé,
amenez le curseur à la fin du texte à copier. Enfin, relâchez
le bouton.
Pour désélectionner du texte, cliquez une fois sur le bouton
gauche de la
souris sans déplacer celle-ci.InsertionLe bouton du milieu sert à coller
le texte faisant l'objet de la sélection primaire. Le texte est
assimilé à des données entrées au clavier.ACTIONSCe sujet décrit les routines d'actiondtterm.bell([Pourcentage])Active la sonnerie lorsque lepourcentageindiqué est supérieur ou inférieur
au volume de base.break ()Envoie un signal d'interruption au processus fils.cancel ()Envoie un caractèreCAN(annulation) au processus fils.do ()Envoie au processus fils la séquence d'échappement
associée à la toucheDo.edit-key(chaîne)Envoie au processus fils la séquence d'échappement
associée à la touche d'édition correspondante. L'interprétation
de ces touches dépend de l'application utilisée. Les valeurs
admises pourchaînesont les suivantes
:find,insert,next,prior,removeetselect.extend-start()Commence à étendre la sélection en
cours.extend-end ()Etend la sélection en cours. La quantité
de texte sélectionné dépend du nombre de clics sur la
souris.function-key-execute(num[,type])Envoie au processus fils la séquence d'échappement
associée à la touche de fonctionnumcorrespondante.
La valeur denumdoit être comprise entre1et35. Sitypea pour valeurfonction(ou
qu'il n'est pas défini), la séquence d'échappement associée
à la touche de fonctionnumest envoyée
au processus fils. Sitypea pour valeurUDK, la chaîne associée à la touchenumdéfinie par l'utilisateur est envoyée au processus
fils.grab-focus ()Le résultat de cette action dépend du nombre
de clics sur la souris: un clic désélectionne tout texte sélectionné
et place le point d'ancrage à l'emplacement du pointeur; deux clics
sélectionnent un mot; trois clics sélectionnent une ligne de
texte et quatre clics, la totalité du texte.hard-reset ()Effectue une réinitialisation globale de l'émulateur
de terminal.help ()Envoie au processus fils la séquence d'échappement
associée à la touche d'aideDEC VT220.
L'interprétation de cette touche dépend de l'application utilisée.keymap(nom)Définit, de manière dynamique, une nouvelle
table de traduction dont le nom de ressource estnomsuivi du suffixeKeymap(les
minuscules et les majuscules ne sont pas interchangeables). Le valeurNoneentraîne la restauration de la table de traduction
d'origine.keypad-key-execute(chaîne)Transmet au processus fils la séquence d'échappement
associée à la touche correspondante du clavier. L'interprétation
de ces touches dépend de l'application utilisée.chaînepeut prendre les valeurs suivantes:f1-f4,space,tab,enter,equal,multiply,add,separator,subtract,decimal,divideet0à9.move-cursor(sens)Transmet au processus fils la séquence d'échappement
associée au sens du déplacement du curseur. L'interprétation
de ces touches dépend de l'application utilisée.senspeut prendre les valeurs suivantes:up,down,backwardetforward.redraw-display ()Réorganise la fenêtre de texte.scroll(nombre[,unité])Fait défiler la mémoire d'affichage vers
le bas ou vers le haut selon quenombreest
inférieur ou supérieur à zéro. Le nombre de lignes
de défilement est fonction de la valeur denombreetunité. Ce dernier
paramètre peut prendre les valeurs suivantes:page,halfpageouline(valeur par défaut).select-adjust ()Etend la sélection. La quantité de texte
sélectionné dépend du nombre de clics:1 clic = 1 caractère2 clics = 1 mot3 clics = 1 ligne4 clics = tamponselect-all ()Sélectionne la totalité du texte.select-page ()Sélectionne la totalité du texte affiché.self-insert ()Envoie au processus fils le caractère associé
à la touche actionnée.soft-reset ()Effectue une réinitialisation partielle du terminal.stop(état)Active, désactive ou fait passer à l'état
inverse le processus de lecture des données issues du processus fils.étatpeut prendre les valeurs suivantes:toggle,onetoff.string(chaîne)Insère la chaîne de caractères spécifiée,
comme si elle avait été entrée au clavier.chaînedoit figurer entre apostrophes si elle contient des
blancs ou des caractères autres que des caractères alphanumériques.
Si elle commence par0x, elle est interprétée
comme une constante au format hexadécimal.tab ()Envoie une tabulation au processus fils.visual-bell ()Fait clignoter la fenêtre de manière rapide.Edition de liens avec les touches virtuellesLes liens effectués avec les touches virtuelles sont propres
au constructeur. Ils ne sont pas pris en compte lorsque la zone d'entrée
est le widgetdtterm. Pour plus de détails
sur les liens avec les touches et les boutons virtuels, reportez-vous àVirtualBindings(3X).TRADUCTIONSdttermfournit des traductions de Primitive.
Si vous modifiez une traduction en mode#overrideou#augment, vous risquez d'obtenir des résultats
imprévisibles.Shift~Ctrl<Key>KP_Multiply:XtDisplayInstalledAccelerators()~ShiftCtrl<Key>KP_Multiply:XtDisplayAccelerators()ShiftCtrl<Key>KP_Multiply:XtDisplayTranslations()<Key>osfCancel:process-cancel()<Key>osfCopy:copy-clipboard()<Key>osfCut:copy-clipboard()<Key>osfPaste:paste-clipboard()<Key>osfBeginLine:beginning-of-buffer()<Key>osfEndLine:end-of-buffer()Shift<Key>osfUp:scroll(1,line)Shift<Key>osfDown:scroll(-1,line)<Key>osfUp:move-cursor(up)<Key>osfDown:move-cursor(down)<Key>osfLeft:move-cursor(backward)<Key>osfRight:move-cursor(forward)<Key>Find:vt-edit-key(find)<Key>Insert:vt-edit-key(insert)<Key>Select:vt-edit-key(select)<Key>Do:vt-edit-key(do)<Key>Help:vt-edit-key(help)<Key>Menu:vt-edit-key(menu)~Shift<Key>osfPageUp:vt-edit-key(prior)~Shift<Key>osfPageDown:vt-edit-key(next)<Key>osfPageUp:scroll(-1,page)<Key>osfPageDown:scroll(1,page)Mod1<Key>Break:soft-reset()Shift<Key>Break:hard-reset()~Shift ~Mod1<Key>Break:vt-break()Ctrl<Key>Cancel:stop(long)~Ctrl<Key>Cancel:stop()~Shift<Key>Tab:tab()~Mod1<Key>KP_Space:keypad-key-execute(space)~Mod1<Key>KP_Tab:keypad-key-execute(tab)~Mod1<Key>KP_Enter:keypad-key-execute(enter)~Mod1<Key>KP_F1:keypad-key-execute(f1)~Mod1<Key>KP_F2:keypad-key-execute(f2)~Mod1<Key>KP_F3:keypad-key-execute(f3)~Mod1<Key>KP_F4:keypad-key-execute(f4)~Mod1<Key>KP_Equal:keypad-key-execute(equal)~Mod1<Key>KP_Multiply:keypad-key-execute(multiply)~Mod1<Key>KP_Add:keypad-key-execute(add)~Mod1<Key>KP_Separator:keypad-key-execute(separator)~Mod1<Key>KP_Subtract:keypad-key-execute(subtract)~Mod1<Key>KP_Decimal:keypad-key-execute(decimal)~Mod1<Key>KP_Divide:keypad-key-execute(divide)~Mod1<Key>KP_0:keypad-key-execute(0)~Mod1<Key>KP_1:keypad-key-execute(1)~Mod1<Key>KP_2:keypad-key-execute(2)~Mod1<Key>KP_3:keypad-key-execute(3)~Mod1<Key>KP_4:keypad-key-execute(4)~Mod1<Key>KP_5:keypad-key-execute(5)~Mod1<Key>KP_6:keypad-key-execute(6)~Mod1<Key>KP_7:keypad-key-execute(7)~Mod1<Key>KP_8:keypad-key-execute(8)~Mod1<Key>KP_9:keypad-key-execute(9)Shift<Key>F1:vt-function-key-execute(1, UDK)Shift<Key>F2:vt-function-key-execute(2, UDK)Shift<Key>F3:vt-function-key-execute(3, UDK)Shift<Key>F4:vt-function-key-execute(4, UDK)Shift<Key>F5:vt-function-key-execute(5, UDK)Shift<Key>F6:vt-function-key-execute(6, UDK)Shift<Key>F7:vt-function-key-execute(7, UDK)Shift<Key>F8:vt-function-key-execute(8, UDK)Shift<Key>F9:vt-function-key-execute(9, UDK)Shift<Key>F10:vt-function-key-execute(10, UDK)Shift<Key>F11:vt-function-key-execute(11, UDK)Shift<Key>F12:vt-function-key-execute(12, UDK)Shift<Key>F13:vt-function-key-execute(13, UDK)Shift<Key>F14:vt-function-key-execute(14, UDK)Shift<Key>F15:vt-function-key-execute(15, UDK)Shift<Key>F16:vt-function-key-execute(16, UDK)Shift<Key>F17:vt-function-key-execute(17, UDK)Shift<Key>F18:vt-function-key-execute(18, UDK)Shift<Key>F19:vt-function-key-execute(19, UDK)Shift<Key>F20:vt-function-key-execute(20, UDK)Shift<Key>F21:vt-function-key-execute(21, UDK)Shift<Key>F22:vt-function-key-execute(22, UDK)Shift<Key>F23:vt-function-key-execute(23, UDK)Shift<Key>F24:vt-function-key-execute(24, UDK)Shift<Key>F25:vt-function-key-execute(25, UDK)Shift<Key>F26:vt-function-key-execute(26, UDK)Shift<Key>F27:vt-function-key-execute(27, UDK)Shift<Key>F28:vt-function-key-execute(28, UDK)Shift<Key>F29:vt-function-key-execute(29, UDK)Shift<Key>F30:vt-function-key-execute(30, UDK)Shift<Key>F31:vt-function-key-execute(31, UDK)Shift<Key>F32:vt-function-key-execute(32, UDK)Shift<Key>F33:vt-function-key-execute(33, UDK)Shift<Key>F34:vt-function-key-execute(34, UDK)Shift<Key>F35:vt-function-key-execute(35, UDK)~Shift<Key>F1:vt-function-key-execute(1, function)~Shift<Key>F2:vt-function-key-execute(2, function)~Shift<Key>F3:vt-function-key-execute(3, function)~Shift<Key>F4:vt-function-key-execute(4, function)~Shift<Key>F5:vt-function-key-execute(5, function)~Shift<Key>F6:vt-function-key-execute(6, function)~Shift<Key>F7:vt-function-key-execute(7, function)~Shift<Key>F8:vt-function-key-execute(8, function)~Shift<Key>F9:vt-function-key-execute(9, function)~Shift<Key>F10:vt-function-key-execute(10, function)~Shift<Key>F11:vt-function-key-execute(11, function)~Shift<Key>F12:vt-function-key-execute(12, function)~Shift<Key>F13:vt-function-key-execute(13, function)~Shift<Key>F14:vt-function-key-execute(14, function)~Shift<Key>F15:vt-function-key-execute(15, function)~Shift<Key>F16:vt-function-key-execute(16, function)~Shift<Key>F17:vt-function-key-execute(17, function)~Shift<Key>F18:vt-function-key-execute(18, function)~Shift<Key>F19:vt-function-key-execute(19, function)~Shift<Key>F20:vt-function-key-execute(20, function)~Shift<Key>F21:vt-function-key-execute(21, function)~Shift<Key>F22:vt-function-key-execute(22, function)~Shift<Key>F23:vt-function-key-execute(23, function)~Shift<Key>F24:vt-function-key-execute(24, function)~Shift<Key>F25:vt-function-key-execute(25, function)~Shift<Key>F26:vt-function-key-execute(26, function)~Shift<Key>F27:vt-function-key-execute(27, function)~Shift<Key>F28:vt-function-key-execute(28, function)~Shift<Key>F29:vt-function-key-execute(29, function)~Shift<Key>F30:vt-function-key-execute(30, function)~Shift<Key>F31:vt-function-key-execute(31, function)~Shift<Key>F32:vt-function-key-execute(32, function)~Shift<Key>F33:vt-function-key-execute(33, function)~Shift<Key>F34:vt-function-key-execute(34, function)~Shift<Key>F35:vt-function-key-execute(35, function)<KeyRelease>:key-release()<KeyPress>:insert()~Shift~Ctrl<Btn1Down>:grab-focus()Shift~Ctrl<Btn1Down>:extend-start()~Ctrl<Btn1Motion>:select-adjust()~Ctrl<Btn1Up>:extend-end()~Shift<Btn2Down>:process-bdrag()~Shift<Btn2Up>:copy-to()<EnterWindow>:enter()<LeaveWindow>:leave()<FocusIn>:focus-in()<FocusOut>:focus-out()Séquences d'échappement dttermChacun des sujets ci-dessous contient une liste des séquences
d'échappement utilisables.
Pour plus de détails, reportez-vous à la page de manueldtterm(5x).Séquences d'échappement du curseur (mode VT220)Le tableau ci-dessous indique les touches et les séquences d'échappement
correspondantes transmises en mode Normal et en mode Application.Séquence d'échappement clavier

 TOUCHE        Normal      Application
=========     ========     ===========
Haut           Echap[A      EchapOA
Bas            Echap[B      EchapOB
Droite         Echap[C      EchapOC
Gauche         Echap[D      EchapODClavier numérique auxiliaire (mode ANSI)Le tableau ci-dessous indique les touches et les séquences d'échappement
correspondantes transmises en mode Normal et en mode Application.Mode clavier numérique: Application

 TOUCHE               Normal      Application
=========             ======      ===========
espace                espace      EchapOA
tabulation            tabulation  EchapOI
Entrée                CR/CR-LF    EchapOM
PF1                   EchapOP     EchapOP
PF2                   EchapOQ     EchapOQ
PF3                   EchapOR     EchapOR
PF4                   EchapOS     EchapOS
* (multiplication)    *           EchapOj
+ (plus)              +           EchapOk
, (virgule)           ,           EchapOl
- (moins)             -           EchapOm
. (point)             .           EchapOn
/ (division)          /           EchapOo
0                     0           EchapOp
1                     1           EchapOq
2                     2           EchapOr
3                     3           EchapOs
4                     4           EchapOt
5                     5           EchapOu
6                     6           EchapOv
7                     7           EchapOw
8                     8           EchapOx
9                     9           EchapOy
=(égal)               =           EchapOXTouches de fonction VT220Le tableau ci-dessous indique les touches et les séquences d'échappement
correspondantes transmises.TOUCHE       Touche d'échappement transmise
=========     ==============================
F1                   Echap[11~
F2                   Echap[12~
F3                   Echap[13~
F4                   Echap[14~
F5                   Echap[15~
F6                   Echap[17~
F7                   Echap[18~
F8                   Echap[19~
F9                   Echap[20~
F10                  Echap[21~
F11                  Echap[23~
F12                  Echap[24~
F13                  Echap[25~
F14                  Echap[26~
F15                  Echap[28~
F16                  Echap[29~
F17                  Echap[31~
F18                  Echap[32~
F19                  Echap[33~
F20                  Echap[34~
Aide                 Echap[28~
Menu                 Echap[29~
Recherche            Echap[1~
Inser                Echap[2~
Suppr                Echap[3~
Sélection            Echap[4~
Préc.                Echap[5~
Suiv.                Echap[6~Touches de fonction SUNLe tableau ci-dessous indique les touches et les séquences d'échappement
correspondantes transmises.TOUCHE       Séquence d'échappement transmise
=========     ================================
F1                   Echap[224z
F2                   Echap[225z
F3                   Echap[226z
F4                   Echap[227z
F5                   Echap[228z
F6                   Echap[229z
F7                   Echap[230z
F8                   Echap[231z
F9                   Echap[232z
F10                  Echap[233z
F11                  Echap[192z
F12                  Echap[193z
F13                  Echap[194z
F14                  Echap[195z
F15                  Echap[196z
F16                  Echap[197z
F17                  Echap[198z
F18                  Echap[199z
F19                  Echap[200z
F20                  Echap[201z
F21 (R1)             Echap[208z
F22 (R2)             Echap[209z
F23 (R3)             Echap[210z
F24 (R4)             Echap[211z
F25 (R5)             Echap[212z
F26 (R6)             Echap[213z
F27 (R7)             Echap[214z
F28 (R8)             Echap[215z
F29 (R9)             Echap[216z
F30 (R10)            Echap[217z
F31 (R11)            Echap[218z
F32 (R12)            Echap[219z
F33 (R13)            Echap[220z
F34 (R14)            Echap[221z
F35 (R15)            Echap[222z
Aide                 Echap[196z
Menu                 Echap[197z
Recherche            Echap[1z
Inser                Echap[2z
Suppr                Echap[3z
Sélection            Echap[4z
Préc.                Echap[5z
Suiv.                Echap[6zSéquences d'échappement reçuesLe tableau suivant décrit les séquences d'échappement
prises en charge par
dtterm.Séquence
d'échappement      Description ou fonction
============       =======================
Ctrl+G             Signal sonore (Ctrl+G)
Ctrl+H             Retour arrière (Ctrl+H)
Ctrl+I             Tabulation horizontale (HT) (Ctrl+I)
Ctrl+J             Avance ligne ou interligne (NL) (Ctrl+J)
Ctrl+K             Tabulation verticale (identique à Avance ligne)
Ctrl+L             Avance page ou Nouvelle page (identique à Avance ligne)
Ctrl+M             Retour chariot (Ctrl+M)
Echap ( B          Affecte le jeu de caractères G0 à la police de base ASCII.
Echap ( 0          Affecte le jeu de caractères G0 à la police graphique DEC.
Echap ) B          Affecte le jeu de caractères G1 à la police de base ASCII.
Echap ) 0          Affecte le jeu de caractères G1 à la police graphique DEC.
Echap * B          Affecte le jeu de caractères G2 à la police de base ASCII.
Echap * 0          Affecte le jeu de caractères G2 à la police graphique DEC.
Echap + B          Affecte le jeu de caractères G3 à la police de base ASCII.
Echap + 0          Affecte le jeu de caractères G3 à la police grapique DEC.
Ctrl+N             Effectue un mappage de G1 sur GL.
Ctrl+O             Effectue un mappage de G0 sur GL.
Echap n            Effectue un mappage de G2 sur GL.
Echap o            Effectue un mappage de G3 sur GL.
Echap N            Effectue un mappage de G2 sur GL, pour le caractère n + 1.
Echap O            Effectue un mappage de G3 sur GL, pour le caractère n + 1.
Echap Espace F     Sélection des caractères de contrôle C1 sur 7 bits.
                   Dans ce mode, l'utilitaire dtterm transmet à l'hôte tous
                   les caractères de contrôle C1 sous la forme de séquences
                   d'échappement sur 7 bits. Par exemple, CSI est transmis
                   à l'hôte sous la forme\f2Esc\fP [.
Echap Espace G     Sélection des caractères de contrôle C1 sur 8 bits.
                   Dans ce mode, l'utilitaire dtterm transmet à l'hôte tous
                   les caractères de contrôle C1 sous la forme de codes de
                   contrôle sur 8 bits. Par exemple, CSI est transmis à l'hôte
                   sous la forme d'une valeur hexadécimale (0x9B).
Echap#8            DECALN
Echap7             DECSC
Echap8             DECRC
Echap=             DECPAM
Echap>             DECPNM
EchapD             IND
EchapE             NEL
EchapH             HTS
EchapM             RI
EchapPpi;pi|pi/chiffres hexadécimaux;pi/chiffres hexadécimaux;...Echap\
                   DCS
EchapZ             DECID
Echapc             RIS
Echapn             Sélection du jeu de caractères G2 (LS2)
Echapo             Sélection du jeu de caractères G3 (LS3)
Echap[pi"p       DECSCL
Echap[pi@        ICH
Echap[piA        CUU
Echap[piB        CUD
Echap[piC        CUF
Echap[piD        CUB
Echap[piF        Passer à lapième ligne précédente (CPL)
Echap[piG        CHA
Echap[pi;piH     CUP
Echap[piJ        ED
Echap[piK        EL
Echap[piL        IL
Echap[piM        DL
Echap[piP        DCH
Echap[piS        SU
Echap[piT        SD
Echap[piX        Supprimerpicaractères (ECH)
Echap[pic        Transmettre les attributs de l'unité
Echap[pi;pif     HVP
Echap[pig        TBC
Echap[pih        SM
Echap[pil        RM
Echap[pim        SGR
Echap[pin        DSR
Echap[pi;pir     DECSTBM
Echap[pix        Demander les paramètres du terminal
Echap[?pih       DECSET
Echap[?pil       DECRSET
Echap[?pin       DSR
Echap[?pir       Restaurer les valeurs DEC (mode propriétaire)
Echap[?pis       Sauvegarder les valeurs DEC (mode propriétaire)
Echap]?pi;piCtrl+G
                 Définir les paramètres de texte
Echap]p1;p2;p3tSéquences d'échappement SUN
Echap_piEsc&\     Programme d'application
Echap[?piK       DECSEL
Echap[?piJ       DECSED
Echap!p         DECSTRdtterm - Aide sur le clavierAide sur le clavierCe sujet fournit la liste des fonctions clavier associées à
dtterm.
Il traite uniquement des touches spécifiques à dtterm. Ces touches
n'ont d'effet que lorsque la zone d'entrée clavier est la zone
de texte du terminal, par opposition à une boîte de dialogue,
un menu déroulant ou un menu instantané.
D'autres touches de modification peuvent, parfois, rester sans effet.
Par exemple, Maj F1 ne permet pas d'obtenir le même résultat
que F1,
contrairement à Maj Tabulation et Tabulation.<Key>Pos1          Revient au début de la mémoire tampon.
Maj<Key>Pos1       Avance jusqu'à la fin de la mémoire tampon.
Maj<Key>PgAr       Remonte d'une page.
Maj<Key>PgAv       Avance d'une page.
<Key>Haut          Transmet la séquence d'échappement, conformément à
                   la procédure décrite dans dtterm(5x).
<Key>Bas
<Key>Gauche
<Key>Droite
<Key>PgAr
<Key>PgAv
<Key>Recherche
<Key>Inser
<Key>Sélection

<Key>Annul.        Active/désactive l'affichage des données liées
                   au processus secondaire.
<Key>Tabulation    Envoie une tabulation.
<Key>Attn          Envoie une instruction d'interruption RS232
                   au processus secondaire.
Alt<Key>Attn       Effectue une réinitialisation partielle, conformément
                   à la procédure décrite dans dtterm(5x).
Maj<Key>Attn       Effectue une réinitialisation globale, conformément
                   à la procédure décrite dans dtterm(5x).
<Key>F1            Transmet les touches de fonction, conformément
 à                 à la procédure décrite dans dtterm(5x).
<Key>F35
Maj<Key>F1         Transmet, s'il y a lieu, la séquence définie par
 à                 l'utilisateur associée à cette touche.
Maj<Key>F35
Ctrl<Key>F10       Active la barre de menus.
MajCtrl<Key>F10    Active le menu instantané.
Echap               Transmet le caractère d'échappement.
<Key>KP_F1         Transmet la séquence d'échappement, conformément
 à                 à la procédure décrite dans dtterm(5x).
<Key>KP_F4
<Key>KP_0          Transmet le caractère ou la séquence d'échappement,
 à                 conformément à la procédure décrite dans dtterm(5x).
<Key>KP_9
<Key>KP_Equal
<Key>KP_Multiply
<Key>KP_Add
<Key>KP_Separator
<Key>KP_Subtract
<Key>KP_Decimal
<Key>KP_Divide
<Key>KP_Space
<Key>KP_Tab
<Key>KP_EnterVeuillez remarquer que les claviers de tous les constructeurs n'incluent
pas nécessairement chacune de ces touches. Consultez la documentation
de votre distributeur local pour obtenir des informations sur les affectations
de touches de remplacement.Boîte de dialogue Options généralesLa boîte de dialogue Options générales contient
les zones suivantes:Chacune de ces zones contient un ou plusieurs menus d'options dans lesquels
vous pouvez sélectionner la valeur des paramètres.Toutes les modifications apportées dans la boîte de dialogue
Options générales s'appliquent uniquement à la fenêtredttermà partir de laquelle vous l'avez
appelée.Caractéristiques du curseurPour modifier une caractéristique du curseur,
sélectionnez une option du menu repéré par un bouton
ou, pour l'option
Vitesse clignotement, tapez une valeur dans la zone de texte.Modification de l'aspect du curseurCurseur:aspectChoisissez Général dans
le menu Options de la fenêtredtterm.Amenez le pointeur sur la liste Style
curseur, appuyez sur le bouton 1
de la souris et sélectionnez le style de votre choix (Boîte,
Soulignement
ou Invisible).Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder
vos modifications ou également fermer la boîte de dialogue.Désactivation du clignotement du curseurCurseur:clignotementCurseur clignotantChoisissez Général dans
le menu Options de la fenêtredtterm.Amenez le pointeur sur le bouton du
menu Curseur clignotant,
appuyez sur le bouton 1 de la souris et sélectionnez le comportement
que vous souhaitez associer au curseur.La valeur par défaut Activé déclenche le clignotement
du curseur;
la valeur Désactivé le supprime.Si vous avez activé le clignotement
du curseur et que la vitesse
de clignotement par défaut de 250 millisecondes ne vous convient pas,
tapez la vitesse de votre choix dans la zone de texte prévue à
cet effet.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement
sauvegarder vos modifications ou également fermer la boîte de
dialogue.CouleursCette option permet d'inverser les couleurs de premier et
d'arrière-plan d'une fenêtredtterm.
Elle prend par défaut la valeur
Normal, c'est-à-dire que les couleurs de premier et d'arrière-plan
choisies s'affichent normalement.Inversion des couleurs de premier et d'arrière-planChoisissez Général dans
le menu Options de la fenêtredtterm.Amenez le pointeur sur le bouton du
menu Arrière-plan fenêtre,
appuyez sur le bouton 1 de la souris et sélectionnez le mode
d'affichage des couleurs.La valeur par défaut Normal affiche les couleurs telles quelles,
tandis que la valeur Inverse intervertit les couleurs de premier
et d'arrière-plan.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder
vos modifications ou également fermer la boîte de dialogue.Mode de défilementCette option permet d'activer et de désactiver le
défilement continu. Par défaut, il est désactivé.Activation du défilement continuChoisissez Général dans
le menu Options de la fenêtredtterm.Amenez le pointeur sur le bouton du
menu Défilement continu, appuyez
sur le bouton 1 de la souris et sélectionnez le mode de défilement
de
votre choix.Selon que l'option Désactivé ou Activé est sélectionnée,
le défilement
s'effectue page par page ou ligne par ligne, respectivement.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder
vos modifications ou également fermer la boîte de dialogue.Paramètres du signalPour intervenir sur les paramètres du signal, sélectionnez
une option
du menu repéré par un bouton ou, pour l'option Distance marge,
tapez une
valeur dans la zone de texte prévue à cet effet.Remplacement d'un signal sonore par un signal visibleType de signalChoisissez Général dans
le menu Options de la fenêtredtterm.Amenez le pointeur sur le bouton du
menu Type signal, appuyez sur le
bouton 1 de la souris et sélectionnez le type de signal de votre choix.Si vous choisissez Audible (valeur par défaut), le signal prend
la forme
d'une sonnerie. Si vous choisissez Visible, en revanche, le signal
se manifeste par un clignotement de la couleur d'arrière-plan.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder
vos modifications ou également fermer la boîte de dialogue.Activation d'un avertissement margeCete option peut prendre la valeur Désactivé (par
défaut) ou Activé.Choisissez Général dans
le menu Options de la fenêtredtterm.Amenez le pointeur sur le bouton du
menu Avertissement marge, appuyez
sur le bouton 1 de la souris et sélectionnez la valeur de votre choix.Si vous choisissez Désactivé, aucun avertissement de marge
n'est défini.
Si vous choisissez Activé, en revanche, un avertissement marge sonore
est émis.Si vous avez activé l'avertissement
de marge et que la distance marge par
défaut de 10 caractères ne vous convient pas, tapez la distance
de votre
choix dans la zone de texte prévue à cet effet.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder
vos modifications ou également fermer la boîte de dialogue.Boîte de dialogue Options de terminalLa boîte de dialogue Options de terminal contient les zones suivantes
:Chacune de ces zones contient un ou plusieurs menus d'options dans lesquels
vous pouvez sélectionner la valeur des paramètres.Toutes les modifications apportées dans la boîte de dialogue
Options de
terminal sont appliquées uniquement à la fenêtredttermà partir de laquelle
vous l'avez appelée.Paramètres du clavierCette zone de la boîte de dialogue Options de terminal comporte
les quatre
menus d'options suivants, qui permettent de définir les paramètres
du clavier:Séquence d'échappement curseurSéquence d'échappement du curseurL'option Séquence d'échappement curseur (voir"Séquences
d'échappement du curseur (mode VT220)"peut prendre l'une ou
l'autre
des valeurs suivantes: Normal (valeur par défaut) ou Application.
En mode normal, les touches de déplacement du curseur déplacent
ce dernier
dans la direction indiquée; en mode Application, elles génèrent
des séquences
d'échappement que l'application utilise pour son propre compte.A partir de la fenêtredtterm, choisissez l'option
Terminal du menu Options.Cliquez sur le bouton situé en
regard du menu d'options
"Séquence d'échappement curseur", dans la zone Paramètres
du clavier
de la boîte de dialogue Options de terminal.Amenez le pointeur sur le mode de votre
choix.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement
sauvegarder vos modifications ou également fermer la boîte de
dialogue.Mode clavier numériqueMode clavier numériqueL'option Mode clavier numérique peut prendre l'une ou l'autre
des
valeurs suivantes: Numérique (valeur par défaut) ou Application
(voir"Clavier numérique auxiliaire
(mode ANSI)".
En mode numérique, l'utilisation des touches du clavier numérique
entraîne
l'affichage de la valeur correspondante dans la fenêtredtterm.
En mode Application, l'utilisation de ces mêmes touches génère
des séquences d'échappement que l'application utilise pour son
propre compte.A partir de la fenêtredtterm, choisissez l'option Terminal du
menu Options.Cliquez sur le bouton situé en
regard du menu d'options
"Mode clavier numérique", dans la zone Paramètres du clavier
de la boîte de dialogue Options de terminal.Amenez le pointeur sur le mode de votre
choix.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder vos modifications ou également
fermer la boîte de dialogue.Séquence d'interligneL'option Séquence d'interligne peut prendre l'une ou l'autre
des valeurs suivantes: Retour chariot seulement (valeur par défaut)
ou Retour chariot et avance ligne. La première de ces options génère
seulement un retour chariot tandis que la deuxième génère
à la fois un retour chariot et une avance ligne.A partir de la fenêtredtterm, choisissez l'option Terminal du menu Options.Cliquez sur le bouton situé en
regard du menu d'options "Séquence d'interligne", dans la zone Paramètres
du clavier de la boîte de dialogue Options de terminal.Amenez le pointeur sur la valeur de
votre choix.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder vos modifications ou également
fermer la boîte de dialogue.Touches de fonction utilisateurL'option Touches de fonction utilisateur peut prendre l'une ou l'autre
des valeurs suivantes: Verrouillé ou Déverrouillé (valeur
par défaut).A partir de la fenêtredtterm, choisissez l'option Terminal du menu Options.Cliquez sur le bouton situé en
regard du menu d'options "Touches de fonction utilisateur", dans la zone Paramètres
du clavier de la boîte de dialogue Options de terminal.Amenez le pointeur sur la valeur de
votre choix.Cliquez sur Appliquer ou sur OK selon
que vous souhaitez uniquement sauvegarder vos modifications ou également
fermer la boîte de dialogue.Paramètres de l'écranParamètres de l'écranCette zone permet de définir les paramètres suivants :