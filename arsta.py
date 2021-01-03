import vedaweb

rv = vedaweb.rv()

pdsnkyah = {}

i = 0
for m in rv.ekmekm():
	i += 1
	for s in m:
		for r in s:
			P = r.strata() == 'P'
			for c in r:
				for p in c:
					pr = p.rupm()
					if not pr in pdsnkyah:
						pdsnkyah[pr] = [0, 0, set()]
					pdsnkyah[pr][int(P)] += 1
					pdsnkyah[pr][2].add(i)

def pdarsta(p):
	P = pdsnkyah[p][1]
	if not P: return -1
	return pdsnkyah[p][0]/pdsnkyah[p][1]

def ttm(p):
	return len(pdsnkyah[pr][2]) > 2

def arsta(pt):
	n = 0
	a = 0
	for p in pt:
		pr = p.rupm()
		if ttm(pr):
			n += 1
			ar = pdsnkyah[pr][0]
			if not ar: return 0
			a += pdsnkyah[pr][1]/ar
	if not a: return -1
	return n/a

def suktarsta(s):
	return arsta([p for r in s for c in r for p in c])
def rgarsta(r):
	return arsta([p for c in r for p in c])