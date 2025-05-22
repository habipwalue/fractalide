@0x3e4f5a6b7c8d;

struct RouteInfo {
  path @0 :Text;
  handlerType @1 :HandlerType;
  params @2 :List(Param);
  
  struct Param {
    name @0 :Text;
    value @1 :Text;
  }
  
  enum HandlerType {
    staticFile @0;
    api @1;
    notFound @2;
  }
}