#include <EUSCompat.h>

/*
 * Compatibility wrapper for legacy dtcm server callers.
 * The modern bison build uses _DtCm_yyparse as the parser entry.
 */
extern int _DtCm_yyparse(void);

void
_DtCm_rule_parser(void)
{
  (void)_DtCm_yyparse();
}
