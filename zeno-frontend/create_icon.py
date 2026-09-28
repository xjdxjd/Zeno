import struct, zlib

width = 32
height = 32
raw = b''
for _ in range(height):
    raw += b'\x00'  # filter byte
    for _ in range(width):
        raw += b'\x00\x00\x00\xff'  # RGBA black

compressed = zlib.compress(raw)

def chunk(t, d):
    return struct.pack('>I', len(d)) + t + d + struct.pack('>I', zlib.crc32(t + d) & 0xffffffff)

ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)

with open(r'C:\.WORK\WORKSPACE\Zeno\zeno-frontend\src-tauri\icons\icon.png', 'wb') as f:
    f.write(b'\x89PNG\r\n\x1a\n')
    f.write(chunk(b'IHDR', ihdr))
    f.write(chunk(b'IDAT', compressed))
    f.write(chunk(b'IEND', b''))

print('icon.png created')
