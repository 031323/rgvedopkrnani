import os
import re

def aksrani(n):
	g = []
	m = n.replace('-', '')
	for i,c in enumerate(m):
		if not g or (c.isalpha() and (not i or m[i - 1] != '्')):
			g.append(c)
		else: g[-1] += c
	return g

def ssvrh(n, a):
	t = ''
	g = aksrani(n)
	
	for i,l in enumerate(g):
		t += l
		if i + 1 < a:
			t += '॒'
		if i == a and not l.endswith('्'):
			t += '॑'
	return t

def ssvrah(n, t):
	a = '\\{\\{sa\\-IPA\\|a\\='
	y = False
	for i in set([int(t[m.start() + len(a) - a.count('\\')]) for m in re.finditer(a, t)]):
		yield ssvrh(n, i)
		y = True
	if not y and (len(aksrani(n)) == 1 or '-' in n and not 'Category' in n):
		yield n

def pages():
	for n in sorted(os.listdir('pages'), key = lambda x:x.replace('-', '')):
		if os.path.isfile('pages/' + n):
			with open('pages/' + n) as f:
				yield((n, f.readline()[:-1], f.readline()[:-1], f.read()))

def list():
	o = open('ssvrsbdah.html', 'w')
	o.write('<title>स॒स्व॒र॒श॒ब्दाः</title><style>a{text-decoration:none;}</style><table><tr><th></th><th>Inherited</th><th>Derived</th><th></th><th></th></tr>')
	i = 0
	for (n, s1, s2, t) in pages():
		d = {'Sanskrit':'V.', 'Aryan':'A.', 'Iranian':'I.', 'European':'E.'}
		s1, s2 = d[s1], d[s2]
		for ss in ssvrah(n, t):
			i += 1
			A = []
			a1 = 'Adjective, Adverb, Ambiposition, Article, Circumposition, Classifier, Conjunction, Contraction, Counter, Determiner, Ideophone, Interjection, Noun, Numeral, Participle, Particle, Postposition, Preposition, Pronoun, Proper noun, Verb'.split(', ')
			a2 = 'Circumfix, Combining form, Infix, Interfix, Prefix, Root, Suffix, Infinitive'.split(', ')
			for a in a1 + a2:
				if '=' + a + '=' in t:
					A.append(a.upper())
			A =', '.join(A)
			td = [''] * 5
			td[0] = str(i) + '.'
			td[1] = s1
			td[2] = s2
			td[3] = '<a href="https://en.wiktionary.org/wiki/' + n + '#Sanskrit">' + ss +'</a>'
			td[4] = A
			o.write('<tr><td>' + '</td><td>'.join(td) + '</td></tr>')
	o.write('</table>')

def find(w):
	for (n, s, t) in pages():
		if w in t:
			print(n)