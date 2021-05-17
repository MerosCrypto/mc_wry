#ifndef MC_WRY_H
#define MC_WRY_H

#include <stdbool.h>

typedef struct BundleOuter {
  bool valid;
  void* bundle;
} BundleOuter;

typedef struct RpcRequest {
  const char *rpc_method;
  const char *params;
} RpcRequest;

BundleOuter new_webview(
  const char *title,
  const char *url,
  const char *(*handler)(RpcRequest request)
);

void webview_run(
  BundleOuter bundle
);

void webview_terminate(
  BundleOuter bundle
);

#endif
