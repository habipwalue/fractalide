@0xf6e5d4c3b2a1;

struct OperationResult {
  union {
    success @0 :Float64;
    error @1 :Text;
  }
}