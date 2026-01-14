default: (build "tests/test8" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
