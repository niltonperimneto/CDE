
# Editeur de texte - Référence

# Touches d'édition et de déplacement du curseur





# Menus















# Fenêtres et boîtes de dialogue





















# Réference générale







# Voir aussi



# Raccourcis
Menus: raccourcis

* **Fermer** 

Alt+F4
* **Copier** 

Ctrl+C
* **Couper** 

Ctrl+X
* **Supprimer** 

Touche Suppr
* **Rechercher/Remplacer** 

Ctrl+F
* **Surimprimer** 

Touche Inser
* **Coller** 

Ctrl+V
* **Imprimer** 

Ctrl+P
* **Tout sélectionner** 

Ctrl+/
* **Annuler** 

Ctrl+Z


Si votre clavier ne dispose pas de la touche Alt, demandez à
l'administrateur système d'identifier la touche équivalente.
# Touches d'édition et de déplacement du curseur
Touches d'éditionTouche CtrlTouche Alt
# Touches d'édition


* **Touche** 

Action
* **Retour arrière** 

Supprime le caractère placé avant le curseur.
* **Suppr** 

Supprime le caractère placé après le curseur d'insertion.
* **Ctrl+Suppr** 

Supprime tous les caractères, de la position du curseur
à la fin de la ligne en cours.
* **Ctrl+Retour arrière** 

Supprime le mot précédent.
* **Maj+Retour arrière** 

Supprime les caractères, de la position du curseur
au début de la ligne.

# Touches de déplacement du curseur
Touche Ctrl utilisée avec les flèches de déplacementFlèches de déplacementFlèches de déplacement du curseurNavigation à l'aide du clavierTouches: déplacement du curseurTouches: déplacement

* **Touche** 

Déplacement du curseur.
* **Flèche vers le haut** 

Remonte d'une ligne.
* **Flèche vers le bas** 

Descend d'une ligne.
* **Flèche vers la gauche** 

Un caractère vers la gauche.
* **Flèche vers la droite** 

Un caractère vers la droite.
* **Ctrl+Flèche vers la&sigspace;droite&sigspace;** 

Un mot vers la droite.
* **Ctrl+Flèche vers la&sigspace;gauche** 

Un mot vers la gauche.
* **Ctrl+Flèche vers le&sigspace;bas&sigspace;** 

Début du paragraphe suivant.
* **Ctrl+Flèche vers le&sigspace;haut** 

Début du paragraphe précédent.
* **Pos1** 

Début de la ligne en cours.
* **Fin** 

Fin de la ligne en cours.
* **Ctrl+Pos1** 

Début du document.
* **Ctrl+Fin** 

Fin du document.


Certaines combinaisons de touches peuvent être différentes sur
votre système. Dans ce cas, demandez à l'administrateur système
d'identifier les touches équivalentes.

&sigspace;

&sigspace;

Pour utiliser l'édition de liens entre touches Emacs, reportez-vous à.

&sigspace;
# Voir aussi











# Edition de liens entre touches Unix
Edition de liens entre touches UnixEdition de liens entre touches: UnixEdition de liens entre touches Emacs

L'édition de liens entre touches Unix permet d'utiliser un
ensemble de touches Emacs étendues, telles que Ctrl+F (avance
caractère) et Ctrl+N (ligne suivante), dans l'Editeur de texte.
Pour activer l'édition de liens entre touches Unix (désactivée
par défaut), effectuez les opérations suivantes:

Ajoutez la ligne suivante au fichier.Xdefaultsde votre répertoire personnel:#include "/usr/dt/app-defaults/`langue`/UNIXbindings"

Remplacez`langue`par la valeur de votre variable
d'environnement de langue. Si le fichier.Xdefaultsn'existe pas, créez-le dans votre répertoire personnel.

Déconnectez-vous de la session en cours.

Reconnectez-vous et relancez l'Editeur de texte.

&sigspace;

Lorsque vous utilisez l'édition de liens entre touches Unix,
l'Editeur de texte permet d'utiliser d'autres raccourcis pour
les commandes suivantes:

* **Commande** 

Autre raccourci
* **Annuler (Ctrl+Z)** 

Ctrl+_
* **Coller (Ctrl+V)** 

Maj+Inser
* **Rechercher/Remplacer (Ctrl+F)&sigspace;** 

Ctrl+S
* **Imprimer (Ctrl+P)** 

pas d'autre raccourci


&sigspace;
Pour modifier ces raccourcis, copiez le fichier/usr/dt/app-defaults/`langue`/UNIXbindingsdans votre fichier.Xdefaults, puis éditez-le.

Lorsque l'édition de liens entre touches Unix est activée, la touche Suppr
supprime le caractère qui précède le curseur, et non celui qui le suit.
# Voir aussi







# Menu Fichier


Menus: Fichier

* **Nouveau** 

Vide la fenêtre de l'Editeur de texte. Si vous n'avez pas
sauvegardé vos modifications dans le document, une boîte
de dialogue s'affiche, vous permettant de sauvegarder
votre travail avant de vider la fenêtre.
* **Ouvrir** 

Ouvre une boîte de dialogue permettant d'ouvrir un fichier
existant.
* **Inclure** 

Affiche une boîte de dialogue permettant d'indiquer un
fichier à insérer dans le document en cours.
* **Sauvegarder** 

Sauvegarde le document dans le fichier en cours. Si
l'option Retour ligne automatique est activée, la
boîte de dialogue Sauvegarder s'affiche. Si vous n'avez
pas encore sauvegardé le document en cours, la boîte de
dialogue Sauvegarder sous s'affiche.
* **Sauvegarder sous** 

Affiche une boîte de dialogue permettant de sauvegarder
un document dans un nouveau fichier.
* **Copier vers fichier** 

Copie les informations en cours de visualisation ou
d'édition vers un autre fichier, sans modifier la session
d'édition. Dans certains cas, cette commande remplace
la commande Sauvegarder sous.
* **Imprimer** 

Affiche une boîte de dialogue permettant de sélectionner des
options d'impression et de lancer l'impression d'un document.
* **Fermer** 

Ferme la fenêtre et sort de l'Editeur de texte.

# Voir aussi







# Menu Editer


Menus: Editer

* **Annuler** 

Annule la dernière opération de découpage, collage, effacement,
remplacement, inclusion ou formatage.
* **Couper** 

Supprime le texte sélectionné et le transfère dans le
presse-papiers.
* **Copier** 

Copie le texte sélectionné dans le presse-papiers.
* **Coller** 

Copie le texte découpé ou collé à la position actuelle du curseur.
* **Effacer** 

Remplace le texte sélectionné par des espaces.
* **Supprimer** 

Supprime le texte sélectionné.
* **Tout sélectionner** 

Sélectionne l'ensemble du document.
* **Rechercher/Remplacer** 

Ouvre une boîte de dialogue permettant de rechercher des mots
ou des phrases dans le document et de modifier les occurrences
détectées.
* **Vérif. orthographe&sigspace;** 

Lance le vérificateur orthographique sur le document.

# Voir aussi







# Menu Format


Menus: Format

* **Paramètres** 

Affiche une boîte de dialogue permettant de définir
les marges et l'alignement des paragraphes, et d'appliquer
les paramètres de formatage au document.
* **Paragraphe** 

Applique les paramètres au paragraphe dans lequel se
trouve le curseur.
* **Tout** 

Applique les paramètres à l'ensemble du document.

# Voir aussi







# Menu Options


Menus: Options

* **Surimprimer** 

Inverse le mode d'entrée de texte, ce qui permet d'écraser
les caractères existants.
* **Retour ligne automatique** 

Inverse le mode d'entrée de texte, ce qui permet un retour
à la ligne automatique lorsque le texte atteint le bord de
la fenêtre. Si cette option n'est pas activée, vous devez
appuyer sur Entrée pour passer à la ligne suivante.
* **Ligne d'état** 

Active ou désactive l'affichage d'une ligne d'état en bas
de la fenêtre. Cette ligne indique le numéro de la ligne
où se trouve le curseur, le nombre total de lignes du
document, les messages de l'application et également si
le mode Surimprimer est activé. Elle permet en outre
d'amener le curseur sur une ligne donnée.

# Voir aussi













# Menu Aide


Menus: Aide

* **Généralités** 

Explique comment lancer l'Editeur de texte.
* **Table des matières&sigspace;** 

Décrit la hiérarchie des sujets d'aide de l'Editeur de texte.
* **Tâches** 

Fournit des instructions permettant d'utiliser l'Editeur
de texte.
* **Référence** 

Affiche des informations de référence sur les fonctionnalités
de l'Editeur de texte (menus, boîtes de dialogue, ressources
et options de ligne de commande).
* **Sur l'élément** 

Transforme le curseur en pointeur d'aide. Cliquez ensuite
sur un élément d'une fenêtre ou d'une boîte de dialogue
de l'Editeur de texte pour en afficher la description.
* **Aide sur l'aide** 

Fournit de l'aide sur l'utilisation des fenêtres d'aide.
* **A propos de l'Editeur de texte** 

Affiche des informations sur la version et le copyright
de l'Editeur de texte.

# Voir aussi





# Fenêtre


Fenêtre de l'Editeur de texte

&sigspace;

* **Barre de menus** 

L'Editeur de texte dispose de cinq menus: Fichier, Editer,
Format, Options et Aide.
* **Fenêtre du document&sigspace;** 

Zone dans laquelle vous entrez le contenu de votre document.
* **Ligne d'état** 

Indique le numéro de la ligne où se trouve le curseur,
le nombre total de lignes du document, les messages de
l'application et également si le mode Surimprimer est
activé. Elle permet en outre d'amener le curseur sur
une ligne donnée.

# Voir aussi







# Boîte de dialogue Ouvrir un fichier


* **Entrez le chemin d'accès ou le nom du dossier&sigspace;** 

Indiquez le chemin d'accès au dossier courant.
* **Filtre** 

Un astérisque (*) affiche tous les fichiers. Vous pouvez
entrer des caractères génériques pour n'afficher que les
fichiers dotés d'une extension donnée. Par exemple, *.doc
répertorie uniquement les fichiers ayant l'extension .doc.
* **Fichiers** 

Répertorie les fichiers du dossier courant.
* **Dossiers** 

Répertorie les dossiers du dossier courant. Vous pouvez
ouvrir un fichier dans le dossier courant, dans un sous-dossier
ou dans un dossier différent.
* **Entrez le nom du fichier** 

Affiche le nom du fichier sélectionné dans la liste Fichiers.
Appuyez sur Entrée ou cliquez sur OK pour ouvrir le fichier.
Vous pouvez également taper le nom du fichier que vous
souhaitez ouvrir.
* **OK** 

Ouvre le fichier indiqué dans la zone Entrez le nom du fichier.
* **Mettre à jour** 

Affiche une nouvelle liste de fichiers après la modification
du critère de filtrage ou l'ouverture d'un nouveau dossier.
* **Annuler** 

Annule l'ouverture.
* **Aide** 

Décrit les zones d'entrée de texte, les listes de
sélection et les boutons de la boîte de dialogue.

# Voir aussi







# Boîte de dialogue Sauvegarder sous


* **Entrez le chemin d'accès ou le nom du dossier&sigspace;** 

Indiquez le chemin d'accès du dossier courant.
* **Filtre** 

Un astérisque (*) affiche tous les fichiers. Vous pouvez
entrer des caractères génériques pour n'afficher que les
fichiers dotés d'une extension donnée. Par exemple, *.doc
répertorie uniquement les fichiers ayant l'extension .doc.
* **Fichiers** 

Répertorie les fichiers du dossier courant.
* **Dossiers** 

Répertorie les dossiers du dossier courant.
* **Entrez le nom du fichier** 

Indiquez le nouveau nom de votre document dans cette zone.
* **OK** 

Sauvegarde le fichier sous le nom donné.
* **Mettre à jour** 

Affiche une nouvelle liste de fichiers après la modification
du critère de filtrage ou l'ouverture d'un nouveau dossier.
* **Annuler** 

Annule la sauvegarde.


Si le retour à la ligne automatique est activé, des options supplémentaires
s'affichent dans la boîte de dialogue:

Ajout de caractères de nouvelle ligne à la fin des lignes avec retour
automatique.

Il s'agit de l'option par défaut. Elle ajoute un caractère de nouvelle
ligne à la fin de chaque ligne avec retour automatique et conserve les
changements de ligne exactement comme ils apparaissent dans votre
document.

Pas d'ajout de caractères de nouvelle ligne. Seuls les changements de
ligne effectués à l'aide de la touche Entrée sont conservés.

Cette option conserve les changements de ligne insérés par la fonction
de retour à la ligne automatique. Lorsque vous réouvrez le document,
le texte s'ajuste à la largeur de la nouvelle fenêtre.
# Voir aussi







# Boîte de dialogue Sauvegarder


&sigspace;
Si le retour à la ligne automatique est activé, la boîte de dialogue
Sauvegarder s'affiche lorsque vous sauvegardez les modifications
effectuées dans le document. Les changements de ligne automatiques
peuvent être gérés de deux façons:

Ajout de caractères de nouvelle ligne à la fin des lignes avec retour
automatique. Il s'agit de l'option par défaut. Elle ajoute un caractère
de nouvelle ligne à la fin de chaque ligne avec retour automatique et
conserve les changements de ligne exactement comme ils apparaissent
dans votre document.

Pas d'ajout de caractères de nouvelle ligne. Seuls les changements de
ligne effectués à l'aide de la touche Entrée sont conservés.

Cette option conserve les changements de ligne insérés par la fonction
de retour à la ligne automatique. Lorsque vous réouvrez le document, le
texte s'ajuste à la largeur de la nouvelle fenêtre.

* **Oui** 

Sauvegarde le fichier en cours ou affiche la boîte de dialogue
Sauvegarder sous.
* **Non** 

Ne sauvegarde pas le fichier.
* **Annuler** 

Annule l'opération.
* **Aide** 

Décrit la boîte de dialogue Sauvegarder.


Cette boîte de dialogue s'affiche également lorsque vous sélectionnez
une option de menu qui entraîne la perte de vos modifications.
# Voir aussi







# Boîte de dialogue Inclure un fichier


* **Entrez le chemin d'accès ou le nom du dossier&sigspace;** 

Indiquez le chemin d'accès du dossier courant.
* **Filtre** 

Un astérisque (*) affiche tous les fichiers. Vous pouvez
entrer des caractères génériques pour n'afficher que les
fichiers dotés d'une extension donnée. Par exemple, *.doc
répertorie uniquement les fichiers ayant l'extension .doc.
* **Fichiers** 

Répertorie les fichiers du dossier courant.
* **Dossiers** 

Répertorie les dossiers du dossier courant.

Vous pouvez ouvrir un fichier dans le dossier courant, dans un
sous-dossier ou dans un dossier différent.
* **Entrez le nom du fichier** 

Affiche le nom du fichier sélectionné dans la liste
Fichiers. Appuyez sur Entrée ou cliquez sur OK pour
ouvrir le fichier. Vous pouvez également taper le nom
du fichier que vous souhaitez inclure.
* **OK** 

Insère le fichier indiqué dans la zone Entrez
le nom du fichier à l'emplacement du curseur.
* **Mettre à jour** 

Affiche une nouvelle liste de fichiers après la
modification du critère de filtrage ou l'ouverture d'un
nouveau dossier.
* **Annuler** 

Annule l'inclusion.
* **Aide** 

Décrit les zones d'entrée de texte, les listes de
sélection et les boutons de la boîte de dialogue.

# Voir aussi







# Boîte de dialogue Vérif. orthographe




* **Mots mal orthographiés** 

Répertorie les mots mal orthographiés détectés dans
le document.
* **Modifier en** 

Tapez le mot correctement orthographié dans cette zone.
* **Rechercher** 

Recherche la première occurrence du mot mal orthographié.
La recherche démarre à la position du curseur d'insertion.
* **Modifier** 

Remplace le mot mis en évidence par le mot correctement
orthographié.
* **Tout modifier** 

Remplace toutes les occurrences du mot mal orthographié.
* **Fermer** 

Ferme la boîte de dialogue.


&sigspace;

La boîte de dialogue Vérif. orthographe n'est disponible qu'en anglais.
# Voir aussi







# Boîte de dialogue Rerchercher/Remplacer




* **Rechercher** 

Tapez la phrase ou le mot recherché dans cette zone.
Lors de la recherche, la distinction majuscules/minuscules
est respectée.
* **Modifier en** 

Tapez le texte de remplacement dans cette zone.
* **Rechercher** 

Détecte la première occurrence du mot mal orthographié.
* **Modifier** 

Remplace le mot mis en évidence par le texte de remplacement.
* **Tout modifier** 

Remplace toutes les occurrences de la chaîne de recherche.
* **Fermer** 

Ferme la boîte de dialogue.

# Voir aussi







# Boîte de dialogue Paramètres de formatage




&sigspace;

* **Marge gauche** 

Nombre de caractères d'indentation du texte imprimé,
à partir du bord gauche du papier.
* **Marge droite** 

Nombre de colonnes du texte.
* **Aligné à gauche** 

Aligne le texte sur la marge gauche.
* **Aligné à droite** 

Aligne le texte sur la marge droite.
* **Justifié** 

Aligne le texte de sorte que les marges droite et
gauche soient égales.
* **Centré** 

Centre le texte.
* **Paragraphe** 

Applique les paramètres au paragraphe dans lequel se
trouve le curseur.
* **Tout** 

Applique les paramètres à l'ensemble du document.
* **Fermer** 

Ferme la boîte de dialogue.

# Voir aussi







# Boîte de dialogue Copier vers fichier


D'autres applications peuvent utiliser l'Editeur de texte comme outil
d'édition de données et restreindre la façon d'enregistrer les modifications.
Par exemple, dans certains cas, la commande Sauvegarder sous peut être
remplacée par la comme Copier vers fichier, ce qui vous permet de créer
une copie des données que vous visualisez ou éditez, sans modifier votre
session d'édition.

* **Entrez le chemin d'accès ou le nom du dossier&sigspace;** 

Indiquez le chemin d'accès du dossier courant.
* **Filtre** 

Un astérisque (*) affiche tous les fichiers. Vous pouvez
entrer des caractères génériques pour n'afficher que les
fichiers dotés d'une extension donnée. Par exemple, *.doc
répertorie uniquement les fichiers ayant l'extension .doc.
* **Fichiers** 

Répertorie les fichiers du dossier courant.
* **Dossiers** 

Répertorie les dossiers du dossier courant.
* **Entrez le nom du fichier** 

Tapez le nouveau nom de votre document dans cette zone.
* **OK** 

Copie les données vers le fichier indiqué.
* **Mettre à jour** 

Affiche une nouvelle liste de fichiers après la modification
du critère de filtrage ou l'ouverture d'un nouveau dossier.
* **Annuler** 

Annule la sauvegarde.


Si le retour à la ligne automatique est activé, des options supplémentaires
s'affichent dans la boîte de dialogue:

Ajout de caractères de nouvelle ligne à la fin des lignes avec retour
automatique.

Il s'agit de l'option par défaut. Elle ajoute un caractère de nouvelle
ligne à la fin de chaque ligne avec retour automatique et conserve les
changements de ligne exactement comme ils apparaissent dans votre
document.

Pas d'ajout de caractères de nouvelle ligne. Seuls les changements de
ligne effectués à l'aide de la touche Entrée sont conservés.

Cette option conserve les changements de ligne insérés par la fonction
de retour à la ligne automatique. Lorsque vous réouvrez le document, le
texte s'ajuste à la largeur de la nouvelle fenêtre.
# Voir aussi





# Boîte de dialogue Fichier existe déjà


Lorsque vous sauvegardez un document, si vous entrez le nom d'un
fichier qui existe déjà, cette boîte de dialogue s'affiche.

Pour écraser le fichier d'origine, cliquez sur OK.

Pour entrer un nom de fichier différent, cliquez sur Annuler et
sélectionnez Sauvegarder sous dans le menu Fichier.
# Voir aussi





# Syntaxe et options de ligne de commande
Editeur de texte: lancement dans une fenêtre de terminal

La syntaxe de la commande de lancement de l'Editeur de texte est la
suivante:dtpad [`options`]

`options`peut avoir l'une des valeurs suivantes:

* **-server** 

Indique que l'Editeur de texte doit
être lancé en mode serveur et qu'aucune fenêtre initiale ne doit s'afficher.
Pour chaque lancement suivant de l'Editeur de texte en mode demandeur, le
serveur créera une nouvelle fenêtre.
* **-standAlone** 

Cette option force le fonctionnement
en mode autonome de l'Editeur de texte. Dans ce mode, il gère les opérations
d'édition indépendamment du serveur. Cette option peut être utile lorsque
vous voulez qu'une instance de l'Editeur de texte s'exécute dans un
environnement différent de celui des autres fenêtres. Par exemple, vous
pouvez lancer une instance dans un environnement local différent ou avec
des ressources de couleurs différentes. Cela revient à attribuer la valeur
True à la ressourcestandAlone.
* **-exitOnLastClose** 

Indique que le processus
serveur de l'Editeur de texte doit prendre fin lorsque la dernière
fenêtre d'édition est fermée. Cette option est utilisée uniquement avec
l'option-server. Si celle-ci n'est pas précisée,
le processus serveur de l'Editeur de texte reste actif indéfiniment,
même lorsque toutes les fenêtres d'édition ont été fermées.
* **-noBlocking** 

Indique que le processus demandeur
de l'Editeur de texte doit prendre fin dès que le processus serveur
signifie qu'il peut accéder à un fichier du dossier référencé. En
l'absence de cette option, le processus demandeur de l'Editeur de texte
se bloque et ne prend fin que lorsque le serveur l'informe que la
fenêtre a été fermée.
* **-statusLine** 

Cette option provoque l'affichage
d'une ligne d'état en bas de la fenêtre d'édition de l'Editeur de texte.
Cette ligne indique le numéro de la ligne sur laquelle se trouve le
curseur, le nombre total de lignes du document, les messages d'application
et également si le mode Surimprimer est activé. Elle permet en outre
d'amener le curseur sur une ligne donnée.
* **-wrapToFit** 

Indique que l'option Retour ligne
automatique est activée au démarrage.

# Voir aussi






Reportez-vous à la page de manueldtpad(1)pour
obtenir la liste et la description complète des options de ligne de
commande de l'Editeur de texte.
# Ressources
Ressources de l'applicationRessources

Les ressources suivantes contrôlent l'aspect et le comportement de
l'Editeur de texte.

Dtpad*server: [ true | false ]

Indique que l'Editeur de texte doit être lancé en mode
serveur pour traiter toutes les demandes d'affichage suivantes.
Si la valeur de cette ressource est égale à True, cela revient
à indiquer l'option de ligne de commande-server.

Dtpad*standAlone: [ true | false ]

Indique si le processus en cours de l'Editeur de texte
doit s'exécuter en mode autonome et gérer lui-même les opérations
d'édition ou en mode demandeur, dans lequel l'édition est gérée par
un processus serveur unique et distinct. Si la valeur de cette
ressource est égale à True, cela revient à indiquer l'option
de ligne de commande-standAlone.

Dtpad*blocking: [ true | false ]

Indique que lorsque l'Editeur de texte est exécuté en mode
demandeur, mode dans lequel l'édition est gérée par un processus
serveur distinct, le processus demandeur ne doit pas prendre fin tant
que les fenêtres associées à la requête d'édition n'ont pas été
fermées. Si la valeur de cette ressource est égale à False, cela
revient à indiquer l'option de ligne de commande-noBlocking.

Dtpad*exitOnLastClose: [ true | false ]

Indique si le processus serveur de l'Editeur de texte doit
prendre fin lorsque la dernière fenêtre d'édition active est fermée.
Si la valeur de cette ressource est égale à False, le processus serveur
continue de s'exécuter, dans l'attente d'un message d'édition d'un
fichier. Si cette ressource est égale à True, le processus serveur
de l'Editeur de texte s'arrête lorsque la dernière fenêtre d'édition
active est fermée.

Dtpad*statusLine: [ true | false ]

Indique si l'Editeur de texte doit afficher la ligne d'état en
bas de la fenêtre d'édition. Si cette ressource est égale à True, cela
revient à indique l'option de ligne de commande-statusLine.

Dtpad*wrapToFit: [ true | false ]

Indique si l'option Retour ligne automatique de l'Editeur
de texte doit être activée (True) ou non (False) lorsque l'éditeur est
lancé. Si la valeur de cette ressource est égale à True, cela revient
indiquer l'option de ligne de commande-wrapToFit.
# Voir aussi






Reportez-vous à la page de manueldtpad(1)pour la liste et la description complète des ressources de l'Editeur de
texte.
# Ensemble de fichiers de l'Editeur de texte
Fichier exécutable de l'Editeur de texteValeurs par défaut de l'applicationRessources

Le fichier exécutable et le fichier des valeurs par défaut de l'application de
l'Editeur de texte sont les suivants:

* **Fichier exécutable** 

/usr/dt/bin/dtpad
* **Fichier des valeurs par défaut de l'application** 

/usr/dt/app-defaults/`lang`/Dtpad

# Voir aussi




