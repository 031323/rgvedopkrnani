import re

with open('arsgrdr.krmh', 'w+') as f:
  f.write('आ॒र्ष॒गृ॒ध्र॒क्र॒मः\n' + '\n'.join([a.split('<')[0] + '.' + str(i) for a in re.findall('[0-9]+\.[0-9]+</a> \([0-9]+', open("arsgrdrkrmh.html").read()) for i in range(1, 1 + int(a.split('(')[-1]))]))

