@0x4f5e6d7c8b9a;

struct HttpResponse {
  statusCode @0 :UInt16;
  statusText @1 :Text;
  headers @2 :List(Header);
  body @3 :Data;
  
  struct Header {
    name @0 :Text;
    value @1 :Text;
  }
}