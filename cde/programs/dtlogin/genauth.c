/*
 * CDE - Common Desktop Environment
 *
 * Copyright (c) 1993-2012, The Open Group. All rights reserved.
 *
 * This file generates random authentication data for XDM authentication.
 */

#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

/* GenerateAuthData - generate random authentication data
 *
 * This function fills the provided buffer with random bytes for
 * X authentication purposes.
 */
void GenerateAuthData(char *auth, int len) {
  int fd;
  int got_entropy = 0;

  /* Try /dev/urandom first */
  fd = open("/dev/urandom", O_RDONLY);
  if (fd >= 0) {
    if (read(fd, auth, len) == len) {
      got_entropy = 1;
    }
    close(fd);
  }

  /* Fallback to pseudo-random if /dev/urandom failed */
  if (!got_entropy) {
    static int seeded = 0;
    int i;

    if (!seeded) {
      srand48(time(NULL) ^ getpid());
      seeded = 1;
    }

    for (i = 0; i < len; i++) {
      auth[i] = (char)(lrand48() & 0xFF);
    }
  }
}
