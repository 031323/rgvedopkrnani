from mwclient import Site

w = Site('en.wiktionary.org')

s = {}

for p in w.categories['Sanskrit terms with IPA pronunciation']:
	s[p.name] = ('Sanskrit', 'Sanskrit', p)

u = ['Aryan', 'Iranian', 'European']

for i in u:
	for p in w.categories['Sanskrit terms derived from Proto-Indo-' + i]:
		s[p.name] = ('Sanskrit', i, p)

for i in u:
	for p in w.categories['Sanskrit terms inherited from Proto-Indo-' + i]:
		s[p.name] = (i, s[p.name][1], p)

for n in s:
	print(n)
	with open('pages/' + n, 'w') as f:
		f.write(s[n][0] + '\n' + s[n][1] + '\n' + s[n][2].text())