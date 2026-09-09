from pathlib import Path
from PIL import Image, ImageDraw, ImageFont
import math

ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'src-tauri'/'icons'
OUT.mkdir(parents=True,exist_ok=True)
SIZE=1024
image=Image.new('RGBA',(SIZE,SIZE),(8,9,10,255))
draw=ImageDraw.Draw(image)
center=SIZE/2
for index in range(860):
    phi=math.acos(1-2*(index+.5)/860)
    theta=math.pi*(1+math.sqrt(5))*index
    x=center+math.sin(phi)*math.cos(theta)*340
    y=center+math.cos(phi)*340
    z=math.sin(phi)*math.sin(theta)
    alpha=int(42+(z+1)*76)
    radius=1.3+(z+1)*1.2
    draw.ellipse((x-radius,y-radius,x+radius,y+radius),fill=(215,235,241,alpha))
draw.ellipse((172,172,852,852),outline=(255,255,255,36),width=2)
font_paths=['C:/Windows/Fonts/segoeuib.ttf','/usr/share/fonts/truetype/msttcorefonts/Arial_Bold.ttf','/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf']
font=None
for path in font_paths:
    try:
        font=ImageFont.truetype(path,235)
        break
    except OSError:
        pass
if font is None: font=ImageFont.load_default()
text='OB'
box=draw.textbbox((0,0),text,font=font,stroke_width=0)
tw,th=box[2]-box[0],box[3]-box[1]
draw.rounded_rectangle((center-tw/2-48,center-th/2-42,center+tw/2+48,center+th/2+58),radius=28,fill=(9,11,13,226),outline=(139,185,202,100),width=3)
draw.text((center-tw/2,center-th/2-20),text,font=font,fill=(247,248,248,255))
for size,name in [(32,'32x32.png'),(128,'128x128.png'),(256,'128x128@2x.png'),(512,'icon.png')]:
    image.resize((size,size),Image.Resampling.LANCZOS).save(OUT/name)
image.save(OUT/'icon.ico',sizes=[(16,16),(24,24),(32,32),(48,48),(64,64),(128,128),(256,256)])
print('generated',len(list(OUT.iterdir())),'icons')
