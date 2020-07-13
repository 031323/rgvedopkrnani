import html
import re
import Levenshtein
import unidecode

def marjnm(s, m):
	for n in m:
		s = s.replace(n, '')
	return s

abcd = 'abcdefgh'

def n(a, b, c, d):
	return int(a)*1000000 + int(b)*1000 + int(c)*10 + {abcd[i]:i for i in range(len(abcd))}[d]

d = [(lambda l: (n(l[0], l[1], l[2], l[3]), l[4], l[5]))(l.split('\t')) for l in open('dcs/data').readlines()[1:]]

suci = dict()

for i, e in enumerate(d):
	for w in e[2].split(' '):
		if not w in suci: suci[w] = []
		suci[w].append(i)

nre = '[0-9]{1,2}\\.[0-9]{3}\\.[0-9]{2}[a-z]'

t = [((lambda x: n(x[:-8], x[-7:-4], x[-3:-1], x[-1]))(re.findall(nre, i)[0]), re.findall('[^>]*$', i[:-6])[0], re.findall('class=["\'][^"\']*["\']', i)[0][7:-2]) for l in [re.findall(nre + '.*</span', html.unescape(marjnm(open('RV/RV'+str(i).zfill(2)+'.html').read(), ['<sup>', '</sup>']))) for i in range(1, 11)] for i in l]

def strnm(s):
	return unidecode.unidecode(marjnm(s, ' '))

def sndanm():
	i = 0
	j = 0
	a = 0
	s = 0

	th = 3
	u = 0

	while i < len(d) and j < len(t):
		if d[i][0] < t[j][0]:
			i = i + 1
		elif d[i][0] > t[j][0]:
			j = j + 1
		else:
			s1 = strnm(d[i][1])
			s2 = strnm(t[j][1])
			l = Levenshtein.distance(s1, s2)
			if l > len(s1) * 0.2:
				u = u + 1
				if u > th:
					print(str(d[i][0]) + ', ' + s1 + ', ' + s2 + ', ' + str(l))
					a = a + 1
				else: s = s + 1
			else:
				s = s + 1
				u = 0
			i = i + 1
			j = j + 1
	print(a)
	print(s)

sndanm()

print(d[suci['indra'][0]])