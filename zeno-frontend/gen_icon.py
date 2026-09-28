from PIL import Image, ImageDraw
import os

def create_icon(size):
    scale = 8
    big = size * scale
    img = Image.new('RGBA', (big, big), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    margin = big // 8
    radius = big // 5
    draw.rounded_rectangle([margin, margin, big - margin, big - margin], radius=radius, fill=(35, 47, 62))
    
    lock_w = big // 3
    lock_h = big // 4
    lock_x = (big - lock_w) // 2
    lock_y = big // 2
    lock_r = big // 16
    draw.rounded_rectangle([lock_x, lock_y, lock_x + lock_w, lock_y + lock_h], radius=lock_r, fill=(255, 255, 255))
    
    sw = int(lock_w * 0.55)
    sh = int(lock_h * 0.6)
    sx = (big - sw) // 2
    sy = lock_y - sh + big // 20
    arc_w = max(2, big // 28)
    draw.arc([sx, sy, sx + sw, sy + sh], start=180, end=0, fill=(255, 255, 255), width=arc_w)
    draw.rectangle([sx + 1, sy + sh // 2, sx + 1 + arc_w, lock_y + 1], fill=(255, 255, 255))
    draw.rectangle([sx + sw - 1 - arc_w, sy + sh // 2, sx + sw - 1, lock_y + 1], fill=(255, 255, 255))
    
    kcx = big // 2
    kcy = lock_y + lock_h // 2 - big // 40
    kr = big // 20
    draw.ellipse([kcx - kr, kcy - kr, kcx + kr, kcy + kr], fill=(35, 47, 62))
    kw = int(kr * 0.7)
    kh = int(kr * 1.5)
    draw.rounded_rectangle([kcx - kw, kcy + kr // 2, kcx + kw, kcy + kr // 2 + kh], radius=kw // 2, fill=(35, 47, 62))
    
    return img.resize((size, size), Image.LANCZOS)

icons_dir = r'C:\.WORK\WORKSPACE\Zeno\zeno-frontend\src-tauri\icons'

base = create_icon(512)
base.save(os.path.join(icons_dir, 'icon.png'), 'PNG')
base.save(os.path.join(icons_dir, 'icon.ico'), format='ICO', sizes=[(16,16),(32,32),(48,48),(64,64),(128,128),(256,256),(512,512)])
create_icon(32).save(os.path.join(icons_dir, '32x32.png'), 'PNG')
create_icon(128).save(os.path.join(icons_dir, '128x128.png'), 'PNG')
create_icon(256).save(os.path.join(icons_dir, '128x128@2x.png'), 'PNG')
print('Done - 512x512 source with 8x supersampling')
