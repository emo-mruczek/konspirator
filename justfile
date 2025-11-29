default: (build "tests/test1" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
