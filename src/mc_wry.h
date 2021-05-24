#ifndef MC_WRY_H
#define MC_WRY_H

#include <stdbool.h>

typedef struct RpcRequest {
  const char *rpc_method;
  const char *params;
} RpcRequest;

void* new_webview(
  const char *title,
  const char *url,
  const char *(*handler)(RpcRequest request)
);

void webview_run(
  void* bundle
);

void webview_terminate(
  void* bundle
);

#endif
