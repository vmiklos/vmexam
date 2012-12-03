import struct
# 0xdeadbeef on bigendian (e.g. ppc), 0xefbeadde on little endian (e.g. x86)
print hex(struct.unpack("=I", "\xde\xad\xbe\xef")[0])
