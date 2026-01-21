
# Editeur d'icônes - Référence











# Chemins de recherche
Chemins de recherche: icônesIcônes: chemins de recherche

Le chemin de recherche des fichiers d'icônes est défini par les
variables d'environnement DTUSERAPPSEARCHPATH et DTAPPSEARCHPATH,
sauf si les variables suivantes existent:

* **DTUSERICONSEARCHPATH** 

Variable personnelle. S'il y a lieu, elle
doit être définie dans/`Rép_personnel`/.dtprofile.
* **DTICONSEARCHPATH** 

Variable système.

# Chemin de recherche par défaut


La valeur par défaut de DTICONSEARCHPATH est la suivante:/`Rép_personnel`/.dt/icons/etc/dt/appconfig/icons/C/usr/dt/appconfig/icons/C
# Modification du chemin de recherche personnel
Ajout: répertoire au chemin de recherche des actionsRépertoire: ajout au chemin de recherche des actionsAction: ajout d'un répertoire au chemin de rechercheChemin de recherche des actionsChemin: recherche des actionsVariable d'environnement VUEACTIONSEARCHPATHVariables d'environnement: DTACTIONSEARCHPATH

Pour ajouter un répertoire au chemin de recherche, ajoutez la ligne
suivante au fichier/`Rép_personnel`/.dtprofile:DTUSERICONSEARCHPATH=`chemin`[,`chemin`...]
# Modification du chemin de recherche système


Les variables indiquant les chemins de recherche système sont définies
dans des fichiers situés dans le répertoire/etc/dt/Xsession.d.

Si DTICONSEARCHPATH existe déjà dans un fichier de ce type, ajoutez
le nouveau répertoire à la valeur définie.

Si tel n'est pas le cas, définissez-la en indiquant le chemin de
recherche par défaut et ceux que vous souhaitez ajouter (le chemin
par défaut est indiqué en commentaire dans le fichier/usr/dt/bin/dtsearchpath).
# Outils de l'Editeur d'icônes
Outils: Editeur d'icônesEditeur d'icônes: outilsOutils: dessin

Tout outil sélectionné le reste jusqu'à ce que vous en choisissiez
un autre. Vous trouverez ci-dessous la liste des outils et une
description de leur fonction.

Crayon&newline;Permet de dessiner des lignes et des pixels à main levée.

Ligne&newline;Permet de dessiner des lignes droites.

Rectangle&newline;Permet de dessiner des rectangles (pleins ou non).

Cercle&newline;Permet de dessiner des cercles (pleins ou non).

Effacer&newline;Permet d'effacer des zones de grande taille.

Coloriage&newline;Permet de remplir une zone avec une couleur sélectionnée.

Polyligne&newline;Permet de dessiner des lignes droites reliées entre elles
(la première et la dernière ne sont pas reliées, de sorte que le
polygone est ouvert).

Polygone&newline;Permet de dessiner des lignes droites reliées entre elles
(la première et la dernière sont reliées, de sorte que le
polygone est fermé).

Ovale&newline;Permet de dessiner des ellipses (pleines ou non).

Sélection&newline;Permet de sélectionner une zone; il s'agit d'une opération
préalable à l'exécution de nombreuses commandes du menu Editer.


# Remplissage


Pour remplir les outils Rectangle, Polygone, Cercle et Ellipse avec une
couleur, sélectionnez l'option Remplir sous la palette d'outils
(voir aussi la rubrique).
# Menus de l'Editeur d'icônes











# Menu Fichier de l'Editeur d'icônes




* **Nouveau** 

Vous invite à spécifier une largeur et une hauteur
(en pixels), puis crée une zone de dessin vide aux
dimensions indiquées (voir aussi la rubrique).
* **Ouvrir** 

Ouvre un fichier bitmap ou pixmap existant
(voir aussi la rubrique).
* **Sauvegarder** 

Sauvegarde une icône sous son nom en cours.
Si elle n'a pas de nom, l'Editeur d'icônes vous invite
à lui en attribuer un (voir aussi la rubrique).
* **Sauvegarder sous** 

Vous invite à indiquer un nouveau nom et sauvegarde l'icône
en cours (voir aussi la rubrique).
* **Quitter** 

Ferme l'Editeur d'icônes. Si vous avez apporté des
modifications et que vous ne les avez pas sauvegardées,
un message vous avertit qu'elles seront perdues si vous
quittez l'Editeur. Pour sauvegarder les modifications,
sélectionnez Annuler, puis Sauvegarder.

# Menu Editer de l'Editeur d'icônes




* **Annuler** 

Annule la dernière opération et restaure l'état précédent
de l'icône. La plupart des opérations de dessin peuvent
être annulées (voir aussi la rubrique.)
* **Couper zone** 

Supprime la zone sélectionnée de l'icône, en la remplissant
avec la couleur transparente. La zone découpée est
sauvegardée dans le presse-papiers de l'Editeur d'icônes
(voir aussi la rubrique).
* **Copier zone** 

Copie la zone sélectionnée dans le presse-papiers de
l'Editeur d'icônes (voir aussi la rubrique).
* **Coller zone** 

Colle le contenu du presse-papiers dans l'icône
(voir aussi les rubriqueset).
* **Pivoter zone** 

Fait pivoter la zone sélectionnée, vers la droite ou
vers la gauche, selon un angle de 90 degrés (voir aussi
la rubrique).
* **Inverser zone** 

Crée une image miroir de la zone sélectionnée, en
l'inversant à la verticale ou à l'horizontale (voir aussi
la rubrique).
* **Ajuster zone** 

Permet de redimensionner (mettre à l'échelle) la zone
sélectionnée (voir aussi la rubrique).
* **Redimensionner l'icône** 

Vous invite à indiquer une nouvelle taille pour l'icône
en cours (voir aussi la rubrique).
* **Ajouter point de repère** 

Permet de désigner un pixel de l'icône en cours comme
point de repère, en cliquant dessus.

Lorsqu'une image est utilisée comme pointeur de la souris,
le point de repère désigne l'emplacementréeldu pointeur
(voir aussi la rubrique).
* **Supprimer point de repère** 

Supprime le point de repère de l'icône en cours.
* **Saisir image écran** 

Capture une zone de l'écran et la charge dans la zone de
dessin (voir aussi la rubrique).
Les coordonnées X-Y affichées au-dessus de la zone de travail
définissent la taille, en pixels, de la zone capturée.
* **Effacer l'icône** 

Efface la zone de dessin en cours (voir aussi la rubrique).

# Menu Options de l'Editeur d'icônes




* **Grille affichée** 

Active ou désactive l'affichage de la grille (par
défaut, elle est affichée).
* **Format de sortie** 

Définit le format de sortie utilisé lors de la
sauvegarde de l'icône.

XBM pour les bitmaps X en noir et blanc. Généralement,
l'extension de ces fichiers est.bm.

XPM pour les pixmaps X couleur (format par
défaut). Généralement, l'extension de ces
fichiers est.pm.

Voir aussi la rubrique.
* **Agrandissement** 

Modifie la taille de l'image affichée dans la
zone de dessin. Les facteurs d'agrandissement
disponibles vont de 2 à 12 (de deux à douze
fois la taille normale).

# Menu Aide de l'Editeur d'icônes




* **Généralités** 

Affiche une introduction à l'utilisation de
l'Editeur d'icônes.
* **Tâches** 

Affiche les instructions associées à la plupart des
tâches de l'Editeur d'icônes.
* **Référence** 

Affiche une description des fonctions de l'Editeur
d'icônes (fenêtres, boîtes de dialogue, menus et
ressources).
* **Sur l'élément** 

Affiche la description de l'élément sélectionné
dans une fenêtre de l'Editeur d'icônes.
* **Aide sur l'aide** 

Affiche l'aide relative à l'utilisation des fenêtres
d'aide.
* **A propose de l'Editeur d'icônes** 

Affiche la version et les informations de copyright
de l'Editeur d'icônes.

# Fenêtres et boîtes de dialogue de l'Editeur d'icônes

# Subtopics







# Fenêtre principale de l'Editeur d'icônes


Les cinq zones principales de cette fenêtre sont les suivantes:

Laligne d'étatsituée sous la barre de menus indique
l'outil sélectionné et les coordonnées du pixel correspondant à la
position du pointeur.

Lazone de dessinest l'emplacement dans lequel
l'image est dessinée.

Lapalette d'outilscontient des outils de dessin
(permettant d'effacer ou de sélectionner des zones, par exemple).

Lapalette de couleurscontient huit couleurs statiques,
huit nuances de gris et six couleurs dynamiques.

Lesicônes en taille réellepermettent de visualiser,
en taille réelle, les versions couleur et noir et blanc des icônes
que vous dessinez.
# Palettes de couleurs de l'Editeur d'icônes




* **Couleurs statiques** 

Huit couleurs standard: noir,
blanc, les trois couleurs primaires
et les trois couleurs secondaires.
* **Nuances de gris** 

Huit nuances de gris (de 10%
à 90%).
* **Couleurs dynamiques** 

Six couleurs dynamiques utilisées
lorsque vous modifiez les couleurs
à l'aide du Gestionnaire de configuration.


Vous pouvez également consulter les rubriques suivantes de l'aide du
Gestionnaire de configuration:

Sélection
d'une palette,

Modification
du nombre de couleurs utilisées par le Bureau.
# Boîtes de dialogue Ouvrir et Sauvegarder sous de l'Editeur d'icônes


* **Entrez le chemin d'accès ou le nom du dossier:** 

Tapez le chemin d'accès complet du dossier contenant l'icône à ouvrir
ou de celui dans lequel vous voulez la sauvegarder.
* **Dossiers** 

Affiche la liste des sous-dossiers du dossier indiqué dans la zone
"Entrez le chemin d'accès ou le nom du dossier". Lorsque vous cliquez
deux fois sur un sous-dossier, les dossiers et fichiers qu'il contient
s'affichent dans les listes correspondantes. Vous pouvez également
sélectionner un dossier dans la liste de dossiers, puis cliquer sur
Mettre à jour.
* **Fichiers** 

Affiche la liste des fichiers du dossier indiqué dans la zone "Entrez
le chemin d'accès ou le nom du dossier". Lorsque vous modifiez le contenu
de cette zone, vous devez sélectionner le bouton Mettre à jour pour que
la liste de fichiers soit régénérée.
* **Entrez un nom de fichier:** 

Indiquez l'icône à charger ou à sauvegarder. Pour ce faire, cliquez sur
son nom (dans la liste de fichiers) ou tapez-le directement dans cette
zone et sélectionnez Ouvrir.

Les noms d'icônes doivent être indiqués sous la forme nom.taille.format.
L'Editeur d'icônes détermine automatiquement la taille et le format
adéquats, en fonction de la taille et du format de sortie sélectionnés
à partir de la barre de menus.
* **Ouvrir ou Sauvegarder** 

Ouvre ou sauvegarde le fichier, puis ferme la boîte de dialogue.
* **Mettre à jour** 

Régénère les listes de dossiers et de fichiers, en fonction du dossier
affiché dans la zone "Entrez le chemin d'accès ou le nom du dossier".
Pour indiquer un dossier dans cette zone, tapez son nom directement et
cliquez sur le bouton Mettre à jour, ou cliquez deux fois sur l'entrée
correspondante dans la liste de dossiers.
* **Annuler** 

Annule l'ouverture ou la sauvegarde et ferme la boîte de dialogue.
* **Aide** 

Affiche l'aide relative à la boîte de dialogue.

# Boîte de dialogue Sauvegarder sous




* **Entrez le chemin d'accès ou le nom du dossier:** 

Tapez le chemin d'accès complet du dossier dans lequel vous voulez
sauvegarder l'icône.
* **Dossiers** 

Affiche la liste des sous-dossiers du dossier indiqué dans la zone
"Entrez le chemin d'accès ou le nom du dossier". Lorsque vous cliquez
deux fois sur un sous-dossier, les dossiers et fichiers qu'il contient
s'affichent dans les listes correspondantes. Vous pouvez également
sélectionner un dossier dans la liste de dossiers, puis cliquer sur
Mettre à jour.
* **Fichiers** 

Affiche la liste des fichiers du dossier indiqué dans la zone "Entrez
le chemin d'accès ou le nom du dossier". Lorsque vous modifiez le contenu
de cette zone, vous devez sélectionner le bouton Mettre à jour pour que
la liste de fichiers soit régénérée.
* **Entrez un nom de fichier:** 

Indiquez le nom de l'icône à sauvegarder. Les noms d'icônes doivent
être indiqués sous la forme nom.taille.format. L'Editeur d'icônes
détermine automatiquement la taille et le format adéquats, en fonction
de la taille et du format de sortie sélectionnés à partir de la barre
de menus.
* **Sauvegarder** 

Sauvegarde le fichier et ferme la boîte de dialogue.
* **Mettre à jour** 

Régénère les listes de dossiers et de fichiers, en fonction du dossier
affiché dans la zone "Entrez le chemin d'accès ou le nom du dossier".
Pour indiquer un dossier dans cette zone, tapez son nom directement et
cliquez sur le bouton Mettre à jour, ou cliquez deux fois sur l'entrée
correspondante dans la liste de dossiers.
* **Annuler** 

Annule la sauvegarde et ferme la boîte de dialogue.
* **Aide** 

Affiche l'aide relative à la boîte de dialogue.

# Boîte de dialogue de confirmation de sortie


Cette boîte de dialogue vous invite à confirmer votre demande de sortie
de l'Editeur d'icônes; ainsi, vous ne risquez pas de perdre des données
accidentellement.

Sélectionnez OK pour continuer ou Annuler pour annuler la demande
de sortie.