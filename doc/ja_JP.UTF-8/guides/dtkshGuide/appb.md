dtksh簡易関数簡易関数dtkshユーティリティは、簡易関数のファイルを含んでいます。この関数は、
シェル・プログラマにとって有益なシェル関数を含むシェル・スクリプトです。シェル
関数は、シェル・プログラマが頻繁に行わなければならない操作を実行します。これらには
ある種類のダイアログ(ヘルプ、エラー、警告など)を手早く作成する関数、いくつかの
ボタンを容易に作成する関数、フォーム・ウィジェットの子に対する制約リソースを容易
に作成する関数が含まれています。
シェル・スクリプトの作成者はこれらの簡易関数を必ず使用する必要はありません。
開発者がより短期間で、よりわかりやすいシェル・スクリプトを、より簡単に作成できる
ように提供されています。シェル・スクリプトがこれらの関数にアクセスする前に、簡易関数を含んでいるファイルを
取り込まなければなりません。簡易関数は、/usr/dt/scripts/DtFuncs.sh.ファイルに
あります。シェル・スクリプトにそれらを取り込みには、次のように記述します。. /usr/dt/lib/dtksh/DtFuncs.dtshDtkshAddButtonsDtkshAddButtonsDtkshAddButtonsはコンポジット・ウィジェット内に同じ種類のボタンを1つ以上
追加します。いくつかのボタンを、メニュー区画またはメニューバーに追加するために、
最も頻繁に使用されます。使用方法は次のとおりです。DtkshAddButtons parent widgetClass label1 callback1
                [label2 callback2 ...]
DtkshAddButtons [-w] parent widgetClass variable1 label1 callback1 \
                [variable2 label2 callback2 ...]-wオプションは、作成する各ボタンのウィジェット・ハンドルを簡易関数が返すように
指示します。ウィジェット・ハンドルは、指定した環境変数に返されます。widgetClassパラメータは、次のいずれかに設定できます。何も指定しなかった場合のデフォルトはXmPushButtonGadgetです。XmPushButtonXmPushButtonGadgetXmToggleButtonXmToggleButtonGadgetXmCascadeButtonXmCascadeButtonGadget次に例を示します。DtkshAddButtons $MENU XmPushButtonGadget Open do_Open Save do_Save
                Quit exit
DtkshAddButtons -w $MENU XmPushButtonGadget B1 Open do_Open B2 Save
                do_SaveDtkshSetReturnKeyControlsDtkshSetReturnKeyControlsDtkshSetReturnKeyControlsは、フォーム・ウィジェット内にテキスト・ウィジェットを
作成し、リターン・キーがフォーム・ウィジェット内のデフォルト・ボタンを動作せず、
フォーム・ウィジェット内の次のテキスト・ウィジェットにフォーカスが移動するように
します。これは、一連のテキスト・ウィジェットを含むウィンドウがあり、最後の
テキスト・ウィジェットにフォーカスがある間にユーザが改行キーを押すまで、デフォルト
が動作しないようにする場合に便利です。使用方法は次のとおりです。DtkshSetReturnKeyControls textWidget nextTextWidget formWidget
                          defaultButtontextWidgetパラメータは、改行キーが押されるとフォーカスが次の
テキスト・ウィジェット(nextTextWidgetパラメータで指定したもの)に移動するように
構成されたウィジェットを指定します。formWidgetパラメータは、デフォルト・ボタンを
含み、2つのテキスト・ウィジェットの親になるフォーム・ウィジェットを指定します。defaultButtonパラメータは、フォーム・ウィジェット内でデフォルト・ボタンとして扱われるコンポーネントを指定します。次に例を示します。DtkshSetReturnKeyControls $TEXT1 $TEXT2 $FORM $OK
DtkshSetReturnKeyControls $TEXT2 $TEXT3 $FORM $OKDtkshUnder、DtkshOver、DtkshRightOf、DtkshLeftOfDtkshUnderDtkshOverDtkshRightOfDtkshLeftOfこれらの簡易関数は、フォーム・ウィジェットの制約条件のクラスの指定を簡易化します。
コンポーネントを他のコンポーネントの端1つに接続する方法を提供します。
これはATTACH_WIDET制約条件を使用して実行されます。使用方法は次のとおりです。DtkshUnder widgetId [offset]
DtkshOver widgetId [offset]
DtkshRightOf widgetId [offset]
DtkshLeftOf widgetId [offset]widgetIdパラメータは、現在のコンポーネントを接続するコンポーネントを指定します。offset値はオプションで、指定されていない場合のデフォルト値は0です。次に例を示します。XtCreateManagedWidget BUTTON4 button4 XmPushButton $FORM \
        labelString:&ldquo;Exit&ldquo; \
        $(DtkshUnder $BUTTON2) \
        $(DtkshRightOf $BUTTON3)DtkshFloatRight、DtkshFloatLeft、DtkshFloatTop、DtkshFloatBottomDtkshFloatRightDtkshFloatLeftDtkshFloatTopDtkshFloatBottomこれらの簡易関数は、フォーム・ウィジェットの制約条件のクラスの指定を簡易化します。
フォーム・ウィジェット内の他のコンポーネントに依存せずに、コンポーネントを配置
する方法を提供します。フォーム・ウィジェットが伸縮しても、コンポーネントは
フォーム・ウィジェット内の相対位置を維持します。コンポーネントに対して他の
フォーム・ウィジェットの制約条件が指定されると、コンポーネントは伸縮します。
これはATTACH_POSITION制約条件を使用して実行されます。使用方法は次のとおりです。DtkshFloatRight [position]
DtkshFloatLeft [position]
DtkshFloatTop [position]
DtkshFloatBottom [position]オプションのpositionパラメータは、コンポーネントの指定した端を配置する
相対位置を指定します。position値はオプションで、
指定されていない場合のデフォルトは0です。次に例を示します。XtCreateManagedWidget BUTTON1 button1 XmPushButton $FORM \
        labelString:&ldquo;Ok&ldquo; \
        $(DtkshUnder $SEPARATOR) \
        $(DtkshFloatLeft 10) \
        $(DtkshFloatRight 40)DtkshAnchorRight、DtkshAnchorLeft、DtkshAnchorTop、DtkshAnchorBottomDtkshAnchorRightDtkshAnchorLeftDtkshAnchorTopDtkshAnchorBottomこれらの簡易関数は、フォーム・ウィジェットの制約条件のクラスの指定を簡易化します。
フォーム・ウィジェットを伸縮してもコンポーネントの位置が変更しないように、
フォーム・ウィジェットの1つの端にコンポーネントを接続する方法を提供します。
ただし、このコンポーネントに設定されている他のフォーム・ウィジェットの制約条件
によっては、サイズの伸縮がまだ行われる場合があります。
これは、ATTACH_FORM制約条件を使用して実行されます。使用方法を次に示します。DtkshAnchorRight [offset]
DtkshAnchorLeft [offset]
DtkshAnchorTop [offset]
DtkshAnchorBottom [offset]オプションのoffsetパラメータは、コンポーネントを配置するフォーム・ウィジェット
の端からの距離を指定します。offsetが指定されていない場合は0を使用します。次に例を示します。XtCreateManagedWidget BUTTON1 button1 XmPushButton $FORM \
        labelString:&ldquo;Ok&ldquo; \
        $(DtkshUnder $SEPARATOR) \
        $(DtkshAnchorLeft 10) \
        $(DtkshAnchorBottom 10)DtkshSpanWidthおよびDtkshSpanHeightDtkshSpanWidthDtkshSpanHeightこれらの簡易関数は、フォーム・ウィジェットの制約条件のクラスの指定を簡易化します。
フォーム・ウィジェットの高さまたは幅を最大に広げるように構成する方法を提供します。
これは、フォーム・ウィジェットに、2つのコンポーネントの端(DtSpanHeightの上下、DtSpanWidthの左右)を接続することによって実行します。、
通常、フォーム・ウィジェットがサイズ変更すると、コンポーネントも必ずサイズ変更します。ATTACH_FORM制約条件がすべてのアタッチメントで使用されます。使用方法は次のとおりです。DtkshSpanWidth [leftOffset rightOffset]
DtkshSpanHeight [topOffset bottomOffset]オプションのoffsetパラメータは、コンポーネントを配置するフォーム・ウィジェットの
端からの距離を指定します。 offsetを指定しなかった場合のデフォルトは0です。次に例を示します。XtCreateManagedWidget SEP sep XmSeparator $FORM \
                      $(DtkshSpanWidth 1 1)DtkshDisplayInformationDialog、DtkshDisplayQuestionDialog、DtDisplayWarningDialog、
DtkshDisplayWorkingDialog、DtkshDisplayErrorDialogDtkshDisplayInformationDialogDtkshDisplayQuestionDialogDtDisplayWarningDialogDtkshDisplayWorkingDialogDtkshDisplayErrorDialogこれらの簡易関数は、Motifフィードバック・ダイアログのそれぞれの単一インスタンスを
作成します。要求したダイアログ型のインスタンスが既に存在している場合は、それを
再利用します。ダイアログの親は、環境変数$TOPLEVELから取得
します。これは、シェル・スクリプトの呼び出しよって設定され、その後、変更されません。
要求されたダイアログのハンドルは、次の環境変数のいずれかに返されます。_DTKSH_ERROR_DIALOG_HANDLE_DTKSH_QUESTION_DIALOG_HANDLE_DTKSH_WORKING_DIALOG_HANDLE_DTKSH_WARNING_DIALOG_HANDLE_DTKSH_INFORMATION_DIALOG_HANDLEダイアログのボタンに独自のコールバックを接続した場合、ダイアログを使用中に
破棄しないでください。ダイアログの管理をやめないと、後で再使用されてしまいます。
ダイアログの破棄が必要な場合は、関連する環境変数をクリアして、簡易関数が
ダイアログを再使用しないようにしてください。使用方法は次のとおりです。DtkshDisplay<name>Dialog title message [okCallback closeCallback
                        helpCallback dialogStyle][OK]([了解])ボタンは常に管理されていて、デフォルト時にはダイアログを管理しません。
[Cancel]([取消し])および[Help]([ヘルプ])ボタンは、コールバックが提供されたときのみ管理されます。dialogStyleパラメータは、関連するブリテン・ボード・リソースがサポートする
標準のリソース設定のいずれかを受け入れます。次に例を示します。DtkshDisplayErrorDialog &ldquo;Read Error&ldquo; &ldquo;Unable to read the file&ldquo;
                 &ldquo;OkCallback&ldquo; \
                 &ldquo;CancelCallback&ldquo; &ldquo;&ldquo; DIALOG_PRIMARY_APPLICATION_MODALDtkshDisplayQuickHelpDialogおよびDtkshDisplayHelpDialogDtkshDisplayQuickHelpDialogDtkshDisplayHelpDialogこれらの簡易関数は、ヘルプ・ダイアログのそれぞれの単一インスタンスを作成します。
すでに、要求したヘルプ・ダイアログの型のインスタンスが存在している場合は、それを
再使用します。ダイアログの親は、$TOPLEVEL環境変数から取得
します。これは、シェル・スクリプトによって設定され、その後は変更されません。
要求したダイアログのハンドルは、次の環境変数のいずれかを返します。_DTKSH_HELP_DIALOG_HANDLE_DTKSH_QUICK_HELP_DIALOG_HANDLEダイアログの破棄が必要な場合は、関連する環境変数をクリアし、簡易関数がダイアログを
再使用しないようにしてください。使用方法は次のおとおりです。DtkshDisplay*HelpDialog title helpType helpInformation [locationId]パラメータの意味は、helpTypeパラメータに指定した値に依存します。
意味は次のとおりです。helpType=HELP_TYPE_TOPIChelpInformation= ヘルプ・ボリューム名locationId= ヘルプ・トピック位置IDhelpType=HELP_TYPE_STRINGhelpInformation= ヘルプ文字列locationId= <未使用>helpType=HELP_TYPE_DYNAMIC_STRINGhelpInformation= ヘルプ文字列locationId= <未使用>helpType=HELP_TYPE_MAN_PAGEhelpInformation= マニュアル・ページ名locationId= <未使用>helpType=HELP_TYPE_FILEhelpInformation= ヘルプ・ファイル名locationId= <未使用>Example:DtkshDisplayHelpDialog &ldquo;Help On Dtksh&ldquo; HELP_TYPE_FILE
                       &ldquo;helpFileName&ldquo;