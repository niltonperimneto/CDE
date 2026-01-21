
# The Messaging Toolkit


The ToolTalk Messaging Toolkit is a higher-level interface of the ToolTalk API.
It provides common definitions and conventions to easily integrate basic
ToolTalk messages and functionality into an application for optimum
inter&hyphen;operability with other applications that follow the same message
protocols.

Although most of the messages in the ToolTalk Messaging Toolkit are the
messages in the standard ToolTalk message sets, the functions of the Messaging
Toolkit transparently take care of several tasks that would otherwise need to be
coded separately. For example, thettdt_file_jointtdt_file_joinfunction will register a
pattern to observe Deleted, Reverted, Moved, and Saved notices for the
specified file in the specified scope; it also invokes a callback message.
# General Description of the ToolTalk Messaging Toolkit


Inter&hyphen;operability is an important theme if independently developed
applications are to work together. The messages in the toolkit have been agreed
upon by developers of inter&hyphen;operating applications; the protocols form a small,
well&hyphen;defined interface that maximizes application autonomy.

The ToolTalk Messaging Toolkit plays a key role in application inter&hyphen;operability
and offers complete support for messaging. The message protocol specification
includes the set of messages and how applications should behave when they
receive the messages. These messages can be retrofitted to any existing
application to leverage the functionality of the application. You can easily add
these messages to existing applications to send, receive, and use shared
information.

Tools that follow the ToolTalk messaging conventions will not use the same
ToolTalk syntax for different semantics, nor will tools fail to talk to each other
because they use different ToolTalk syntax for identical semantics. If these
protocols are observed, cooperating applications can be modified, even
replaced, without affecting one another.

Most of the messages in the Messaging Toolkit are the messages in the
standard ToolTalk message sets. For detailed descriptions of the standard
ToolTalk message sets, see theToolTalk Reference Manual.lists the
functions described in this chapter that partly comprise the ToolTalk
Messaging Toolkit.
# ToolTalk Messaging Toolkit Functions


`Function`

`Description`

ttdt_close

Destroys a ToolTalk communication
endpoint

ttdt_file_event

Announces an event about a file

ttdt_file_join

Registers to observe ToolTalk events
about a file

ttdt_file_notice

Creates and sends a standard ToolTalk
notice about a file

ttdt_file_quit

Unregisters interest in ToolTalk events
about a file

ttdt_file_request

Creates and sends a standard ToolTalk
request about a file

ttdt_Get_Modified

Asks if any ToolTalk client has changes
pending on a file

ttdt_message_accept

Accepts the responsibility for handling a
ToolTalk request

ttdt_open

Creates a ToolTalk communication
endpoint

ttdt_Revert

Requests that a ToolTalk client revert to
the last saved version of a file

ttdt_Save

Requests that a ToolTalk client save a file

ttdt_sender_imprint_on

Causes a tool to emulate the behavior
and characteristics of the specified
ToolTalk tool

ttdt_session_join

Joins a ToolTalk session and registers
patterns and default callbacks for many
standard desktop messages

ttdt_session_quit

Unregisters any patterns and default
callbacks registered when session joined,
and quits the ToolTalk session

ttdt_subcontract_manage

Manages outstanding requests

ttmedia_Deposit

Sends a Deposit request to checkpoint a
document

ttmedia_load

Creates and sends a Media Exchange
request to display, edit, or compose a
document

ttmedia_load_reply

Replies to a Display, Edit, or Compose
request

ttmedia_ptype_declare

Declares the ptype of a Media Exchange
media editor

tttk_block_while

Blocks the program while awaiting a
condition such as a reply

tttk_message_abandon

Fails or rejects a message, then destroys it

tttk_message_create

Creates a message that conforms to
messaging conventions

tttk_message_fail

Fails a message

tttk_message_receive

Retrieves next ToolTalk message

tttk_message_reject

Rejects a message

tttk_op_string

Returns a string for the operation

tttk_string_op

Returns the operation for the string

tttk_Xt_input_handler

Processes ToolTalk events for Xt clients
# Toolkit Conventions


Most of the messaging conventions for the toolkit consist of descriptions of the
standard ToolTalk message sets. This section describes conventions not related
to any particular standard message set.
# Messaging Toolkit Conventions


`Field`

`Description`

fileAttrib

Indicates whether the file attribute of the message can or needs to be set. The ToolTalk service
allows each message to refer to a file, and has a mechanism (called &ldquo;file-scoping&rdquo;) for
delivering messages to clients that are &ldquo;interested in&rdquo; the named file.

opName

The name of the operation or event (also called &ldquo;op&rdquo;). It is important that different tools use
the same opName to mean the same thing. Unless a message is a standard one, its opName
must be unique; for example, prefix the opName withCompany_Product(such as`Acme_HoarkTool_Hoark_My_Frammistat`).

requiredArgs

Arguments that must always be included in the message.

optionalArgs

Extra arguments that may be included in a message. Any optional arguments in a message
must be in the specified order and must follow the required arguments.

vtype argumentName

A description of a particular argument. A vtype is a programmer-defined string that describes
what kind of data a message argument contains. The ToolTalk service uses vtypes only for
matching sent message instances with registered message patterns. Every vtype should by
convention map to a single, well-known data type.
# Using the Messaging Toolkit When Writing Applications


To use the toolkit, include theToolTalk messaging toolkit header filefilesToolTalk messaging toolkit headerToolTalk Messaging Toolkit header file:#include <Tt/tttk.h>
# The ToolTalk Messaging Toolkit


This section contains a description of functions that are part of the ToolTalk
Messaging Toolkit.
# ttdt_closemessage setstoolkitttdt_closetoolkit messagesttdt_closettdt_close
Tt_status     ttdt_close(   const char *     procid,
                            const char *     new_procid,
                            int              sendStopped );

Thettdt_closefunction destroys a ToolTalk communication endpoint. This
function calls theToolTalk functionstt_closett_close functionToolTalk functiontt_close.

If the value ofprocidis!= 0, this function callstt_default_procid_set( procid )tt_default_procid_set(  procid  )

If the value ofnew_procidis!= 0, this function callstt_default_procid_set( new_procid )tt_default_procid_set(  new_procid )

If thesendStoppedparameter is set, this function sends aStopped noticeStopped notice.

Thettdt_closefunction can return any error returned by the ToolTalk
functionstt_default_procid_set functionToolTalk functionstt_default_procid_settt_default_procid_setandtt_close. If the Sending notice
fails, no errors are propagated.
# ttdt_file_eventmessage setstoolkitttdt_file_eventtoolkit messagesttdt_file_eventttdt_file_event
Tt_status    ttdt_file_event( Tt_message     context,
                              Tttk_op        event,
                              Tt_pattern *   patterns,
                              int            send );

Thettdt_file_eventfunction uses the ToolTalk service to announce an
event about a file. This function creates and, optionally, sends a ToolTalk notice
that announces an event pertaining to a specified file. This file is indicated in
the path name that was passed to thettdt_file_joinmessage setstoolkitttdt_file_jointoolkit messagesttdt_file_jointtdt_file_joinfunction when thepatternswere created.

describes the effect of the value of the`event`parameter on the
announcement made.
# Effect of event Parameter


`Event Announced`

`Announcement`

TTDT_MODIFIEDTTDT_MODIFIED

Registers in the scope passed to thettdt_file_joinfunction to announce the event to interested tools that handleGet_Modified requestGet_Modified,Save requestSave, andRevert requestRevert requests.

TTDT_SAVEDTTDT_SAVED,TTDT_REVERTEDTTDT_REVERTED

Unregisters handler patterns for Get_Modified, Save, and
Revert requests.

If thesendparameter is set, this function sends aSaved noticeSaved orReverted noticeReverted notice, respectively, in the scope.

If thesendparameter is set, this function sends theModified noticeModified notice in the
scope.

If thecontextparameter is a value other than zero, messages created by
this routine inherit all contexts whose slotname begins withENV_ENV_.

lists the possible errors that can be returned by this function.
# Possible Errors Returned by ttdt_file_event


`Error Returned`

`Description`

TT_DESKTOP_EINVALerror messagesTT_DESKTOP_EINVALTT_DESKTOP_EINVAL

The event notice was invalid.
Valid event notices areTTDT_MODIFIED,TTD_TSAVED,
andTTDT_REVERTED.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

Thepatternsparameter was null.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum amount
of active messages (2000) it can properly handle.

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running. This
error indicates that the ToolTalk service is either not
installed or not installed correctly.
# ttdt_file_joinmessage setstoolkitttdt_file_jointoolkit messagesttdt_file_jointtdt_file_join
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

Thettdt_file_joinfunction registers to observe ToolTalk events on the
specified file. It registers in the scope to observeDeleted noticeDeleted,Modified noticeModified,Reverted noticeReverted,Moved noticeMoved, andSaved noticeSaved notices.

The callback message argumentTtdt_file_cbTtdt_file_cbtakes the parameters listed
in.
# Parameters taken by Ttdt_file_cb


`Parameter`

`Description`

message

The message being sent.

op

The operation being requested.

pathname

The path name of the file to which the message pertains. This
copy can be freed with the ToolTalk functiontt_freeToolTalk functionstt_freett_free.

clientdata

The client data contained in the message.

same_euid_egid

A flag that identifies the sender; if this value is true, the sender
can be trusted.

same_procid

A flag that identifies the sender; if this value is true, the sender
is the same procid as the receiver.

If the value ofthe_scopeparameter is zero (that is,TT_SCOPE_NONETT_SCOPE_NONE), the
file scope is set to the default (TT_BOTHTT_BOTH); however, if, for example, the
ToolTalk database serverrpc.ttdbserveris not installed on the file server
that ownspathname, the file scope is set toTT_FILE_IN_SESSIONTT_FILE_IN_SESSION.

Thettdt_file_joinfunction associates the value ofthe_scopeand a
copy ofpathnamewith theTt_patternsreturned to allow thettdt_file_quitfunction to access the patterns. The caller can modify or
freepathnameafter thettdt_file_joincall returns.

If the value of thejoinparameter is true, this function callstt_file_join( pathname )tt_file_join(   pathname   )

This function returns a null-terminated array ofTt_pattern. Use thettdt_file_quitmessage setstoolkitttdt_file_quittoolkit messagesttdt_file_quitttdt_file_quitfunction to destroy the array. If an error is returned, the
returned array is an error pointer that can be decoded withtt_ptr_errortt_ptr_error.is a list of the possible errors returned by thettdt_file_joinfunction.
# Possible Errors Returned by ttdt_file_join


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_DBAVAILerror messagesTT_ERR_DBAVAILTT_ERR_DBAVAIL

The ToolTalk service could not access the ToolTalk database
needed for this operation.

TT_ERR_DBEXISTerror messagesTT_ERR_DBEXISTTT_ERR_DBEXIST

The ToolTalk service did not find the specified ToolTalk
database in the expected place.

TT_ERR_PATHerror messagesTT_ERR_PATHTT_ERR_PATH

The ToolTalk service was not able to read a directory in the
specified file path name.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.
# ttdt_file_noticemessage setstoolkitttdt_file_noticetoolkit messagesttdt_file_noticettdt_file_notice
Tt_message   ttdt_file_notice(     Tt_message     context,
                                   Tttk_op        op,
                                   Tt_scope       scope,
                                   const char *   pathname,
                                   int            send_and_destroy );

Thettdt_file_noticefunction creates and, optionally, sends a standard
ToolTalk notice about a file. Use this function to create the following standard
file notices:Created noticeCreated,Deleted noticeDeleted,Moved noticeMoved,Reverted noticeReverted,Saved noticeSaved, andModified noticeModified.

Thettdt_file_eventmessage setstoolkitttdt_file_eventtoolkit messagesttdt_file_eventttdt_file_eventfunction is a higher-level interface than thettdt_file_noticefunction and is the preferred method to send all notices
except the Moved notice.

If thecontextparameter is a value other than zero, messages created by
this routine inherit all contexts whose slotname begins withENV_.

This function creates a notice with the specified`op`andscopeparameters,
and sets its file attribute topathnameparameter.

If thesend_and_destroyparameter is set, this function sends the message
and then destroys it.

If the value of thesend_and_destroyparameter is false, the created
message is returned; if the value of thesend_and_destroyparameter is
true, zero is returned.

If an error occurs, an error pointer is returned. Usett_ptr_errorto find out
theTt_status.describes possible errors returned by this function.
# Possible Errors Returned by ttdt_file_notice


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running. This
error indicates that the ToolTalk service is either not
installed or not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum amount
of active messages (2000) it can properly handle.

TT_ERR_DBAVAILerror messagesTT_ERR_DBAVAILTT_ERR_DBAVAIL

The ToolTalk service could not access the ToolTalk
database needed for this operation.

TT_ERR_DBEXISTerror messagesTT_ERR_DBEXISTTT_ERR_DBEXIST

The ToolTalk service did not find the specified ToolTalk
database in the expected place.

TT_DESKTOP_EINVALerror messagesTT_DESKTOP_EINVALTT_DESKTOP_EINVAL

The operation was moved, and the value of thesend_and_destroyparameter was true.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The path name was null, or was a ToolTalk error pointer.
# ttdt_file_quitmessage setstoolkitttdt_file_quittoolkit messagesttdt_file_quitttdt_file_quit
Tt_status    ttdt_file_quit( Tt_pattern *  patterns,
                             int           quit );

Thettdt_file_quitfunction unregisters interest in ToolTalk events about a
file. This function destroys patterns. If thequitparameter is set, this function
callstt_file_quit( pathname )tt_file_quit( pathname )

Use this function to unregister interest in the path name that was passed to thettdt_file_joinmessage setstoolkitttdt_file_jointoolkit messagesttdt_file_jointtdt_file_joinfunction whenpatternswas created.lists the
possible errors returned by this function.
# Possible Errors Returned by ttdt_file_quit


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_DBAVAILerror messagesTT_ERR_DBAVAILTT_ERR_DBAVAIL

The ToolTalk service could not access the ToolTalk database
needed for this operation.

TT_ERR_DBEXISTerror messagesTT_ERR_DBEXISTTT_ERR_DBEXIST

The ToolTalk service did not find the specified ToolTalk
database in the expected place.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The patterns were null or otherwise invalid.
# ttdt_file_requestmessage setstoolkitttdt_file_requesttoolkit messagesttdt_file_requestttdt_file_request
Tt_message        ttdt_file_request( Tt_message           context,
                                     Tttk_op              op,
                                     Tt_scope             scope,
                                     const char           pathname,
                                     Ttdt_file_cb         cb,
                                     void                 client_data,
                                     int                  send_and_destroy );

Thettdt_file_requestfunction creates, and optionally sends, any
standard Desktop file&hyphen;scoped request (such as Get_Modified, Save, and
Revert).

This function is a lower-level interface than thettdt_Get_Modified,tdt_Get_Modifiedttdt_Save,ttdt_Saveandttdt_Revertttdt_Revertfunctions, which create and send the request
and then block on its reply.

Thettdt_file_requestfunction creates a request with the specified`op`and`scope`, and sets its file attribute to`pathname`. Per Desktop messaging
conventions, an unset Tt_mode argument ofTT_INand the vtypeFileis
added to the request; and if the specified operation isTTDT_GET_MODIFIEDTTDT_GET_MODIFIED,
an unset Tt_mode argument ofTT_OUTand the vtypeBooleanis also added
to the request.

If`context`is not zero, the request created by this routine inherits from`context`all contexts whose slotname are prefixed withENV_.

This function installscbas a message callback for the created request, and
ensures that client data will be passed into the callback. Ifsendis true, this
function sends the request before returning the handle to it.

This function returns the created Tt_message when successful. If an error
occurs, an error pointer is returned. Usett_ptr_errorto find out the
Tt_status.lists the possible errors returned by this function.
# Possible Errors Returned by ttdt_file_request


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service
tries to restart ttsession if it is not running. This error
indicates that the ToolTalk service is either not installed or
not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough available memory to perform the
operation.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum amount of
active messages (2000) it can properly handle.

TT_ERR_DBAVAILerror messagesTT_ERR_DBAVAILTT_ERR_DBAVAIL

The ToolTalk service could not access the ToolTalk database
needed for this operation.

TT_ERR_DBEXISTerror messagesTT_ERR_DBEXISTTT_ERR_DBEXIST

The ToolTalk service did not find the specified ToolTalk
database in the expected place.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The path name was null or otherwise invalid.
# ttdt_Get_Modifiedmessage setstoolkitttdt_Get_Modifiedtoolkit messagesttdt_Get_Modifiedttdt_Get_Modified
int          ttdt_Get_Modified(   Tt_message      context,
                                  const char *    pathname,
                                  Tt_scope        the_scope,
                                  XtAppContext    app2run,
                                  int             ms_timeout );

Thettdt_Get_Modifiedfunction asks if any ToolTalk client has changes
pending on a file. This function sends aGet_Modified requestGet_Modified request and waits for a
reply.

If the`context`parameter is a value other than zero, messages created by
this routine inherit all contexts whose slotname begins withENV_.

The Get_Modified request asks if any ToolTalk client has changes pending
on`pathname`that it intends to make persistent.

Thethe_scopeparameter indicates the scope in which the Get_Modified
request is sent. If the value of this parameter is zero (that is,TT_SCOPE_NONETT_SCOPE_NONE), the file scope is set to the default (TT_BOTHTT_BOTH); however, if,
for example, the ToolTalk database serverrpc.ttdbserveris not installed
on the file server that owns`pathname`, the file scope is set toTT_FILE_IN_SESSIONTT_FILE_IN_SESSION.

Theapp2runandms_timeoutparameters are passed to thetttk_block_whilemessage setstoolkittttk_block_whiletoolkit messagestttk_block_whiletttk_block_whilefunction to block on the reply to the Get_Modified
request sent by this function.

If the Get_Modified request receives an affirmative reply within the specified
time out, thettdt_Get_Modifiedfunction returns non-zero; otherwise, it
returns zero. This call does not return any errors.
# ttdt_message_acceptmessage setstoolkitttdt_message_receivetoolkit messagesttdt_message_acceptttdt_message_accept
Tt_pattern *     ttdt_message_accept(     Tt_message         contract,
                                          Ttdt_contract_cb   cb,
                                          void *             clientdata,
                                          Widget             shell,
                                          int                accept,
                                          int                sendStatus );

Thettdt_message_acceptfunction accepts a contract to handle a ToolTalk
request. A tool calls this function when it wants to accept responsibility for
handling (that is, failing or rejecting) a request.

ATtdt_contract_cb argumentTtdt_contract_cbargument takes the parameters listed in.
# Parameters Taken by the Ttdt_contract_cb Argument


`Parameter`

`Description`

Tt_message msg

The request in the sent state.
The client program must either fail, reject, or reply to
the message.

Tttk_op op

The operation of the incoming request.

Widget shell

The shell passed to thettdt_message_acceptfunction.

void *clientdata

The client data passed to thettdt_message_acceptfunction.

Tt_message contract

The contract passed to thettdt_message_acceptfunction.

If the callback processes the messagemsgsuccessfully, it returns zero;
otherwise, it returns att_error_pointercast toTt_message.

If the callback does not consume the messagemsg, it returns the message and
passes theTT_CALLBACK_CONTINUEroutine down the call stack to offer the
message to other callbacks, or to return it to thett_message_receivecall.

Thettdt_message_acceptfunction registers in the default session for the
handler&hyphen;addressed requests described in.
# Requests for which ttdt_message_accept Registers


`Request`

`How Request Is Handled`

Get_Geometry requestGet_Geometr
y,Set_Geometry requestSet_Geometr
y

If the`shell`parameter is not null, these requests are handled transparently; if the`shell`parameter is
null and thecbparameter is not null, these requests are passed to the callback routine; otherwise, these
requests fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_Iconified requestGet_Iconifi
ed,Set_Iconified requestSet_Iconifi
ed

If the`shell`parameter is not null, these requests are handled transparently; if the`shell`parameter is
null and thecbparameter is not null, these requests are passed to the callback routine; otherwise, these
requests fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_Mapped requestGet_Mapped,Set_Mapped requestSet_Mapped

If the`shell`parameter is not null, these requests are handled transparently; if the`shell`parameter is
null and thecbparameter is not null, these requests are passed to the callback routine; otherwise, these
requests fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Raise requestRaise

If the`shell`parameter is not null, this request is handled transparently; if the`shell`parameter is null
and thecbparameter is not null, these requests are passed to the callback routine; otherwise, these
requests fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Lower requestLower

If the`shell`parameter is not null, this request is handled transparently; if the`shell`parameter is null
and thecbparameter is not null, these requests are passed to the callback routine; otherwise, these
requests fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_XInfo,Get_XInfo requestSet_XInfoSet_XInfo request

If the`shell`parameter is not null, these requests are handled transparently; if the`shell`parameter is
null and thecbparameter is not null, these requests are passed to the callback routine; otherwise, these
requests fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Pause requestPause

If thecbparameter is not null, this request is passed to the callback routine; otherwise, it fails with the
errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Resume requestResume

If thecbparameter is not null, this request is passed to the callback routine; otherwise, it fails with the
errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Quit requestQuit

If thecbparameter is not null, this request is passed to the callback routine; otherwise, it fails with the
errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_Status requestGet_Status

If thecbparameter is not null, this request is passed to the callback routine; otherwise, it fails with the
errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

If thecontractargument has aTT_WRN_START_MESSAGETT_WRN_START_MESSAGEmessage status,
the message caused the tool to be started.

The started tool should join any scopes it wants to serve before
accepting the contract so that it will receive any other messages already
dispatched to its ptype; otherwise, the tool should undeclare its ptype while it
is busy. If the tool does not join any scopes, the dispatched messages will cause
other instances of the ptype to be started.

If theacceptargument is true, thettdt_message_acceptfunction callstt_message_accept( contract )tt_message_accept(   contract   )

If thesendStatusargument is true, thettdt_message_acceptfunction
sends a Status notice to the requestor, using the parameters (if any) passed to
thettdt_openmessage setstoolkitttdt_opentoolkit messagesttdt_openttdt_openfunction.

This function returns a null-terminated array ofTt_pattern. Use thetttk_patterns_destroymessage setstoolkittttk_patterns_destroytoolkit messagestttk_patterns_destroytttk_patterns_destroyfunction to destroy the array. If an error is
returned, the returned array is an error pointer that can be decoded withtt_ptr_errortt_ptr_error.is a list of the possible errors returned by thettdt_message_acceptfunction.
# Possible Errors Returned by ttdt_message_accept


`Returned Error`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The pointer passed does not point at an object of the correct
type for this operation. For example, the pointer may point to
an integer when a character string is needed.

TT_ERR_UNIMPerror messagesTT_ERR_UNIMPTT_ERR_UNIMP

The ttsession for the default session is a version (1.0 or 1.0.1)
that does not support thett_message_acceptfunction.

`Note:`If the contract argument has aTT_WRN_START_MESSAGEmessage status, messages to the tool's ptype will remain blocked
until the contract is rejected, replied to, or failed.
# ttdt_openmessage setstoolkitttdt_opentoolkit messagesttdt_openttdt_open
char *        ttdt_open(    int *          ttfd,
                            const char *   toolname,
                            const char *   vendor,
                            const char *   version,
                            int            sendStarted );

Thettdt_openfunction creates a ToolTalk communication endpoint. This
function callstt_open functionToolTalk functionstt_opentt_openandtt_fdtt_fdfunctions. Thettdt_openfunction
associatestoolname,vendor, and`version`with the created procid. It
initializes the new procid's default contexts fromenviron(5)environ(5). If thesendStartedargument is set, this function sends aStarted noticeStarted notice.

Thettdt_openfunction returns the created procid in a string that can be
freed with thett_free functionToolTalk functionstt_freett_freefunction.

This function can return any error returned by thett_openandtt_fdfunctions. If the Started notice fails, errors are not propagated.
# ttdt_Revertmessage setstoolkitttdt_Reverttoolkit messagesttdt_Revertttdt_Revert
Tt_status    ttdt_Revert( Tt_message       context,
                          const char *    pathname,
                          Tt_scope        the_scope,
                          XtAppContext    app2run,
                          int             ms_timeout );

Thettdt_Revertfunction requests a ToolTalk client to revert a file. It sends aRevert requestRevert request inthe_scopeand waits for a reply. The Revert request asks
the handling ToolTalk client to discard any changes pending onpathname.

If the`context`parameter is a value other than zero, messages created by
this routine inherit all contexts whose slotname begins withENV_.

If the value of thethe_scopeparameter is zero (that is,TT_SCOPE_NONETT_SCOPE_NONE),
the file scope is set to the default (TT_BOTHTT_BOTH); however, if, for example, the
ToolTalk database server rpc.ttdbserver is not installed on the file server that
owns`pathname`, the file scope is set toTT_FILE_IN_SESSIONTT_FILE_IN_SESSION.

Theapp2runandms_timeoutparameters are passed to thetttk_block_whilemessage setstoolkittttk_block_whiletoolkit messagestttk_block_whiletttk_block_whilefunction to block on the reply to the Revert request
sent by this function.

If the request receives an affirmative reply within the indicated timeout, thettdt_Revertfunction returnsTT_OK; otherwise, it returns either thett_message_statustt_message_statusof the failure reply, or one of the errors listed in.
# Possible Errors Returned by ttdt_Revert


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running.
This error indicates that the ToolTalk service is either
not installed or not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or
invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum
amount of active messages (2000) it can properly
handle.

TT_ERR_DBAVAILerror messagesTT_ERR_DBAVAILTT_ERR_DBAVAIL

The ToolTalk service could not access the ToolTalk
database needed for this operation.

TT_ERR_DBEXISTerror messagesTT_ERR_DBEXISTTT_ERR_DBEXIST

The ToolTalk service did not find the specified
ToolTalk database in the expected place.

TT_DESKTOP_ETIMEOUTerror messagesTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEOUT

No reply was received before the allotted timeout.

TT_DESKTOP_ETPROTOerror messagesTT_DESKTOP_EPROTOTT_DESKTOP_EPROTO

The request was failed; however, the handler set the
tt_message_status of the failure reply to TT_OK
instead of a specific error status.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

Path name was null, or was a ToolTalk error pointer.
# ttdt_Savemessage setstoolkitttdt_Savetoolkit messagesttdt_Savettdt_Save
Tt_status    ttdt_Save( Tt_message     context,
                        const char *   pathname,
                        Tt_scope    the_scope,
                        XtAppContext     app2run,
                        int            ms_timeout );

Thettdt_Savefunction requests a ToolTalk client to save a file. It sends aSave requestSave request inthe_scopeand waits for a reply. The Save request asks the
handling ToolTalk client to discard any changes pending onpathname.

If the`context`parameter is a value other than zero, messages created by
this routine inherit all contexts whose slotname begins withENV_.

If the value of thethe_scopeparameter is zero (that is,TT_SCOPE_NONETT_SCOPE_NONE),
the file scope is set to the default (TT_BOTHTT_BOTH); however, if, for example, the
ToolTalk database serverrpc.ttdbserveris not installed on the file server
that owns`pathname`, the file scope is set toTT_FILE_IN_SESSIONTT_FILE_IN_SESSION.

Theapp2runandms_timeoutparameters are passed to thetttk_block_whilemessage setstoolkittttk_block_whiletoolkit messagestttk_block_whiletttk_block_whilefunction to block on the reply to the Save request sent
by this function.

If the request receives an affirmative reply within the indicated timeout, thettdt_Savefunction returnsTT_OK; otherwise, it returns either thett_message_statustt_message_statusof the failure reply, or one of the errors listed in.
# Possible Returns of the ttdt_Save function


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running.
This error indicates that the ToolTalk service is either
not installed or not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or
invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum
amount of active messages (2000) it can properly
handle.

TT_ERR_DBAVAILerror messagesTT_ERR_DBAVAILTT_ERR_DBAVAIL

The ToolTalk service could not access the ToolTalk
database needed for this operation.

TT_ERR_DBEXISTerror messagesTT_ERR_DBEXISTTT_ERR_DBEXIST

The ToolTalk service did not find the specified
ToolTalk database in the expected place.

TT_DESKTOP_ETIMEOUTerror messagesTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEOUT

No reply was received before the allotted timeout.

TT_DESKTOP_ETPROTOerror messagesTT_DESKTOP_EPROTOTT_DESKTOP_EPROTO

The request was failed; however, the handler set the
tt_message_status of the failure reply to TT_OK
instead of a specific error status.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

Path name was null, or was a ToolTalk error pointer.
# ttdt_sender_imprint_onmessage setstoolkitttdt_sender_imprint_ontoolkit messagesttdt_sender_imprint_onttdt_sender_imprint_on
Tt_status    ttdt_sender_imprint_on(  const char *   handler,
                                      Tt_message     contract,
                                      char **        display,
                                      int *          width,
                                      int *          height,
                                      int *          xoffset,
                                      int *          yoffset,
                                      XtAppContext   app2run,
                                      int            ms_timeout );

Thettdt_sender_imprint_onfunction causes the calling tool (&ldquo;ToolB&rdquo;) to
adopt the behavior and certain characteristics of another tool (&ldquo;ToolA&rdquo;). ToolB
adopts ToolA's X11 display, locale, and current working directory; it also learns
ToolA's X11 geometry so that it can position itself appropriately.

If the`display`parameter is null, the$DISPLAYenvironment variables$DISPLAYenvironment variable$DISPLAYis set to
ToolA's display; otherwise, ToolA's display is returned in this parameter. The
returned value is a string that can be freed with theToolTalk functionstt_freett_free functionToolTalktt_freefunction.

This function sends aGet_Geometry requestGet_Geometry request to ToolA. If ToolA does not return
a value for any or all of the geometry parameters:

If a value for the`width`parameter is not returned, it is set to-1.

If a value for the`height`parameter is not returned, it is set to-1.

If a value for thexoffsetparameter is not returned, it is set toINT_MAX.

If a value for the`yoffset`parameter is not returned, it is set toINT_MAX.

If the`width`,`height`,`xoffset`, and`yoffset`parameters in thettdt_sender_imprint_onfunction are all set to null, aGet_Geometry requestGet_Geometry
request is not sent to ToolA.

Theapp2runandms_timeoutparameters are passed to thetttk_block_whilemessage setstoolkittttk_block_whiletoolkit messagestttk_block_whiletttk_block_whilefunction to block on the replies to the Get_Geometry
request sent by this function.

lists the possible errors that can be returned by this function.
# Possible Errors Returned by the ttdt_sender_imprint_on


`Error Returned`

`Description`

TT_DESKTOP_ETIMEDOUTerror messagesTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEDOUT

One or more of the sent requests did not complete
before the allotted timeout.

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running.
This error indicates that the ToolTalk service is either
not installed or not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or
invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform
the operation.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum
amount of active messages (2000) it can properly
handle.
# ttdt_session_joinmessage setstoolkitttdt_session_jointoolkit messagesttdt_session_jointtdt_session_join
Tt_message   ( *Ttdt_contract_cb)       (  Tt_message      msg,
                                           void *          clientdata
                                           Tt_message       contract );

Tt_pattern * ttdt_session_join( const char *        sessid,
                                Ttdt_session_cb     cb,
                                Widget              shell,
                                void *              clientdata,
                                int                 join );

Thettdt_session_joinfunction joins a ToolTalk session as a &ldquo;good
desktop citizen&rdquo;; that is, it registers patterns and default callbacks for many
standard desktop message interfaces when it joins the sessionsessid.lists the message interfaces for which this function currently
registers.
# Standard Messages for which the ttdt_session_join Registers


`Request`

`How Message Is Handled`

Get_Environment requestGet_Environment,Set_Environment requestSet_Environment

These messages are handled transparently.

Get_Locale requestGet_Locale,Set_Locale requestSet_Locale

These messages are handled transparently.

Get_Situation requestGet_Situation,Set_Situation requestSet_Situation

These messages are handled transparently.

Signal requestSignal

This message is handled transparently.

Get_Sysinfo requestGet_Sysinfo

This message is handled transparently.

Get_Geometry requestGet_Geometry,Set_Geometry requestSet_Geometry

If the value of the`shell`parameter is not null and the shell is a realizedmappedWhenManaged
applicationShellWidget, these messages are handled transparently; if the shell is not amappedWhenManaged applicationShellWidget, these messages fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_Iconified requestGet_Iconified,Get_Iconified requestGet_Iconified

If the value of the`shell`parameter is not null and the shell is a realizedmappedWhenManaged
applicationShellWidget, these messages are handled transparently; if the shell is not amappedWhenManaged applicationShellWidget, these messages fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_Mapped requestGet_Mapped,Set_Mapped requestSet_Mapped

If the value of the`shell`parameter is not null and the shell is a realizedmappedWhenManaged
applicationShellWidget, these messages are handled transparently; if the shell is not amappedWhenManaged applicationShellWidget, these messages fail with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Raise requestRaise

If the value of the`shell`parameter is not null and the shell is a realizedmappedWhenManagedapplicationShellWidget, this message is handled transparently; if the shell is not amappedWhenManaged applicationShellWidget, this message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Lower requestLower

If the value of the`shell`parameter is not null and the shell is a realizedmappedWhenManagedapplicationShellWidget, this message is handled transparently; if the shell is not amappedWhenManaged applicationShellWidget, this message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_XInfo requestGet_XInfo

If the value of the`shell`parameter is not null, this message is handled transparently; otherwise,
this message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Set_XInfo requestSet_XInfo

If the value of the`shell`parameter is not null and the shell is a realizedmappedWhenManaged
applicationShellWidget, this message is handled transparently; if the shell is not amappedWhenManaged applicationShellWidget, this message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Pause requestPause

If thecbparameter is not null, this message is passed to the callback; the cb parameter is null, this
message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Resume requestResume

If thecbparameter is not null, this message is passed to the callback; thecbparameter is null, this
message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Quit requestQuit

If thecbparameter is not null, this message is passed to the callback; thecbparameter is null, this
message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Get_Status requestGet_Status

If thecbparameter is not null, this message is passed to the callback; thecbparameter is null, this
message fails with the errorTT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP.

Do_Command requestDo_Command

If thecbparameter is not null, this message is passed to the callback; thecbparameter is null, this
message fails with the error`TT_DESKTOP_ENOTSUPerror messagesTT_DESKTOP_ENOTSUPTT_DESKTOP_ENOTSUP`.

If thesessidparameter is null, the default session is joined.

If thejoinparameter is set, the specified session is joined.

Attdt_contract_cbmessage setstoolkitttdt_contract_cbtoolkit messagesttdt_contract_cbTtdt_contract_cbmessage takes the parameters described in.
If the callback does not consume the message, it returns the message; if it
consumes the message, it returns either zero or a error pointer cast toTt_message.
# Parameters taken by Ttdt_session_cb


`Parameter`

`Description`

Tt_messagemsg

The request in the sent state.
The client program must either fail, reject, or reply to the
message.

`Note`: Destroy the messagemsgafter it is processed.

void *clientdata

The clientdata passed to either thettdt_session_joinorttdt_message_acceptfunction.

Tt_messagecontract

The contract passed to thettdt_message_acceptfunction. If the callback is installed by thettdt_session_joinfunction, the value for thecontractparameter is always zero.

Thettdt_session_joinfunction returns a null-terminated array ofTt_pattern, which can be passed to thettdt_session_quitmessage setstoolkitttdt_session_quittoolkit messagesttdt_session_quitttdt_session_quitfunction to be
destroyed. If an error occurs, the returned array that is an error pointer. Usett_ptr_errortt_ptr_errorto find theTt_status.lists the possible errors
returned.
# Possible Errors Returned by the ttdt_session_join


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_SESSIONerror messagesTT_ERR_SESSIONTT_ERR_SESSION

An out&hyphen;of&hyphen;date or invalid ToolTalk session was specified.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The pointer passed does not point at an object of the correct
type for this operation. For example, the pointer may point to
an integer when a character string is needed.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.
# ttdt_session_quitmessage setstoolkitttdt_session_quittoolkit messagesttdt_session_quitttdt_session_quit
Tt_status    ttdt_session_quit( const char *     sessid,
                                Tt_pattern *     sess_pats,
                                int              quit );

Thettdt_session_quitfunction quits a ToolTalk session as a &ldquo;good
desktop citizen&rdquo;; that is, it unregisters all the patterns and default callback it
registered when it joined the session.

This function destroys all patterns insess_pats. If the`quit`parameter is set,
it quits the sessionsessid; if thesessidparameter is null, it quits the default
session.

lists the errors that can be returned by this function.
# Possible Errors Returned by the ttdt_session_quit


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_SESSIONerror messagesTT_ERR_SESSIONTT_ERR_SESSION

An out&hyphen;of&hyphen;date or invalid ToolTalk session was specified.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The pointer passed does not point at an object of the correct
type for this operation. For example, the pointer may point to
an integer when a character string is needed.
# ttdt_subcontract_managemessage setstoolkitttdt_subcontract_managetoolkit messagesttdt_subcontract_managettdt_subcontract_manage
Tt_pattern * ttdt_subcontract_manage(  Tt_message         subcontract,
                                       Ttdt_contract_cb   cb,
                                       Widget             shell,
                                       void *             clientdata );

Thettdt_subcontract_managefunction manages an outstanding request. It
allows the requesting tool to manage the standard Desktop interactions with
the tool that is handling the request. This function registers in the default
session forTT_HANDLER-addressedGet_Geometry requestGet_Geometry andGet_XInfo requestGet_XInfo requests,
andStatus noticeStatus notices.

If the`shell`parameter is null, the request or notice is passed to thecbparameter; otherwise, the request is handled transparently.

Thettdt_subcontract_managefunction returns a null-terminated array of
Tt_pattern, which can be passed to thettdt_session_quitmessage setstoolkitttdt_session_quittoolkit messagesttdt_session_quitttdt_session_quitfunction to be
destroyed. If an error occurs, the returned array that is an error pointer. Usett_ptr_errortt_ptr_errorto find theTt_status.lists the possible errors
returned.
# Possible Errors Returned by the ttdt_subcontract_manage


`Error Returned`

`Description`

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

Thesubcontractparameter was not a validTt_message.

TT_ERR_EINVALerror messagesTT_ERR_EINVALTT_ERR_EINVAL

Both the`shell`andcbparameters were null.
# ttmedia_Depositmessage setstoolkitttmedia_Deposittoolkit messagesttmedia_Depositttmedia_Deposit
Tt_status   ttmedia_Deposit(     Tt_message              load_contract,
                                 const char *            buffer_id,
                                 const char *            media_type,
                                 const unsigned char *   new_contents,
                                 int                     new_len,
                                 const char *            pathname,
                                 XtAppContext            app2run,
                                 int                     ms_timeout );

Thettmedia_Depositfunction sends a Deposit request to checkpoint a
document that was the subject of a Media Exchange load_contract request such
as Edit, Compose, or Open.

This function creates and sends a Deposit request and returns the success or
failure of that request.

load_contractis the request that caused this editor to load the document

buffer_idis the id of the buffer this editor created if the document was
loaded by an Open request

media_typeis the vtype of thecontentsargument of the sent request

new_contentsand new_len are the values for thecontentsargument

After the request is sent,app2runandms_timeoutare passed to thetttk_block_whilefunction to wait for the reply.
# Possible Errors Returned by the ttmedia_Deposit


TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running. This
error indicates that the ToolTalk service is either not
installed or not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough available memory to perform the
operation.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum amount
of active messages (2000) it can properly handle.

TT_ERR_DBAVAILerror messagesTT_ERR_DBAVAILTT_ERR_DBAVAIL

The ToolTalk service could not access the ToolTalk
database needed for this operation.

TT_ERR_DBEXISTerror messagesTT_ERR_DBEXISTTT_ERR_DBEXIST

The ToolTalk service did not find the specified ToolTalk
database in the expected place.

TT_DESKTOP_ETIMEOUTerror messagesTT_DESKTOP_ETIMEOUTTT_DESKTOP_ETIMEOUT

No reply was received before the allotted timeout.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

Path name was null, or was a ToolTalk error pointer.
# ttmedia_loadmessage setstoolkitttmedia_loadtoolkit messagesttmedia_loadttmedia_load
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

Thettmedia_load functioncreates and, optionally, sends a Media
Exchange request to display, edit, or compose a document. This function
creates and sendsDisplay requestDisplay,Edit requestEdit, orCompose requestCompose requests.

Use thettdt_subcontract_managemessage setstoolkitttdt_subcontract_managetoolkit messagesttdt_subcontract_managettdt_subcontract_managefunction immediately after
sending the request created by this message to manage the standard
interactions with the handler of the request.

If value of the`context`argument is not zero, messages created by this routine
inherit all contexts whose slotname begins withENV_ENV_.

Theclientdataargument is passed to thecbargument when the reply is
received, or when intermediate versions of the document are checkpointed
throughDeposit requestDeposit requests.

The`op`argument must be eitherTTME_DISPLAYTTME_DISPLAY,TTME_EDITTTME_EDIT, orTTME_COMPOSETTME_COMPOSE.

Themedia_typeargument names the data format of the document. This
argument usually determines which application is chosen to handle the
request.

Thecontentsandlenarguments specify the document. If the value of both
of these arguments is zero and the value of the`file`argument is not zero, the
document is assumed to be contained in the specified file.

If thedocnameargument is not null, it is used as the title of the document.

If thesendargument is true, the message is sent before it is returned.

lists the parameters taken by aTtmedia_load_msg_cb messageTtmedia_load_msg_cbmessage.
# Parameters Taken by the Ttmedia_load_msg_cb


`Parameter`

`Description`

Tt_message msg

The reply to the request, or aDeposit requestDeposit request with amessageIDargument that names thett_message_idt_message_idof
the load request. If the value of this parameter is a Deposit
request, the client program must either fail or reply to the
request.

`Note`: Destroy the messagemsgafter it is processed.

Tttk_op op

The operation of the message (eitherTTME_DEPOSITTTME_DEPOSITor
the operation passed to thettmedia_loadmessage setstoolkitttmedia_loadtoolkit messagesttmedia_loadttmedia_loadmessage).

unsigned char *
contents
int len
char *file

The contents of the arriving document. If thelenargument is zero, the document is contained in the
specified file. If thecontentsor`file`arguments are
non-null, use the ToolTalk functiontt_free functionToolTalk functionstt_freett_freeto free them.

void *clientdata

The client data passed to thettmedia_loadmessage setstoolkitttmedia_loadtoolkit messagesttmedia_loadttmedia_loadmessage.

If the message is processed successfully, the callback returns zero; if the
processing results in an error, the callback returns an error pointer cast toTt_message.

If the callback does not consume the messagemsg, it returns the message and
the toolkit passes theTT_CALLBACK_CONTINUEroutine down the call stack to
offer the message to other callbacks, or to return it to thett_message_receivecall.

Upon completion, thettmedia_loadfunction returns the request it was
asked to build. If an error occurs, this function returns an error pointer. Usett_ptr_errortt_ptr_errorto find theTt_status.lists the possible errors
returned.
# Possible Errors Returned by the ttmedia_load


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running. This
error indicates that the ToolTalk service is either not
installed or not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.

TT_ERR_OVERFLOWerror messagesTT_ERR_OVERFLOWTT_ERR_OVERFLOW

The ToolTalk service has received the maximum amount
of active messages (2000) it can properly handle.
# ttmedia_load_replymessage setstoolkitttmedia_load_replytoolkit messagesttmedia_load_replyttmedia_load_reply
Tt_message    ttmedia_load_reply   (   Tt_message             contract,
                                       const unsigned char *  new_contents,
                                       int                    new_len,
                                       int                    reply_and_destroy );

Use thettmedia_load_replyfunction to reply to a Media Exchange request
to display, edit, or compose a document.

If both thenew_contentsandnew_lenarguments are non-zero, their value
is used to set the new contents of the document in the appropriate output
argument of thecontractargument. If thereply_and_destroyargument
is true, a reply is made to thecontractargument and then the message is
destroyed.

lists the possible errors returned.
# Possible Errors Returned by the ttmedia_load_reply


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running. This
error indicates that the ToolTalk service is either not
installed or not installed correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_NUMerror messagesTT_ERR_NUMTT_ERR_NUM

TT_ERR_NOTHANDLERerror messagesTT_ERR_NOTHANDLERTT_ERR_NOTHANDLER
# ttmedia_ptype_declaremessage setstoolkitttmedia_ptype_declaretoolkit messagesttmedia_ptype_declarettmedia_ptype_declare
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

Thettmedia_ptype_declarefunction declares the ptype of a Media
Exchange media editor. This function initializes an editor that implements the
Media Exchange message interface for a particular media type.

It calls thecbargument when the editor is asked to edit a document of the
kind supported byptype.

It installs a toolkit-internal operation number (opnum) callback on a series of
signatures that the ptype is assumed to contain. The toolkit-internal opnum
callback passesclientdatato thecbargument when a request is received
that matches one of these signatures. The opnums start atbase_opnum,
which must be zero or a multiple of 1000.

If thedeclareargument is true, it callstt_ptype_declare( ptype )tt_ptype_declare(   ptype  )

If the ptype implements several different media types, thettmedia_ptype_declarefunction can be called multiple times. Each call
must have a differentbase_opnumvalue.

Thettmedia_ptype_declarefunction can be called multiple times;
however, thedeclareargument can &ldquo;true&rdquo; only once.

lists the parameters taken by aTtmedia_load_pat_cb messageTtmedia_load_pat_cbmessage.
# Parameters Taken by Ttmedia_load_pat_cb


`Parameter`

`Description`

Tt_message msg

The request sent. The client program must either fail,
reject, or reply to the request.

Tttk_op op

The operation of the incoming request (eitherTTME_COMPOSETTME_COMPOSE,TTME_EDITTTME_EDIT, orTTME_DISPLAYTTME_DISPLAY.

Tt_status diagnosis

The error code with which the toolkit recommends the
request should be failed (for example,TT_DESKTOP_ENODATAerror messagesTT_DESKTOPTT_DESKTOP_ENODATA). If the diagnosis is notTT_OKand the callback routine returns the messagemsg, the
toolkit fails the messagemsgand destroys it.

unsigned char *
contents
int len
char *file

The contents of the arriving document. If thelenargument is zero, the document is contained in
specified file. If value of thecontentsor`file`arguments is non-null, use the ToolTalk functiontt_free functionToolTalk functionstt_freett_free to free them.

char * docname

The name of the document, if any.

void * clientdata

The client data passed to thettmedia_ptype_declaremessage.

If the message is processed successfully, the callback returns zero; if the
processing results in an error, the callback returns an error pointer cast toTt_message.

If the callback does not consume the messagemsgand the value of thediagnosisargument is notTT_OK, it returns the message and the toolkit
passes theTT_CALLBACK_CONTINUEroutine down the call stack to offer the
message to other callbacks, or to return it to thett_message_receivecall.

If an error occurs, this function returns one of the errors listed in.
# Possible Errors Returned by the ttmedia_ptype_declare


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_PTYPEerror messagesTT_ERR_PTYPETT_ERR_PTYPE

The ToolTalk service could not locate the specified ptype.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The pointer passed does not point at an object of the correct
type for this operation. For example, the pointer may point to
an integer when a character string is needed.
# tttk_block_whilemessage setstoolkittttk_block_whiletoolkit messagestttk_block_whiletttk_block_while
Tt_status    tttk_block_while( const int *  blocked,
                               int          ms_timeout );

Thetttk_block_whilefunction blocks the program while it awaits a reply
for thems_timouttime.
# tttk_message_abandonmessage setstoolkittttk_message_abandontoolkit messagestttk_message_abandontttk_message_abandon
Tt_status   tttk_message_abandon     ( Tt_message     msg );

Thetttk_message_abandonfunction abandons the request, and then
destroys it.

A program should abandon a message when it does not understand the
message and wants to dispose of it.

If an error occurs, this function returns one of the errors listed in.
# Possible Errors Returned by the tttk_message_abandon


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk
service tries to restart ttsession if it is not running. This
error indicates that the ToolTalk service is either not
installed or not installed correctly.

TT_ERR_POINTERerror messagesTT_ERR_POINTERTT_ERR_POINTER

The pointer passed does not point at an object of the
correct type for this operation. For example, the
pointer may point to an integer when a character
string is needed.

TT_ERR_NOTHANDLERerror messagesTT_ERR_NOTHANDLERTT_ERR_NOTHANDLER
# tttk_message_createmessage setstoolkittttk_message_createtoolkit messagestttk_message_createtttk_message_create
Tt_message   tttk_message_create( Tt_message             context,
                                  Tt_class               the_class,
                                  Tt_scope               the_scope,
                                  const char *           handler,
                                  const char *           op,
                                  Tt_message_callback    callback );

Thetttk_message_createfunction creates a message that conforms to the
conventions. This function provides a simple way to create a message that
propagates inherited contexts from one message to another.

Thetttk_message_createfunction creates a message and copies onto it all
the context slots from`context`whose slotname begins withENV_. The created
message is given aTt_classvalue ofthe_classand aTt_scopevalue ofthe_scope.

If thehandlerparameter is null, the message is given aTt_addressofTT_PROCEDURETT_PROCEDURE; otherwise, the message isTT_HANDLER&hyphen;addressed to that
procid.

If the`op`argument is not null, the message's`op`argument is set to that value.

If the callback argument is not null, it is added to the message as a message
callback.

If successful, thetttk_message_createfunction returns the createdTt_message, which can be modified, sent, and destroyed in the same way as
any otherTt_message.

If an error occurs, an error pointer is returned. Usett_ptr_errortt_ptr_errorto find theTt_status.lists the possible errors returned.
# Possible Errors Returned by the tttk_message_create


`Error Returned`

`Description`

TT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP

The ttsession process is not available. The ToolTalk service tries
to restart ttsession if it is not running. This error indicates that
the ToolTalk service is either not installed or not installed
correctly.

TT_ERR_PROCIDerror messagesTT_ERR_PROCIDTT_ERR_PROCID

The process identifier specified is out of date or invalid.

TT_ERR_NOMEMerror messagesTT_ERR_NOMEMTT_ERR_NOMEM

There is not enough memory available to perform the
operation.
# tttk_message_destroymessage setstoolkittttk_message_destroytoolkit messagestttk_message_destroytttk_message_destroy
Tt_status   tttk_message_destroy ( Tt_message    msg );

Thetttk_message_destroyfunction destroys any message that conforms to
the conventions.

This message can be used in place of thett_message_destroy messagett_message_destroymessage.

Thetttk_message_destroyfunction destroys any patterns that may have
been stored on the message by thettdt_message_acceptorttdt_subcontract_managefunctions and then passes the messagemsgto
thett_message_destroyfunction.

This function returns the value returned by thett_message_destroyfunction.
# tttk_message_failmessage setstoolkittttk_message_failtoolkit messagestttk_message_failtttk_message_fail
Tt_status    tttk_message_fail( Tt_message     msg,
                                Tt_status      status,
                                const char *   status_string,
                                int            destroy );

Thetttk_message_failfunction fails the messagemsgand then destroys it.

A program should abandon a message when it does not understand the
message and wants to dispose of it.

A message whose state isTT_SENTcan be failed. If the message is a
handler&hyphen;addressed message, or if it has a tt_message_status ofTT_WRN_START_MESSAGETT_WRN_START_MESSAGE, it can be failed.

This function returnsTT_DESKTOP_ENOTSUP.
# tttk_message_receivemessage setstoolkittttk_message_receivetoolkit messagestttk_message_receivetttk_message_receivemessage setstoolkittttk_message_receivetoolkit messagestttk_message_receivetttk_message_receive
Tt_status    tttk_message_receive( const char* procid );

Thetttk_message_receivefunction calls thett_message_receivett_message_receivefunction to retrieve the next ToolTalk message.

Ifprocid!= 0, this function callstt_default_procid_set( procid )
# tttk_message_rejectmessage setstoolkittttk_message_rejecttoolkit messagestttk_message_rejecttttk_message_rejectmessage setstoolkittttk_message_rejecttoolkit messagestttk_message_rejecttttk_message_reject
Tt_status    tttk_message_reject( Tt_message      msg,
                                  Tt_status       status,
                                  const char*     status_string,
                                  int             destroy );

Thetttk_message_rejectfunction rejects the messagemsgand then
destroys it.

A program should abandon a message when it does not understand the
message and wants to dispose of it.

A message whose state isTT_SENTcan be rejected. If the message is`not`a
handler&hyphen;addressed message, or if it has a tt_message_status other thanTT_WRN_START_MESSAGETT_WRN_START_MESSAGE, it can be rejected.

This function returnsTT_DESKTOP_ENOTSUP.
# tttk_op_stringmessage setstoolkittttk_op_stringtoolkit messagestttk_op_stringtttk_op_string
char         *tttk_op_string(    Tttk_op     op);

Thetttk_op_stringfunction returns string for the operation`op`if
successful; otherwise, this function returns zero.

Use thett_freefunction to free the string returned.Tttk_op   tttk_string_op(     const char *    opstring );

Thetttk_string_opfunction returns a string containg the operation for the
specified string. On error, this function returnsTTDT_OP_NONE.
# tttk_Xt_input_handlermessage setstoolkittttk_Xt_input_handlertoolkit messagestttk_Xt_input_handlertttk_Xt_input_handler
void         tttk_Xt_input_handler(   XtPointer     procid,
                                      int *         source,
                                      XtInputId *   id );

Thetttk_Xt_input_handlerfunction processes ToolTalk events for Xt
clients. Use this function as your Xt input handler unless you expect some
messages not to be consumed by callbacks.

This function passes theprocidargument to thetttk_message_receive functiontttk_message_receivefunction and passes any returned message (that is, messages that are not
consumed by callbacks) to thetttk_message_abandontttk_message_abandonfunction.

If this function returns the errorTT_ERR_NOMPerror messagesTT_ERR_NOMPTT_ERR_NOMP, thetttk_Xt_input_handlerfunction will pass theidparameter to theXtRemoveInput functioXtRemoveInputfunction.