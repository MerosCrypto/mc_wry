version     = "0.3.3"
author      = "kayabaNerve (Luke Parker)"
description = "Heavily opinionated set of bindings for Wry."
license     = "MIT"

requires "nim >= 1.2.10"

installFiles = @[
  "mc_wry.nim",
  "Cargo.toml",
  "Cargo.lock"
]

installDirs = @[
  "src",
  "target"
]

requires "nim > 1.2.10"
