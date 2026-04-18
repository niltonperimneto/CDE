/*
 * CDE - Common Desktop Environment
 *
 * Minimal audit implementation to satisfy legacy ToolTalk callers.
 */
#include <util/tt_audit.hpp>
#include <util/tt_trace.hpp>

Tt_status _Tt_audit::entry(const char *argskey, _Tt_entry_pt func, ...)
{
    va_list ap;

    va_start(ap, func);
    _Tt_trace::entry(argskey, func, ap);
    va_end(ap);

    return TT_OK;
}
