
# Referencia de Crear Acción

# Información general sobre Crear Acción





# Ventanas de Crear Acción









# Archivos creados por Crear Acción
Crear Acción: archivos creadosnombre de acción

La salida de Crear Acción es:

Una definición de acción que ejecuta un mandato. Se crea un archivo`CarpetaInicio`/.dt/types/`nombre_acción`.dten el que se
incluye la definición.

icono de acción

Unicono de acciónpara la acción. El icono de acción se coloca en
el directorio de inicio del usuario. El icono ejecuta el mandato de
acción cuando se efectúa una doble pulsación sobre el mismo.
De forma opcional, puede hacer que el icono de acción sea una zona
de soltar especificando tipos de datos que se puedan soltar.

Si utiliza las funciones avanzadas de Crear Acción, también puede crear:

Uno o más tipos de datos para los archivos de datos de la aplicación.

Acciones de Abrir e Imprimir para el tipo de datos

Tanto el tipo o tipos de datos como las acciones Abrir e Imprimir
también se graban en el archivo`CarpetaInicio`/.dt/types/`nombre_acción`.dt.
# Limitaciones de Crear Acción

# Limitaciones de Crear Acción
Crear Acción: limitaciones de acciones

No se puede utilizar Crear Acción para crear la acción de una
aplicación si:

La línea de mandatos requiere un parámetro que no sea de archivo.
Por ejemplo, no puede utilizar Crear Acción
para escribir una acción para el mandato:lp -d`dispositivo``nombre_archivo`

donde el usuario tiene que proporcionar un dispositivo cada vez que se
ejecuta el mandato.
(Puede crear una acción de forma manual para que haga esto.)

Desea que el usuario vea una etiqueta en el icono de
acción diferente al nombre de la acción.
Por ejemplo, no puede utilizar Crear Acción para proporcionar una
versión de una acción existente en el idioma del entorno.

La acción necesita algunas de las características avanzadas de la
base de datos de la acción, por ejemplo acciones que realicen lo
siguiente:

Ejecutar mandatos en sistemas remotos desde la definición de
acción.

Ejecutar otras acciones.

Debe ejecutarse como un usuario diferente (por ejemplo, como
superusuario).

Llevar a cabo una amplia utilización de la función de
"establecer correspondencia".

Mantener comportamientos muy diferentes, según el número de
argumentos de archivo que proporciona la acción.
# Limitaciones del tipo de datos
Crear Acción: limitaciones del tipo de datos

No se puede utilizar Crear Acción para crear el tipo
de datos de una aplicación si:

El tipo de datos ha de tener asociadas acciones adicionales que no
sean las de Abrir e Imprimir.

La acción Abrir para el tipo de datos no es el mandato de la acción.
Por ejemplo, no puede utilizar Crear Acción para crear el tipo de
datos que proporciona un icono exclusivo para el
directorio que representa el grupo de aplicaciones de la aplicación.
# Ventana principal Crear Acción
Crear Acción: ventana principal

Para obtener información sobre las tareas, consulte.

* **Nombre de Acción** 

Escriba el nombre de la acción. El nombre es sensible a las mayúsculas y minúsculas y
no puede incluir espacios en blanco.
* **Iconos de Acción** 

Muestra la imagen del icono que el icono
de acción utilizará.
* **Buscar Conjunto** 

Muestra el cuadro de diálogo Buscar Conjunto. Utilice
el cuadro de diálogo Buscar Conjunto para seleccionar un archivo de
mapa de pixels o de mapa de bits diferente que ya exista.
* **Editar Icono** 

Abre el Editor de iconos. Utilice el Editor de iconos
para crear un
icono nuevo o editar un icono existente.
* **Mandato Cuando Se Abre la Acción** 

Escriba el mandato.
Utilice la sintaxis$`n`para un argumento de archivo.
* **Texto de Ayuda para Icono de Acción** 

Escriba el texto de la ayuda
sobre el tema para el icono de acción.
* **Tipo de Ventana** 

Seleccione el tipo de ventana que creará la acción:

Gráfica: la aplicación muestra su propia ventana

Terminal (Cierre Automático): la acción muestra una ventana de
terminal que se cierra cuando finaliza la acción

Terminal (Cierre Manual): la acción muestra una ventana de terminal
que el usuario debe cerrar manualmente

Sin Salida: la aplicación no necesita ninguna ventana

# Funciones avanzadas


Utilice las funciones avanzadas sólo si el mandato del campo "Mandato
Cuando Se Abre la Acción (Se Pulsa Dos Veces)" incluye un argumento
de archivo.

* **Cuando Se Abra la Acción, Solicitar a los Usuarios** 

Escriba el
indicador de solicitud de archivo que se mostrará cuando se pulse dos
veces el icono de acción.
* **Tipos de Datos Que Usan Esta Acción** 

Una lista de los tipos de
datos que se han creado para esta acción.

Esta lista es de sólo lectura. Las entradas se añaden cuando se crean
los tipos de datos mediante el cuadro de diálogo Añadir Tipo de
Datos.
* **Añadir** 

Abre el cuadro de diálogo Añadir Tipo de Datos para crear un
nuevo tipo de datos.
* **Suprimir** 

Suprime el tipo de datos seleccionado de la lista "Tipos de
Datos Que Usan Esta Acción".
* **Editar** 

Edita los tipos de datos que se han seleccionado
en la lista "Tipos de Datos Que Usan Esta Acción".
* **Tipos de Datos que Se Pueden Soltar** 

Seleccione en función de si el icono de acción
aceptará cualquier archivo de tipo de datos, o sólo los archivos de
tipos de datos de la lista "Tipos de Datos Que Usan Esta Acción".

# Cuadro de diálogo Añadir Tipo de Datos
cuadro de diálogo Añadir Tipo de Datos

Para obtener información sobre las tareas, consulte.

* **Nombre de la Familia de Tipos de Datos** 

El nombre del tipo de datos. Se
proporciona un nombre automáticamente. Puede editar el campo de
texto. El nombre es sensible a las mayúsculas y minúsculas y
no puede incluir espacios.
* **Características de Identificación** 

Una lista de los criterios para
escribir el archivo.
Utilice Editar para establecer y modificar características.
* **Editar** 

Muestra el cuadro de diálogo Caracterísiticas de Identificación.
* **Texto de Ayuda para este Icono de Tipo de Datos** 

Escriba el texto de
la ayuda sobre el tema para archivos de este tipo de datos.
* **Iconos de Tipo de Datos** 

Muestra la imagen de icono que el tipo de
datos utilizará.
* **Buscar Conjunto** 

Muestra el cuadro de diálogo Buscar Conjunto. Utilice
el cuadro de diálogo Buscar Conjunto para seleccionar un archivo de
mapa de pixels o de mapa de bits diferente que ya exista.
* **Editar Icono** 

Abre el Editor de iconos. Utilice el Editor de iconos
para crear un
icono nuevo o editar un icono existente.
* **Mandato para Abrir este Tipo de Datos** 

Muestra el mandato que se
ejecuta cuando el usuario efectúa una doble pulsación sobre el tipo
de datos. Este es el mismo mandato que aparece en el campo "Mandato
Cuando Se Abre la Acción (Se Pulsa Dos Veces)".
* **Mandato para Imprimir este Tipo de Datos** 

Escribe la línea de
mandatos que proporciona la aplicación para imprimir el tipo de datos.
* **OK** 

Crea la información acerca del tipo de datos, añade el tipo de
datos a la lista "Tipos de Datos Que Usan esta
Acción" y cierra el cuadro de diálogo.
* **Aplicar** 

Crea la información acerca del tipo de datos y añade el tipo
de datos a la lista "Tipos de Datos Que Usan Esta Acción".
El cuadro de diálogo permanece abierto.
* **Cancelar** 

Cierra el cuadro de diálogo Añadir Tipo de Datos sin crear
ningún tipo de datos.
* **Ayuda** 

Muestra la ayuda en línea.

# Cuadro de diálogo Características de Identificación
cuadro de diálogo Características de Identificación

Para obtener información sobre las tareas, consulte.

* **Incluir Todo** 

Archivos: seleccione si el tipo de datos sólo se aplica a
archivos.

Carpetas: seleccione si el tipo de datos sólo se aplica a
carpetas.
* **Patrón de Nombre** 

Seleccione el cuadro de comprobación y escriba el
patrón de nombre. Puede utilizar
estos caracteres especiales:

*: coincide con cualquier secuencia de caracteres.

?: coincide con cualquier carácter único.
* **Patrón de Permiso** 

Seleccione el cuadro de comprobación y a
continuación elija los botones de selección pertinentes. Seleccione
Ambos si el permiso no es relevante.
* **Contenido** 

Seleccione la casilla de verificación y escriba los datos
que el archivo debe contener.
* **Tipo** 

Seleccione el tipo de datos. Utilice Serie para datos de texto
(ASCII).
* **Byte de Inicio** 

Escriba la ubicación del archivo donde empezar a
buscar los datos.
Utilice1para empezar desde el principio del archivo.
* **OK** 

Aplica las características a las Características de
Identificación en el cuadro de diálogo Añadir Tipo de Datos
y cierra el cuadro de diálogo Características de Identificación.
* **Cancelar** 

Cierra el cuadro de diálogo sin guardar los cambios.
* **Borrar** 

Borra los valores para el valor predeterminado.
* **Ayuda** 

Muestra la ayuda en línea.

# Cuadro de diálogo Buscar Conjunto
cuadro de diálogo Buscar Conjunto

El cuadro de diálogo Buscar Conjunto le permite especificar la imagen
del icono a utilizar para una acción o un tipo de datos. Para obtener
información acerca de las tareas, consulte.

* **Carpetas Icono** 

Lista las carpetas en la ruta de búsqueda del icono. Efectúe
una doble pulsación sobre una carpeta para ver los iconos que
contiene.
* **Archivos de Iconos** 

Lista los iconos en el directorio. Efectúe una
pulsación en un archivo de icono para seleccionarlo.
El nombre se visualiza en el campo de texto Entrar Nombre Archivo Iconos.
* **Entrar Nombre Archivo Iconos** 

Campo de texto para entrar elnombre basedel archivo de iconos. El contenido de este
campo cambia cuando se pulsa un elemento en la lista Archivos de Iconos.
* **OK** 

Convierte el icono especificado en el campo Entrar Nombre
Archivo Iconos en el icono designado y cierra el cuadro de diálogo
Buscar Conjunto.
* **Cancelar** 

Cierra el cuadro de diálogo sin cambiar el icono.
* **Ayuda** 

Muestra la ayuda en línea.

# Cuadro de diálogo Abrir, Crear Acción
Crear Acción: abrir un archivo de acciónacción: abriracción: editar

* **Entre la vía de acceso o el nombre de la carpeta&sigspace;** 

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

Lista las carpetas que se encuentran en la carpeta actual.
Se puede abrir un archivo de la carpeta actual, una
subcarpeta o una carpeta distinta.
* **Entre el nombre del archivo** 

Muestra el nombre de archivo
seleccionado en la lista Archivos.
Pulse Intro o bien OK para abrir el archivo. También
puede escribir el nombre del archivo que desee
abrir.
* **OK** 

Abre el archivo especificado en el campo Entre el
nombre del archivo.
* **Actualizar** 

Muestra una nueva lista de archivos después de
cambiar la tecla de filtro o de cambiar a una carpeta nueva.
* **Cancelar** 

Cancela la operación Abrir.
* **Ayuda** 

Describe los campos de entrada de texto, listas de
selección y botones del cuadro de diálogo.
