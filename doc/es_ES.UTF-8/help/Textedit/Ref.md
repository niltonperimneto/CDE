
# Referencia del Editor de textos

# Teclas de edición y de cursor del Editor de textos





# Menús del Editor de textos















# Ventana y cuadros de diálogo del Editor de textos





















# Referencia general







# Consulte también



# Teclas de método abreviado del menú del Editor de textos
menús: teclas de método abreviado

* **Cerrar** 

Alt+F4
* **Copiar** 

Control+C
* **Cortar** 

Control+X
* **Suprimir** 

Tecla Supr
* **Buscar/Cambiar** 

Control+B
* **Sustitución** 

Tecla Insert
* **Pegar** 

Control+V
* **Imprimir** 

Control+I
* **Seleccionar Todo** 

Control+/
* **Deshacer** 

Control+Z


Si su teclado no dispone de tecla Alt, solicite al
administrador del sistema que identifique la tecla correspondiente.
# Teclas de cursor y de edición
teclas de edicióntecla ControlTecla Alt
# Teclas de edición


* **Tecla** 

Acción
* **Retroceso** 

Suprime el carácter anterior a la posición del cursor
* **Supr** 

Suprime el carácter que sigue al
cursor de inserción
* **Control+Supr** 

Suprime todos los caracteres desde el cursor hasta el final de la línea actual
* **Control+Retroceso** 

Suprime la palabra anterior
* **Despl+Retroceso** 

Suprime caracteres desde el cursor hasta el inicio de la línea

# Teclas de cursor
tecla de control, usada con teclas de direcciónteclas de direcciónteclas de movimiento del cursordesplazamiento mediante tecladoteclas: movimiento del cursorteclas: dirección

* **Tecla** 

Movimiento del cursor
* **Flecha arriba** 

Arriba una línea
* **Flecha abajo** 

Abajo una línea
* **Flecha izquierda** 

Un carácter a la izquierda
* **Flecha derecha** 

Un carácter a la derecha
* **Control+Flecha&sigspace;Derecha&sigspace;** 

Una palabra a la derecha
* **Control+Flecha&sigspace;Izquierda** 

Una palabra a la izquierda
* **Control+Flecha&sigspace;Abajo&sigspace;** 

Inicio del párrafo siguiente
* **Control+Flecha&sigspace;Arriba** 

Inicio del párrafo anterior
* **Inicio** 

Inicio de la línea actual
* **Fin** 

Final de la línea actual
* **Control+Inicio** 

Inicio del documento
* **Control+Fin** 

Final del documento


Algunas combinaciones de teclas pueden ser distintas según el
sistema que se esté utilizando.
Si es así, solicite al administrador del sistema que identifique las
teclas correspondientes.

&sigspace;

&sigspace;

Para utilizar asignaciones de teclas Emacs en el Editor de textos,
consulte.

&sigspace;
# Consulte también











# Asignaciones de teclas Unix
Asignaciones de teclas Unixasignaciones de teclas: UnixAsignaciones de teclas Emacs

Las asignaciones de teclas Unix proporcionan un juego de teclas
ampliadas Emacs, como
Control+F (avanzar carácter) y Control+N (línea siguiente), en el
Editor de textos. Para habilitar las asignaciones de teclas Unix
(cuyo valor predeterminado es desactivado), realice lo siguiente:

Edite el archivo.Xdefaultsen el directorio de
inicio añadiendo esta línea al archivo:#include "/usr/dt/app-defaults/`lang`/UNIXbindings"

Sustituya`lang`por el valor de la variable de entorno
de su idioma. Si el archivo.Xdefaultsno
existe, cree el archivo en el directorio de inicio.

Salga de la sesión actual.

Inicie la sesión y reinicie el Editor de textos.

&sigspace;

Al utilizar asignaciones de teclas Unix, el Editor de textos
proporciona teclas alternativas de método abreviado de menú para
estos mandatos:

* **Mandato** 

Tecla alternativa de método abreviado del menú
* **Deshacer (Control+Z)** 

Control+_
* **Pegar (Control+V)** 

Despl+Insert
* **Buscar/Cambiar (Control+B)&sigspace;** 

Control+S
* **Imprimir (Control+I)** 

sin alternativa


&sigspace;
Si desea modificar estas teclas de método abreviado, copie el
contenido del archivo/usr/dt/app-defaults/`lang`/UNIXbindingsen el archivo.Xdefaults, y realice a
continuación los cambios.

Al habilitar las asignaciones de teclas Unix, la tecla Supr suprimirá
el carácter anterior y no el carácter que sigue al cursor.
# Consulte también







# Menú Archivo del Editor de textos


menús:Archivo

* **Nuevo** 

Borra la ventana del Editor de textos. Si el documento
incluye cambios que no se han guardado, se mostrará un cuadro de
diálogo que le permitirá guardar el documento antes de que se cierre
la ventana.
* **Abrir** 

Muestra un cuadro de diálogo para abrir un archivo
existente.
* **Incluir** 

Muestra un cuadro de diálogo para especificar un archivo
que se va a insertar en el documento
actual.
* **Guardar** 

Guarda el documento en el archivo actual. Si el ajuste
automático está activado, se muestra el diálogo Guardar. Si el
documento actual no se ha guardado con anterioridad, se mostrará el
cuadro de diálogo Guardar como.
* **Guardar como** 

Muestra un cuadro de diálogo para guardar un
en un archivo nuevo.
* **Copiar en archivo** 

Copia la información que se muestra o se está
editando en un archivo aparte sin cambiar la sesión de edición. En
algunas ocasiones, el mandato Guardar como se sustituye por el
mandato Copiar en archivo.
* **Imprimir** 

Muestra un cuadro de diálogo para la selección de
opciones de impresión y para imprimir un documento.
* **Cerrar** 

Cierra la ventana del Editor de textos y sale de éste.

# Consulte también







# Menú Editar del Editor de textos


menús:Editar

* **Deshacer** 

Deshace la última operación de cortar, pegar, borrar,
suprimir, sustituir, incluir o formatear
* **Cortar** 

Elimina el texto seleccionado y lo almacena en el portapapeles
* **Copiar** 

Copia el texto seleccionado y lo almacena en el portapapeles
* **Pegar** 

Copia el texto de la última operación de cortar o copiar
en la posición actual del cursor
* **Borrar** 

Sustituye por espacios el texto seleccionado
* **Suprimir** 

Suprime el texto seleccionado
* **Seleccionar todo** 

Selecciona todo el texto del documento
* **Buscar/Cambiar** 

Abre un cuadro de diálogo que sirve para buscar
palabras o frases en el documento y realizar los
cambios en las coincidencias localizadas
* **Verificar ortografía&sigspace;** 

Ejecuta el corrector ortográfico en
el documento

# Consulte también







# Menú Formato del Editor de textos


menús:Formato

* **Valores** 

Muestra un cuadro de diálogo para la configuración
de márgenes y alineación de párrafos, y para aplicar la configuración
del formato en el documento.
* **Párrafo** 

Aplica los valores al párrafo que contiene el
cursor
* **Todo** 

Aplica los valores a todo el documento

# Consulte también







# Menú Opciones del Editor de textos


menús: Opciones

* **Sustitución** 

Conmuta la modalidad de entrada de texto para que se
pueda escribir sobre caracteres ya existentes.
* **Reiniciar** 

Conmuta la modalidad de entrada de texto que hace que
las líneas se reinicien automáticamente en el extremo de la ventana. Si
Reiniciar no está activado, cada vez que finalice una línea deberá
pulsar Intro.
* **Línea de estado** 

Conmuta la visualización de la línea de estado al
final de la ventana. La línea de estado informa del número de línea
en la que se encuentra el cursor, el número total de líneas del
documento, los mensajes de la aplicación e indica si la modalidad
de Sustitución está activada.
Asimismo, proporciona un método para desplazar el cursor a un número
de línea específico.

# Consulte también













# Menú Ayuda del Editor de textos


menús:Ayuda

* **Información general** 

Explica cómo iniciar el Editor de textos.
* **Tabla de contenido &sigspace;** 

Proporciona una guía de los temas de
ayuda del Editor de textos.
* **Tareas** 

Proporciona instrucciones sobre las tareas para la mayoría
de las operaciones del Editor de textos.
* **Referencia** 

Resume las funciones del Editor de textos, como los
menús, cuadros de diálogo, recursos y opciones de línea de mandatos.
* **Sobre el tema** 

Cambia el cursor por un puntero especial que se
puede pulsar sobre un elemento en la ventana del Editor de textos o del
cuadro de diálogo. Se mostrará una descripción del
elemento que se haya pulsado.
* **Uso de la ayuda** 

Proporciona ayuda sobre el uso de las ventanas de Ayuda.
* **Acerca del Editor de textos** 

Muestra la versión y la
información del copyright para el Editor de textos.

# Consulte también





# Ventana del Editor de textos


ventana, Editor de textos

&sigspace;

* **Barra de menús** 

El Editor de textos proporciona cinco menús:
Archivo, Editar, Formato, Opciones y Ayuda.
* **Ventana de Documento&sigspace;** 

El área en el que se escribe el
contenido del documento.
* **Línea de estado** 

Informa del número de línea en el que se encuentra
el cursor, el número total de líneas del documento, los mensajes de
la aplicación e indica si la modalidad de Sustitución está activada.
Asimismo, proporciona un método para desplazar el cursor a un número
de línea específico.

# Consulte también







# Cuadro de diálogo Abrir un archivo del Editor de textos


* **Entrar nombre de ruta o de carpeta&sigspace;** 

Identifica el
nombre de ruta de la carpeta actual.
* **Filtro** 

Un asterisco (*) muestra todos los archivos. Se pueden
entrar caracteres comodín para mostrar sólo los
archivos que coincidan con una extensión
determinada. Por ejemplo, *.doc lista sólo los
archivos con la extensión .doc.
* **Archivos** 

Lista los archivos que se encuentran en la carpeta
actual.
* **Carpetas** 

Lista las carpetas que se encuentran en la carpeta
actual. Se puede abrir un archivo de la carpeta actual, una
subcarpeta o una carpeta distinta.
* **Entrar nombre del archivo** 

Muestra el nombre de archivo
seleccionado en la lista Archivos.
Pulse Intro o bien OK para abrir el archivo. También
puede escribir el nombre del archivo que desee
abrir.
* **OK** 

Abre el archivo especificado en el campo Entre un
nombre de archivo.
* **Actualizar** 

Muestra una nueva lista de archivos después de
cambiar la tecla de filtro o de cambiar a una carpeta nueva.
* **Cancelar** 

Cancela la operación Abrir.
* **Ayuda** 

Describe los campos de entrada de texto, listas de
selección y botones del cuadro de diálogo.

# Consulte también







# Cuadro de diálogo Guardar como del Editor de texto


* **Entrar nombre de ruta o carpeta&sigspace;** 

Identifica el nombre
de ruta de la carpeta actual.
* **Filtro** 

Un asterisco (*) muestra todos los archivos. Se pueden
entrar caracteres comodín para mostrar sólo los
archivos que coincidan con una extensión
determinada. Por ejemplo, *.doc lista sólo los
archivos con la extensión .doc.
* **Archivos** 

Lista los archivos que se encuentran en la
carpeta actual.
* **Carpetas** 

Lista las carpetas que se encuentran en la carpeta actual.
* **Entrar nombre del archivo** 

Campo en el que se escribe un nombre
nuevo de archivo para el documento.
* **OK** 

Guarda el archivo con el nombre que se ha proporcionado.
* **Actualizar** 

Muestra una nueva lista de archivos después de
cambiar la clave de filtro o de cambiar a una carpeta nueva.
* **Cancelar** 

Cancela la operación Guardar como.


Si utiliza el reinicio automático de texto, aparecerán estas opciones adicionales en el
cuadro de diálogo:

Añadir caracteres de nueva línea al final de las líneas reiniciadas.
Esta es la opción predeterminada. Añade un carácter de línea nueva al
final de cada línea reiniciada y conserva los saltos de línea tal
como aparecen en el documento.

No añadir nuevas líneas. Sólo se mantendrán los saltos de línea
creados por Intro.

Esta opción mantiene los saltos de línea insertados por el
reinicio automática.
Si se vuelve a abrir el documento, el texto se adaptará a la anchura
de la nueva ventana.
# Consulte también







# Cuadro de diálogo Guardar del Editor de textos


&sigspace;
Si se utiliza la opción Reiniciar, se visualiza el cuadro de diálogo
Guardar al guardar cambios en el documento. Los saltos de línea
insertados por el reinicio automático se pueden gestionar de dos modos:

Añadir caracteres de nueva línea al final de las líneas reiniciadas.
Esta es la opción predeterminada. Añade un carácter de línea nueva al
final de cada línea reiniciada y conserva los saltos de línea tal
como aparecen en el documento.

No añadir nuevas líneas. Sólo se mantendrán los saltos de línea
creados por Intro.

Esta opción mantiene los saltos de línea insertados por el
reinicio automático.
Si se vuelve a abrir el documento, el texto se adaptará a la anchura
de la nueva ventana.

* **Sí** 

Guarda el archivo actual o muestra el cuadro de diálogo
Guardar como para guardar el documento
* **No** 

Sigue con la operación sin guardar el archivo.
* **Cancelar** 

Cancela la operación
* **Ayuda** 

Describe el cuadro de diálogo Guardar


También se muestra el diálogo Guardar si se selecciona un mandato del
menú que dé como resultado la pérdida de los cambios de edición
actuales.
# Consulte también







# Cuadro de diálogo Incluir un archivo del Editor de textos


* **Entrar el nombre de ruta o carpeta&sigspace;** 

Identifica el nombre de ruta de la carpeta actual.
* **Filtro** 

Un asterisco (*) muestra todos los archivos. Se pueden
entrar caracteres comodín para mostrar sólo los
archivos que coincidan con una extensión
determinada. Por ejemplo, *.doc lista sólo los
archivos con la extensión .doc.
* **Archivos** 

Lista los archivos que se encuentran en la
carpeta actual.
* **Carpetas** 

Lista las carpetas que se encuentran en la carpeta
actual.
Se puede insertar un archivo desde la carpeta actual,
desde una subcarpeta o desde una carpeta distinta.
* **Entrar nombre del archivo** 

Muestra el nombre de archivo
seleccionado en la lista Archivos.
Pulse OK o bien Intro para incluir el archivo. También
puede escribir el nombre del archivo que desee
incluir.
* **OK** 

Inserta el archivo especificado en el campo Entrar nombre
del archivo en la posición del cursor.
* **Actualizar** 

Muestra una nueva lista de archivos después de
cambiar la tecla de filtro o de cambiar a una carpeta nueva.
* **Cancelar** 

Cancela la operación Incluir.
* **Ayuda** 

Describe los campos de entrada de texto, listas de
selección y botones del cuadro de diálogo.

# Consulte también







# Cuadro de diálogo Ortografía del Editor de textos




* **Palabras con errores ortográficos** 

Lista las palabras ortográficamente incorrectas
encontradas en el documento.
* **Cambiar A** 

Campo en el que se escribe la palabra ortográficamente correcta.
* **Buscar** 

Busca la primera aparición de la palabra
incorrecta. La búsqueda se efectúa desde la posición del cursor de inserción.
* **Cambiar** 

Sustituye la palabra resaltada por la ortográficamente correcta.
* **Cambiar Todo** 

Sustituye todas las apariciones de la palabra incorrecta.
* **Cerrar** 

Cierra el cuadro de diálogo.


&sigspace;

El mandato Verificar ortografía sólo está disponible para el idioma inglés.
# Consulte también







# Cuadro de diálogo Buscar/Cambiar del Editor de textos




* **Buscar** 

Campo en el que se escribe la palabra o frase que se desea
buscar. La opción Buscar es sensible a mayúsculas y minúsculas.
* **Cambiar A** 

Campo en el que se escribe el texto de sustitución.
* **Buscar** 

Busca la primera aparición de la palabra incorrecta.
* **Cambiar** 

Reemplaza la palabra resaltada por el texto de sustitución.
* **Cambiar Todo** 

Sustituye todas las apariciones de la cadena de búsqueda.
* **Cerrar** 

Cierra el cuadro de diálogo.

# Consulte también







# Cuadro de diálogo Valores de Formato del Editor de textos




&sigspace;

* **Margen izquierdo** 

El número de caracteres con el que se va a
indentar el texto impreso a partir del extremo izquierdo del papel
* **Margen derecho** 

El número de columnas para el texto
* **Alineación izquierda** 

Alinea las líneas de texto en el margen izquierdo
* **Alineación derecha** 

Alinea las líneas de texto en el margen derecho
* **Justificar** 

Alinea el texto en un estilo de bloque con los mismos
márgenes derecho e izquierdo
* **Centrar** 

Se centran las líneas de texto
* **Párrafo** 

Aplica la configuración al párrafo que contiene el cursor
* **Todo** 

Aplica la configuración a todo el documento
* **Cerrar** 

Cierra el cuadro de diálogo

# Consulte también







# Cuadro de diálogo Copiar en archivo del Editor de textos


Hay otras aplicaciones que utilizan el Editor de textos como
herramienta para editar información y que restringen el modo de
guardar los cambios de edición. Por ejemplo, en algunos casos, el
mandato Guardar como puede ser sustituido por Copiar en archivo, lo
que permite crear una copia de la información que se está
visualizando o editando sin tener que cambiar la sesión de edición al
archivo nuevo.

* **Entrar nombre de ruta o carpeta&sigspace;** 

Identifica el nombre
de ruta de la carpeta actual.
* **Filtro** 

Un asterisco (*) muestra todos los archivos. Se pueden
entrar caracteres comodín para mostrar sólo los
archivos que coincidan con una extensión
determinada. Por ejemplo, *.doc lista sólo los
archivos con la extensión .doc.
* **Archivos** 

Lista los archivos que se encuentran en la
carpeta actual.
* **Carpetas** 

Lista las carpetas que se encuentran en la carpeta actual.
* **Entrar nombre del archivo** 

Campo en el que se escribe un nombre
nuevo de archivo para el documento.
* **OK** 

Copia la información con el nombre de archivo proporcionado.
* **Actualizar** 

Muestra una nueva lista de archivos después de
cambiar la tecla de filtro o de cambiar a una carpeta nueva.
* **Cancelar** 

Cancela la operación Guardar como.


Si utiliza Reiniciar, aparecerán estas opciones adicionales en el
cuadro de diálogo:

Añadir caracteres de nueva línea al final de las líneas reiniciadas.
Esta es la opción predeterminada. Añade un carácter de línea nueva al
final de cada línea reiniciada y conserva los saltos de línea tal
como aparecen en el documento.

No añadir nuevas líneas. Sólo se mantendrán los saltos de línea
creados por Intro.

Esta opción mantiene los saltos de línea insertados por el reinicio
del texto.
Si se vuelve a abrir el documento, el texto se adaptará a la anchura
de la nueva ventana.
# Consulte también





# Cuadro de diálogo El archivo ya existe del Editor de textos


Cuando se guarda un documento, se visualizará este cuadro de diálogo
si se entra un nombre de archivo ya existente.

Para sobreescribir el archivo original, pulse OK.

Para entrar un nombre de archivo distinto, pulse Cancelar y
seleccione Guardar como en el menú Archivo.
# Consulte también





# Sintaxis y opciones de línea de mandatos del Editor de textos
Editor de textos: inicio en una ventana de terminal

La sintaxis de línea de mandatos para iniciar el Editor de textos es la
siguiente:dtpad [`opciones`]

siendo`opciones`uno o más de:

* **-server** 

Especifica que el servidor se iniciará en modalidad
de servidor sin que se muestre ninguna pantalla inicial. Las
invocaciones posteriores del Editor de textos que se ejecuten en
modalidad de petición predeterminada harán que el servidor cree una
ventana de edición aparte para cada petición.
* **-standAlone** 

Esta opción fuerza al Editor de
textos a ejecutarse en modalidad autónoma, en la que éste maneja su
propia información de edición independientemente del servidor del
Editor de textos. Esta opción puede ser útil si se desea trabajar con
el Editor de textos en un entorno distinto al de otras ventanas del
Editor de textos. Un ejemplo de ello lo constituiría la ejecución de
una sesión en un entorno de idioma distinto o con diferentes recursos
de colores. Esta opción equivale a configurar el recursostandAloneen True (verdadero).
* **-exitOnLastClose** 

Especifica la finalización
del proceso del servidor del Editor de textos cuando se cierre la
última ventana de edición en la pantalla. Esta opción sólo se
puede aplicar con la opción-server. Si no se
especifica esta opción, el proceso de servidor del Editor de textos
permanecerá activo indefinidamente, incluso si se han cerrado todas
las ventanas de edición.
* **-noBlocking** 

Especifica que la petición del
Editor de textos finalizará en cuanto el servidor del Editor de
textos indique que puede acceder a un archivo en la carpeta de
referencia. En esta opción, el proceso de petición del Editor de
textos quedará bloqueado, y sólo saldrá cuando reciba la notificación
del servidor del Editor de textos de que la ventana se ha cerrado.
* **-statusLine** 

Esta opción hace que el Editor
de textos muestre una línea de estado al final de la ventana de
edición. La línea de estado informa del número de línea
en la que se encuentra el cursor, el número total de líneas del
documento, los mensajes de la aplicación e indica si la modalidad
de Sustitución está activada.
Asimismo, proporciona un método para desplazar el cursor a un número
de línea específico.
* **-wrapToFit** 

Especifica que Reiniciar está
activado en el momento de arranque.

# Consulte también






Consulte la página mandtpad(1)para
obtener una lista y una descripción completa de las opciones de línea
de mandatos del Editor de textos.
# Recursos del Editor de textos
recursos de la aplicaciónrecursos

Los recursos que se enumeran a continuación controlan la aparición y
el comportamiento del Editor de textos.

Dtpad*server: [ true | false ]

Especifica que el Editor de textos debe iniciarse en
modalidad de servidor para procesar todas las peticiones de edición
posteriores para la pantalla. Si este recurso se establece en True
(verdadero), es equivalente a especificar la opción de línea de
mandatos-server.Dtpad*standAlone: [ true | false ]

Especifica si el proceso actual del Editor de textos debe
ejecutarse en modalidad autónoma, en la que maneja su propia
información de edición, o en modalidad de petición, en la que se
gestiona la edición mediante un único proceso por separado de servidor
del Editor de textos. Si este recurso se establece en True
(verdadero), es equivalente a especificar la opción de línea de
mandatos-standAlone.Dtpad*blocking: [ true | false ]

Especifica que, cuando el Editor de textos se ejecuta en la
modalidad de petición predeterminada, en la que se gestiona la
edición mediante un proceso aparte de servidor del Editor de textos,
el proceso de petición no saldrá hasta que se cierre la ventana
asociada a la petición de edición. Si este recurso se establece en False
(falso), es equivalente a especificar la opción de línea de
mandatos-noBlocking.Dtpad*exitOnLastClose: [ true | false ]

Especifica si el proceso de servidor del Editor de textos
debe terminar cuando se cierre la última ventana de edición activa.
Si este recurso se establece en False (falso), el servidor del Editor
de textos continuará ejecutándose, y estará preparado para recibir un
mensaje para editar un archivo. Si este recurso se establece en True
(verdadero), el servidor del Editor de textos finalizará cuando se
cierre la ventana de edición.

Dtpad*statusLine: [ true | false ]

Especifica si el Editor de textos mostrará la línea de
estado al final de la ventana de edición. Si este recurso se
establece en True (verdadero), es equivalente a especificar la opción de línea de
mandatos-statusLine.Dtpad*wrapToFit: [ true | false ]

Especifica si el Editor de textos tiene habilitada la opción Reiniciar
(True) o inhabilitada (False) al arrancar el editor. Si este recurso se
establece en True (verdadero), es equivalente a especificar la opción de línea de
mandatos-wrapToFit.
# Consulte también






Consulte la página mandtpad(1)para
obtener una lista y una descripción completa de los recursos del
Editor de textos.
# Catálogo de archivos del Editor de textos
Editor de textos, ejecutablevalores predeterminados de la aplicaciónrecursos

El archivo ejecutable del Editor de textos y el archivo de valores
predeterminados de la aplicación son:

* **Archivo ejecutable** 

/usr/dt/bin/dtpad
* **Archivo de valores predeterminados de la aplicación** 

/usr/dt/app-defaults/`lang`/Dtpad

# Consulte también




