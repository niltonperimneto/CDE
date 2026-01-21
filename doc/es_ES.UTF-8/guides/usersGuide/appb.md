Ejecuci&oacute;n de sesiones localizadasPuede personalizar en muchos idiomas diferentes la interfaz de usuario
del escritorio. Se pueden modificar diversos elementos tales como pantallas,
idiomas predeterminados, fonts, m&eacute;todos de entrada (teclado) e iconos.
Adem&aacute;s, se pueden adaptar al entorno local los men&uacute;s, la ayuda
en l&iacute;nea y los mensajes de error y est&aacute;n disponibles en diversos
idiomas.espec&iacute;fico de idiomasesionessesi&oacute;nespec&iacute;fica de idiomaIdioma predeterminado en el inicio de sesi&oacute;nLos mensajes y men&uacute;s de la ventana inicial de inicio de sesi&oacute;n
se muestran en el idioma predeterminado. Si no se ha establecido el idioma
predeterminado, los mensajes y men&uacute;s se muestran utilizando un entorno
nacional &ldquo;C&rdquo; gen&eacute;rico. Puede cambiar el idioma en el men&uacute;
Opciones de la pantalla de inicio de sesi&oacute;n.Inicio de una sesi&oacute;n con idioma espec&iacute;ficoespec&iacute;fico de idiomainicio
de sesi&oacute;ninicio de sesi&oacute;nespec&iacute;fico de idiomainicio de sesi&oacute;n, espec&iacute;fico de idiomaEs f&aacute;cil iniciar una sesi&oacute;n en un idioma determinado utilizando
el escritorio. Sin embargo, pueden ser necesarios determinados requisitos
de hardware, tales como teclados e impresoras, para hacer m&aacute;s utilizable
la sesi&oacute;n adaptada al entorno nacional. Estos requisitos var&iacute;an
con el idioma, el juego de caracteres y el pa&iacute;s. El software y los
fonts pueden aumentar m&aacute;s la adaptaci&oacute;n efectiva del sistema
al entorno nacional. Para iniciar una sesi&oacute;n espec&iacute;fica de idioma:Utilice el men&uacute; Opciones de la pantalla de inicio de
sesi&oacute;n para seleccionar un idioma.La lista de idiomas incluye todos los idiomas a los que se da soporte.Inicie la sesi&oacute;n en la forma habitual, con su nombre
y contrase&ntilde;a.Creaci&oacute;n o edici&oacute;n de un archivo
espec&iacute;fico de idiomaespec&iacute;fico de idiomadatosPuede crear, editar e imprimir archivos espec&iacute;ficos de idioma.
Puede tambi&eacute;n dar nombres espec&iacute;ficos de idioma a los archivos.
Sin embargo, para archivos de la administraci&oacute;n del sistema que se
comparten en una red, los nombres de archivo s&oacute;lo deben contener caracteres
ASCII. Los diversos sistemas de la red pueden estar utilizando entornos nacionales
diferentes.Si uno inicia la sesi&oacute;n en el escritorio para un idioma determinado,
todas las aplicaciones se invocar&aacute;n utilizando ese idioma. Sin embargo,
puede todav&iacute;a invocar una aplicaci&oacute;n con otro idioma.Si desea crear un archivo con un idioma diferente, invoque el Editor
de textos especificando el idioma deseado.Creaci&oacute;n y edici&oacute;n de un archivo espec&iacute;fico para
un idiomacreaci&oacute;narchivo con caracteres
espec&iacute;ficos de un idiomaedici&oacute;narchivo con caracteres espec&iacute;ficos
de un idiomaarchivoscon caracteres espec&iacute;ficos de un idioma[archivosespec&iacute;ficos de un idiomacaracteres en un archivoPara crear o editar un archivo espec&iacute;fico de un idioma, podr&aacute;
activar el Editor de textos directamente con el idioma o configurar la variable
de entornoLANGantes de activar
dicho editor.Para activar el Editor de textos con el idioma directamente, invoque
el comandodtpadcon el idioma especificado en la opci&oacute;n-xnllanguage. Por ejemplo:/usr/dt/bin/dtpad -xnllanguagejapones_nombredellugarPara configurarLANGantes
de invocar el editor, efect&uacute;e las siguientes operaciones:En una ventana Terminal, configure la variable de entornoLANGcon el idioma deseado. Por ejemplo, para
configurar el lugar en japon&eacute;s, escriba:Terminalconfiguraci&oacute;n del idiomamediante la variable de entorno LANGLANG=japones_nombredellugardondejapones_nombredellugarespecifica
el juego de caracteres japon&eacute;s. Refi&eacute;rase a la plataforma espec&iacute;fica
para determinar el valor dejapones_nombre_del_lugar.En la misma ventana, invoque el Editor de textos (dtpad) bajo el idioma deseado escribiendo:Editor de textosactivaci&oacute;n con un idioma espec&iacute;ficoactivaci&oacute;nEditor de textos con un idioma espec&iacute;ficoespec&iacute;fico del idiomaEditor de textos/usr/dt/bin/dtpad &Podr&aacute; introducir caracteres japoneses si se han instalado archivos
espec&iacute;ficos para el lugar. Tambi&eacute;n podr&aacute; utilizar la
sesi&oacute;n del Editor de textos para modificar un archivo en japon&eacute;s
creado anteriormente.Veapara obtener un ejemplo de la especificaci&oacute;n de un juego de fonts.Utilizaci&oacute;n de un emulador de terminal espec&iacute;fico
para un idiomaEn el siguiente ejemplo se utilizadttermy se activa
un emulador de terminal japon&eacute;s. Se supone que el idioma predeterminado
no es japon&eacute;s, que se utiliza el shell Korn y que se han instalado
los archivos espec&iacute;ficos para el lugar.emulador de terminalespec&iacute;fico
para un idiomaespec&iacute;fico
para un idiomaemulador de terminalactivaci&oacute;nemulador de terminal
con un idioma espec&iacute;ficoEscriba lo siguiente en la l&iacute;nea de comando de una
ventana de terminal de shell Korn:LANG=japones_nombredellugardttermdondejapones_nombredellugarespecifica
el juego de caracteres japon&eacute;s. Refi&eacute;rase a su plataforma espec&iacute;fica
para determinar el valor dejapones_nombredellugar.Especificaci&oacute;n de fontsfontsinternacionalizaci&oacute;n&lt;$startrange>internacionalizaci&oacute;n y fontsGeneralmente el usuario cambia los fonts utilizando el Gestor de estilos,
el cual a su vez reinicia el Gestor de espacios de trabajo, restableciendo
los fonts del escritorio. Puede tambi&eacute;n personalizar fonts desde la
l&iacute;nea de mandatos o en archivos de recursos. En un entorno internacionalizado,
el usuario debe especificar fonts que sean independientes del c&oacute;digo
de caracteres. Esto es necesario debido a que la especificaci&oacute;n puede
ser utilizada en diversos entornos nacionales con c&oacute;digos de caracteres
diferentes al del font (charset). Por tanto,
todas las listas de fonts se deben especificar con un conjunto de fonts.Especificaci&oacute;n de fontsespecificaci&oacute;n de fontsUnaespecificaci&oacute;n de fontsincluida en una lista de fonts puede
ser un nombre LFD (Logical Function Description) de X o un alias del nombreXLFDXLFD. Por ejemplo, lo siguiente
son especificaciones v&aacute;lidas de fonts para un font de 14 puntos:-