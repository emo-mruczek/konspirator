default: (build "tests/err02.imp" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
