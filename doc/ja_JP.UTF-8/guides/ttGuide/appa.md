
# メッセージ・ツールキット


ToolTalkメッセージ・ツールキットは、 ToolTalkアプリケーション・プログラミン
グ・インタフェース（API）のハイレベルなインタフェースです。同じメッセージ・プロト
コルに準拠する他のアプリケーションとの最適な相互運用のための基本的なToolTalkのメッ
セージと機能をアプリケーションへ簡単に統合できるよう共通の定義と規約を提供します。

ToolTalkメッセージ・ツールキットのメッセージの大部分は標準ToolTalkメッセージ
・セットに含まれています。メッセージ・ツールキットの関数は別々にコーディングする
必要があるいくつかのタスクを透過的に処理します。たとえば、ttdt_file_jointtdt_file_join関数はパターンを登録し、指定された
配信範囲にある指定のファイルへのDeleted、Reverted、Moved、およびSaved通知を監視し
ます。コールバック・メッセージも呼び出します。
# ToolTalkメッセージ・ツールキットの一般的な説明


相互運用は、別々に開発されたアプリケーションを同時に実行する場合には重要な
テーマです。相互運用アプリケーションの開発者は、ツールキットのメッセージを一致させ
ています。つまり、プロトコルは小型の十分に定義されたインタフェースを形成します。こ
のインタフェースは、アプリケーションの自律性を最大限にします。

ToolTalkメッセージ・ツールキットは、アプリケーションの相互運用において重要で
あり、メッセージに対する完全なサポートを提供します。メッセージ・プロトコル仕様は、
メッセージの設定とアプリケーションがメッセージを受信したときの動作を含んでいます。
これらのメッセージは、アプリケーションの機能を利用するために既存のアプリケーション
に対して更新できます。共有している情報を送信、受信、および使用するために既存の
アプリケーションにこれらのメッセージを簡単に追加することができます。

ToolTalkメッセージ規約に準拠しているツールは、意味が異なる場合同じToolTalk
構文を使用しません。同じ意味の場合は異なるToolTalk構文を使用するのでツール間の通信
に失敗することもありません。プロトコルが監視されている場合、互いに影響を及ぼすこと
なく連携するアプリケーションを変更したり置き換えることもできます。

メッセージ・ツールキットのメッセージの大部分は標準ToolTalkメッセージ・セット
に含まれています。標準ToolTalkメッセージ・セットの詳細については、ToolTalk Reference Manualを参照してください。に、この章で説明する
関数の一覧を示します。これらの関数は、ToolTalkメッセージ・ツールキットの一部を構成
します。
# ToolTalkメッセージ・ツールキット関数


`関数`

`説明`

ttdt_close

ToolTalk通信終端を破棄します。

ttdt_file_event

ファイルに関するイベントを通知します

ttdt_file_join

ファイルに関するToolTalkイベントを監視できるように登録します。

ttdt_file_notice

ファイルに関する標準ToolTalk通知を作成して送信します。

ttdt_file_quit

ファイルに関するToolTalkイベントにおける配信対象を登録解除します。

ttdt_file_request

ファイルに関する標準ToolTalk要求を作成して送信します。

ttdt_Get_Modified

ファイルに変更内容を保留しているToolTalkクライアントがないかどうかを
問い合わせます。

ttdt_message_accept

ToolTalk要求の処理を引き受けます。

ttdt_open

ToolTalk通信終端を作成します。

ttdt_Revert

ファイルの内容を最後に保存した内容に戻すようToolTalkクライアントに
要求します。

ttdt_Save

ToolTalkクライアントがファイルを保存するよう要求します。

ttdt_sender_imprint_on

ツールに指定のToolTalkツールの動作や特性をエミュレートさせます。

ttdt_session_join

ToolTalkセッションに参加し、多くの標準デスクトップ・メッセージの
パターンとデフォルトのコールバックを登録します。

ttdt_session_quit

セッションに参加した時に登録したパターンとデフォルトのコールバックを
すべて登録解除し、ToolTalkセッションを終了します。

ttdt_subcontract_manage

未処理の要求を管理します。

ttmedia_Deposit

ドキュメントにチェックポイントを設定するためのDeposit要求を送信
します。

ttmedia_load

ドキュメントの表示、編集、作成のためのMedia Exchange要求を作成して
送信します。

ttmedia_load_reply

Display要求、Edit要求、Compose要求に返信します。

ttmedia_ptype_declare

Media Exchangeメディア・エディタのptypeを宣言します。

tttk_block_while

返信などの待機中にプログラムをブロックします。

tttk_message_abandon

メッセージを無視または拒否してから破棄します。

tttk_message_create

メッセージ規約に準拠しているメッセージを作成します。

tttk_message_fail

メッセージを無視します。

tttk_message_receive

次のToolTalkメッセージを取り出します。

tttk_message_reject

メッセージを拒否します。

tttk_op_string

オペレーションに対する文字列を返します。

tttk_string_op

文字列に対するオペレーションを返します。

tttk_Xt_input_handler

XtクライアントのためのToolTalkイベントを処理します。
# ツールキットの規約


ツールキットのメッセージ規約の大部分は、標準ToolTalkメッセージ・セットの記述
で構成されます。この節では、特定の標準メッセージ・セットには関係ない規約について
説明します。
# メッセージ・ツールキット規約


`フィールド`

`説明`

fileAttrib

メッセージのファイル属性を設定できるかどうか、または設定する必要があ
るかどうかを示します。ToolTalkサービスには、各メッセージがファイルを参照し、名前が
付いているファイルを「配信対象」としているクライアントにメッセージを配信
できる機能（「ファイル配信範囲指定機能」と呼ぶ）があります。

opName

オペレーションまたはイベントの名前（「op」ともいう）。重要
なのは、ツールが異なっても意味が同じものに対しては同じopNameを使用することです。
メッセージが標準のものではない場合、そのopNameは一意でなければいけません。たとえば
、opNameにCompany_Product（`Acme_HoarkTool_Hoark_My_Frammistat`など）という接頭辞を付けます。

requiredArgs

メッセージに必ず含まれていなければならない引き数。

optionalArgs

メッセージに含まれることもある特別引き数。メッセージ内のオプションの
引き数は、指定の順序で必須の引き数の後ろに指定しなければなりません。

vtype argumentName

特定の引き数についての記述。vtypeは、メッセージ引き数に含まれるデー
タの種類を表す文字列で、プログラマが定義します。ToolTalkサービスは、送信メッセージ
・インスタンスと登録されているメッセージ・パターンとを照合する場合にだけvtypeを
使用します。各vtypeは、規約により一般的な単一の既知のデータ型に対応づけなければな
りません。
# アプリケーション記述時のメッセージ・ツールキットの使用


ツールキットを使用するには、ToolTalkメッセージ・ツールキットのヘッダ・ファイルファイルToolTalkメッセージ・ツールキットのヘッダToolTalkメッセージ・ツールキットのヘッダ・ファイルを組み込みます。#include <Tt/tttk.h>
# ToolTalkメッセージ・ツールキット


この節では、ToolTalkメッセージ・ツールキットの一部である関数について説明しま
す。
# ttdt_closeメッセージ・セットツールキットttdt_closeツールキット・メッセージttdt_closettdt_close
Tt_status     ttdt_close(   const char *     procid,
                            const char *     new_procid,
                            int              sendStopped );

ttdt_close関数は、ToolTalk通信終端を破棄します。この関
数は、ToolTalk関数tt_closett_close関数ToolTalk関数tt_closeを呼び出します。

procidの値が!= 0の場合、この関数は次を
呼び出します。tt_default_procid_set( procid )tt_default_procid_set(  procid  )

new_procidの値が!= 0の場合、この関数
は次を呼び出します。tt_default_procid_set( new_procid )tt_default_procid_set(  new_procid )

sendStoppedパラメータが設定されている場合、この関数はStopped通知Stopped通知を送信します。

ttdt_close関数は、ToolTalk関数tt_default_procid_set関数ToolTalk関数tt_default_procid_settt_default_procid_setおよびtt_closeが返すどのようなエラーも可能性があります。Sending通知が失敗した場合、
エラーは伝達されません。
# ttdt_file_eventメッセージ・セットツールキットttdt_file_eventツールキット・メッセージttdt_file_eventttdt_file_event
Tt_status    ttdt_file_event( Tt_message     context,
                              Tttk_op        event,
                              Tt_pattern *   patterns,
                              int            send );

ttdt_file_event関数は、ToolTalkサービスを介してファイル
に関するイベントを通知します。この関数は、指定されたファイルに関係のあるイベントを
知らせるToolTalkメッセージを作成し、必要に応じて送信します。このファイルは、ttdt_file_joinメッセージ・セットツールキットttdt_file_joinツールキット・メッセージttdt_file_joinpatternsの作成時に、ttdt_file_join関数に渡されたパス名で示されます。

では、`event`パラメータの値に対応する通知内容を示します。
# eventパラメータの通知内容


`通知されるevent`

`通知内容`

TTDT_MODIFIEDTTDT_MODIFIED

ttdt_file_join関数に渡された配信範囲を登録し、Get_Modified要求Get_Modified要求、Save要求Save要求、Revert要求Revert要求を処理する配信対象ツールへイベントを通知します。

TTDT_SAVEDTTDT_SAVED,TTDT_REVERTEDTTDT_REVERTED

Get_Modified要求、Save要求、Revert要求のハンドラ・パターンを登録解除
します。

sendパラメータが設定された場合、配信範囲に
応じてSaved通知Saved通知かReverted通知Reverted通知を送信します。

sendパラメータを設定すると、配信範囲にModified通知Modified通知を送信します。

contextパラメータがゼロ以外の値のとき、このルーチンによっ
て作成されるメッセージはスロット名がENV_ENV_で始まるすべてのコンテキストを継承します。

では、この関数が返す
可能性のあるエラーの一覧を示します。
# ttdt_file_eventが返す可能性のあるエラー


`エラーの値`

`説明`

TT_DESKTOP_EINVALエラー・メッセージTT_DESKTOP_EINVALTT_DESKTOP_EINVAL

イベント通知が無効です。有効なイベント通知は、TTDT_MODIFIED、TTD_TSAVED、TTDT_REVERTEDです。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

patternsパラメータがNULLです。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクティ
ブ・メッセージの最大数(2000)に達しました。

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。
# ttdt_file_joinメッセージ・セットツールキットttdt_file_joinツールキット・メッセージttdt_file_jointtdt_file_join
Tt_message  ( *Ttdt_file_cb) (  Tt_message     msg,
                                Tttk_op        op,
                                char *         pathname,
                                void *         clientdata,
                                int            same_euid_egid,
                                int            same_procid );

Tt_pattern * ttdt_file_join(    const char *   pathname,
                                Tt_scope          the_scope,
                                int               join,
                                Ttdt_file_cb       cb,
                                void *             clientdata );

ttdt_file_join関数は、指定のファイルに関するToolTalk
イベントを監視できるように登録します。Deleted通知Deleted、Modified通知Modified、Reverted通知Reverted、Moved通知Moved、Saved通知Savedの５種類の通知を監視するように配信範囲に登録します。

コールバック・メッセージ引き数Ttdt_file_cbTtdt_file_cbは、に示すパラメータ
を取ります。
# Ttdt_file_cbが取るパラメータ


`パラメータ`

`説明`

message

送信中のメッセージ。

op

要求されているオペレーション。

pathname

メッセージに関連付けられたファイルのパス名。このコピーは、tt_freeToolTalk関数tt_freeToolTalk関数tt_freeで解放できます。

clientdata

メッセージに含まれているクライアント・データ

same_euid_egid

送信側を識別するフラグ。この値がtrueの場合は、送信側の信頼性は高いで
す。

same_procid

送信側を識別するフラグ。この値がtrueの場合は、送信側のprocidは受信側
と同じです。

the_scopeパラメータの値がゼロ（つまり、TT_SCOPE_NONETT_SCOPE_NONE）の場合、
ファイルの配信範囲はデフォルト（TT_BOTHTT_BOTH）に設定されます。しかし、たとえば
ToolTalkデータベース・サーバrpc.ttdbserverがpathnameを所有するファイル・サーバにインストールされていない
場合、ファイル配信範囲はTT_FILE_IN_SESSIONTT_FILE_IN_SESSIONに設定されます。

ttdt_file_joinはthe_scopeの値およ
びpathnameのコピーとをTt_patterns型の戻り
値に関連付けることで、ttdt_file_quit関数がパターンにアクセス
することを可能にします。呼び出し側は、ttdt_file_join呼び出し
が返ると、pathnameを変更または解放することができます。

joinパラメータの値がtrueの場合、この関数は次を呼び出します。tt_file_join( pathname )tt_file_join(   pathname   )

この関数は、NULLで終了するTt_pattern型の配列を返しま
す。この配列を破棄するには、ttdt_file_quitメッセージ・セットツールキットttdt_file_quitツールキット・メッセージttdt_file_quitttdt_file_quit関数を使用します。エラーが返される
場合、返された配列はtt_ptr_errortt_ptr_errorで解読できるエラー・ポインタです。は、ttdt_file_join関数が返す可能性のあるエラーの一覧です。
# ttdt_file_joinが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが
実行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがイン
ストールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_DBAVAILエラー・メッセージTT_ERR_DBAVAILTT_ERR_DBAVAIL

ToolTalkサービスが、このオペレーションに必要なToolTalkデータベースに
アクセスできませんでした。

TT_ERR_DBEXISTエラー・メッセージTT_ERR_DBEXISTTT_ERR_DBEXIST

ToolTalkサービスが、指定されたToolTalkデータベースを予期した場所で見
つけることができませんでした。

TT_ERR_PATHエラー・メッセージTT_ERR_PATHTT_ERR_PATH

ToolTalkサービスが、指定されたファイル・パス名でディレクトリを読み取
ることができませんでした。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。
# ttdt_file_noticeメッセージ・セットツールキットttdt_file_noticeツールキット・メッセージttdt_file_noticettdt_file_notice
Tt_message   ttdt_file_notice(     Tt_message     context,
                                   Tttk_op        op,
                                   Tt_scope       scope,
                                   const char *   pathname,
                                   int            send_and_destroy );

ttdt_file_notice関数は、ファイルに関する標準ToolTalk
通知を作成し、必要に応じて送信します。Created通知Created、Deleted通知Deleted、Moved通知Moved、Reverted通知Reverted、Saved通知Saved、Modified通知Modifiedの６種類の標準ファイル通知を作成するには、この関数を使用しま
す。

ttdt_file_eventメッセージ・セットツールキットttdt_file_eventツールキット・メッセージttdt_file_eventttdt_file_event関数は、ttdt_file_notice関数よりもハイレベルなインタフェースです。Moved通知以外のすべての通知を
送信する場合は、この関数を使用するようにしてください。

contextパラメータがゼロ以外の値のとき、このルーチンによっ
て作成されるメッセージは、スロット名がENV_で始まるすべての
コンテキストを継承します。

この関数は指定の`op`パラメータとscopeパラメータで通知を作成し、そのファイルの属性をpathnameパラメータに設定します。

send_and_destroyパラメータを設定すると、この関数はメッ
セージを送信してから破棄します。

send_and_destroyパラメータの値がfalseの場合、作成された
メッセージが返されます。send_and_destroyパラメータの値がtrue
の場合、ゼロが返されます。

エラーが発生すると、エラー・ポインタが返されます。Tt_statusを調べるには、tt_ptr_errorを使用します。では、この関数が返す
可能性のあるエラーについて説明します。
# ttdt_file_noticeが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクティ
ブ・メッセージの最大数(2000)に達しました。

TT_ERR_DBAVAILエラー・メッセージTT_ERR_DBAVAILTT_ERR_DBAVAIL

ToolTalkサービスが、このオペレーションに必要なToolTalkデータベースに
アクセスできませんでした。

TT_ERR_DBEXISTエラー・メッセージTT_ERR_DBEXISTTT_ERR_DBEXIST

ToolTalkサービスが、指定されたToolTalkデータベースを予期した場所で見
つけることができませんでした。

TT_DESKTOP_EINVALエラー・メッセージTT_DESKTOP_EINVALTT_DESKTOP_EINVAL

オペレーションが移動され、send_and_destroyパラメータの値がtrueです。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

パス名がNULLか、ToolTalkエラー・ポインタでした。
# ttdt_file_quitメッセージ・セットツールキットttdt_file_quitツールキット・メッセージttdt_file_quitttdt_file_quit
Tt_status    ttdt_file_quit( Tt_pattern *  patterns,
                             int           quit );

ttdt_file_quit関数は、ファイルのToolTalkイベントにおけ
る配信対象を登録解除します。この関数はパターンを破棄します。quitパラメータが設定されると、この関数は次を呼び出します。tt_file_quit( pathname )tt_file_quit( pathname )

ttdt_file_joinメッセージ・セットツールキットttdt_file_joinツールキット・メッセージttdt_file_joinpatternsの作成時にttdt_file_join関数に渡したパス名における配信対象を登録解除する場合にこの関数を使用し
ます。にこの関数が返す可能性
のあるエラーの一覧を示します。
# ttdt_file_quitが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_DBAVAILエラー・メッセージTT_ERR_DBAVAILTT_ERR_DBAVAIL

ToolTalkサービスが、このオペレーションに必要なToolTalkデータベースに
アクセスできませんでした。

TT_ERR_DBEXISTエラー・メッセージTT_ERR_DBEXISTTT_ERR_DBEXIST

ToolTalkサービスが、指定されたToolTalkデータベースを予期した場所で見
つけることができませんでした。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

パターンがNULLが、無効です。
# ttdt_file_requestメッセージ・セットツールキットttdt_file_requestツールキット・メッセージttdt_file_requestttdt_file_request
Tt_message        ttdt_file_request( Tt_message           context,
                                     Tttk_op              op,
                                     Tt_scope             scope,
                                     const char           pathname,
                                     Ttdt_file_cb         cb,
                                     void                 client_data,
                                     int                  send_and_destroy );

ttdt_file_request関数は、標準デスクトップのファイルに対
する配信範囲指定要求（Get_Modified、Save、Revertなど）を作成し、必要に応じて送信し
ます。

この関数は、ttdt_Get_Modified関数、tdt_Get_Modifiedttdt_Save関数、ttdt_Savettdt_Revertttdt_Revert関数よりもローレベルのインタフェースです。要求を作成してから送信し、そ
の応答に応じてブロックします。

ttdt_file_request関数は、指定の`op`と`scope`で要求を作成し、そのファイル
属性を`pathname`に設定します。デスクトップ・メッセ
ージ規約に従い、TT_INとvtypeFileのまだ
設定されていないTt_mode型引き数が要求に追加されます。指定のオペレーションがTTDT_GET_MODIFIEDTTDT_GET_MODIFIEDの場合は、TT_OUTとvtypeBooleanのまだ設定されていな
いTt_mode型引き数も要求に追加されます。

`context`以外の値のとき、このルーチンによって
作成される要求は、スロット名がENV_で始まるすべてのコンテキス
トを`context`から継承します。

この関数は、作成された要求のメッセージ・コールバックとしてcbをインストールし、クライアント・データが確実にコールバックに渡されるよ
うにします。sendがtrueの場合、この関数はハンドルを返す前に要
求を送信します。

この関数は正常終了時に、作成されたTt_messageを返します。エラーが発生すると、
エラー・ポインタが返されます。Tt_statusを調べるには、tt_ptr_errorを使用します。には、この関数が返す
可能性のあるエラーの一覧を示します。
# ttdt_file_requestが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクテ
ィブ・メッセージの最大数(2000)に達しました。

TT_ERR_DBAVAILエラー・メッセージTT_ERR_DBAVAILTT_ERR_DBAVAIL

ToolTalkサービスが、このオペレーションに必要なToolTalkデータベースに
アクセスできませんでした。

TT_ERR_DBEXISTエラー・メッセージTT_ERR_DBEXISTTT_ERR_DBEXIST

ToolTalkサービスが、指定されたToolTalkデータベースを予期した場所で見
つけることができませんでした。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

パス名がNULLか、無効です。
# ttdt_Get_Modifiedメッセージ・セットツールキットttdt_Get_Modifiedツールキット・メッセージttdt_Get_Modifiedttdt_Get_Modified
int          ttdt_Get_Modified(   Tt_message      context,
                                  const char *    pathname,
                                  Tt_scope        the_scope,
                                  XtAppContext    app2run,
                                  int             ms_timeout );

ttdt_Get_Modified関数は、ファイルに変更内容を保留してい
るToolTalkクライアントがないかどうか問い合わせます。この関数は、Get_Modified要求Get_Modified要求を送信して応答を待ちます。

`context`パラメータがゼロ以外の値のとき、この
ルーチンによって作成されるメッセージは、スロット名がENV_で
始まるすべてのコンテキストを継承します。

Get_Modified要求は、固定表示を行うつもりの`pathname`に変更内容を保留しているToolTalkクライアントがないかどうか問い合わせ
ます。

the_scopeパラメータは、Get_Modified要求が送信される配信
範囲を示します。このパラメータの値がゼロ（つまりTT_SCOPE_NONETT_SCOPE_NONE）の場合、ファイルの配信範囲は
デフォルト（TT_BOTHTT_BOTH）に設定されます。しかし、
たとえばToolTalkデータベース・サーバrpc.ttdbserverが、`pathname`を所有するファイル・サーバにインストール
されていない場合、ファイル配信範囲はTT_FILE_IN_SESSIONTT_FILE_IN_SESSIONに設定されます。

app2runパラメータとms_timeoutパラメータは、tttk_block_whileメッセージ・セットツールキットtttk_block_whileツールキット・メッセージtttk_block_whileこの関数が送信するGet_Modified要求への応答をブロックするために、tttk_block_while関数に渡されます。

Get_Modified要求が指定のタイムアウト時間内に肯定応答を受信すると、ttdt_Get_Modified関数はゼロ以外を返します。そうでない場合は、
ゼロを返します。この呼び出しはエラーを返しません。
# ttdt_message_acceptメッセージ・セットツールキットttdt_message_receiveツールキット・メッセージttdt_message_acceptttdt_message_accept
Tt_pattern *     ttdt_message_accept(     Tt_message         contract,
                                          Ttdt_contract_cb   cb,
                                          void *             clientdata,
                                          Widget             shell,
                                          int                accept,
                                          int                sendStatus );

ttdt_message_accept関数は、ToolTalk要求を処理することを
受け入れます。ツールは、要求の処理（つまり、無視したり拒否したりすること）を受け入
れる場合にこの関数を呼び出します。

Ttdt_contract_cb argumentTtdt_contract_cb引き数は、に示すパラメータを
取ります。
# Ttdt_contract_cb引き数が取るパラメータ


`パラメータ`

`説明`

Tt_message msg

通信状態にある要求。

クライアント・プログラムは、この要求を無視または拒否する、あるいはメッセージに応答
します。

Tttk_op op

着信中の要求のオペレーション

Widget shell

ttdt_message_accept関数に渡すシェル。

void *clientdata

ttdt_message_accept関数に渡すクライアント・
データ。

Tt_message contract

ttdt_message_accept関数に渡すコントラクト。

メッセージmsgを正常に処理すると、コールバックはゼロを
返します。そうでない場合は、Tt _messageに送られたtt_error_pointerを返します。

メッセージmsgを処理しない場合、コールバックはメッセージ
を返し、TT_CALLBACK_CONTINUEルーチンを呼び出しスタックに渡し
てメッセージを他のコールバックに提供するか、メッセージをtt_message_receive呼び出しに返します。

ttdt_message_accept関数は、ハンドラを宛先とする要求（を参照してください）
をデフォルト・セッションで登録します。
# ttdt_message_acceptが登録する要求


`要求`

`How Request Is Handled`

Get_Geometry要求Get_Geometr
y,Set_Geometry要求Set_Geometr
y

`shell`パラメータがNULLでない場合は、
これらの要求は透過的に処理されます。`shell`パラメー
タがNULLで、cbパラメータがNULLでない場合は、これらの要求はコ
ールバック・ルーチンに渡されます。それ以外の場合は、これらの要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_Iconified要求Get_Iconifi
ed,Set_Iconified要求Set_Iconifi
ed

`shell`パラメータがNULLでない場合は、
これらの要求は透過的に処理されます。`shell`パラメー
タがNULLで、cbパラメータがNULLでない場合は、これらの要求はコ
ールバック・ルーチンに渡されます。それ以外の場合は、これらの要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_Mapped要求Get_Mapped,Set_Mapped要求Set_Mapped

`shell`パラメータがNULLでない場合は、
これらの要求は透過的に処理されます。`shell`パラメー
タがNULLで、cbパラメータがNULLでない場合は、これらの要求はコ
ールバック・ルーチンに渡されます。それ以外の場合は、これらの要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Raise要求Raise

`shell`パラメータがNULLでない場合は、
これらの要求は透過的に処理されます。`shell`パラメー
タがNULLで、cbパラメータがNULLでない場合は、これらの要求はコ
ールバック・ルーチンに渡されます。それ以外の場合は、これらの要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Lower要求Lower

`shell`パラメータがNULLでない場合は、
これらの要求は透過的に処理されます。`shell`パラメー
タがNULLで、cbパラメータがNULLでない場合は、これらの要求はコ
ールバック・ルーチンに渡されます。それ以外の場合は、これらの要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_XInfo,Get_XInfo要求Set_XInfoSet_XInfo要求

`shell`パラメータがNULLでない場合は、
これらの要求は透過的に処理されます。`shell`パラメー
タがNULLで、cbパラメータがNULLでない場合は、これらの要求はコ
ールバック・ルーチンに渡されます。それ以外の場合は、これらの要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Pause要求Pause

cbパラメータがNULLでない場合、この要求はコールバ
ック・ルーチンに渡されます。それ以外の場合は、この要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Resume要求Resume

cbパラメータがNULLでない場合、この要求はコールバ
ック・ルーチンに渡されます。それ以外の場合は、この要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Quit要求Quit

cbパラメータがNULLでない場合、この要求はコールバ
ック・ルーチンに渡されます。それ以外の場合は、この要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_Status要求Get_Status

cbパラメータがNULLでない場合、この要求はコールバ
ック・ルーチンに渡されます。それ以外の場合は、この要求はエラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

contract引き数がTT_WRN_START_MESSAGETT_WRN_START_MESSAGEメッセージ・ステータスの時、
メッセージはツールを起動します。

起動したツールは、そのptypeにすでにディスパッチされているほかのメッセージを
受信するため、コントラクトを受け入れる前に使用したい配信範囲に参加する必要がありま
す。そうしないと、ツールは動作中にそのptypeの宣言を解除しなければなりません。ツー
ルがどの配信範囲にも参加していないと、ディスパッチされたメッセージによってptypeの
他のインスタンスが起動されます。

acceptがtureの場合、ttdt_message_accept関数は次を呼び出します。tt_message_accept( contract )tt_message_accept(   contract   )

sendStatus引き数がtrueの場合、ttdt_openメッセージ・セットツールキットttdt_openツールキット・メッセージttdt_openttdt_open関数に渡されたパラメータ（存在する場合）
を使用して、ttdt_message_accept関数は要求側にStatus通知を送信
します。

この関数は、NULLで終了するTt_pattern型の配列を返しま
す。この配列を破棄する場合は、tttk_patterns_destroyメッセージ・セットツールキットtttk_patterns_destroyツールキット・メッセージtttk_patterns_destroytttk_patterns_destroy関数を使用します。エラーが
返される場合、返された配列はtt_ptr_errortt_ptr_errorで解読されるエラー・ポインタに
なります。は、ttdt_message_accept関数が返す可能性のあるエラーの一覧です。
# ttdt_message_acceptが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

渡されたポインタが、このオペレーションに適した型のオブジェクトを指し
ていません。たとえば、文字列が必要なときにポインタは整数を指している場合などです。

TT_ERR_UNIMPエラー・メッセージTT_ERR_UNIMPTT_ERR_UNIMP

デフォルト・セッションのttsessionは、tt_message_accept関数をサポートしていないバージョン（1.0または1.0.1)です。

`注:`contract引き数のメッセージ・ステータスがTT_WRN_START_MESSAGEの場合、ツールのptypeに対して送信される
メッセージは、contractが拒否、応答、または無視されるまでブロックされます。
# ttdt_openメッセージ・セットツールキットttdt_openツールキット・メッセージttdt_openttdt_open
char *        ttdt_open(    int *          ttfd,
                            const char *   toolname,
                            const char *   vendor,
                            const char *   version,
                            int            sendStarted );

ttdt_open関数は、ToolTalk通信終端を作成します。この
関数は、tt_open関数ToolTalk関数tt_opentt_open関数とtt_fdtt_fd関数を呼び出します。ttdt_open関数は、toolname、vendor、`version`と作成された
procidとを関連付けます。新しいprocidのデフォルト・コンテキストをenviron(5)environ(5)から初期化します。sendStarted引き数が設定された場合は、Started通知Started通知を送信します。

ttdt_open関数は、tt_free関数ToolTalk関数tt_freett_free関数で解放できる文字列に、作成されたprocid
を返します。

この関数は、tt_open関数およびtt_fd関数で発生したどのようなエラーも返すことができます。Started通知が失敗した場合、
エラーは伝達されません。
# ttdt_Revertメッセージ・セットツールキットttdt_Revertツールキット・メッセージttdt_Revertttdt_Revert
Tt_status    ttdt_Revert( Tt_message       context,
                          const char *    pathname,
                          Tt_scope        the_scope,
                          XtAppContext    app2run,
                          int             ms_timeout );

ttdt_Revert関数は、ファイル内容を元に戻すようにToolTalk
クライアントに要求します。この関数は、Revert要求Revert要求をthe_scopeに送信し、応答を待ちます。
Revert要求は、pathnameに保留中の変更を破棄するように、処理中
であるToolTalkクライアントに求めます。

`context`パラメータがゼロ以外の値のとき、この
ルーチンによって作成されるメッセージは、スロット名ENV_で始ま
るすべてのコンテキストを継承します。

the_scopeパラメータの値がゼロ（つまりTT_SCOPE_NONETT_SCOPE_NONE）の場合、
ファイルの配信範囲はデフォルト（TT_BOTHTT_BOTH）に設定されます。しかし、たとえばToolTalk
データベース・サーバrpc.ttdbserverが`pathname`を
所有するファイル・サーバにインストールされていない場合、ファイル配信範囲はTT_FILE_IN_SESSIONTT_FILE_IN_SESSIONに設定されます。

app2runパラメータとms_timeoutパラ
メータは、tttk_block_whileメッセージ・セットツールキットtttk_block_whileツールキット・メッセージtttk_block_whileこの関数が送信するRevert要求への応答をブロックするために、tttk_block_while関数に渡されます。

要求が指定のタイムアウト時間内に肯定応答を受信すると、ttdt_Revert関数はTT_OKを返します。そうでない場合は、失敗した
応答に対するtt_message_statustt_message_statusの戻り値、またはに示すエラーのいずれ
かを返します。
# ttdt_Revertが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクティ
ブ・メッセージの最大数(2000)に達しました。

TT_ERR_DBAVAILエラー・メッセージTT_ERR_DBAVAILTT_ERR_DBAVAIL

ToolTalkサービスが、このオペレーションに必要なToolTalkデータベースに
アクセスできませんでした。

TT_ERR_DBEXISTエラー・メッセージTT_ERR_DBEXISTTT_ERR_DBEXIST

ToolTalkサービスが、指定されたToolTalkデータベースを予期した場所で見
つけることができませんでした。

TT_DESKTOP_ETIMEOUTエラー・メッセージTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEOUT

指定されたタイムアウト時間内に応答を受信しませんでした。

TT_DESKTOP_ETPROTOエラー・メッセージTT_DESKTOP_EPROTOTT_DESKTOP_EPROTO

要求が無視されました。しかしハンドラは、特定のエラー・ステータスの
代わりに失敗した応答に対するtt_message_statusの戻り値をTT_OKに設定します。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

パス名がNULLか、ToolTalkエラー・ポインタです。
# ttdt_Saveメッセージ・セットツールキットttdt_Saveツールキット・メッセージttdt_Savettdt_Save
Tt_status    ttdt_Save( Tt_message     context,
                        const char *   pathname,
                        Tt_scope    the_scope,
                        XtAppContext     app2run,
                        int            ms_timeout );

ttdt_Save関数は、ファイルを保存するようにToolTalkクライ
アントに要求します。この関数は、Save要求Save要求をthe_scopeに送信し、応答を待ちます。Save
要求は、pathnameに保留中の変更を破棄するように、処理中である
ToolTalkクライアントに求めます。

`context`パラメータの値がゼロ以外の値のとき、
このルーチンによって作成されるメッセージは、スロット名がENV_で始まるすべてのコンテキストを継承します。

the_scopeパラメータの値がゼロ（つまりTT_SCOPE_NONETT_SCOPE_NONE）の場合、
ファイル配信範囲はデフォルト（TT_BOTHTT_BOTH）に設定されます。しかし、たとえばToolTalk
データベース・サーバrpc.ttdbserverが`pathname`を所有するファイル・サーバにインストール
されていない場合、ファイル配信範囲はTT_FILE_IN_SESSIONTT_FILE_IN_SESSIONに設定されます。

app2runパラメータとms_timeoutパラ
メータは、この関数が送信するSave要求への応答をブロックするために、tttk_block_whileメッセージ・セットツールキットtttk_block_whileツールキット・メッセージtttk_block_whiletttk_block_while関数に渡されます。

要求が指定のタイムアウト時間内に肯定応答を受信すると、ttdt_Save関数は、TT_OKを返します。そうでない場合は、失敗し
た応答に対するtt_message_statustt_message_statusの戻り値、またはに示すエラーのいずれ
かを返します。
# ttdt_Saveが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクティ
ブ・メッセージの最大数(2000)に達しました。

TT_ERR_DBAVAILエラー・メッセージTT_ERR_DBAVAILTT_ERR_DBAVAIL

ToolTalkサービスが、このオペレーションに必要なToolTalkデータベースに
アクセスできませんでした。

TT_ERR_DBEXISTエラー・メッセージTT_ERR_DBEXISTTT_ERR_DBEXIST

ToolTalkサービスが、指定されたToolTalkデータベースを予期した場所で見
つけることができませんでした。

TT_DESKTOP_ETIMEOUTエラー・メッセージTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEOUT

指定されたタイムアウト時間内に応答を受信できませんでした。

TT_DESKTOP_ETPROTOエラー・メッセージTT_DESKTOP_EPROTOTT_DESKTOP_EPROTO

要求が無視されました。しかし、ハンドラは、特定のエラー・ステータスの
代わりに失敗した応答に対するtt_message_statusの戻り値をTT_OKに設定します。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

パス名がNULLか、ToolTalkエラー・ポインタです。
# ttdt_sender_imprint_onメッセージ・セットツールキットttdt_sender_imprint_onツールキット・メッセージttdt_sender_imprint_onttdt_sender_imprint_on
Tt_status    ttdt_sender_imprint_on(  const char *   handler,
                                      Tt_message     contract,
                                      char **        display,
                                      int *          width,
                                      int *          height,
                                      int *          xoffset,
                                      int *          yoffset,
                                      XtAppContext   app2run,
                                      int            ms_timeout );

ttdt_sender_imprint_on関数を呼び出すと、呼び出し側ツー
ル(以下、ツールBとします)で別のツール(以下ツールAとします)の動作と特定の特性が採
用されます。ツールBは、ツールAのX11ディスプレイ、ロケール、現在のワーキング・ディ
レクトリを採用します。更に、自身を正しい位置に置くためにツールAのX11ジオメトリを
取得します。

`display`パラメータがNULLの場合、$DISPLAY環境変数$DISPLAY環境変数$DISPLAYは、ツールAのディスプレイに設定
されます。`display`パラメータがNULLでない場合、
ツールAのディスプレイがこのパラメータに返されます。戻り値は、ToolTalk関数tt_freett_free 関数ToolTalkのtt_free関数で解放できる文字列です。

この関数は、Get_Geometry要求Get_Geometry要求をツールAに送信します。ツールAがジオメトリ・パラメータ
に対して値を返さない場合は、次のようになります。

`width`パラメータに対して値が返されない場合、-1を設定します。

`height`パラメータに対して値が返されない場
合、-1を設定します。

xoffsetパラメータに対して値が返されない場合、INT_MAXを設定します。

`yoffset`パラメータに対して値が返されない場合
、INT_MAXを設定します。

ttdt_sender_imprint_on関数の`width`パラメータ、`height`パラメータ、`xoffset`パラメータ、および`yoffset`パラメータすべてにNULLを設定すると、Get_Geometry要求Get_Geometry要求はツールAに送信されません。

app2runパラメータとms_timeoutパラ
メータは、この関数が送信するGet_Geometry要求への応答をブロックするために、tttk_block_whileメッセージ・セットツールキットtttk_block_whileツールキット・メッセージtttk_block_whiletttk_block_while関数に渡されます。

は、この関数が返す
可能性のあるエラーの一覧です。
# ttdt_sender_imprint_onが返す可能性のあるエラー


`エラーの値`

`説明`

TT_DESKTOP_ETIMEDOUTエラー・メッセージTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEDOUT

指定されたタイムアウト時間内に送信された要求の一部が完了しませんでし
た。

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクティ
ブ・メッセージの最大数(2000)に達しました。
# ttdt_session_joinメッセージ・セットツールキットttdt_session_joinツールキット・メッセージttdt_session_jointtdt_session_join
Tt_message   ( *Ttdt_contract_cb)       (  Tt_message      msg,
                                           void *          clientdata
                                           Tt_message       contract );

Tt_pattern * ttdt_session_join( const char *        sessid,
                                Ttdt_session_cb     cb,
                                Widget              shell,
                                void *              clientdata,
                                int                 join );

ttdt_session_join関数は、「デスクトップの良き市民
」としてToolTalkセッションに参加します。つまり、セッションsessidへの参加時に多くの標準デスクトップ・メッセージ・インタ
フェースのパターンとデフォルト・コールバックを登録します。は、この関数が現在
登録しているメッセージ・インタフェースの一覧です。
# ttdt_session_joinが登録している標準メッセージ


`要求`

`処理方法`

Get_Environment要求Get_Environment,Set_Environment要求Set_Environment

これらのメッセージは透過的に処理されます。

Get_Locale要求Get_Locale,Set_Locale要求Set_Locale

これらのメッセージは透過的に処理されます。

Get_Situation要求Get_Situation,Set_Situation要求Set_Situation

これらのメッセージは透過的に処理されます。

Signal要求Signal

このメッセージは透過的に処理されます。

Get_Sysinfo要求Get_Sysinfo

このメッセージは透過的に処理されます。

Get_Geometry要求Get_Geometry,Set_Geometry要求Set_Geometry

`shell`パラメータの値がNULLでなく、シ
ェルが実体化されたmappedWhenManagedapplicationShellWidgetの場合、このメッセージは透過的に処理さ
れます。シェルがmappedWhenManaged applicationShellWidgeでない
場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_Iconified要求Get_Iconified,Get_Iconified要求Get_Iconified

`shell`パラメータの値がNULLでなく、シ
ェルが実体化されたmappedWhenManagedapplicationShellWidgetの場合、このメッセージは透過的に処理さ
れます。シェルがmappedWhenManaged applicationShellWidgeでない
場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_Mapped要求Get_Mapped,Set_Mapped要求Set_Mapped

`shell`パラメータの値がNULLでなく、シ
ェルが実体化されたmappedWhenManagedapplicationShellWidgetの場合、このメッセージは透過的に処理さ
れます。シェルがmappedWhenManaged applicationShellWidgeでない
場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Raise要求Raise

`shell`パラメータの値がNULLでなく、シ
ェルが実体化されたmappedWhenManagedapplicationShellWidgetの場合、このメッセージは透過的に処理さ
れます。シェルがmappedWhenManaged applicationShellWidgeでない
場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Lower要求Lower

`shell`パラメータの値がNULLでなく、シ
ェルが実体化されたmappedWhenManagedapplicationShellWidgetの場合、このメッセージは透過的に処理さ
れます。シェルがmappedWhenManaged applicationShellWidgeでない
場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_XInfo要求Get_XInfo

`shell`パラメータの値がNULLでない場
合、このメッセージは透過的に処理されます。それ以外の場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Set_XInfo要求Set_XInfo

`shell`パラメータの値がNULLでなく、シ
ェルが実体化されたmappedWhenManagedapplicationShellWidgetの場合、このメッセージは透過的に処理さ
れます。シェルがmappedWhenManaged applicationShellWidgeでない
場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Pause要求Pause

cbパラメータがNULLでない場合、このメッセージはコ
ールバックに渡されます。cbパラメータがNULLの場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Resume要求Resume

cbパラメータがNULLでない場合、このメッセージはコ
ールバックに渡されます。cbパラメータがNULLの場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Quit要求Quit

cbパラメータがNULLでない場合、このメッセージはコ
ールバックに渡されます。cbパラメータがNULLの場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Get_Status要求Get_Status

cbパラメータがNULLでない場合、このメッセージはコ
ールバックに渡されます。cbパラメータがNULLの場合は、エラーTT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUPとともに異常終了します。

Do_Command要求Do_Command

cbパラメータがNULLでない場合、このメッセージはコ
ールバックに渡されます。cbパラメータがNULLの場合は、エラー`TT_DESKTOP_ENOTSUPエラー・メッセージTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP`とともに異常終了します。

sessidパラメータがNULLの場合、デフォルト・セッションに
参加します。

joinパラメータが設定されている場合、指定のセッションに
参加します。

ttdt_contract_cbメッセージ・セットツールキットttdt_contract_cbツールキット・メッセージttdt_contract_cbTtdt_contract_cbメッセージは、に示すパラメータを
取ります。コールバックはメッセージを処理しない場合は、メッセージを返します。メッセ
ージを処理する場合は、ゼロあるいはTt_messageに送られたエラー
・ポインタを返します。
# Ttdt_session_cbが取るパラメータ


`パラメータ`

`説明`

Tt_messagemsg

通信状態にある要求。

クライアント・プログラムは、この要求を無視または拒否する、あるいはメッセージに応答
します。

`注`:処理後にメッセージmsgを破棄してください。

void *clientdata

ttdt_session_joinまたはttdt_message_accept関数のどちらかに渡されるクライアント・デー
タ。

Tt_messagecontract

ttdt_message_accept関数に渡されるコントラクト。
コールバックがttdt_session_join関数によってインストールされる
と、contractパラメータの値は必ずゼロになります。

ttdt_session_join関数は、NULLで終了するTt_pattern型の配列を返します。この配列は、ttdt_session_quitメッセージ・セットツールキットttdt_session_quitツールキット・メッセージttdt_session_quitttdt_session_quit関数に渡され破棄することができま
す。エラーが発生すると、返された配列はエラー・ポインタになります。Tt_Statusを調べるには、tt_ptr_errortt_ptr_errorを使用します。は、返される可能性の
あるエラーの一覧です。
# ttdt_session_joinが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_SESSIONエラー・メッセージTT_ERR_SESSIONTT_ERR_SESSION

旧式、または無効なToolTalkセッションが指定されました。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

渡されたポインタが、このオペレーションに適した型のオブジェクトを指し
ていません。たとえば、文字列が必要なときにポインタは整数を指している場合などです。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。
# ttdt_session_quitメッセージ・セットツールキットttdt_session_quitツールキット・メッセージttdt_session_quitttdt_session_quit
Tt_status    ttdt_session_quit( const char *     sessid,
                                Tt_pattern *     sess_pats,
                                int              quit );

ttdt_session_quit関数は、「デスクトップの良き市民
」としてToolTalkセッションを終了します。つまり、セッションへの参加時に登録し
たすべてのパターンとデフォルト・コールバックを登録解除します。

この関数は、sess_patsで指定されたすべてのパターンを破棄
します。`quit`パラメータが設定されると、セッシ
ョンsessidを終了します。sessidパラメータ
がNULLの場合は、デフォルト・セッションを終了します。

は、この関数が返す
可能性のあるエラーの一覧です。
# ttdt_session_quitが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_SESSIONエラー・メッセージTT_ERR_SESSIONTT_ERR_SESSION

旧式、または無効なToolTalkセッションが指定されました。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

渡されたポインタが、このオペレーションに適した型のオブジェクトを指し
ていません。たとえば、文字列が必要なときにポインタは整数を指している場合などです。
# ttdt_subcontract_manageメッセージ・セットツールキットttdt_subcontract_manageツールキット・メッセージttdt_subcontract_managettdt_subcontract_manage
Tt_pattern * ttdt_subcontract_manage(  Tt_message         subcontract,
                                       Ttdt_contract_cb   cb,
                                       Widget             shell,
                                       void *             clientdata );

ttdt_subcontract_manage関数は、未処理の要求を管理します
。この関数が呼び出されると、要求側ツールは要求を処理しているツールとの間で行われる
デスクトップ上の標準的な対話を管理できます。この関数は、TT_HANDLERを宛先とするGet_Geometry要求Get_Geometry要求とGet_XInfo要求Get_XInfo要求、
およびStatus通知Status通知のための登録をデフォルト・セッションで行います。

`shell`パラメータがNULLの場合、要求または通知
がcbパラメータに渡されます。`shell`パラメータがNULでない場合は、要求は透過的に
処理されます。

ttdt_subcontract_manage関数は、NULLで終了するTt_pattern
型の配列を返します。この配列は、ttdt_session_quitメッセージ・セットツールキットttdt_session_quitツールキット・メッセージttdt_session_quitttdt_session_quit関数に渡して破棄することができま
す。エラーが発生すると、返された配列はエラー・ポインタになります。Tt_Statusを調べるには、tt_ptr_errortt_ptr_errorを使用します。は、返される可能性の
あるエラーの一覧です。
# ttdt_subcontract_manageが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

subcontractパラメータが有効なTt_messageではありません。

TT_ERR_EINVALエラー・メッセージTT_ERR_EINVALTT_ERR_EINVAL

`shell`パラメータとcbパラメータの両方がNULLです。
# ttmedia_Depositメッセージ・セットツールキットttmedia_Depositツールキット・メッセージttmedia_Depositttmedia_Deposit
Tt_status   ttmedia_Deposit(     Tt_message              load_contract,
                                 const char *            buffer_id,
                                 const char *            media_type,
                                 const unsigned char *   new_contents,
                                 int                     new_len,
                                 const char *            pathname,
                                 XtAppContext            app2run,
                                 int                     ms_timeout );

ttmedia_Deposit関数はDeposit要求を送信して、Edit、
Compose、OpenなどのMedia Exchange load_contract要求の変換対象であったドキュメント
にチェックポイントを設定します。

この関数はDeposit要求を作成して送信し、その要求が成功したか失敗したかを返し
ます。

load_contractは、このエディタがドキュメントを読み込むこ
とを要求します。

buffer_idは、ドキュメントがOpen要求により読み込まれた
場合に、このエディタが作成したバッファのIDです。

media_typeは、送信された要求のcontents引き数のvtypeです。

new_contentsとnew_lenは、contentsの引き数の値です。

要求が送信されると、app2runとms_timeoutは、応答を待つために、tttk_block_while関数に渡され
ます。
# ttmedia_Depositが返す可能性のあるエラー


TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクティ
ブ・メッセージの最大数(2000)に達しました。

TT_ERR_DBAVAILエラー・メッセージTT_ERR_DBAVAILTT_ERR_DBAVAIL

ToolTalkサービスが、このオペレーションに必要なToolTalkデータベースに
アクセスできませんでした。

TT_ERR_DBEXISTエラー・メッセージTT_ERR_DBEXISTTT_ERR_DBEXIST

ToolTalkサービスが、指定されたToolTalkデータベースを予期した場所で見
つけることができませんでした。

TT_DESKTOP_ETIMEOUTエラー・メッセージTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEOUT

指定されたタイムアウト時間内に応答を受信しませんでした。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

パス名がNULL、または、ToolTalkエラー・ポインタです。
# ttmedia_loadメッセージ・セットツールキットttmedia_loadツールキット・メッセージttmedia_loadttmedia_load
Tt_message   (*Ttmedia_load_msg_cb) (  Tt_message         msg,
                                       void *             clientdata,
                                       Tttk_op            op,
                                       unsigned char *    contents,
                                       int                len,
                                       char *             file  );

Tt_message   ttmedia_load( Tt_message             context,
                           Ttmedia_load_msg_cb    cb,
                           void *                 clientdata,
                           Tttk_op                op,
                           const char *           media_type,
                           const unsigned char *  contents,
                           int                    len,
                           const char *           file,
                           const char *           docname,
                           int                    send );

ttmedia_load関数は、ドキュメントの表示、編集、作成を行う
Media Exchange要求を作成し、必要に応じて送信します。この関数は、Display要求Display、Edit要求Edit、Compose要求Composeのいずれかの要求を作成して送信します。

要求のハンドラとの標準的な対話を管理するには、このメッセージで作成した要求の
送信後すぐにttdt_subcontract_manageメッセージ・セットツールキットttdt_subcontract_manageツールキット・メッセージttdt_subcontract_managettdt_subcontract_manage関数を使用してください。

`context`引き数の値がゼロ以外のとき、この
ルーチンによって作成されるメッセージは、スロット名がENV_ENV_で始まるすべてのコンテキストを継承します。

clientdata引き数は、応答を受信する場合、またはドキュメ
ントの中間バージョンがDeposit要求Deposit要求によってチェックポイントを設定される場合にcb引き数に渡されます。

`op`引き数は、TTME_DISPLAYTTME_DISPLAY、TTME_EDITTTME_EDIT、TTME_COMPOSETTME_COMPOSEのいずれかでなければなりません。

media_type引き数は、ドキュメントのデータ形式に名前を付
けます。通常、この引き数により、要求を処理するためにどのアプリケーションを選択する
か決定します。

contents引き数とlen引き数はドキュ
メントを指定します。これらの両方の引き数の値がゼロで、`file`引き数がゼロでない場合、ドキュメントは指定の
ファイルに格納されていると想定されます。

docname引き数がNULLでないとき、その値はドキュメントの
タイトルとして使用されます。

send引き数がtrueのとき、メッセージは返される前に送信さ
れます。

は、Ttmedia_load_msg_cb messageTtmedia_load_msg_cbメッセージが取るパラメータの
一覧です。
# Ttmedia_load_msg_cbがとるパラメータ


`パラメータ`

`説明`

Tt_message msg

要求に対する対応。または読み込み要求のDeposit要求tt_message_idt_message_idを指定するmessageID引き数を持ったDeposit要求。このパラメータの値が
Deposit要求である場合、クライアント・プログラムは要求に応答するか、無視しなければ
なりません。

`注`: メッセージmsgは処理後に破棄してください。

Tttk_op op

メッセージのオペレーション（TTME_DEPOSITTTME_DEPOSITあるいはttmedia_loadメッセージ・セットツールキットttmedia_loadツールキット・メッセージttmedia_loadttmedia_loadメッセージに渡されたオペレーション）
。

unsigned char *
contents

int len

char *file

到着中のドキュメントの内容。len引き数がゼロのと
き、ドキュメントは指定のファイルに格納されています。contents引き数または`file`引き数がNULLでない場合は、
ToolTalk関数tt_free 関数ToolTalk関数tt_freett_freeを使用してそれらを解放してください。

void *clientdata

ttmedia_loadメッセージ・セットツールキットttmedia_loadツールキット・メッセージttmedia_loadttmedia_loadメッセージに渡されたクライアント・
データ。

メッセージが正常に処理されると、コールバックはゼロを返します。処理中にエラー
が発生した場合は、コールバックはTt_messageに伝えられたエラー
・ポインタを返します。

コールバックがメッセージmsgを処理しない場合、コールバッ
クはメッセージを返します。このときツールキットは、TT_CALLBACK_CONTINUEルーチンを呼び出しスタックに渡してメッセージを他のコールバックに
提供するか、メッセージをtt_message_receive関数に返します。

終了時にttmedia_load関数は、作成した要求を返します。
エラーが発生した場合、この関数はエラー・ポインタを返します。Tt_Statusを調べるには、tt_ptr_errortt_ptr_errorを使用します。は、返される可能性の
あるエラーの一覧です。
# ttmedia_loadが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。

TT_ERR_OVERFLOWエラー・メッセージTT_ERR_OVERFLOWTT_ERR_OVERFLOW

ToolTalkサービスが受信したメッセージの数が、正しく処理可能なアクティ
ブ・メッセージの最大数(2000)に達しました。
# ttmedia_load_replyメッセージ・セットツールキットttmedia_load_replyツールキット・メッセージttmedia_load_replyttmedia_load_reply
Tt_message    ttmedia_load_reply   (   Tt_message             contract,
                                       const unsigned char *  new_contents,
                                       int                    new_len,
                                       int                    reply_and_destroy );

ドキュメントの表示、編集、作成を行うMedia Exchange要求に対して応答するには、ttmedia_load_reply関数を使用します。

new_contents引き数とnew_len引き数
の両方がゼロでない場合、それらの値はcontract引き数の適切な
出力引き数に、ドキュメントの新しい内容を設定するのに使用されます。reply_and_destroy引き数がtrueの場合、contract引き数に対して応答が行われ、その後メッセージは破棄されます。

は、返される可能性の
あるエラーの一覧です。
# ttmedia_load_replyが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NUMエラー・メッセージTT_ERR_NUMTT_ERR_NUM

TT_ERR_NOTHANDLERエラー・メッセージTT_ERR_NOTHANDLERTT_ERR_NOTHANDLER
# ttmedia_ptype_declareメッセージ・セットツールキットttmedia_ptype_declareツールキット・メッセージttmedia_ptype_declarettmedia_ptype_declare
Tt_message   (*Ttmedia_load_pat_cb) (  Tt_message         msg,
                                       void *             clientdata,
                                       Tttk_op            op,
                                       Tt_status          diagnosis,
                                       unsigned char *    contents,
                                       int                len,
                                       char *             file,
                                       char *             docname );

Tt_status    ttmedia_ptype_declare( const char *            ptype,
                                    int                     base_opnum,
                                    Ttmedia_load_pat_cb     cb,
                                    void *                  clientdata,
                                    int                     declare );

ttmedia_ptype_declare関数は、Media Exchangeメディア・
エディタのptypeを宣言します。この関数は、特定のメディア型用のMedia Exchange
メッセージ・インタフェースを実装するエディタを初期化します。

エディタがptypeでサポートされる種類のドキュメントを編集
するように要求される場合、この関数はcb引き数を呼び出します。

この関数は、ptypeに含まれていると想定される一連のシグニチャに、ツールキット
内部のオペレーション番号(opnum)コールバックをインストールしま
す。ツールキット内部のopnumコールバックは、これらのシグニチャのいずれかに一致する
要求を受信すると、clientdataをcb引き数
に渡します。opnumsは、base_opnumからはじまり、ゼロか1000の
倍数でなければなりません。

declare引き数がtrueのとき、この関数は次を呼び出します。tt_ptype_declare( ptype )tt_ptype_declare(   ptype  )

ptypeが複数の異なるメディア型を実現する場合、ttmedia_ptype_declare関数を２回以上呼び出すことができます。base_opnumには、各呼び出しで異なる値を指定しなければなりませ
ん。

ttmedia_ptype_declare関数は何回でも呼び出すことができま
すが、declare引き数を「true」に設定できるのは１回だ
けです。

は、Ttmedia_load_pat_cb messageTtmedia_load_pat_cbメッセージが取るパラメータ
の一覧です。
# Ttmedia_load_pat_cbメッセージが取るパラメータ


`パラメータ`

`説明`

Tt_message msg

送信された要求。クライアント・プログラムは、この要求を無視または拒否
する、あるいは要求に応答しなければなりません。

Tttk_op op

着信中のオペレーション（TTME_COMPOSETTME_COMPOSE、TTME_EDITTTME_EDIT、TTME_DISPLAYTTME_DISPLAYのいずれか）。

Tt_status diagnosis

ツールキットが要求を無視するように勧告するときのエラーコード
（たとえば、TT_DESKTOP_ENODATAエラー・メッセージTT_DESKTOPTT_DESKTOP_ENODATA）。diagnosisがTT_OKではなく、コールバック・ルーチンがメッセージmsgを返す場合、
ツールキットはメッセージmsgを無視し、破棄します。

unsigned char *
contents

int len

char *file

着信中のドキュメントの内容。len引き数がゼロのと
き、ドキュメントは指定のファイルに格納されています。contents引き数または`file`引き数がNULLでない場合は、
ToolTalk関数tt_free関数ToolTalk関数tt_freett_freeを使用してそれらの引き数を解放します。

char * docname

ドキュメントに名前が付けられている場合は、その名前。

void * clientdata

ttmedia_ptype_declareメッセージに渡す
クライアント・データ。

メッセージが正常に処理されると、コールバックはゼロを返します。処理中にエラー
が発生した場合は、コールバックはTt_messageに伝えられたエラー
・ポインタを返します。

コールバックがメッセージmsgを処理せず、diagnosis引き数の値がTT_OKでない場合、
コールバックはメッセージを返します。このとき、ツールキットはTT_CALLBACK_CONTINUEルーチンを呼び出しスタックに渡してメッセ
ージを他のコールバックに提供するか、メッセージをtt_message_receive呼び出しに返します。

エラーが発生すると、この関数はにあるエラーのいずれ
かを返します。
# ttmedia_ptype_declareが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_PTYPEエラー・メッセージTT_ERR_PTYPETT_ERR_PTYPE

ToolTalkサービスが、指定されたptypeを検出することができませんでした。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

渡されたポインタが、このオペレーションに適した型のオブジェクトを指し
ていません。たとえば、文字列が必要なときにポインタは整数を指している場合などです。
# tttk_block_whileメッセージ・セットツールキットtttk_block_whileツールキット・メッセージtttk_block_whiletttk_block_while
Tt_status    tttk_block_while( const int *  blocked,
                               int          ms_timeout );

tttk_block_while関数は、ms_timoutで
指定されたタイムアウト時間中、応答を待っているプログラムをブロックします。
# tttk_message_abandonメッセージ・セットツールキットtttk_message_abandonツールキット・メッセージtttk_message_abandontttk_message_abandon
Tt_status   tttk_message_abandon     ( Tt_message     msg );

tttk_message_abandon関数は、要求を放棄した後に破棄しま
す。

プログラムは、メッセージを理解できずに処分したいときはメッセージを放棄しなけ
ればなりません。

エラーが発生すると、この関数はにあるエラーのうちの
いずれかを返します。
# tttk_message_abandonが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインスト
ールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_POINTERエラー・メッセージTT_ERR_POINTERTT_ERR_POINTER

渡されたポインタが、このオペレーションに適した型のオブジェクトを指し
ていません。たとえば、文字列が必要なときにポインタは整数を指している場合などです。

TT_ERR_NOTHANDLERエラー・メッセージTT_ERR_NOTHANDLERTT_ERR_NOTHANDLER
# tttk_message_createメッセージ・セットツールキットtttk_message_createツールキット・メッセージtttk_message_createtttk_message_create
Tt_message   tttk_message_create( Tt_message             context,
                                  Tt_class               the_class,
                                  Tt_scope               the_scope,
                                  const char *           handler,
                                  const char *           op,
                                  Tt_message_callback    callback );

tttk_message_create関数は、規約に従ったメッセージを作成
します。この関数を使用すれば、継承されたコンテキストをあるメッセージから別のメッセ
ージに伝達するメッセージを簡単に作成できます。

tttk_message_create関数はメッセージを作成し、スロット名
がENV_で始まる`context`からすべてのコンテキスト・スロットを新しいメッセージ上にコピーします。作成された
メッセージには、the_classパラメータに指定されたTt_class値とthe_scopeパラメータに指定さ
れたTt_scope値が設定されます。

handlerパラメータがNULLの場合、メッセージにはTT_PROCEDURETT_PROCEDUREのTt_addressが設定されま
す。handlerパラメータがNULLでない場合は、メッセージはTT_HANDLERを経由してprocidに送信されます。

`op`引き数がNULLでない場合、その値にはメッセ
ージの`op`引き数が設定されます。

コールバック引き数がNULLでない場合、その値がメッセージ・コールバックとして
メッセージに追加されます。

正常終了時には、tttk_message_create関数は作成したTt_messageを返します。これは、他のTt_messageと同じ方法で変更、送信、破棄することができます。

エラーが発生すると、エラー・ポインタが返されます。Tt_statusを調べるには、tt_ptr_errortt_ptr_errorを使用します。は、返される可能性の
あるエラーの一覧です。
# tttk_message_createが返す可能性のあるエラー


`エラーの値`

`説明`

TT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMP

ttsessionプロセスを使用できません。ToolTalkサービスは、ttsessionが実
行されていない場合はその再起動を試みます。このエラーは、ToolTalkサービスがインス
トールされていないか正しくインストールされていないかのどちらかを示します。

TT_ERR_PROCIDエラー・メッセージTT_ERR_PROCIDTT_ERR_PROCID

指定されたプロセス識別子が旧式か、無効です。

TT_ERR_NOMEMエラー・メッセージTT_ERR_NOMEMTT_ERR_NOMEM

オペレーションを実行するのに十分なメモリがありません。
# tttk_message_destroyメッセージ・セットツールキットtttk_message_destroyツールキット・メッセージtttk_message_destroytttk_message_destroy
Tt_status   tttk_message_destroy ( Tt_message    msg );

tttk_message_destroy関数は、規約に従うすべてのメッセー
ジを破棄します。

このメッセージは、tt_message_destroy messagett_message_destroyメッセージの代わりに使用できます。

tttk_message_destroy関数は、ttdt_message_accept関数あるいは、ttdt_subcontract_manage関数によってメッセージに格納された
すべてのパターンを破棄し、その後メッセージmsgをtt_message_destroy関数に渡します。

この関数は、tt_message_destroy関数によって返された値を
返します。
# tttk_message_failメッセージ・セットツールキットtttk_message_failツールキット・メッセージtttk_message_failtttk_message_fail
Tt_status    tttk_message_fail( Tt_message     msg,
                                Tt_status      status,
                                const char *   status_string,
                                int            destroy );

tttk_message_fail関数は、メッセージmsgを無視した後で破棄します。

プログラムは、メッセージを理解できずに処分したいときはメッセージを破棄しなけ
ればなりません。

状態がTT_SENTであるメッセージは無視できます。メッセージ
が、ハンドラが宛先指定されているメッセージ、あるいはTT_WRN_START_MESSAGETT_WRN_START_MESSAGEのtt_message_statusを持つ場合、その
メッセージを無視できます。

この関数は、TT_DESKTOP_ENOTSUPを返します。
# tttk_message_receiveメッセージ・セットツールキットtttk_message_receiveツールキット・メッセージtttk_message_receivetttk_message_rejectメッセージ・セットツールキットtttk_message_rejectツールキット・メッセージtttk_message_rejecttttk_message_receive
Tt_status    tttk_message_receive( const char* procid );

tttk_message_receive関数は、tt_message_receivett_message_receive関数を呼び出し、次のToolTalk
メッセージを取り出します。

procidが0でない場合、この関数は次を呼び出し
ます。tt_default_procid_set( procid )
# tttk_message_rejectメッセージ・セットツールキットtttk_message_rejectツールキット・メッセージtttk_message_rejecttttk_message_rejectメッセージ・セットツールキットtttk_message_rejectツールキット・メッセージtttk_message_rejecttttk_message_reject
Tt_status    tttk_message_reject( Tt_message      msg,
                                  Tt_status       status,
                                  const char*     status_string,
                                  int             destroy );

tttk_message_reject関数は、メッセージmsgを拒否した後で破棄します。

プログラムは、メッセージを理解できずに処分したいときはメッセージを放棄しなけ
ればなりません。

状態がTT_SENTであるメッセージは拒否できます。
メッセージが、`ハンドラが宛先指定されているメッセージでない`、あるいはTT_WRN_START_MESSAGETT_WRN_START_MESSAGE以外のtt_message_statusを持つ場合、その
メッセージを拒否できます。

この関数は、TT_DESKTOP_ENOTSUPを返します。
# tttk_op_stringメッセージ・セットツールキットtttk_op_stringツールキット・メッセージtttk_op_stringtttk_op_string
char         *tttk_op_string(    Tttk_op     op);

tttk_op_string関数は、正常終了する場合はオペレーション`op`に対する文字列を返します。そうでない場合、この
関数はゼロを返します。

返された文字列を解除するには、tt_free関数を使用して
ください。Tttk_op   tttk_string_op(     const char *    opstring );

tttk_string_op関数は、指定の文字列に対するオペレーショ
ンを保持している文字列を返します。エラーが発生すると、この関数はTTDT_OP_NONEを返します。
# tttk_Xt_input_handlerメッセージ・セットツールキットtttk_Xt_input_handlerツールキット messagestttk_Xt_input_handlertttk_Xt_input_handler
void         tttk_Xt_input_handler(   XtPointer     procid,
                                      int *         source,
                                      XtInputId *   id );

tttk_Xt_input_handler関数は、Xtクライアントのための
ToolTalkイベントを処理します。一部のメッセージがコールバックで処理されないと予期
される場合を除き、この関数をXt入力ハンドラとして使用します。

この関数は、procid引き数をtttk_message_receive関数tttk_message_receive関数に渡し、すべての返されたメッセージ（つまり、コールバックで処理されない
メッセージ）をtttk_message_abandontttk_message_abandon関数に渡します。

この関数がエラーTT_ERR_NOMPエラー・メッセージTT_ERR_NOMPTT_ERR_NOMPを返すと、tttk_Xt_input_handler関数はidパラメータをXtRemoveInput functioXtRemoveInput関数に渡します。