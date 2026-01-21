#ifndef _AGENT_H_RPCGEN
#define _AGENT_H_RPCGEN
#include "cm.h"
#include "libcsa_xdr.h"
#include <csa/csa.h>
#include <rpc/rpc.h>

#define AGENTVERS 1

#define update_callback 1

/* _DtCm_xdr_Update_Status is used in agent.c.
   libcsa_xdr.h provides xdr_Update_Status.
   We map the CDE name to the exported Rust name.
*/
#define _DtCm_xdr_Update_Status xdr_Update_Status

struct conn;
struct Table_Res_4;
extern enum Update_Status *_DtCm_update_callback_1(struct Table_Res_4 *,
                                                   struct conn *);
/* Forward declaration for the server function */
/* Based on usage in agent.c which casts it to (char *(*)(char *)) */
/* But usually it matches the generated signature */
/* We will just declare it here if needed, or rely on agent.c being fixed?
   Actually agent.c is existing code. We should provide the header it expects.
   rpcgen generated header usually declares the server stub?
   No, registerrpc takes a function pointer.
   The function is defined in agent.c. If it's used before definition, agent.c
   should declare it. But if it relied on it being in agent.h... Let's check
   agent.c content.
*/
#endif
