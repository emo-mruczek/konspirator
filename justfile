default: (build "tests/test7" "dupa")

build in out:
  RUSTFLAGS=-Awarnings cargo run {{in}} {{out}}
