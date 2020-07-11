import html
import re

abcd = 'abcdefgh'

d = [(int(l[0])*1000000 + int(l[1])*1000 + int(l[2])*10 + {abcd[i]:i for i in range(len(abcd))}[l[3]], l[4], l[5]) for l in [l.split('\t') for l in open('dcs/data').readlines()[1:]]]

t = [i for l in [re.findall('">[0-9/.a-h]{9}<.*</span', html.unescape(open('RV/RV'+str(i).zfill(2)+'.html').read())) for i in range(1, 10)] for i in l]

print(set([i[2] for i in t]))