はじめに共通デスクトップ環境 デスクトップ KornShell ユーザーズガイドは、KornShell(kshell)で、Motif
アプリケーションを作成するために必要な情報を提供しています。はじめに必要な基本的
な情報に加えて、複雑さの増したいくつかのスクリプト例を説明します。
このガイドを通して、dtkshという用語は、デスクトップのKornShell
を意味しています。対象読者このガイドは、Motifアプリケーションを早くかつ簡単に作成したいが、Cプログ ラミング
言語を使用する時間や知識がない、またはC言語を使用したくないプログラマの方を対象
としています。kshellのプログラミング、Motif、Xtイントリンシクス、Xlibをよく理解
している必要があります。C言語に関する理解も役立ちます。マニュアルの構成第1章「デスクトップKornShellの紹介」は、dtkshスクリプトでMotifアプリケーションを
記述し始めるときに必要な基本的な情報について説明ます。第2章「スクリプト例」は2つの簡単なdtkshスクリプトについて説明します。
1つめのスクリプトは、プリテン・ボード・ウィジェットにプッシュ・ボタンを作成しま
す。2つめのスクリプトは、1つめのスクリプトを拡張して、プッシュ・ボタンにコール
バックを追加します。第3章「上級トピック」は、dtkshスクリプトに関するより上級のトピックについて
説明します。第4章「複雑なスクリプト」第2章で説明したスクリプトに比べてかなり複雑なスクリプトについて
説明します。このスクリプトは、findコマンドに対するグラフィック・インタフェース
作成しています。付録A「dtkshコマンド」すべてのdtkshコマンドのリストを示します。付録B「dtksh簡易関数」は、他のマニュアルには記述されていないコマンドまたは
ドキュメントのマニュアル・ページで構成されています。付録C「script_findスクリプトのリスト」は、第4章で説明した複雑なスクリプトの完成したリストを示します。関連文書以下の文書に、kshellKのプログラミング、Motif、Xtイントリンシクス、Xlibの情報が記載されています。Desktop KornShell
Graphical Programming For the Common Desktop Environment Version 1.0,by J. Stephen Pendergrast, Jr., published by Addison-Wesley, Reading, MA
01867.The New KornShell Command and Programming
Language, by Morris I. Bolsky and David G. Korn, published by
Prentice-Hall, Englewood Cliffs, NJ 07632.KornShell Programming Tutorial,
by Barry Rosenberg, published by Addison-Wesley, Reading, MA 01867.Motif Programmer's Guide,
Open Software Foundation, 11 Cambridge Center, Cambridge, MA 02142, published
by Prentice-Hall, Englewood Cliffs, NJ 07632.Motif Programmer's Reference,
Open Software Foundation, 11 Cambridge Center, Cambridge, MA 02142, published
by Prentice-Hall, Englewood Cliffs, NJ 07632.Motif Reference Guide,
by Douglas A. Young, published by Prentice-Hall, Englewood Cliffs, NJ 07632.Mastering Motif Widgets(Second
Edition), by Donald L. McMinds, published by Addison-Wesley, Reading, MA
01867The X Window System Programming and Applications
with Xt Motif Edition, by Douglas A. Young, published by Prentice-Hall,
Englewood Cliffs, NJ 07632.The Definitive Guides to the X Window
System, Volume 1: Xlib Programming Manual, by Adrian Nye, published
by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 2: Xlib Reference Manual, edited by Adrian Nye,
published by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 3: X Window System User's Guide, by Valerie Quercia
and Tim O'Reilly, published by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 4: X Toolkit Intrinsics Programming Manual, by
Adrian Nye and Tim O'Reilly, published by O'Reilly and Associates, Sebastopol,
CA 95472.The Definitive Guides to the X Window
System, Volume 5: X Toolkit Intrinsics Reference Manual,edited
by Tim O'Reilly, published by O'Reilly and Associates, Sebastopol, CA 95472.The Definitive Guides to the X Window
System, Volume 6: Motif Programming Manual, by Dan Heller, published
by O'Reilly and Associates, Sebastopol, CA 95472.DocBook SGMLのマークアップの規則この本は、DocBook文書型定義(DTD)を使用したStructured Generalized Markup Language (SGML) で記述されています。
次の表で、様々な意味で使用されているDocBookのマークアップについて説明します。DocBook SGMLのマークアップマークアップ表示意味使用例AaBbCc123コマンド名lsコマンドを使用してすべてのファイルをリストします。AaBbCc123コマンド・オプション名ls&minus;aを使用してすべてのファイルをリストします。AaBbCc123コマンド行の位置フォルダ:
実際に使用する特定の名前または値に置換されます。ファイルを削除するには、rmfilenameを使用します。AaBbCc123ファイルおよびディレクトリの名前.loginファイルを編集します。AaBbCc123本のタイトル、新しい語句、または強調したい語句ユーザーズ・ガイドの第6章を
参照してください。
これらはクラスオプションとよばれています。
これはルートで行わなければいけません。[ ]アイコン、ボタン、メニューなどのラベル名[了解]ボタンシェルのプロンプト文字次の表は、この本で使用しているシェルのプロンプト文字を示しています。シェルのプロンプト文字