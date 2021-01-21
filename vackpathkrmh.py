from indic_transliteration import sanscript
import vedaweb
rv = vedaweb.rv()

path=[]

for m in rv.ekmekm():
	for s in m:
		def sodh(p):
			q = p
			for c in '॒॑' + ''.join([str(i) for i in range(9)]):
				q = q.replace(c,'')
			return q.replace('fgvedaH maRqalaM ','').strip()
		path.append('\n'.join([sodh(sanscript.transliterate(r.source('gasuns'), sanscript.DEVANAGARI, sanscript.SLP1)) for r in s]))

class n_gramah:
	def __init__(self, mhttmm_n):
		self.gramh = []
		for i in range(mhttmm_n):
			self.gramh.append({})
	
	def pathgramah(self, n, s):
		for l in range(len(s) - n + 1):
			w = s[l:l+n]
			if not [c for c in w if c in ' \n\t-']:
				yield w
	
	def gramsngrhnm(self, s):
		for i in range(len(self.gramh)):
			for w in self.pathgramah(i + 1, s):
				if w in self.gramh[i]:
					self.gramh[i][w] += 1
				else: self.gramh[i][w] = 1
	
	def nvgramah(self, n, m, path, ptitani):
		i = n - 1
		nvtvm = lambda x: len(set([w for w in self.pathgramah(n, path[x]) if not w in self.gramh[i] or self.gramh[i][w] == m]))/len(path[x])
		si = max([i for i in range(len(path)) if not i in ptitani], key = nvtvm)
		nv = nvtvm(si)
		self.gramsngrhnm(path[si])
		return (si, nv)
		
def krmh():
	g = n_gramah(2)
	ptitani = []
	m = 0
	while len(ptitani) < len(path):
		nvtvm = 1
		kincidgrhitm = False
		while nvtvm:
			(si, nvtvm) = g.nvgramah(2, m, path, ptitani)
			if nvtvm:
				kincidgrhitm = True
				ptitani.append(si)
				print(path[si])
				print(m)
				print(len(g.gramh[1].keys()))
		if not kincidgrhitm:
			return
		m += 1

krmh()
