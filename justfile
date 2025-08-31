default: (build "tests/2.imp" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
