Conceptos del Gestor de archivosPara comprender mejor el Gestor de archivos, tal vez le interese leer
los siguientes temas.Sistema jerárquico de archivosSistema jerárquico de archivosarchivos: sistema, jerárquicodefinición: archivoSi no está familiarizado con los sistemas, la idea de un sistema
jerárquico de archivos también puede resultarle nueva. Este
tema describe los componentes básicos del sistema jerárquico
de archivos.¿Qué es un archivo?Unarchivoes un contenedor que mantiene
información. La mayoría de los archivos que utiliza contienen
información (datos) en algún formato determinado&emdash;un documento,
una hoja de cálculo, un diagrama. El formato es la manera determinada
en que se disponen los datos dentro del archivo. El formato de un archivo
se conoce como su tipo de datos.Cuando el Gestor de archivos es uno de sus modos de vista de icono,
puede identificar el tipo de datos de un archivo por el icono que se utiliza
para representar el archivo. Cada tipo de datos tiene un icono diferente.La mayoría de los programas de aplicación comprenden un
número limitado de tipos de datos. Por ejemplo, un editor de documentos
probablemente no puede leer un archivo de hoja de cálculo. El escritorio
le ayuda a reconocer diferentes tipos de archivos utilizando una base de datos
de "data types" tipo de datos. Un tipo de datos identifica los archivos de
un formato determinado y los asocia con las aplicaciones adecuadas. En la
mayor parte de los casos, cuando se efectúe una doble pulsación
en un archivo, el escritorio ejecutará de forma automática la
aplicación que comprenda el tipo de datos de dicho archivo.El tamaño máximo que se permite para un nombre de archivo
varía de un sistema a otro. Algunos sistemas operativos no permiten
nombres de archivo superiores a 14 caracteres. Si es necesario, consulte al
administrador del sistema.¿Qué es una carpeta?definición: carpetaUnacarpetaes un contenedor para archivos,
similar a una carpeta en un archivador. De hecho, el Gestor de archivos utiliza
un icono de carpeta para representar una carpeta. Una carpeta puede contener
otras carpetas&emdash; algunas veces se denominansubcarpetas.
Con carpetas y subcarpetas, puede crear múltiples capas de organización
que constituyen una jerarquía. (en otros contextos, se suele hacer
referencia a las carpetas como directorios.)Si ha dibujado una imagen de la jerarquía de carpetas con cada
subcarpeta bajo la carpeta que la contiene&emdash;su carpeta padre&emdash;y
ha dibujado una línea que vaya de cada carpeta a su carpeta padre,
la imagen será como un árbol boca abajo. Por tanto, a menudo
llamamos a la jerarquía de carpetas unárbolde carpetas.Dentro de cualquier carpeta individual, cada nombre de archivo debe
tener un nombre exclusivo. Sin embargo, los archivos de diferentes carpetas
pueden tener el mismo nombre.A medida que vaya pasando de una carpeta a otra, se hará referencia
a su ubicación actual como lacarpeta actual.¿Qué es una ruta?definición: rutaruta: definidaLa ubicación de un archivo suele especificarse listando
los nombres de las carpetas y subcarpetas que conducen al archivo&emdash;esta
lista se llama unaruta. (Consulte.)
La ruta de un archivo está visible en dos lugares del Gestor de archivos.
Primero se muestra en la ruta con iconos como una cadena de carpetas. En segundo
lugar, se muestra en forma de texto en la línea de ruta de texto por
encima del área de visualización.Rutas y nombres de rutaLa ruta para un objeto es una forma de especificar el lugar en que está
ubicado el objeto en el sistema de archivos. Existen tres formas de especificar
la ruta: ruta absoluta, ruta relativa y ruta completa.Rutas absolutasUna ruta es una rutaabsolutasi empieza
en lacarpeta raíz. La carpeta raíz es
la carpeta superior en el árbol de jerarquía de carpetas. Si
una ruta empieza con una barra inclinada (/),
es una ruta absoluta especificada desde la carpeta raíz. Por ejemplo,
la siguiente es una ruta absoluta para el archivocarta:/usr/dt/config/cartaRutas relativasUna ruta es una rutarelativasi describe
la ubicación de un archivo o carpeta tal como se relaciona con la carpeta
actual. Si se encuentra en una carpeta y desea moverse hacia abajo en el árbol
de carpetas, no es necesario que escriba el nombre de ruta completo. Sólo
es necesario que escriba la ruta que empieza con el nombre de la siguiente
carpeta de la ruta. Si una ruta no empieza con una barra inclinada, se trata
de una ruta relativa. Por ejemplo, si la carpeta actual es/usr/dt, y desea trasladarse a la carpeta ''/usr/dt/config/cartas'',
utilizará la siguiente ruta relativa:config/cartas.. (carpeta padre)Existen dos nombres de carpeta especiales que son útiles al especificar
rutas relativas. La carpeta.(a veces llamada
"punto") representa la carpeta actual. La carpeta..(a veces llamada "punto-punto") representa la carpetapadre&emdash;la
carpeta que queda un nivel por encima en la jerarquía de carpetas.
Por ejemplo, si su carpeta actual es/usr/dt/config/panels, la ruta relativa para el archivosys.dtwmrcpasa a ser:../sys.dtwmrcya que el archivo está en la carpeta/usr/dt/config, un nivel por encima de la carpeta actual.Vea tambiénPropiedad y seguridad del objetoHay tres grupos de usuarios que pueden acceder a objetos:propietario,grupoyotros.
El acceso se divide en tres funciones: permiso delectura,
permiso deescrituray permiso deejecución.¿Quién dispone de acceso?Las tres clases básicas de usuarios son:PropietarioGeneralmente la persona que ha creado el archivo.GrupoVarios usuarios que se han sido agrupados por el administrador del sistema.
Por ejemplo, los miembros de un departamento pueden pertenecer al mismo grupo.OtrosTodos los demás usuarios del sistema.¿Qué clase de acceso?Los permisos de acceso de un archivo especifican cómo pueden
acceder a dicho archivo el propietario, los miembros del grupo y otros usuarios.Permiso de lecturaPermite obtener acceso para copiar o ver el contenido del objeto.Permiso de escrituraPermite el acceso para cambiar el contenido del objeto o eliminar el
objeto.Permiso de ejecuciónPara un archivo, permite el acceso paraejecutarel archivo (para acciones, scripts y archivos ejecutables). Para una carpeta,
permite el acceso para ejecutar mandatos, scripts y acciones dentro de dicha
carpeta.Con el Gestor de archivos, puede examinar y cambiar los permisos de
acceso para archivos o carpetas. Consultey.EjemplosPara hacer que una carpeta sea privada:Cambie las propiedades de la carpeta,
otorgándose a usted mismo (el propietario) permiso de lectura, escritura
y ejecución, pero sin otorgar permisos para el grupo y los demás
usuarios. Esto significa que sólo usted y el usuario root pueden examinar
el contenido de la carpeta.Para hacer que un objeto que ha creado esté disponible para que
puedan utilizarlo todos los usuarios, y protegerlo para que nadie sobreescriba
en el mismo de forma inadvertida:Cambie las propiedades del archivo,
otorgando permiso de lectura y ejecución al propietario, al grupo y
a los demás usuarios. No otorgue a nadie permiso de escritura.Permisos predeterminadosEl administrador del sistema pueda alterar los permisos predeterminados
que se utilizan cuando crea un nuevo archivo o carpeta.
Para determinar cuáles son los valores predeterminados
actuales, cree un nuevo archivo o carpeta, y a continuación
seleccione Cambiar permisos desde el menú Seleccionado para
examinar los permisos predeterminados.Cómo hacer que los objetos sean más accesibles - Introducción &newline;
Objetos del espacio de trabajoobjetos del espacio de trabajoespacio de trabajo: objetosEl Gestor de archivos proporciona una forma de examinar todos los objetos
del sistema de archivos. Sin embargo, el objeto sólo está visible
cuando se examina la carpeta en la que se encuentra.Para hacer que un objeto sea más accesible, puede colocarlo directamente
en el fondo del espacio de trabajo. El espacio de trabajo es el área
o superficie en que aparecen las ventanas. (Vea.)
Cuando se coloca un objeto en este lugar, se denomina unobjeto
del espacio de trabajo.Colocar un objeto en el espacio de trabajo no altera el archivo o carpeta
original. De hecho, el icono que aparece en el escritorio en realidad sólo
es un método abreviado (enlace) para acceder al archivo o carpeta real.
Cualquier operación que se realiza en el objeto del espacio de trabajo
en realidad se realiza en el archivo o carpeta que representa.Objetos del espacio de trabajo que aparecen en un espacio de trabajoCuando coloca un objeto en el espacio de trabajo, sólo
aparece en el espacio de trabajo actual. Si desea que el objeto esté
en otros espacios de trabajo, debe conmutar a dichos espacios de trabajo y
colocar el objeto en los mismos.Uso de los objetos del espacio de trabajoLos objetos del espacio de trabajo se utilizan exactamente igual
que los objetos de las ventanas del Gestor de archivos o del Gestor de aplicaciones.
Para ejecutar una acción predeterminada de un objeto, efectúe
una doble pulsación sobre su icono en el escritorio.Cada objeto del espacio de trabajo también tiene un menú
emergente que contiene mandatos y acciones para el objeto. Para mostrar el
menú emergente para un objeto del espacio de trabajo utilizando el
ratón, señale el icono y a continuación pulse y mantenga
pulsado el botón 3 del ratón. Para mostrar el menú mediante
el teclado, pulse Alt+Tab hasta que se resalte el icono, y a continuación
pulse Despl+F10.Patrones de coincidencia para buscar archivoscaracteres comodín, coincidenciacoincidencia de caracteres comodínCuando especifica un nombre de archivo o carpeta, puede incluir caracteres
comodín como por ejemplo el asterisco (*)
y el interrogante (?). El*coincide con cualquier cadena de cero o más caracteres,
y?coincide con cualquier carácter
único.Ejemplosba*Coincide con todos los nombres que empiezan con la cadenababa?Coincide con todos los nombres de tres letras que empiezan con la cadenaba*.vfCoincide con todos los nombres que terminan con la extensión.vf*.???Coincide con todos los nombres que tienen una extensión de punto
de tres caracteresEl nombre y el contenido del archivo pueden especificarse utilizando
la misma sintaxis de expresión regular que permite el mandatofind. (Consulte la página manualfind (1)para obtener más información.)Uso del Gestor de archivos como examinador de iconosLos archivos cuyos nombres terminan en.pmo.bmcontienen dibujos de icono. Estos son
los iconos que el Gestor de archivos utiliza cuando crea iconos. De forma
predeterminada, debe abrir estos archivos para ver los dibujos que contienen.
Si habilita el examinador de icono, el Gestor de archivos hará que
el icono de cada archivo sea como el dibujo que está almacenado dentro
del archivo.Para averiguar cómo volver a configurar el Gestor de archivos
para el examinador de icono, consulte: