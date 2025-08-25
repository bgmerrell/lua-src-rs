#include <stdio.h>

// io.tmpfile() => nil
FILE *lua_tmpfile(void) { return NULL; }

/* WASI: no system(). Return -1 per POSIX "error" convention. */
int system(const char *cmd) {
  (void)cmd;
  return -1;
}
