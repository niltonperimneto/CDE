
# Fenêtre du Gestionnaire d'aide avec volumes d'aide


Un volume d'aide contient un ou plusieurs volumes. Vous pouvez sélectionner
et visualiser des volumes d'aide donnés à partir du Gestionnaire d'aide.

La fenêtre du Gestionnaire d'aide comporte un bouton de navigation
supplémentaire, intitulé Niveau supérieur. Après avoir consulté différents
volumes d'aide, vous pouvez utiliser ce bouton pour retourner à l'écran
principal du Gestionnaire d'aide.
# Voir aussi



# Recherche d'expressions régulières


* **Caractère** 

Signification
* **&sigspace;. (point)** 

Correspond à n'importe quel caractère.
* **&sigspace; * (astérisque)** 

Correspond à 0 ou plusieurs éléments
du caractère précédent.
* **&sigspace; ? (point d'interrogation)&sigspace;** 

Correspond à 0 ou 1
élément du caractère précédent.
* **&sigspace; | (barre verticale)** 

Spécifie deux schémas de recherche et
correspond aux deux (OU logique).
* **&sigspace; () (parenthèses)** 

Encadrent une expression.


Pour rechercher dans une expression régulière un caractère ayant une
signification particulière, faites-le précéder de &newline; (barre oblique
inversée).

&sigspace;
# Exemples


L'expression suivante recherche des entrées d'index contenant le mot
"souris", suivi d'un certain nombre de caractères, puis de "clic":souris.*clic

L'expression suivante recherche des entrées d'index contenant le mot
"souris" ou "clic":souris | clic

L'exemple suivant recherche des entrées d'index contenant "Gestionnaire de
sessions" ou "Gestionnaire de configuration":Gestionnaire.*(session | configuration)
# Voir aussi


.

Pour plus de détails sur les expressions régulières, reportez-vous
à la page de manuelregexp(5).