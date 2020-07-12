import urllib.request as ur
import os

def k(w, i):
	return w.split(' ')[i]

d = { **{ k(w, 0):k(w, 1) for w in '''

dcs/data https://raw.githubusercontent.com/OliverHellwig/sanskrit/master/dcs/data/rigveda/pada-and-analysis.dat
spoken/spoken https://github.com/indic-dict/stardict-sanskrit/raw/d39a9aa2d33b24e19faf5a33522dbefb63677563/sa-head/en-entries/spokensanskrit/archives/spokensanskrit.gls

'''.split('\n') if w }, **{ 'RV/RV' + n + '.html' : 'https://liberalarts.utexas.edu/lrc/rigveda/RV' + n + '.php' for n in [str(i).zfill(2) for i in range(1, 11)] } }

for dir in 'RV dcs spoken'.split(' '):
	if not os.path.exists(dir):
		os.makedirs(dir)

for _o in d:
	if not os.path.isfile(_o):
		with ur.urlopen(d[_o]) as f:
			with open(_o, 'w') as o:
				print(_o)
				o.write(f.read().decode('utf-8'))
				print(_o)