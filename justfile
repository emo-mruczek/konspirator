default: (build "tests/test9" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
