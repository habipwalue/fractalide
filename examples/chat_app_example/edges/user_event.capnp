@0xc3b2a1d4e5f1d2;

struct UserEvent {
  userId @0 :Text;
  username @1 :Text;
  eventType @2 :EventType;
  roomId @3 :Text;
  timestamp @4 :UInt64;
}

enum EventType {
  join @0;
  leave @1;
  typing @2;
}