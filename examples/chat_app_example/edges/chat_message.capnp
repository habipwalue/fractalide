@0xd4e5f1d2c3b2a1;

struct ChatMessage {
  userId @0 :Text;
  username @1 :Text;
  content @2 :Text;
  timestamp @3 :UInt64;
  roomId @4 :Text;
}