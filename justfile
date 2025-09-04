default: (build "tests/7.imp" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
