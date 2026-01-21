dtterm - TâchesLancer dttermVous pouvez lancer un émulateur de terminaldtterm&ProductName;:à partir du Tableau de bord,à partir du Gestionnaire de
fichiers,à l'aide d'une commande entrée
dans une fenêtre de terminal existante,à partir du Gestionnaire d'applications,à l'aide de l'option Nouveau
du menu déroulant Fenêtre dedtterm.Lancer dtterm à partir du Tableau de bordLancement: émulateur de terminalFermeture: émulateur de terminalTerminal: boutonBouton: terminalL'icône Terminal se trouve dans le panneau secondaire Applications
personnelles.Cliquez sur l'icône Terminal.
Le témoin d'activité se met à clignoter, ce qui indique
que le terminal a été activé.La fenêtredtterms'affiche peu après.Lancer dtterm à partir du Gestionnaire de fichiersChoisissez Ouvrir terminal dans le
menu Fichier.dtterms'ouvre alors: le répertoire
courant qui lui est associé est le même que celui de la vue du
Gestionnaire de fichiers.Lancer un émulateur de terminal autre que dttermTout émulateur de terminal autre quedttermdoit être lancé à partir de la ligne
de commande, dans une fenêtre d'émulateur de terminal existante.A l'invite de ligne de commande, tapez
le nom de l'émulateur accompagné, éventuellement, d'options.
Par exemple, pour lancerxterm, tapez:xterm  [options] &optionsreprésente des options de personnalisation de l'émulateur
de terminal.&indique que l'émulateur de terminal s'exécute en arrière-plan,
ce qui vous permet de poursuivre votre travail dans la fenêtre d'origine.Sauf indication contraire, l'émulateur de terminal est lancé
dans l'espace de travail en cours.Lancer dtterm à partir du menu FenêtreChoisissez Nouveau dans le menu Fenêtre,
à partir d'une fenêtredttermexistante. Un double de la fenêtredtterms'affiche.ExemplesLa commande suivante ouvre une fenêtredttermdans l'espace de travailNotes sur
le projet:dtterm -xrm '*workspaceList: "Notes sur le projet"' &La commande suivante ouvre une fenêtredttermsur un écran du système "lgmcd":dtterm -display lgmcd:0.1 &Voir aussiPour plus de détails sur les
options de la commandedtterm, reportez-vous
à la page de manueldtterm (1X).Fermer dttermFermeture:dttermMenu Gestionnaire de fenêtres: boutonTapezexitsur la ligne de commande et appuyez sur Entréeouchoisissez
Fermer dans le menu Gestionnaire de fenêtresouchoisissez
Fermer dans le menu déroulant qui s'affiche lorsque vous cliquez sur
le bouton situé dans l'angle supérieur gauche du cadre du gestionnaire
de fenêtres.La sélection de Quitter à partir du menu Fenêtre
dedttermest la méthode recommandée pour
refermerdtterm.Insérer du texte grâce à la méthode copier-collerCoupe: texteCollage: texteTexte: copier-collerCopier du texteSélectionnez le texte à
copier à l'aide du bouton 1 de la souris (le texte s'affiche alors
en surbrillance).Une fois l'opération terminée,
relâchez le bouton de la souris.Le texte n'est pas supprimé de son emplacement d'origine.Coller du textePositionnez le curseur à l'endroit
où vous souhaitez insérer le texte.Cliquez sur le bouton 2 de la souris.Une copie de la sélection en cours est "collée" à
l'emplacement choisi. Si vous avez d'autres exemplaires à coller, renouvelez
la procédure ci-dessus.Redimensionner la fenêtre dttermRedimensionnement de la fenêtreSélectionnez Taille fenêtre
dans le menu Options.Sélectionnez une valeur:80x24132x24Par défautIl est possible que la taille de l'écran et de la police utilisés
ne permette pas le redimensionnement à 132 colonnes. Dans ce cas, la
taille maximale autorisée est utilisée.Vous pouvez également effectuer cette opération à
l'aide des dispositifs de contrôle du Gestionnaire de
fenêtresVoir aussiRedimensionner le contenu de la fenêtreFonctionRedimensionnementLe redimensionnement d'une fenêtre d'émulateur de terminal
n'est pas nécessairement répercuté sur les applications
qu'elle contient. Utilisez la procédure ci-dessous pour redimensionner
la représentation de l'application à l'écran.A l'invite de ligne de commande, tapez
:eval `resize`Voir aussiLancer des applications dans une fenêtre dttermA l'invite de ligne de commande, entrez
la commande de lancement de l'application.La commande de lancement d'une application respecte la syntaxe générique
suivante:application[options] &applicationNom de l'application.optionsListe d'informations facultatives à transmettre à l'application.&Indique que l'application s'exécute en arrière-plan, ce
qui signifie que vous pouvez continuer à travailler dans la fenêtre
de l'émulateur de terminal.ExemplePour lancer une horloge numérique à partir de la
ligne de commande, tapez:xclock -digital &Voir aussiPour connaître la commande et
les options à utiliser avec telle ou telle application, reportez-vous
à la page de manuel correspondante ou à une autre source de
documentation.Etablir une connexion à un système éloignéA l'aide de la commande rloginPour vous connecter à un système hôte éloigné,
utilisez la commanderloginà partir
d'une fenêtre d'émulateur de termi