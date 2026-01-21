#ifndef _DtSearch_Rust_h
#define _DtSearch_Rust_h

#include <Dt/SearchE.h>

#ifdef __cplusplus
extern "C" {
#endif

int RsDtSearchInit(char *argv0, char *userid, long switches, char *config_file,
                   FILE *err_file, char ***dbnames, int *dbcount);

int RsDtSearchReinit(void); /* Rust stub ignored args for now */

int RsDtSearchQuery(USRBLK *usrblk, char *dbname, int search_type, char *date1,
                    char *date2, DtSrResult **dittolist, long *dittocount,
                    char *stems, int *stemcount);

int RsDtSearchGetKeytypes(char *dbname, int *ktcount, DtSrKeytype **keytypes);

int RsDtSearchRetrieve(char *dbname, DB_ADDR dba, char **cleartext,
                       long *clearlen, int *fzkeyi);

int RsDtSearchHighlight(char *dbname, char *cleartext, DtSrHitword **hitwords,
                        long *hitwcount, int search_type, char *stems,
                        int stemcount);

void RsDtSearchFreeResults(DtSrResult **dittolist);

char *RsDtSearchGetMessages(void);

void RsDtSearchAddMessage(char *msg);

void RsDtSearchExit(int exit_code);

#ifdef __cplusplus
}
#endif

#endif
