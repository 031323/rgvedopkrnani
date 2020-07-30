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
	if not P: return 'annta'
	return pdsnkyah[p][0]/pdsnkyah[p][1]

def ttm(p):
	return len(pdsnkyah[pr][2]) > 1

def rgarsta(r):
	n = 0
	a = 0
	for c in r:
		for p in c:
			pr = p.rupm()
			if ttm(pr):
				n += 1
				ar = pdsnkyah[pr][0]
				if not ar: return 0
				a += pdsnkyah[pr][1]/ar
	if not a: return 'annta'
	return n/a

ma = [None] * 10

i = 0
for m in rv.ekmekm():
	n = 0
	ar = 0
	for s in m:
		for r in s:
			n += 1
			if rgarsta(r) == 'annta':
				print(r.source('vnh'))
			else: ar += rgarsta(r)
	ma[i] = ar/n
	i += 1

print([(m, ma[m-1]) for m in sorted(range(1, 11), reverse = True, key = lambda x: ma[x-1])])