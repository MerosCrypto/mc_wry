static:
  const currentFolder: string = currentSourcePath().substr(0, high(currentSourcePath()) - 10)
  {.passC: "-I" & currentFolder & "src/".}
  {.passL: "-L" & currentFolder & "target/release/".}
  {.passL: "-lmc_wry".}

  #See https://github.com/MerosCrypto/mc_webview/commit/0554545321b66252ef1bc139f9a5f4794de11ae2 for why.
  discard staticExec "cargo build --release --manifest-path=" & currentFolder & "Cargo.toml"

  when defined(linux):
    #Required so the string is considered static.
    const libs: string = staticExec "pkg-config --libs gtk+-3.0 webkit2gtk-4.0 gtksourceview-3.0"
    {.passL: libs.}
    {.passL: "-lm".}
  elif defined(macosx):
    {.passL: "-framework WebKit".}

  when not defined(Windows):
    {.passL: "-lpthread".}

{.push header: "mc_wry.h".}

type
  WebView* = pointer

  RpcRequest* {.importc: "RpcRequest".} = object
    rpc_method*: cstring
    params*: cstring

proc newWebView*(
  title: cstring,
  url: cstring,
  rpc: proc (
    req: RpcRequest
  ): cstring {.cdecl.}
): WebView {.importc: "new_webview".}

proc run*(
  w: WebView
) {.importc: "webview_run".}

proc terminate*(
  w: WebView
) {.importc: "webview_terminate".}

{.pop.}
