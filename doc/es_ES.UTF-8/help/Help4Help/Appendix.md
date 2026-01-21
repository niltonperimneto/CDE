
# Ventana Gestor de ayuda con Familias de ayudas


Una familia de ayudas incluye uno o más volúmenes de ayuda. Desde el
Gestor de ayudas puede seleccionar y visualizar volúmenes de ayuda
individuales.

La ventana Gestor de ayuda contiene un botón de desplazamiento
adicional denominado Nivel superior. Tras examinar distintos
volúmenes de ayuda, puede utilizar Nivel superior para volver a la
pantalla principal del Gestor de ayuda.
# Consulte también



# Búsqueda de patrones de expresión regular


* **Carácter** 

Significado
* **&sigspace;. (punto)** 

Coincide con cualquier carácter
* **&sigspace;* (asterisco)** 

Coincide con 0 o más de los caracteres
anteriores
* **&sigspace;? (signo de interrogación)&sigspace;** 

Coincide con 0 ó 1
de los caracteres anteriores
* **&sigspace;| (barra vertical)** 

Especifica dos patrones de búsqueda
y coincide con cualquier patrón (OR lógico)
* **&sigspace;() (paréntesis)** 

Encierra una expresión de patrón


Para buscar un carácter que tenga un significado especial en una
expresión regular especifique una &newline; (barra inclinada invertida) antes
del carácter.

&sigspace;
# Ejemplos


Esta expresión busca las entradas de índice que contienen la
palabra "ratón" seguida de cualquier número de caracteres seguidos de
"pulsar".ratón.*pulsar

Esta expresión busca las entradas de índice que contienen la
palabra "ratón" o "pulsar".ratón | pulsar

Este ejemplo busca las entradas de índice que contienen "Gestor
de sesiones" o "Gestor de estilos".(sesión | estilo).*gestor
# Consulte también




Para obtener más información sobre expresiones regulares, consulte
la página manregexp(5)