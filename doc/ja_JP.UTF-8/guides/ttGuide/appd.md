
# 例

# Ttdt_contract_cbの例


は、アプリケーションの
ためのTtdt_contract_cbTtdt_contract_cbコールバックの典型的なアルゴリズムの例です。
このアプリケーションは、Pause要求、Resume要求、Quit要求を処理しますが、ツールキット
には、X11関連の要求しか処理させません。

この例にあるコールバックは、contractパラメータがゼロ
以外の値を持っている場合に処理を行います。したがって、ttdt_message_acceptに渡されるTtdt_contract_cbコールバックとして使用することもできます。
# Ttdt_contract_cbの典型的なアルゴリズム
Tt_message
myContractCB(
        Tt_message      msg,
        void           *clientdata,
        Tt_message      contract
)
{
        char *opString = tt_message_op( msg );
        Tttk_op op = tttk_string_op( opString );
        tt_free( opString );
        int silent = 0;
        int force  = 0;
        Boolean cancel = False;
        Boolean sensitive = True;
        char *status, command;
        switch (op) {
            case TTDT_QUIT:
                tt_message_arg_ival( msg, 0, &amp;silent );
                tt_message_arg_ival( msg, 1, &amp;force );
                if (contract == 0) {
                        /* Quit entire application */
                        cancel = ! myQuitWholeApp( silent, force );
                } else {
                        /* Quit just the specified request being worked on */
                        cancel = ! myCancelThisRequest(contract, silent, force);
                }
                if (cancel) {
                        /* User canceled Quit; fail the Quit request */
                        tttk_message_fail( msg, TT_DESKTOP_ECANCELED, 0, 1 );
                } else {
                        tt_message_reply( msg );
                        tttk_message_destroy( msg );
                }
                return 0;
            case TTDT_PAUSE:
                sensitive = False;
            case TTDT_RESUME:
                if (contract == 0) {
                        int already = 1;
                        if (XtIsSensitive( myTopShell) != sensitive) {
                                already = 0;
                                XtSetSensitive( myTopShell, sensitive );
                        }
                        if (already) {
                                tt_message_status_set(msg,TT_DESKTOP_EALREADY);
                        }
                } else {
                        if (XtIsSensitive( thisShell) == sensitive) {
                                tt_message_status_set(msg,TT_DESKTOP_EALREADY);
                        } else {
                                XtSetSensitive( thisShell, sensitive );
                        }
                }
                tt_message_reply( msg );
                tttk_message_destroy( msg );
                return 0;
            case TTDT_GET_STATUS:
                if (contract == 0) {
                        status = &ldquo;Message about status of entire app&ldquo;;
                } else {
                        status = &ldquo;Message about status of this request&ldquo;;
                }
                tt_message_arg_val_set( msg, 0, status );
                tt_message_reply( msg );
                tttk_message_destroy( msg );
                return 0;
            case TTDT_DO_COMMAND:
                if (! haveExtensionLanguage) {
                        tttk_message_fail( msg, TT_DESKTOP_ENOTSUP, 0, 1 );
                        return 0;
                }
                command = tt_message_arg_val( msg, 0 );
                result = myEval( command );
                tt_free( command );
                tt_message_status_set( msg, result );
                if (tt_is_err( result )) {
                        tttk_message_fail( msg, result, 0, 1 );
                } else {
                        tt_message_reply( msg );
                        tttk_message_destroy( msg );
                }
                return 0;
        }
        /* Unrecognized message; do not consume it */
        return msg;
}
# Ttdt_file_cbTtdt_file_cbの例


は、このコールバックの
典型的なアルゴリズムの例です。
# Ttdt_file_cbの典型的なアルゴリズム
Tt_message
myFileCB(
        Tt_message      msg,
        Tttk_op         op,
        char           *pathname,
        int             trust,
        int             isMe
)
{
        tt_free( pathname );
        Tt_status status = TT_OK;
        switch (op) {
            case TTDT_MODIFIED:
                if ((_modifiedByMe) &amp;&amp; (! isMe)) {
                        // Hmm, the other editor either does not know or
                        // does not care that we are already modifying the
                        // file, so the last saver will win.
                } else {
                        // Interrogate user if she ever modifies the buffer
                        _modifiedByOther = 1;
                        XtAddCallback( myTextWidget, XmNmodifyVerifyCallback,
                                       myTextModifyCB, 0 );
                }
                break;
            case TTDT_GET_MODIFIED:
                tt_message_arg_ival_set( msg, 1, _modifiedByMe );
                tt_message_reply( msg );
                break;
            case TTDT_SAVE:
                status = mySave( trust );
                if (status == TT_OK) {
                        tt_message_reply( msg );
                } else {
                        tttk_message_fail( msg, status, 0, 0 );
                }
                break;
            case TTDT_REVERT:
                status = myRevert( trust );
                if (status == TT_OK) {
                        tt_message_reply( msg );
                } else {
                        tttk_message_fail( msg, status, 0, 0 );
                }
                break;
            case TTDT_REVERTED:
                if (! isMe) {
                        _modifiedByOther = 0;
                }
                break;
            case TTDT_SAVED:
                if (! isMe) {
                        _modifiedByOther = 0;
                        int choice = myUserChoice( myContext, myBaseFrame,
                                                 &ldquo;Another tool has saved &ldquo;
                                                 &ldquo;this file.&ldquo;, 2, &ldquo;Ignore&ldquo;,
                                                 &ldquo;Revert&ldquo; );
                        switch (choice) {
                            case 1:
                                myRevert( 1 );
                                break;
                        }
                }
                break;
            case TTDT_MOVED:
            case TTDT_DELETED:
                // Do something appropriate
                break;
        }
        tttk_message_destroy( msg );
        return 0;
}
# Ttmedia_load_msg_cbTtmedia_load_msg_cbの例


は、このコールバックの
典型的なアルゴリズムの例です。
# Ttmedia_load_msg_cbの典型的なアルゴリズム
Tt_message
myLoadMsgCB(
        Tt_message      msg,
        void           *clientData,
        Tttk_op         op,
        unsigned char  *contents,
        int             len,
        char           *file
)
{
    if (len > 0) {
                // Replace data with len bytes in contents
        } else if (file != 0) {
                // Replace data with data read from file
        }
        if (op == TTME_DEPOSIT) {
                tt_message_reply( msg );
        }
        tttk_message_destroy( msg );
        return 0;
}
# Ttmedia_load_pat_cbTtmedia_load_pat_cbの例


は、このコールバックの
典型的なアルゴリズムの例です。
# Ttmedia_load_pat_cbの典型的なアルゴリズム
Tt_message
myAcmeSheetLoadCB(
        Tt_message      msg,
        void           *client_data,
        Tttk_op         op,
        Tt_status       diagnosis,
        unsigned char  *contents,
        int             len,
        char           *file,
        char           *docname
)
{
        Tt_status status = TT_OK;
        if (diagnosis != TT_OK) {
                // toolkit detected an error
                if (tt_message_status( msg) == TT_WRN_START_MESSAGE) {
                        //
                        // Error is in start message!  We now have no
                        // reason to live, so tell main() to exit().
                        //
                        myAbortCode = 2;
                }
                // let toolkit handle the error
                return msg;
        }
        if ((op == TTME_COMPOSE) &amp;&amp; (file == 0)) {
                // open empty new buffer
        } else if (len > 0) {
                // load contents into new buffer
        } else if (file != 0) {
                if (ttdt_Get_Modified( msg, file, TT_BOTH, myCntxt, 5000 )) {
                        switch (myUserChoice( &ldquo;Save, Revert, Ignore?&ldquo; )) {
                            case 0:
                                ttdt_Save( msg, file, TT_BOTH, myCntxt, 5000 );
                                break;
                            case 1:
                                ttdt_Revert( msg, file, TT_BOTH, myCntxt, 5000);
                                break;
                        }
                }
                // load file into new buffer
        } else {
                tttk_message_fail( msg, TT_DESKTOP_ENODATA, 0, 1 );
                tt_free( contents ); tt_free( file ); tt_free( docname );
                return 0;
        }
        int w, h, x, y = INT_MAX;
        ttdt_sender_imprint_on( 0, msg, 0, &amp;w, &amp;h, &amp;x, &amp;y, myCntxt, 5000 );
        positionMyWindowRelativeTo( w, h, x, y );
        if (maxBuffersAreNowOpen) {
                // Un-volunteer to handle future requests until less busy
                tt_ptype_undeclare( &ldquo;Acme_Calc&ldquo; );
        }
        if (tt_message_status( msg) == TT_WRN_START_MESSAGE) {
                //
                // Join session before accepting start message,
                // to prevent unnecessary starts of our ptype
                //
                ttdt_session_join( 0, myContractCB, myShell, 0, 1 );
        }
        ttdt_message_accept( msg, myContractCB, myShell, 0, 1, 1 );
        tt_free( contents ); tt_free( file ); tt_free( docname );
        return 0;
}
# Ttmedia_ptype_declareTtmedia_ptype_declare関数のptypeシグニチャの例


は、メディアptypeのシグニチャ・
レイアウトの例です。
# メディアptypeのシグニチャ・レイアウトの例
ptype Acme_Calc {
    start &ldquo;acalc&ldquo;;
    handle:
        /*
         * Display Acme_Sheet
         * Include in tool's ptype if tool can display a document.
         */
        session Display( in    Acme_Sheet  contents) => start opnum = 1;
        session Display( in    Acme_Sheet  contents,
                         in    messageID   counterfoil) => start opnum = 2;
        session Display( in    Acme_Sheet  contents,
                         in    title       docName) => start opnum = 3;
        session Display( in    Acme_Sheet  contents,
                         in    messageID   counterfoil,
                         in    title       docName) => start opnum = 4;
        /*
         * Edit Acme_Sheet
         * Include in tool's ptype if tool can edit a document.
         */
        session Edit(    inout Acme_Sheet  contents) => start opnum = 101;
        session Edit(    inout Acme_Sheet  contents,
                         in    messageID   counterfoil) => start opnum = 102;
        session Edit(    inout Acme_Sheet  contents,
                         in    title       docName) => start opnum = 103;
        session Edit(    inout Acme_Sheet  contents,
                         in    messageID   counterfoil,
                         in    title       docName) => start opnum = 104;
        /*
         * Compose Acme_Sheet
         * Include in tool's ptype if tool can compose a document from scratch.
         */
        session Edit(    out   Acme_Sheet  contents) => start opnum = 201;
        session Edit(    out   Acme_Sheet  contents,
                         in    messageID   counterfoil) => start opnum = 202;
        session Edit(    out   Acme_Sheet  contents,
                         in    title       docName) => start opnum = 203;
        session Edit(    out   Acme_Sheet  contents,
                         in    messageID   counterfoil,
                         in    title       docName) => start opnum = 204;
        /*
         * Mail Acme_Sheet
         * Include in tool's ptype if tool can mail a document.
         */
        session Mail(    in    Acme_Sheet  contents) => start opnum = 301;
        session Mail(    inout Acme_Sheet  contents) => start opnum = 311;
        session Mail(    inout Acme_Sheet  contents,
                         in    title       docName) => start opnum = 313;
        session Mail(    out   Acme_Sheet  contents) => start opnum = 321;
        session Mail(    out   Acme_Sheet  contents,
                         in    messageID   counterfoil) => start opnum = 323;
};
# Xt入力処理関数の例


は、Xt入力処理関数
の例です。
# Xt入力処理関数の例
int myTtFd;
char *myProcID;
myProcID = ttdt_open( &amp;myTtFd, &ldquo;WhizzyCalc&ldquo;, &ldquo;Acme&ldquo;, &ldquo;1.0&ldquo;, 1 );
/* ... */
/* Process the message that started us, if any */
tttk_Xt_input_handler( myProcID, 0, 0 );
/* ... */
XtAppAddInput( myContext, myTtFd, (XtPointer)XtInputReadMask,tttk_Xt_input_handlertttk_Xt_input_handler, myProcID );