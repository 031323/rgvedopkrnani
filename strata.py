import vedaweb
import sys

rv = vedaweb.rv()

ar = sys.argv

l = len(ar)

if l == 3:
	print(''.join([r.strata() for r in rv[int(ar[1])][int(ar[2])]]))

elif l==4:
	print(rv[int(ar[1])][int(ar[2])][int(ar[3])])