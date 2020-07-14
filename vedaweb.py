import lxml.etree

class krmh:
	def __iter__(self):
		self.i = 0
		return self
	def __next__(self):
		self.i += 1
		if self.i <= len(self):
			return self[self.i]
		raise StopIteration

class root:
	def __init__(self, r):
		self.r = r

class pdm(root):
	def drmani(self):
		return [s.get('value') for s in self.r.findall('.//{http://www.tei-c.org/ns/1.0}symbol')]

class crnm(krmh):
	def __init__(self, r, c):
		self.r = r
		self.c = c
	def smhita(self):
		return self.r[self.c*2-2].text
	def __getitem__(self, p):
		return pdm(self.r[self.c*2-1][p-1])
	def __len__(self):
		return len(self.r[self.c*2-1].getchildren())

class rk(krmh, root):
	def strata(self):
		return frozenset([p[2][1].text for p in self.r[1]])
	def __getitem__(self, c):
		return crnm(self.r[0], c)
	def __len__(self):
		return len(self.r[0].getchildren())

class suktm(krmh, root):
	def __getitem__(self, R):
		return rk(self.r[R])
	def __len__(self):
		return len(self.r.getchildren()) - 1

class mndlm(krmh, root):
	def __getitem__(self, s):
		return suktm(self.r[1][0][0][s-1])
	def __len__(self):
		return len(self.r[1][0][0].getchildren())

class rv(krmh):
	def __init__(self):
		self.mndlani = [None] * 10
	def __getitem__(self, m):
		if m > 0 and m < 11:
			if not self.mndlani[m-1]:
				print('mndlm ' + str(m))
				f = 'c-salt_vedaweb_tei/rv_book_' + str(m).zfill(2) + '.tei'
				mn = mndlm(lxml.etree.parse(f).getroot())
				self.mndlani[m-1] = mn
			return self.mndlani[m-1]
		else:
			raise IndexError()
	def __next__(self):
		self.i += 1
		if self.i < 11:
			return self[self.i]
		raise StopIteration
	def __len__(self):
		return 10