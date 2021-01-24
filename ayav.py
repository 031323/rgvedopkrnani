with open('iast') as f:
	s = f.read()
	for r in 'e:ae é:áe ai:āe aí:ā́e o:ao ó:áo au:āo aú:ā́o'.split(' '):
		p = r.split(':')
		s = s.replace(p[0], p[1])
	with open('iast2', 'w+') as f2:
		f2.write(s)