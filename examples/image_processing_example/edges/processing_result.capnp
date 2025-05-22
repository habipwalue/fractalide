@0xb5c6d1e2f3a4;

struct ProcessingResult {
  union {
    success @0 :Void;
    error @1 :Text;
  }
}