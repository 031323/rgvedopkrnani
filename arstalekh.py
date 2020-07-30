import arsta
import vedaweb
from PIL import Image

rv = vedaweb.rv()

a = [[] for i in range(10)]

i = 0
prardym = 0
avrardym = 0
for m in rv.ekmekm():
	for s in m:
		for r in s:
			ar = arsta.rgarsta(r)
			prardym = max(prardym, ar)
			avrardym = (min(ar, avrardym) if avrardym else ar) if ar else avrardym
			a[i].append(ar)
	i += 1

l = []
s = sorted([ar for m in a for ar in m if ar > 0])
for i, ar in enumerate(s):
	if i >= (len(l) + 1) * len(s) / 255 + 1:
		l.append(ar)

def vrnh(f):
	al = [i for i in range(254) if l[i] > f]
	c = 1 + min(al) if al else 255
	return (c, c, c, 255)

im= Image.new('RGBA', (max([len(m) for m in a]), 490), '#00000000')
p = im.load()
for mi, m in enumerate(a):
	for x, ar in enumerate(m):
		for i in range(40):
			print(str(x) + ' ' + str(mi) + ' ' + str(i))
			p[x, mi * 50 + i] = vrnh(ar) if ar > 0 else (vrnh(prardym) if ar else (0, 0, 0, 255))

im.save('arstalekh.png', 'PNG')