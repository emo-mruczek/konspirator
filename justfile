default: (build "tests/4.imp" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
