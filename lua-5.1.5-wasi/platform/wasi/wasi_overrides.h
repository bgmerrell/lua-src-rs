#include <stdio.h>

// Our stub is provided by platform/wasi/wasi_stubs.c
extern FILE *lua_tmpfile(void);

// Redirect tmpfile() to our stub
#undef tmpfile
#define tmpfile lua_tmpfile

// No tmpnam/tmpfile on WASI → provide constants/stubs
#undef L_tmpnam
#define L_tmpnam 256

#undef LUA_TMPNAMBUFSIZE
#define LUA_TMPNAMBUFSIZE 256

#undef lua_tmpnam
#define lua_tmpnam(b, e)                                                       \
  do {                                                                         \
    (void)(b);                                                                 \
    (e) = 1;                                                                   \
  } while (0)
