default: (build "tests/9.imp" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
