Tareas de dttermPara iniciar dttermHay varias maneras de iniciar un emulador de terminaldttermdel &ProductName;:Desde el Panel frontalDesde el Gestor de archivosDesde un mandato en una ventana del
terminalDesde el Gestor de aplicacionesDesde Nueva en el menú desplegable
de Ventana dedttermPara iniciar dtterm desde el Panel frontaliniciar: emulador de terminalcerrar: emulador de terminalTerminal: botónbotón: TerminalEl control de terminal se encuentra en el subpanel de Aplicaciones
personales.Pulse el control del terminal. El indicador
luminoso de espera
parpadea para indicar que el terminal se está activando.La ventana dedttermaparece transcurridos unos segundos.Para iniciar dtterm desde el Gestor de archivosEn el menú Archivo, elija Abrir
terminal.De esta forma, se abredttermcon el
mismo directorio actual que
la vista del Gestor de archivos.Para iniciar un emulador de terminal que no sea dttermPara utilizar un emulador de terminal que no seadtterm,
inícielo desde una línea de mandatos en una ventana de emulador
ya existente.En el indicador de la línea
de mandatos, escriba el nombre del emulador
de terminal y también las opciones que desee. Por ejemplo, para iniciarxterm, especifique:xterm  [opciones] &opcionesrepresenta elementos opcionales para personalizar el emulador de terminal&especifica que el emulador de terminal se ejecuta en el fondo,
de modo que puede continuar trabajando en la
ventana original mientras el emulador de terminal se esté
ejecutando.El emulador de terminal se inicia en el espacio de trabajo actual, a
menos
que las opciones indiquen lo contrario.Para iniciar dtterm desde el menú VentanaEn el menú Ventana de una ventana
existente dedtterm, elija Nuevo.
Hecho esto, aparecerá una ventana dedttermduplicada.EjemplosEl mandato siguiente inicia una ventana dedttermen el espacio de
trabajoNotas del proyecto:dtterm -xrm '*workspaceList: "Notas del proyecto"' &El mandato siguiente inicia una ventana dedttermen una pantalla
del sistema denominado "lgmcd":dtterm -display lgmcd:0.1 &Consulte tambiénSi desea obtener detalles sobre las
opciones disponibles paradtterm,
consulte la página de manualdtterm (1X).Para cerrar dttermcerrar:dttermmenú de ventana: botónEscribaexiten la línea de mandatos y pulse Intro.O bienelija Salir
en el menú Ventana.O bienelija Cerrar
en el menú desplegable del gestor de ventanas (al
que puede acceder desde el botón situado en el ángulo superior
izquierdo de la trama del gestor de ventanas).Se recomienda seleccionar la opción Salir del
menú Ventana dedttermpara cerrardtterm.Para copiar y pegar textocortar: textopegar: textotexto: cortar y pegarPara copiar textoMediante el botón 1 del ratón,
arrastre el puntero por el texto que desee copiar. Así, el texto aparecerá
resaltado.Suelte el botón 1 del ratón
tras haber resaltado todo el texto que desee copiar.El texto no se elimina de la posición original.Para pegar textoSitúe el cursor donde desee
insertar el texto.Pulse el botón 2 del ratón.En la ubicación que haya indicado se copia la selección
actual. Repitiendo los pasos antes mencionados, se pueden pegar copias adicionales.Redimensionar la ventana dttermredimensionar ventanaSeleccione Tamaño de ventana
del menú Opciones.Seleccione uno de estos tres tamaños:80x24132x24PredeterminadoEn algunos casos, y en función del tamaño de la pantalla
y del font que esté utilizando, es posible que no pueda redimensionar
la ventanadtterma 132 columnas. Si ello
ocurre,dttermse redimensionará al
número máximo de columnas que permita la situación.También puede redimensionardttermutilizando Controles de Gestor de ventanas.Consulte tambiénRedimensionar el contenido de la ventanafunciónresizeAl cambiar el tamaño de una ventana de emulador de terminal,
puede que las aplicaciones que se ejecuten en la ventana no tengan conocimiento
de este cambio de tamaño. Para redimensionar la salida de la aplicación
se utiliza el siguiente procedimiento.En el indicador de línea de
mandatos, escriba:eval `resize`Consulte tambiénIniciar aplicaciones en una ventana dttermEntre el mandato para iniciar la aplicación
en el indicador de la
línea de mandatos.La sintaxis general para iniciar una aplicación es la siguiente:aplicación[opciones] &aplicaciónEl nombre de la aplicación.opcionesLista de información adicional que se traspasa a la
aplicación.&especifica que la aplicación se ejecuta como proceso de fondo,
así que se puede seguir usando la ventana del emulador de terminal
mientras la aplicación se esté ejecutando.EjemploPara iniciar un reloj digital desde la línea de mandatos:xclock -digital &Consulte tambiénPara buscar el mandato y las opciones
que hay que utilizar para una aplicación, consulte la página
de manual o la documentación apropiada.Para iniciar una sesión en un sistema remotoMediante rloginEl mandatorlogin, en una ventana
existente del emulador de terminal, sirve para conectarse a un host remoto.
Una vez que la ventana hace las veces de terminal del host remoto, ya se pueden
ejecutar las aplicaciones, redireccionando la pantalla a su sistema si así
lo desea.EjemploEl mandato siguiente conecta a un sistema llamado "there", ejecuta
el cliente "xload" y redirecciona la pantalla de nuevo al sistema original.
Supongamos que su sistema se denominahere.rlogin there
xload -display here:0Uso de remshEl mandatoremshactiva un shell en
el host remoto, ejecuta un cliente (frecuentemente activando un emulador de
terminal en dicho host) y redirige la visualización de vuelta al sistema
original, si se desea. Sintaxis:remshremoto-ncliente-displaysistema:ventana[.pantalla]donde:remotoEl nombre del host remotoclienteEl programa que se desea ejecutar en el sistema remotosistema:ventana[.pantalla]El host y la pantalla donde han de aparecer
los resultados.EjemploEl mandato siguiente ejecutaxloaden el sistema remoto denominadothere, y direcciona la salida al sistemahere.remsh there -n /usr/bin/X11/xload -display here:0.0 &El mandatoremshse utiliza con frecuencia
al personalizar un menú
para acceder a otros hosts.Para configurar dttermPara configurar dttermUn recurso es una variable cuyo valor afecta a algunos atributos dedtterm. Recursos son, por ejemplo, el color
de primer plano, el
color de fondo, la altura y la amplitud. Los recursos se encuentran
en una base de datos de recursos.
He aquí unos ejemplos de recursos dedtterm:Dtterm*saveLines:  4s
Dtterm*scrollBar: TrueLos archivos app-default de las aplicaciones del escritorio se
encuentran en el directorio/usr/dt/app-defaults/idioma.
El Gestor de sesiones carga los recursos al arrancar la sesión. Para
obtener información sobre la carga de los recursos en el
GESTOR_RECURSOS por parte del Gestor de sesiones, consulte el
apartado "Loading the Session Resources" de la publicaciónCDE Advanced User's & System Administrator's Guide.Definir recursos accesibles en el sistemañada los recursos al archivo/etc/dt/config/idioma/sys.resources.
(Es posible que tenga que crear el archivo).Por ejemplo, si en/etc/dt/config/C/sys.resourcesse especifica:AnApplication*resource: valorel recursoAnApplication*resourcequeda
definido en cada
propiedad del GESTOR_RECURSOS de usuario en el siguiente inicio de
sesión.Definir recursos personalesAñada los recursos al archivoDirectorioInicio/.Xdefaults.Efectúe una doble pulsación
sobre Recargar recursos en el grupo de
aplicaciones de Herramientas_Escritorio.Especificar barras de desplazamientoEspecifique un recursoscrollBarpara el
emulador de terminal.
Si el valor descrollBares True,dttermtendrá barras de desplazamiento.
Si el valor es False, no
las tendrá.Finalice la sesión y vuelva
a conectarse para la sesión actual.
(Para la sesión de inicio, defina la sesión de inicio, finalice
la
sesión y luego vuelva a iniciarla.)EjemplosPara definir barras de desplazamiento en todas las ventanas dedtterm:Dtterm*scrollBar:     TruePara definir recursos sólo en las ventanas dedttermdenominadas
"localTerminal":localTerminal*scrollBar:   TruePara definir caracteres de control de terminalEdite el recursottyModesutilizando uno de los métodos descritos enDefinición de recursos de
dtterm.La sintaxis de este recurso es:ttyModes:nombre^Cdondenombrees el control, yCel carácter. Por ejemplo,
el valor por omisión dettyModeses:ttyModes:       erase ^H intr ^C kill ^U start ^Q stop ^S swtch ^@Dado quedttermsóloemulaun terminal, podría ser que los
caracteres de control no ofrecieran el aspecto al que usted está
acostumbrado a ver en un terminal físico. ElttyModesrecurso
sirve para definir caracteres de control para el emulador de terminal.Por omisión, el Gestor de inicio de sesión define los
siguientes
caracteres de control.Nombre controlCarácter (Efecto)borrar^H (Retroceso borra caracteres)inter^C (Interrupción - cancela la operación actual y vuelve
a
mostrar el indicador de línea de mandatos)matar^U (Suprime la línea de entrada)iniciar^Q (Inicia la salida del subproceso)conmut^@ (Conmuta entre capas en una shell)El carácter "^" equivale a la
teclaCTRL, de tal forma que para interrumpir una operación
en curso hay que pulsarCTRLC. Al definirttyModes, utilice la tecla^en lugar de
la teclaCTRL.