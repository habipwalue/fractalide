@0x9a8b7c6d5e4f;

struct HttpRequest {
  method @0 :Method;
  path @1 :Text;
  headers @2 :List(Header);
  body @3 :Data;
  
  struct Header {
    name @0 :Text;
    value @1 :Text;
  }
  
  enum Method {
    get @0;
    post @1;
    put @2;
    delete @3;
    head @4;
    options @5;
    patch @6;
  }
}