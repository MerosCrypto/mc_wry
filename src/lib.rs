extern crate libc;

use std::{
  ptr,
  mem,
  os::raw::c_char,
  ffi::{CStr, CString}
};

use serde_json;

use wry::{
  application::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
  },
  webview::{self, WebViewBuilder, WebView}
};

pub struct Bundle {
  event_loop: EventLoop<()>,
  // Stored so the Window doesn't close immediately; allows closing later via dropping this
  #[allow(dead_code)]
  webview: WebView
}

#[repr(C)]
pub struct RpcRequest {
  rpc_method: *const c_char,
  params: *const c_char
}

fn create_bundle<F: Fn(&Window, webview::RpcRequest) -> Option<webview::RpcResponse> + 'static>(
  title: &str,
  url: &str,
  handler: F
) -> wry::Result<Bundle> {
  let event_loop: EventLoop<()> = EventLoop::new();

  let window: Window = WindowBuilder::new()
    .with_title(title)
    .build(&event_loop)?;

  let webview: WebView = WebViewBuilder::new(window)?
    .with_file_drop_handler(|_, _| true)
    .with_rpc_handler(handler)
    .with_url(url)?
    .build()?;

  Ok(
    Bundle {
      event_loop,
      webview
    }
  )
}

#[no_mangle]
pub unsafe extern "C" fn new_webview(
  title: *const c_char,
  url: *const c_char,
  handler: extern "C" fn(
    request: RpcRequest
  ) -> *const c_char
) -> *mut Bundle {
  match create_bundle(
    CStr::from_ptr(title).to_str().unwrap(),
    CStr::from_ptr(url).to_str().unwrap(),
    move |_, request| {
      let method: CString = CString::new(request.method.as_str()).unwrap();
      let params: CString = CString::new(request.params.unwrap().to_string()).unwrap();
      Some(
        webview::RpcResponse::new_result(
          request.id,
          Some(
            serde_json::from_str(
              CStr::from_ptr(
                handler(
                  RpcRequest {
                    rpc_method: method.as_ptr(),
                    params: params.as_ptr()
                  }
                )
              ).to_str().unwrap()
            ).unwrap()
          )
        )
      )
    }
  ) {
    Ok(bundle) => Box::into_raw(Box::new(bundle)),
    Err(_) => ptr::null_mut()
  }
}

fn run(
  event_loop: EventLoop<()>
) -> ! {
  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::UserEvent(_) => *control_flow = ControlFlow::Exit,
      _ => {}
    }
  })
}

#[no_mangle]
pub unsafe extern "C" fn webview_run(
  bundle: *mut Bundle
) {
  run(Box::from_raw(bundle).event_loop);
}

/*
fn terminate(
  proxy: EventLoopProxy<()>
) -> ! {
  // This SHOULD properly close the event loop and that will call exit on its own
  // That said, Rust panics when we do this, claiming it hit unreachable code
  // A pure Rust version of this model (foreign thread sent event to trigger quit) worked without issue though
  // So there's some FFI issue. Because of that, terminate solely closes the window (see below), leaving the event loop running
  proxy.send_event(()).unwrap();
}
*/

#[no_mangle]
pub unsafe extern "C" fn webview_terminate(
  bundle: *mut Bundle
) {
  mem::drop(Box::from_raw(bundle));
}
