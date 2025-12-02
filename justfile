default: (build "tests/test4" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
