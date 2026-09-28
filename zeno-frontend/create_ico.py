import struct, zlib

def create_ico():
    width = 32
    height = 32
    
    # Create PNG data for the icon
    raw = b''
    for _ in range(height):
        raw += b'\x00'  # filter byte
        for _ in range(width):
            raw += b'\x11\x44\x88\xff'  # RGBA blue-ish color
    
    compressed = zlib.compress(raw)
    
    def chunk(t, d):
        return struct.pack('>I', len(d)) + t + d + struct.pack('>I', zlib.crc32(t + d) & 0xffffffff)
    
    ihdr = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)
    
    png_data = b'\x89PNG\r\n\x1a\n'
    png_data += chunk(b'IHDR', ihdr)
    png_data += chunk(b'IDAT', compressed)
    png_data += chunk(b'IEND', b'')
    
    # ICO format
    # ICONDIR header
    ico = struct.pack('<HHH', 0, 1, 1)  # reserved, type=1 (icon), count=1
    # ICONDIRENTRY
    ico += struct.pack('<BBBBHHII', width, height, 0, 0, 1, 32, len(png_data), 22)
    # PNG data
    ico += png_data
    
    with open(r'C:\.WORK\WORKSPACE\Zeno\zeno-frontend\src-tauri\icons\icon.ico', 'wb') as f:
        f.write(ico)
    
    print('icon.ico created')

create_ico()
