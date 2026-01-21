
# Gestionnaire d'applications - Référence
Référence

La présente section contient des informations de référence relatives au
Gestionnaire d'applications, classées par catégorie.
# Généralités







# Menus du Gestionnaire d'applications











# Fenêtre et boîtes de dialogue du Gestionnaire d'applications





















# Utilisation de la souris
Boutons de la sourisSélection d'objets
# Bouton 1&emdash;Sélectionner et faire glisser


Le bouton 1 de la souris (par défaut, le bouton gauche) permet
de sélectionner des objets et de les faire glisser.

Pour sélectionner un objet, cliquez sur son icône.

Pour étendre la sélection, appuyez sur la touche Ctrl et, tout en la
maintenant enfoncée, cliquez sur une autre icône.
# Bouton 2&emdash;Faire glisser


Cette opération permet de:

déplacer des objets (appuyez sur le bouton 1 et, tout en le maintenant
enfoncé, faites glisser une icône);

copier des objets (appuyez sur la touche Ctrl et faites glisser un objet);

créer un lien symbolique (appuyez sur la touche Maj et faites glisser
un objet).

Sur une souris à deux boutons, appuyez simultanément sur les deux boutons.
# Bouton 3&emdash;Menus
Menus contextuels

Le bouton 3 (par défaut, le bouton droit) permet d'afficher des menus
instantanés.

Amenez le pointeur de la souris à un emplacement où un menu est
disponible, puis appuyez sur le bouton 3et maintenez-le enfoncé.
# Noms de fichiers
Gestionnaire d'applications:noms de fichiersNoms de fichiers:Gestionnaire d'applicationsLibellés d'actionsLibellés d'icônes d'actions

Dans les Gestionnaires de fichiers et d'applications, les fichiers
et les dossiers sont représentés sous forme d'icônes, dont le libellé
correspond au nom du fichier ou du dossier associé.

Les icônes d'actions peuvent déroger à cette règle: lorsque vous affichez
le menu associé à l'icône d'action Horloge digitale du groupe d'applications
Outils_Bureau, vous constatez que le nom de fichier, apparaissant dans la
partie supérieure du menu, ne correspond pas au libellé de l'icône.

Le nom de fichier réel apparaît dans les boîtes de dialogue Copier un
fichier et Lier un fichier.
# Emplacement du dossier du Gestionnaire d'applications
Gestionnaire d'applications:emplacement du dossier

Le comportement du Gestionnaire d'applications est comparable à celui
du Gestionnaire de fichiers; il s'agit en fait d'une vue d'un dossier
spécial dans lequel les applications enregistrées sont stockées.
Le Gestionnaire de connexion crée le dossier du Gestionnaire
d'applications lors de chaque connexion.

Il n'est généralement pas nécessaire de connaître l'emplacement du
Gestionnaire d'applications. Cependant, cela peut être utile lors
des procédures de résolution des incidents.

Le Gestionnaire d'applications se trouve dans le répertoire suivant:/var/dt/appconfig/appmanager/`nom_dossier_spécial`

nom_dossier_spécialest un nom unique attribué par le système,
associé au système et au nom de connexion.

Ce nom nedoit pasêtre modifié directement.dtappgatherAction Recharger Applications

Une fois que le Gestionnaire de connexion a créé le dossier, il
lance le programme du Bureaudtappgather, qui rassemble tous
les groupes d'applications.

Au cours d'une session, vous pouvez cliquer deux fois sur Recharger
Applications (dans le groupe d'application Outils_Bureau) pour
relancer le programmedtappgather.
# Barre de menus du Gestionnaire d'applications


La barre de menus du Gestionnaire d'applications contient les
options suivantes:








# Menu Fichier du Gestionnaire d'applications


Gestionnaire d'applications:menu Fichier

* **Nouveau dossier** 

Permet d'indiquer le nom du dossier à créer.
* **Nouveau fichier** 

Permet d'indiquer le nom du fichier à créer.
* **Remonter** 

Permet de passer au niveau supérieur de la hiérarchie
de dossiers.
* **Passer à** 

Permet d'afficher la boîte de dialogue Passer à, dans
laquelle vous pouvez indiquer un nouveau nom de dossier
ou sélectionner un dossier parmi ceux que vous avez
déjà ouverts.
* **Rechercher** 

Permet d'afficher la boîte de dialogue Rechercher, à
partir de laquelle vous pouvez rechercher des fichiers
et des dossiers, en indiquant un masque ou un nom.
* **Fermer** 

Permet de fermer la vue en cours du Gestionnaire
d'applications.

# Menu Sélectionné(s) du Gestionnaire d'applications


Gestionnaire d'applications:menu Sélectionné(s)

La plupart des groupes d'applications ne peuvent être modifiés que par
l'administrateur système; il est donc possible que vous n'ayez pas accès
à certaines options de menus.

* **Déplacer** 

Permet d'indiquer le chemin d'accès dans lequel
le fichier doit être déplacé.
* **Copier** 

Permet d'indiquer le chemin d'accès dans lequel le fichier
sélectionné doit être copié. Cette commande n'est disponible
que lorsqu'un seul fichier est sélectionné.
* **Copier comme lien** 

Permet d'indiquer le chemin d'accès du lien à associer
à l'objet sélectionné.
* **Afficher dans l'espace de travail** 

Permet de placer l'objet sélectionné dans l'angle droit
du fond de l'espace de travail.
* **Mettre dans la corbeille** 

Permet de placer les objets sélectionnés dans la corbeille.
* **Renommer** 

Permet d'accéder à la zone d'édition du nom de
l'objet sélectionné.
* **Modifier les droits d'accès** 

Permet d'afficher la boîte de dialogue Modifier les
droits d'accès associée à l'objet sélectionné.
* **Tout sélectionner** 

Permet de sélectionner tous les objets de la vue en cours
du Gestionnaire d'applications.
* **Tout désélectionner** 

Permet de désélectionner tous les objets de la vue en cours
du Gestionnaire d'applications.
* **Actions** 

Si des actions sont associées à l'objet sélectionné, elles
s'affichent au bas du menu (elles apparaissent également
dans le menu instantané de l'objet).

# Menu Visualiser du Gestionnaire d'applications


Gestionnaire d'applications:menu VisualiserOptions d'affichage du Gestionnaire d'applications

* **Ouvrir nouvelle vue** 

Permet d'ouvrir une nouvelle vue du dossier en cours
dans le Gestionnaire d'applications.
* **Définir les options de vue** 

Permet d'afficher la boîte de dialogue Définir les options
de vue, à partir de laquelle l'aspect et le comportement
de la vue en cours peuvent être modifiés.
* **Sauvegarder comme options par défaut** 

Permet de sauvegarder les options en cours, la taille de
la fenêtre et la liste de filtres comme options par défaut.
* **Afficher les objets cachés** 

Permet d'afficher ou de cacher des fichiers.
Vous pouvez indiquer les données cachées à l'aide
de l'option Définir les options de filtre.
* **Définir les options de filtre** 

Permet d'afficher la boîte de dialogue Définir les options
de filtre, dans laquelle vous pouvez indiquer les fichiers
à cacher (en fonction de leur type de données ou de leur nom).
* **Mettre à jour** 

Régénère le contenu du dossier en cours et le réaffiche
en prenant en compte les éventuelles modifications.

La mise à jour ne s'applique pas aux applications ajoutées
depuis l'ouverture de la session. Pour régénérer le contenu
du Gestionnaire d'applications, cliquez deux fois sur
Recharger applications, dans le groupe d'applications
Outils_Bureau.

# Menu Aide du Gestionnaire d'applications


Gestionnaire d'applications:menu Aide

* **Généralités** 

Permet d'afficher des informations générales sur le
Gestionnaire d'applications.
* **Tâches** 

Permet d'afficher des informations sur l'utilisation
du Gestionnaire d'applications.
* **Référence** 

Permet d'afficher des informations sur les fenêtres, les
menus et les boîtes de dialogue du Gestionnaire
d'applications.
* **Sur l'élément** 

Permet de modifier la forme du curseur; vous pouvez
ensuite cliquer sur un élément de la fenêtre du Gestionnaire
d'applications pour afficher l'aide correspondante.
* **Aide sur l'aide** 

Permet d'afficher l'aide relative à l'utilisation des
fenêtres d'aide.
* **A propos du Gestionnaire d'applications** 

Permet d'afficher le numéro de version et les informations de
copyright du Gestionnaire d'applications.

# Menus instantanés des objets


Gestionnaire d'applications:menus instantanésMenus instantanés:Gestionnaire d'applicationsUn menu instantané est associé à la plupart des objets du
Gestionnaire d'applications.

Les deux premières options de ce menu (Afficher dans l'espace de
travail et Mettre dans la corbeille) sont communes à tous les objets).

Les actions déterminées par le type de données figurent dans la
partie inférieure du menu (elles apparaissent également dans le menu
Actions qui s'affiche lorsque l'icône de l'objet est sélectionnée).

* **Afficher dans l'espace de travail** 

Permet de placer l'objet sur le Bureau de l'espace de
travail en cours. L'emplacement des objets est déterminé
par la ressourceobjectPlacement(par défaut, l'angle
droit de l'écran).
* **Mettre dans la corbeille** 

Permet de supprimer l'objet, en le plaçant dans la corbeille.
* **Aide** 

Permet d'afficher l'aide relative aux menus instantanés.
* **Actions** 

Si des actions sont associées à l'objet, elles s'affichent
dans la partie inférieure du menu.

# Fenêtre du Gestionnaire d'applications


Gestionnaire d'applications:fenêtre principaleLe niveau supérieur du Gestionnaire d'applications contient des icônes
de groupes d'applications.

Un groupe d'applications est un dossier contenant une ou
plusieursicônes d'applications(ouicônes d'actions).
Il peut également contenir d'autres types de fichiers
d'applications, tels que des fichiers "read me" et des modèles).
# Boîte de dialogue Copier un fichier


Gestionnaire d'applications:copie d'objetsCopie d'objets

* **Objet sélectionné** 

Permet d'afficher l'objet à copier.
* **Dossier de destination** 

Permet d'indiquer le chemin d'accès complet du dossier
cible.
* **Nom de la copie** 

Permet d'indiquer le nom du nouvel objet.
* **OK** 

Permet d'effectuer l'opération et de fermer la boîte
de dialogue.
* **Annuler** 

Permet d'annuler l'opération et de fermer la boîte de
dialogue.
* **Afficher icône** 

&newline;Permet d'afficher l'icône qui représentera le nouveau
fichier.
* **Aide** 

Permet d'afficher l'aide relative à la boîte de dialogue.


Pour copier des fichiers ou des dossiers, vous pouvez également
utiliser la souris.
# Boîte de dialogue Lier un fichier/dossier




* **Objet sélectionné** 

Permet d'afficher l'objet à lier.
* **Dossier de destination** 

Permet d'indiquer le chemin d'accès complet du dossier
cible.
* **Nom de la copie** 

Permet d'indiquer le nom du nouvel objet.
* **OK** 

Permet d'effectuer l'opération et de fermer la
boîte de dialogue.
* **Annuler** 

Permet d'annuler l'opération et de fermer la boîte
de dialogue.
* **Afficher icône** 

&newline;Permet d'afficher l'icône qui représentera le
nouveau fichier.
* **Aide** 

Permet d'afficher l'aide relative à la boîte
de dialogue.


Pour lier des fichiers ou des dossiers, vous pouvez également
utiliser la souris.
# Boîte de dialogue Déplacer un fichier


Gestionnaire d'applications:déplacement d'objetsDéplacement d'objets

* **Objet sélectionné** 

Permet d'afficher l'objet à déplacer.
* **Dossier de destination** 

Permet d'indiquer le chemin d'accès complet du dossier
cible.
* **OK** 

Permet d'effectuer l'opération et de fermer la
boîte de dialogue.
* **Annuler** 

Permet d'annuler l'opération et de fermer la boîte
de dialogue.
* **Aide** 

Permet d'afficher l'aide relative à la modification
du nom des fichiers et des dossiers.

# Boîte de dialogue Nouveau fichier




* **Nouveau nom de fichier** 

&newline;Permet d'indiquer le nom du nouveau fichier ou dossier.
Si vous le créez dans un dossier différent, vous devez
indiquer son chemin d'accès complet.
* **OK** 

Permet de créer le fichier et de fermer la boîte
de dialogue.
* **Appliquer** 

Permet de créer le fichier sans fermer la boîte de
dialogue, afin de pouvoir la réutiliser.
* **Annuler** 

Permet d'annuler la commande et de fermer la boîte de
dialogue.
* **Afficher icône** 

Si vous modifiez le type du fichier, l'icône correspondante
peut en être affectée. Pour prévisualiser la nouvelle icône,
sélectionnez cette option; l'icône affichée dans la boîte
de dialogue est modifiée. Par exemple, si vous indiquez un
nom dont le suffixe est.tif, puis sélectionnez Afficher
icône, l'icône du type de données TIFF s'affiche.
* **Aide** 

Permet d'afficher l'aide relative à la boîte de dialogue.

# Boîte de dialogue Nouveau dossier




* **Nom du nouveau dossier** 

&newline;Permet d'indiquer le nom du nouveau dossier.
Si vous le créez dans un dossier différent, vous devez
indiquer son chemin d'accès complet.
* **OK** 

Permet de créer le dossier et de fermer la boîte
de dialogue.
* **Appliquer** 

Permet de créer le dossier sans fermer la boîte de
dialogue, afin de pouvoir la réutiliser.
* **Annuler** 

Permet d'annuler la commande et de fermer la boîte de
dialogue.
* **Afficher icône** 

Si vous modifiez le type de données, l'icône du nouveau
dossier peut en être affectée. Pour prévisualiser la
nouvelle icône, sélectionnez cette option; l'icône affichée
dans la boîte de dialogue est modifiée.
* **Aide** 

Permet d'afficher l'aide relative à la boîte de dialogue.

# Boîte de dialogue Définir les options de filtre


Gestionnaire d'applications:filtrage d'objetsFiltrage d'objets

* **Sélectionner les types de fichiers à cacher/afficher** 

Il s'agit d'un bouton de bascule, qui permet
d'afficher la liste des types de données définis sur
le système. Sélectionnez ceux qui seront cachés
ou affichés par le Gestionnaire d'applications,
selon le libellé du bouton.
* **Tout sélectionner** 

Permet de sélectionner tous les types de données.
Si vous en désélectionnez certains, eux seuls
s'afficheront dans la zone de visualisation du
Gestionnaire d'applications.
* **Tout désélectionner** 

Permet de désélectionner tous les types de données.
* **Chaîne de filtrage (facultatif)** 

Permet de filtrer par nom de fichier. Par exemple, si vous
tapez*.o, le Gestionnaire d'applications n'affiche pas
les fichiers dont le suffixe est.o. Les types de
données indiqués dans cette zone s'ajoutent aux types de
données sélectionnés dans la liste d'icônes, située dans
la partie supérieure de la boîte de dialogue.
* **OK** 

Permet d'appliquer les critères de filtrage en cours et
de fermer la boîte de dialogue.
* **Appliquer** 

Permet d'appliquer les critères de filtrage en cours sans
fermer la boîte de dialogue.
* **Par défaut** 

Permet de rétablir la liste des critères de filtrage
par défaut, qui comprend notamment DOT_FILE, DOT_FOLDER
et CURRENT_FOLDER. Pour que cette liste soit prise en
compte, sélectionnez Appliquer ou OK.
* **Annuler** 

Permet de rétablir les derniers critères appliqués et
de fermer la boîte de dialogue.
* **Aide** 

Permet d'afficher l'aide relative au filtrage d'objets.

# Boîte de dialogue Recherche de fichiers ou de dossiers


Cette boîte de dialogue permet de rechercher, dans un dossier et
dans ses sous-dossiers, des fichiers en fonction de leur nom ou
de leur contenu.

* **Nom du fichier ou du dossier** 

Permet d'indiquer le nom du fichier ou du dossier à
rechercher (les caractères génériques sont autorisés).
* **Contenu** 

Permet de rechercher la chaîne indiquée dans
les fichiers et les dossiers.
* **Dossier de recherche** 

Permet d'indiquer le chemin d'accès du dossier à
partir duquel la recherche doit être lancée; elle
portera également sur ses sous-dossiers.
* **Fichiers trouvés** 

Permet d'afficher la liste des fichiers ou des dossiers
détectés lors de la recherche. Pour afficher un fichier ou
un dossier dans une vue du Gestionnaire d'applications,
cliquez deux fois dessus.
* **Ouvrir nouvelle vue** 

Permet d'ouvrir, dans le Gestionnaire d'applications, une
vue du dossier contenant le fichier sélectionné dans la
zone Fichiers trouvés. Si un dossier a été détecté lors
de la recherche, son contenu s'affiche.
* **Afficher dans l'espace de travail** 

Permet d'afficher sur le fond de l'espace de travail en
cours le fichier ou le dossier sélectionné.
* **Lancer** 

Permet de lancer la recherche.
* **Arrêter** 

Permet de mettre fin à la recherche en cours. Vous pouvez
sélectionner ce bouton à tout moment, y compris lorsque
le curseur a la forme d'un sablier.
* **Annuler** 

Permet d'interrompre la recherche en cours et de fermer
la boîte de dialogue.
* **Aide** 

Permet d'afficher l'aide relative à la recherche d'objets.

# Boîte de dialogue Modifier les droits d'accès


La boîte de dialogue Modifier les droits d'accès permet de définir de
nouvelles permissions (lecture, écriture et exécution) pour les fichiers
et les dossiers dont vous êtes le propriétaire. Si vous tentez d'effectuer
cette opération sur un dossier ou un fichier dont vous n'êtes pas le
propriétaire, la boîte de dialogue indiquant les droits d'accès en cours
s'affiche, mais vous ne pouvez pas les modifier. Cette boîte de dialogue
indique également le chemin d'accès complet, la taille, ainsi que la
date et l'heure de la dernière modification du fichier ou du dossier.

* **Nom du propriétaire** 

Nom de l'utilisateur auquel l'objet appartient. Seul
l'administrateur système (utilisateur root) peut modifier
la propriété d'un objet.
* **Nom du groupe** 

Nom du groupe d'utilisateurs auquel les droits d'accès
affichés dans la colonne Groupe de la liste des permissions
sont accordés. Si vous êtes le propriétaire de l'objet,
vous pouvez indiquer un autre groupe dont vous êtes membre.
L'utilisateur root peut définir n'importe quel groupe
existant.
* **Autorisations** 

Si vous êtes le propriétaire de l'objet, vous pouvez modifier
les droits d'accès en lecture, écriture et exécution associés.
Pour ce faire, cochez les cases appropriées.
* **OK** 

Permet d'appliquer les sélections en cours et de fermer
la boîte de dialogue.
* **Annuler** 

Permet de fermer la boîte de dialogue sans appliquer
les modifications.
* **Aide** 

Permet d'afficher l'aide relative à la modification des
droits d'accès.

# Boîte de dialogue Définir les caractéristiques d'une vue


Gestionnaire d'applications:caractéristiques de vueCaractéristiques de vue dans le Gestionnaire d'applicationsCette boîte de dialogue permet de modifier les caractéristiques
d'affichage des fichiers et des dossiers dans le Gestionnaire
d'applications.


# En-têtes


Cette option permet d'indiquer les lignes d'en-tête à afficher
dans la fenêtre du Gestionnaire d'applications.

* **Chemin d'icônes** 

Permet d'afficher le dossier en cours sous la forme d'une
chaîne d'icônes de dossiers.
* **Chemin texte** 

Permet d'afficher sous forme de texte le chemin d'accès
complet du dossier en cours, juste au-dessus de la zone
de visualisation principale. Pour le modifier, cliquez
dessus. Pour ouvrir l'un des dossiers du chemin d'accès,
cliquez deux fois dessus.
* **Ligne de message** 

Permet d'afficher le nombre de fichiers, de dossiers et
de fichiers cachés contenus dans le dossier en cours.



# Position


Cette option permet de définir la présentation des icônes dans une
vue du Gestionnaire d'applications.

* **Aléatoire** 

Les objets s'affichent à l'endroit où vous les placez.
* **Rangées et colonnes** 

Les objets sont triés et disposés sous forme de
lignes et de colonnes.



# Afficher
Structure hiérarchique de dossiersStructure hiérarchique (Gestionnaire d'applications)

* **Par dossier** 

Permet d'afficher le contenu du dossier en cours.
* **Par structure hiérarchique** 

Permet d'afficher le contenu du dossier en cours sous
forme d'arborescence.
* **Dossiers seulement** 

Si vous sélectionnez l'option Par structure hiérarchique,
seuls les dossiers s'affichent. Il s'agit de l'option
par défaut.
* **Dossiers, puis fichiers** 

Si vous sélectionnez l'option Par structure hiérarchique,
seuls les dossiers s'affichent, puis les fichiers lorsque
vous cliquez sur le signe +.
* **Dossiers et fichiers** 

Si vous sélectionnez l'option par Structure hiérarchique,
les dossiers et les fichiers s'affichent.



# Représentation


* **Par nom seulement** 

Permet d'afficher seulement les noms des objets.
* **Par grandes icônes** 

Permet d'afficher les icônes (de taille normale) et
les noms des objets; il s'agit de l'option par défaut.
* **Par petites icônes** 

Permet d'afficher les icônes (de taille réduite) et
les noms des objets.

Format étendu
* **Par nom, date, taille...** 

Permet d'afficher des informations détaillées sur
l'objet: nom, date de modification, taille, permissions,
propriétaire et groupe.



# Tri


Cette option permet de sélectionner le critère de tri pour l'affichage
des fichiers et des dossiers s'affichent.

* **Alphabétique** 

Sélectionnez Croissant (de A à Z, de a à z) ou
Décroissant (de Z à A, de z à a). L'ordre par
défaut est alphabétique (de A à Z).
* **Par type de fichier** 

Les fichiers sont groupés selon leur type de fichier.
* **Par date** 

Sélectionnez Croissant (du plus ancien au plus récent)
ou Décroissant (du plus récent au plus ancien).
* **Par taille** 

Sélectionnez Croissant (du plus petit au plus grand)
ou décroissant (du plus grand au plus petit).

# Ordre


Cette option permet de sélectionner l'ordre selon lequel les fichiers
et les dossiers s'affichent.

* **Croissant** 

Du plus ancien au plus récent, du plus petit au plus grand.
* **Décroissant** 

Du plus récent au plus ancien, du plus petit au plus grand.
