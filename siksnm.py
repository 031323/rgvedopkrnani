import vedaweb

rv = vedaweb.rv()

def sngrhh():
	ss = []
	mi = 0
	for m in rv.ekmekm():
		mi += 1
		for si, s in enumerate(m):
			if s[1].strata() == 'A':
				ss.append((mi, si, '\n'.join([c.smhita() for r in s for c in r]), [p.mulm() for r in s for c in r for p in c]))
	return ss

def apricyh(s, pricitani):
	return len(set([p for p in s[3] if not p in pricitani]))#/len(s[3])

def pricykrmh(ss):
	pricitani = set()
	s = ss
	t = []
	for i in range(len(s)):
		i = min(range(len(s)), key = lambda x: apricyh(s[x], pricitani))
		t.append(s[i])
		#pricitani = pricitani.union(set(s[i][3]))
		s = s[:i] + s[i+1:]
	return t