@0xd1e2f3a4b5c6;

struct ImageData {
  width @0 :UInt32;
  height @1 :UInt32;
  channels @2 :UInt8;  # 1 for grayscale, 3 for RGB, 4 for RGBA
  data @3 :Data;       # Raw pixel data
}