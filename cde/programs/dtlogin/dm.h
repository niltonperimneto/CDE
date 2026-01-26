/*
 * dm.h - Display Manager header for Dtlogin (Reconstructed v8)
 *
 * Based on xdm dm.h and Usage in Dtlogin sources.
 */

#ifndef _DM_H_
#define _DM_H_

#include <X11/Xauth.h>
#include <X11/Xdmcp.h>
#include <X11/Xfuncproto.h>
#include <X11/Xlib.h>
#include <X11/Xresource.h>
#include <errno.h>
#include <fcntl.h>
#include <limits.h>
#include <pwd.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/param.h>
#include <sys/select.h>
#include <sys/time.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>

#ifndef MAXPATHLEN
#define MAXPATHLEN 1024
#endif

#ifndef LAST_SESSION_FILE
#define LAST_SESSION_FILE "/.dt/sessions/lastsession"
#endif

#define DM_GETTY_USER 1

/* Session type notification values */
#define NOTIFY_ALT_DTS 100

/* Resource DB for X resources */
extern XrmDatabase XresourceDB;

/* Language list from util.c */
extern char languageList[2048];

#ifndef NGROUPS
#define NGROUPS 32 /* Fallback */
#endif

#ifndef NAME_MAX
#include <sys/param.h>
#endif

#define BITMASK(i) (1 << (i))

/* Typedefs for Signal Handling */
#if defined(SYSV) || defined(SVR4) || defined(__linux__)
typedef void SIGVAL;
#else
typedef int SIGVAL;
#endif

/* Wait Status Types */
#if defined(SYSV) || defined(SVR4) || defined(__linux__)
typedef int waitType;
#define waitVal(w) (w)
#else
#include <sys/resource.h>
typedef union wait waitType;
#define waitVal(w) ((w).w_status)
#endif

#define waitCompose(sig, core, status) ((sig) << 8 | (core) << 7 | (status))

typedef fd_set FD_TYPE;

/* Proto Display Structure */
struct protoDisplay {
  struct protoDisplay *next;
  struct sockaddr *address;
  int addrlen;
  CARD16 displayNumber;
  CARD16 connectionType;
  ARRAY8 connectionAddress;
  CARD32 sessionID;
  Xauth *fileAuthorization;
  Xauth *xdmcpAuthorization;
  ARRAY8 authenticationData;
  ARRAY8 key; /* DES key for XDM-AUTHENTICATION */
  time_t date;
};

#define PROTO_TIMEOUT (60 * 5) /* 5 minutes */

/* Display Types and Status */

typedef enum { Local, Foreign } DisplayLocation;

typedef enum { Permanent, Transient } DisplayLifetime;

typedef enum { FromFile, FromXDMCP } DisplayOrigin;

typedef struct {
  DisplayLocation location;
  DisplayLifetime lifetime;
  DisplayOrigin origin;
} DisplayType;

typedef enum { notRunning, running, zombie, phoenix, suspended } DisplayStatus;

typedef enum { MissingEntry, NewEntry, OldEntry } DisplayState;

/* Main Display Structure */
struct display {
  struct display *next;

  /* Display info */
  char *name;  /* Display name (e.g. :0, host:0) */
  char *class; /* Display class */
  int type;    /* DM_CONST? */
  DisplayType displayType;

  /* State */
  DisplayStatus status;
  DisplayState state;
  int pid;       /* ProcessID of session/sub-daemon */
  int serverPid; /* ProcessID of X Server */

  /* Resources matching resource.c */
  int serverAttempts;
  int openDelay;
  int openRepeat;
  int openTimeout;
  int startAttempts;
  int pingInterval;
  int pingTimeout;
  int terminateServer; /* boolean */
  int grabServer;      /* boolean */
  int grabTimeout;
  int resetSignal;
  int termSignal;
  int resetForAuth;             /* boolean */
  int authorize;                /* boolean */
  char **authNames;             /* auth protocol names */
  unsigned short *authNameLens; /* auth protocol name lengths */
  int authNameNum;              /* number of auth protocols */
  char *authFile;
  char *clientAuthFile;

  char *resources;      /* Xresources file */
  char *xrdb;           /* xrdb program path */
  char *cpp;            /* cpp program path */
  char *setup;          /* Xsetup script */
  char *startup;        /* Xstartup script */
  char *reset;          /* Xreset script */
  char *session;        /* Xsession script */
  char *userPath;       /* PATH for user session */
  char *systemPath;     /* PATH for system scripts */
  char *systemShell;    /* Shell to run scripts */
  char *failsafeClient; /* Failsafe xterm */
  char *userAuthDir;    /* Directory for user auth */
  char *chooser;        /* Chooser program */
  char *language;       /* Default language */
  char *langList;       /* Language list */
  char *environStr;     /* Environment variables */
  int dtlite;           /* boolean */
  int xdmMode;          /* boolean */
  char *verifyName;     /* Authentication Name */
  char *pmSearchPath;   /* Pixmap search path */
  char *bmSearchPath;   /* Bitmap search path */
  int useChooser;       /* boolean */

  /* Internal fields */
  char **argv;           /* server arguments */
  struct sockaddr *from; /* XDMCP origin address */
  int fromlen;
  CARD16 displayNumber;
  CARD16 connectionType;
  CARD32 sessionID;
  ARRAY8 clientAddr;
  char *utmpId;

  /* Extras inferred from usage */
  int startTries;   /* Current start attempt count */
  char *gettyLine;  /* tty device for getty */
  char *gettySpeed; /* speed for getty */

  /* Authorization data (xdmcp/auth) */
  unsigned short authorizationNameLen;
  char *authorizationName;
  unsigned short authorizationDataLen;
  char *authorizationData;

  /* Authorizations from auth.c usage */
  Xauth **authorizations;
  int authNum;
  XdmcpNetaddr peer;
  int peerlen;
  int sessionType; /* Session type from waitVal */
  int gettyState;  /* Getty state */
};

/* Structs from verify.c/auth.c */
struct greet_info {
  char *name;
  char *password;
  char *string;
};

struct verify_info {
  int uid;
  int gid;
#ifdef NGROUPS
  int groups[NGROUPS];
  int ngroups;
#endif
  char **argv;
  char **userEnviron;
  char **systemEnviron;
  long audid;
  char audflg;
};

/* Global Variables matching resource.c and dm.c */
extern int debugLevel;
extern char *config;
extern char *servers;
extern int request_port;
extern char *accessFile;
extern char *errorLogFile;
extern int errorLogSize;
extern int daemonMode;
extern char *pidFile;
extern int lockPidFile;
extern char *authDir;
extern int quiet;
extern int autoRescan;
extern int removeDomainname;
extern char *keyFile;
extern char **exportList;
extern char *sysParmsFile;
extern char *timeZone;
extern char *fpHead;
extern char *fpTail;
extern int wakeupInterval;
extern int langListTimeout;
extern struct passwd puser;

extern struct display *FindDisplayByPid(int pid);
extern struct display *FindDisplayByServerPid(int pid);
extern struct display *FindDisplayBySessionID(CARD32 sessionID);
extern void StartDisplays(void);
extern void StopDisplay(struct display *d);
extern int StartDisplay(struct display *d);
extern void RemoveDisplay(struct display *d);
extern void ForEachDisplay(void (*f)(struct display *));
extern struct display *NewDisplay(char *name, char *class);
extern int AnyDisplaysLeft(void);
extern int addressEqual(char *a1, int l1, char *a2, int l2);

/* Function Prototypes */
extern void Debug(char *fmt, ...);
extern void LogInfo(unsigned char *fmt, ...);
extern void LogError(unsigned char *fmt, ...);
extern void LogPanic(unsigned char *fmt, ...);
extern void LogOutOfMem(unsigned char *fmt, ...);

extern int AnyWellKnownSockets(void);
extern int CreateWellKnownSockets(void);
extern void DestroyWellKnownSockets(void);
extern void WaitForSomething(void);

extern void CleanUpChild(void);
extern void CheckErrorFile(void);
extern void InitErrorLog(void);
extern void TrimErrorFile(void);
extern int SyncErrorFile(int force);
extern void BecomeDaemon(void);
extern void SetAuthorization(struct display *d);
extern int SetTitle(char *name, char *arch);
extern char **setEnv(char **e, char *name, char *value);

/* Server management */
extern int StartServer(struct display *d);
extern int SaveServerAuthorizations(struct display *d, Xauth **auths, int cnt);
extern void LoadDisplayResources(struct display *d);
extern void SetLocalAuthorization(struct display *d);
extern void SendFailed(struct display *d, char *msg);
extern int DisposeProtoDisplay(struct protoDisplay *pdpy);

/* Utility functions */
extern char **parseArgs(char **argv, char *string);
extern char *localHostname(void);
extern char *NetworkAddressToHostname(CARD16 connectionType,
                                      ARRAY8Ptr connectionAddress);

extern unsigned char *ReadCatalog(int set, int msg, char *def);

/* Resource handling */
extern void InitResources(int argc, char **argv);
extern void LoadDMResources(void);
extern void ReinitResources(void);
extern int ScanAccessDatabase(void);
extern int ScanServers(void);
extern void MakeLangList(void);
extern int ParseDisplay(char *source, DisplayType *acceptableTypes,
                        int numTypes, struct passwd *puser);
extern struct display *FindDisplayByName(char *name);
extern int WaitForServer(struct display *d);
extern void WaitForChild(void);
extern void SetAppName(void);
extern int GettyRunning(struct display *d);
extern void GettyMessage(struct display *d, int msgnum);
extern void RegisterCloseOnFork(int fd);
extern int ConvertAddr(char *addr, int *len, char **str);
extern void ManageSession(struct display *d);
/* extern void CloseOnFork(int fd); */

/* Session/Environment functions */
extern char **parseEnv(char **e, char *string);
extern char **systemEnv(struct display *d, char *user, char *home);
extern char *getEnv(char **e, char *name);
extern char **setLang(struct display *d, char **e, char *lang);
extern void printEnv(char **e);
extern int source(struct verify_info *verify, char *file);
extern void pseudoReset(Display *dpy);
extern void SetHourGlassCursor(Display *dpy, Window window);
extern int PingServer(struct display *d, Display *dpy);
extern void ResetServer(struct display *d);
extern void SetUserAuthorization(struct display *d, struct verify_info *verify);
extern int execute(char **argv, char **env);
extern int LoadXloginResources(struct display *d);
extern void getGroups(char *name, struct verify_info *verify, int gid);
extern char **setDt(struct display *d, char **argv, int notify_dt);
extern char **userEnv(struct display *d, char *user, char *home, char *shell,
                      struct passwd *p);

typedef void (*ChooserFunc)(CARD16 connectionType, ARRAY8Ptr addr,
                            char *closure);
extern void ForEachChooserHost(ARRAY8Ptr addr, CARD16 connectionType,
                               ChooserFunc function, char *closure);

/* XDMCP/Chooser related */
extern void init_session_id(void);
extern int XdmCheckAuthentication(struct protoDisplay *pdpy,
                                  ARRAY8Ptr displayID, ARRAY8Ptr name,
                                  ARRAY8Ptr data);
extern int ValidAuthorization(unsigned short nameLen, char *name);
extern int AcceptableDisplayAddress(ARRAY8Ptr clientAddress,
                                    CARD16 connectionType, xdmOpCode type);

/* Utility/Session helpers */
extern int CloseOnFork(void);
extern int waitCode(int w);
extern int waitSig(int w);
/* Note: Verify is declared in vg.h for legacy dtgreet compatibility.
 * Files that need the session.c version (Verify(display*, greet*, verify*))
 * should add a local declaration. */
extern int Willing(ARRAY8Ptr addr, CARD16 connectionType, ARRAY8Ptr auth,
                   ARRAY8Ptr status, xdmOpCode type);
extern int ForEachMatchingIndirectHost(ARRAY8Ptr clientAddress,
                                       CARD16 connectionType, ChooserFunc func,
                                       char *closure);
extern struct protoDisplay *FindProtoDisplay(struct sockaddr *address,
                                             int addrlen, CARD16 displayNumber);
extern struct protoDisplay *NewProtoDisplay(struct sockaddr *from, int fromlen,
                                            CARD16 displayNumber,
                                            CARD16 connectionType,
                                            ARRAY8Ptr clientAddress,
                                            CARD32 sessionID);
extern ARRAY8Ptr Accept(struct sockaddr *from, int fromlen,
                        CARD16 displayNumber);
extern int UseChooser(ARRAY8Ptr clientAddress, CARD16 connectionType);
extern int SelectAuthorizationTypeIndex(ARRAY8Ptr auth,
                                        ARRAYofARRAY8Ptr authTypes);
extern int CheckAuthentication(struct protoDisplay *pdpy, ARRAY8Ptr displayID,
                               ARRAY8Ptr name, ARRAY8Ptr data);
extern void SetProtoDisplayAuthorization(struct protoDisplay *pdpy,
                                         unsigned short authType,
                                         char *authData);
extern struct display *FindDisplayByAddress(struct sockaddr *addr, int addrlen,
                                            CARD16 displayNumber);

extern void MitInitAuth(unsigned short name_len, char *name);
extern Xauth *MitGetAuth(unsigned short name_len, char *name);
extern void XdmInitAuth(unsigned short name_len, char *name);
extern Xauth *XdmGetAuth(unsigned short name_len, char *name);
extern void XdmGetXdmcpAuth(struct protoDisplay *pdpy, unsigned short name_len,
                            char *name);

#define UNMANAGE_DISPLAY 12
#define OBEYSESS_DISPLAY 13
#define REMANAGE_DISPLAY 14
#define OPENFAILED_DISPLAY 15
#define RESERVER_DISPLAY 16
#define SUSPEND_DISPLAY 17

#define LANGUAGESIZE 256

#ifndef FALSE
#define FALSE 0
#endif
#ifndef TRUE
#define TRUE 1
#endif

#endif /* _DM_H_ */
