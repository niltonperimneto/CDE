
# アプリケーション・ビルダの概念









# アプリケーション・ビルダのプロセスの概説
アプリケーション・ビルダのプロセスプロセス, インタフェースの作成

アプリケーション・ビルダのユーザ・インタフェースを作成するプロセスを、以下に説明します。

インタフェースの設計。ピクチャを描きます。

アプリケーション・ビルダのインタフェースの "スペック" を実行します。
インタフェースの設計の各部品として、アプリケーション・ビルダのどのオブジェクトを使用するかを決定します。

アプリケーション・ビルダを起動してください。

メイン・ウィンドウを、デスクトップ上にドラッグ＆ドロップしてください。
これは、仕上げるアプリケーションのすべての機能の開始点になります。

コントロール区画を、メイン・ウィンドウ上にドラッグ＆ドロップしてください。
コントロールは、コントロール区画上にドロップされます。

適切なコントロールを、コントロール区画上にドラッグ＆ドロップしてください。
アプリケーション・ビルダのグループ、配列、およびアタッチメント・ツールを使用して、コントロールを適切に配置してください。

他のメイン・ウィンドウおよびカスタム・ダイアログを、デスクトップ上にドラッグ＆ドロップして、インタフェースの他の部分を作成してください。

インタフェース内のオブジェクト間のプログラム接続を作成してください。
テスト・モードで、接続をテストしてください。

インタフェースを改善して、正常になるまでテストを継続してください。

アプリケーション・ビルダのコード・ジェネレータを実行して、コードを生成してください。
ユーザ・コードを追加して、アプリケーションの機能を完成させてください。

アプリケーションが完成するまで、コンパイル、実行、テストを反復してください。
# アプリケーション・ビルダのプロジェクトとモジュール
プロジェクトモジュール




# プロジェクトとは何か?


アプリケーション・ビルダを使用して、グラフィカル・ユーザ・インタフェースを作成する場合は、1 個以上のモジュールで構成されるプロジェクト上で作業します。
それ自体によって構築されたアプリケーション・ビルダは、30 個以上のモジュールで構成される単一のプロジェクトです。

プロジェクト・ファイルは、[ファイル] メニューで [新規プロジェクト] を選択した時に、あるいはアプリケーション・ビルダの新規セッションで、ウィンドウをデスクトップ上にドラッグ＆ドロップした時に、開始されます。

プロジェクト・ファイルは、[ファイル] メニューで [プロジェクトを保存] を選択した時に、あるいは [プロジェクト・オーガナイザ] の [プロジェクト] メニューで [保存] を選択した時に、保存されます。
プロジェクト・ファイルには、.bip(builder interface project の略) という接尾辞が付きます。
# 関連項目



# モジュールとは何か?


モジュールは、プロジェクトの論理ユニットです。アプリケーション・ビルダのそれぞれのウィンドウおよびダイアログは、アプリケーション・ビルダのプロジェクトのモジュールです。

プロジェクト内のすべてのモジュール・ファイルは、プロジェクトを保存する時に保存されます。[プロジェクト・オーガナイザ] の [モジュール] メニューで、[保存] を選択すると、特定のモジュールを明示的に保存できます。

保存したモジュール・ファイルには、.bil(builder interface language の略) という接尾辞が付きます。
# 関連項目



# 例: メッセージ・コードの書き方


で説明したようにメッセージを作成したら、いつ、どのようにそれを表示するかを決めなければなりません。
メッセージは通常、ロジックのある部分の実行後に表示されます。
たとえば、名前を受け入れるように設計されたテキスト・フィールドにユーザが数字を入力した場合に、数字は有効ではないことをユーザに通知するエラー・メッセージを固定表示したいことがあります。

Motif のメッセージ・ボックスは、モード付きかモードなし (ブロック化かブロック化なしに相当) の 2 通りの方法のどちらかで表示できます。
アプリケーション・ビルダのコード・ジェネレータ (dtcodegen) には、表示の 2 つのモードに対応する 2 個のルーチンがあります。
それらは、dtb_utils.cにあり、以下の名前です。

dtb_show_modal_message()

dtb_show_message()

特定のメッセージをモード付きで表示する場合は、dtb_show_ modal_message()を使用してください。
特定のメッセージをモードなしで表示する場合は、dtb_show_message()を使用してください。
# 関連項目





# モード付きメッセージのコーディング


上記で説明したように、メッセージをモード付きで表示する場合は、dtb_show_modal_message()を使用してください。
このルーチンは、ユーザがメッセージ・ボックス・ボタンを押したことを示す値を返しま
す。
値は、enumで、dtb_utils.hの中で定義されます。/*
	 * Returns answer value for modal MessageBox
	 */
	typedef enum {
	    DTB_ANSWER_NONE,
	    DTB_ANSWER_ACTION1,
	    DTB_ANSWER_ACTION2,
	    DTB_ANSWER_ACTION3,
	    DTB_ANSWER_CANCEL,
	    DTB_ANSWER_HELP
	} DTB_MODAL_ANSWER;

その後、たとえばスイッチ文を介して戻り値を試験し、コードの該当な部分を実行できます。

ここでは、メッセージをモード付きで表示する例を示します。fooという簡単なアプリケーションを作成するとします。
プロジェクト名はfoo.bipで、1 個のモジュールfoo.bilで構成されています。
モジュールfoo.bilには、メイン・ウィンドウと、コントロール区画と、 2 個のテキスト・フィールドがあります。
1 個はユーザが人の名を入力するためのもので、もう 1 個は姓を入力するためのものです。
ユーザが数字を入力した場合は、エラー・メッセージを固定表示して、数字は受け入れられないことをユーザに知らせ、1 組のオプションを表示します。
ユーザは、最初に戻ること、つまり入力したテキストを消去することができます。
あるいは、継続すること、つまり入力したテキストはそのまま残して、テキストの変更方法はユーザの裁量に任せることができます。

呼び出し関数接続は、両方のテキスト・フィールドに対して実行され、ユーザが何かを入力するたびに呼び出されます。
最初のテキスト・フィールドに対する関数は、入力された文字が数字であるかどうかをチェックします。
数字の場合は、エラー・メッセージをモード付きで固定表示します。void 
verify_first_nameCB(
    Widget widget,
    XtPointer clientData,
    XtPointer callData
)
{
    /*** DTB_USER_CODE_START vvv Add C variables and code below vvv ***/ 
    char                *text = (char *)NULL;
    int                 textlen = 0;
    DTB_MODAL_ANSWER    answer = DTB_ANSWER_NONE;
    DtbFooMainwindowInfo instance = (DtbFooMainwindowInfo) clientData;
 
    /*** DTB_USER_CODE_END   ^^^ Add C variables and code above ^^^ ***/

    /*** DTB_USER_CODE_START vvv Add C code below vvv ***/
  
    text = XmTextFieldGetString(widget); 
    if ((text != NULL) && (*text != NULL))
    {
        textlen = strlen(text);
        if (isdigit(text[textlen-1]))
        {
            dtb_foo_message_initialize(&amp;dtb_foo_message);
            answer = dtb_show_modal_message(instance->textfield,
                        &amp;dtb_foo_message, NULL, NULL, NULL);
            switch (answer)
            {
                case DTB_ANSWER_ACTION1:        /* Start Over */
                    XmTextFieldSetString(widget, "");
                    break;
 
                case DTB_ANSWER_ACTION2:	/* Continue */
                    break;
            }
        }
    }
 
    /*** DTB_USER_CODE_END   ^^^ Add C code above ^^^ ***/
}
# 関連項目



# モードなしのメッセージのコーディング


モードなしのメッセージを固定表示する場合は、dtb_show_message()を使用しなければなりません。
メッセージ・ボックス・ボタンに対するコールバックは、で説明するように、必ず [接続エディタ] を介して指定してください。
メッセージ・ボックス用に指定されるボタンは、[接続エディタ] のメッセージ・オブジェクトの [いつ] 項目として表示されます。

上記と同じ例を使用して、ユーザが数字を入力した時に、姓のテキスト・フィールドが、エラー・メッセージをモードなしで表示するようにします。
前述のように、まず、1 組の呼び出し関数接続を、メッセージ・ボックスの [やり直し] と [継続] というラベルが付いたボタンに対して実行する必要があります。
コードを生成する時に、これらのルーチンが正しいことを実行するようにコードを追加してください。
この場合、やり直しルーチンはテキスト・フィールドを消去し、継続ルーチンは何もしません。void 
verify_last_nameCB(
    Widget widget,
    XtPointer clientData,
    XtPointer callData
)
{
    /*** DTB_USER_CODE_START vvv Add C variables and code below vvv ***/ 
    char                *text = (char *)NULL;
    int                 textlen = 0;
    DtbFooMainwindowInfo instance = (DtbFooMainwindowInfo) clientData;
 
    /*** DTB_USER_CODE_END   ^^^ Add C variables and code above ^^^ ***/

    /*** DTB_USER_CODE_START vvv Add C code below vvv ***/
                          
    text = XmTextFieldGetString(widget); 
    if ((text != NULL) && (*text != NULL))
    {
        textlen = strlen(text);
        if (isdigit(text[textlen-1]))
        {
            dtb_foo_message_initialize(&amp;dtb_foo_message);
            dtb_show_message(instance->textfield,
                        &amp;dtb_foo_message, NULL, NULL);
        }
    }
 
    /*** DTB_USER_CODE_END   ^^^ Add C code above ^^^ ***/
}


void
start_overCB(
    Widget widget,
    XtPointer clientData,
    XtPointer callData
)
{
    /*** DTB_USER_CODE_START vvv Add C variables and code below vvv ***/
 
    DtbFooMainwindowInfo instance = (DtbFooMainwindowInfo) clientData;
 
    /*** DTB_USER_CODE_END   ^^^ Add C variables and code above ^^^ ***/

    /*** DTB_USER_CODE_START vvv Add C code below vvv ***/               
 
    XmTextFieldSetString(dtb_foo_mainwindow.textfield2, "");

    /*** DTB_USER_CODE_END   ^^^ Add C code above ^^^ ***/
}
 

void
continueCB(
    Widget widget,
    XtPointer clientData,
    XtPointer callData
)
{
    /*** DTB_USER_CODE_START vvv Add C variables and code below vvv ***/
    /*** DTB_USER_CODE_END   ^^^ Add C variables and code above ^^^ ***/

    /*** DTB_USER_CODE_START vvv Add C code below vvv ***/               
    /*** DTB_USER_CODE_END   ^^^ Add C code above ^^^ ***/
}

上記の 2 つのルーチンstart_overCB()とcontinueCB()は、2 個のボタンに対するコールバックとして、dtb_show_message()に対する呼び出しを介して追加されます。
ここに、dtb_utils.cからコールバックを追加するコードの部分を示します。/* Add Callbacks if necessary */
    if (mbr->action1_callback != (XtCallbackProc) NULL)
        XtAddCallback(msg_dlg, XmNokCallback, mbr->action1_callback, NULL);
    if (mbr->cancel_callback != (XtCallbackProc) NULL)
        XtAddCallback(msg_dlg, XmNcancelCallback, mbr->cancel_callback,
NULL);
    if (mbr->action2_callback != (XtCallbackProc) NULL)
    { 
        action_btn = dtb_MessageBoxGetActionButton(msg_dlg, DTB_ACTION2_BUTTON);
        if (action_btn != NULL)
            XtAddCallback(action_btn, XmNactivateCallback,
                          mbr->action2_callback, NULL);
    }  
    if (mbr->action3_callback != (XtCallbackProc) NULL)
    {
        action_btn = dtb_MessageBoxGetActionButton(msg_dlg, DTB_ACTION3_BUTTON);        if (action_btn != NULL)
            XtAddCallback(action_btn, XmNactivateCallback,
                          mbr->action3_callback, NULL);
    }

mbrストラクチャは、メッセージに必要なすべての情報を含んでいます。
メッセージ・オブジェクトがdtb_<modulename>_<messagename>_initialize()ルーチンを介して作成されると、[メッセージ・エディタ] で指定された値がストラクチャに入ります。
この例では、dtb_foo_message_initialize()です。
# 関連項目



# アタッチメントについて


アタッチメントおよびアタッチメントに基づくグループは、インタフェース内のオブジェクトの動的配置動作を確立します。
動的配置動作は、サイズ変更動作中にオブジェクトが一貫した関係を維持することを保証します。
アタッチメントは、国際化アプリケーションが、多くのロケールで良好に動作できるようにします。
デフォルトでは、すべての子オブジェクトは、その上端と左端が、親オブジェクトの上端と左端に接続されます。
したがって、メイン・ウィンドウ上にドロップされたコントロール区画は、その左端と上端がメイン・ウィンドウの左端と上端に接続されます。
同様に、コントロール区画にドロップされたボタンは、コントロール区画に接続されます。

親オブジェクトのサイズが、上方向または左方向に変更される場合、子オブジェクトは、上端と左端の距離を維持して親と共に移動します。

区画オブジェクトが、親の上または左の端にドロップされる場合は、その端に、ゼロ (0) オフセットで接続されます。
親の左と上の端から、右と下に一定の距離だけ離してドロップされた場合は、正のオフセットを持ちます。

区画オブジェクトのサイズが、右端および下端から変更される場合、それに従って親オブジェクトも伸縮して、親の右端および下端に接続されます。
# アタッチメントの例: カスタム・ダイアログ


アタッチメントの例として、アプリケーション・ビルダのカスタム・ダイアログ・オブジェクトを参照してください。
カスタム・ダイアログの下部にあるボタンは、それぞれが格納するダイアログ・パネルの上端と左右の端に接続されています。
これらは、パネルの上端から 5 ピクセルと、パネルの左端からのパーセントに応じて変わるように接続されています。
(ボタン 1 の左端は 10% で右端は 30%、ボタン 2 は 40% と 60%、ボタン 3 は 70% と 90%)

ボタン 1 の左端は常にパネルの端から 10% で、右端は常にパネルの端から 30% です。
したがって、ボタン 1 は常にパネルの幅全体の 20% 分の幅です。
ボタン 2 の端は、パネルの左端から 40% と 60% です。
ボタン 3 の端は、パネルの左端から 70% と 90% です。
この 3 個のボタンはパネルの伸縮に応じて伸縮し、相互の距離は常にパネルの幅全体の 10% です。