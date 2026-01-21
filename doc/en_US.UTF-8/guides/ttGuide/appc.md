
# Writing Thread-Safe ToolTalk Applications
thread-safe ToolTalk applicationswriting

The multithread-safe ToolTalk
library enables application developers to write
multithreaded applications for ToolTalk without having to manage
locks on ToolTalk resources explicitly in their application code.
An extended version of theXthreads.hthread API wrappers
is used; therefore, multithreaded libtt should be readily
portable to various thread implementations.To protect internal libtt resources, a process-wide lock is introduced and
maintained. Unlike the case of window toolkits, few applications
spend a significant amount of their time in libtt, so the added
parallelism of more fine-grained locking techniques would not
result in a perceptible performance improvement.A few ToolTalk global values, such as the defaultprocidand the
storage stack manipulated by thett_markandtt_releaseAPI
calls, must have a consistent state as seen by one thread across
ToolTalk API calls &mdash these values are made into thread-specific data.
Additional API calls are introduced to initialize this behavior
and manipulate the new data. When a thread-specific value has been
set for the current thread, existing API calls are modified
to reference the thread-specific values instead of the
process-wide values. If no thread-specific value
has been set for the current thread, then the process-wide value
is used.The multithreaded feature may not be
available on all platforms. For this reason, an API for querying for the
existence of the feature is included in the library. The intent is
that even platforms that do not enable the multithreaded
feature should implement the new API calls, returning`TT_ERR_UNIMP`for the thread-specific ones; this
allows application developers to do run-time checks without
having to deal with the problem of unresolved symbols.Following is a summary of the API:tt_feature_enabled&mdash determine whether the specified feature has been enabled.thread-safe ToolTalk applicationstt_feature_enabledtt_feature_enabled functiontt_feature_required&mdash declare that the specified feature is required, and enable it if it is available.thread-safe ToolTalk applicationstt_feature_requiredtt_feature_required functiontt_thread_session_set&mdash set the default session for this thread.thread-safe ToolTalk applicationstt_thread_session_settt_thread_session_set functiontt_thread_session&mdash fetch the default session for this thread.thread-safe ToolTalk applicationstt_thread_sessiontt_thread_session functiontt_thread_procid_set&mdash set the default process ID for this thread.thread-safe ToolTalk applicationstt_thread_procid_settt_thread_procid_set functiontt_procid_session&mdash return the session associated with the specified process ID.thread-safe ToolTalk applicationstt_procid_sessiontt_procid_session functionTttt_c&mdash typedefs and enums.thread-safe ToolTalk applicationsTttt_cTttt_c typdef/enum declarations