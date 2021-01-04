import vedaweb

rv = vedaweb.rv()

def sngrhh():
	ss = []
	mi = 0
	for m in rv.ekmekm():
		mi += 1
		for si, s in enumerate(m):
			if 1:#s[1].strata() == 'A':
				ss.append((mi, si, '\n'.join([c.smhita() for r in s for c in r]), [p.mulm() for r in s for c in r for p in c], len(s), ))
	return ss

def apricyh(s, pricitani):
	return len(set([p for p in s[3] if not p in pricitani]))#/len(s[3])

def pricykrmh(ss):
	pricitani = set()
	s = ss
	t = []
	for i in range(len(s)):
		i = min(range(len(s)), key = lambda x: apricyh(s[x], pricitani))
		t.append(s[i]+tuple([apricyh(s[i], pricitani)]))
		pricitani = pricitani.union(set(s[i][3]))
		s = s[:i] + s[i+1:]
	return t

def grdrkrmh(ss, fn, t):
	with open(fn, 'w+') as f:
		f.write(f'<html><head><title>{t}</title></head><body>')
		nirdesh = lambda x, y, z, i: f'{i}. <a href="https://vedaweb.uni-koeln.de/rigveda/view/id/{x}.1">{x}</a> ({y}) ({z})'
		f.write('<br>'.join([nirdesh(f'{s[0]}.{s[1]+1}', s[4], s[5], si+1) for si, s in enumerate(ss)]))
		f.write('</body></html>')

ss = sngrhh()
grdrkrmh(pricykrmh(ss), 'grdrkrmh.html', "गृ॒ध्र॒क्र॒मः")
