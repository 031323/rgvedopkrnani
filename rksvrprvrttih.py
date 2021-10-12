import re

rch = [r.split('।') for r in re.findall(r'[0-9]+\.[0-9]+\.[0-9]+[^\n॥]+?(?=॥)', open('tmp').read())]

def udattanth(s):
	return not re.findall(r'॑[^॒\n]*$', s)

def anudattanth(s):
	return not udattanth(s)

u = [r for r in rch if len(r) == 1 and udattanth(r[-1])]
a = [r for r in rch if len(r) == 1 and anudattanth(r[-1])]

Uu = [r for r in rch if len(r) > 1 and udattanth(r[-2]) and udattanth(r[-1])]
Ua = [r for r in rch if len(r) > 1 and udattanth(r[-2]) and anudattanth(r[-1])]
Au = [r for r in rch if len(r) > 1 and anudattanth(r[-2]) and udattanth(r[-1])]
Aa = [r for r in rch if len(r) > 1 and anudattanth(r[-2]) and anudattanth(r[-1])]
uU = [a for r in rch for a in r[:-1] if udattanth(a) and udattanth(r[-1])]
aU = [a for r in rch for a in r[:-1] if anudattanth(a) and udattanth(r[-1])]
uA = [a for r in rch for a in r[:-1] if udattanth(a) and anudattanth(r[-1])]
aA = [a for r in rch for a in r[:-1] if anudattanth(a) and anudattanth(r[-1])]

def nidrsnm(a):
	def snkya(s):
		for i, d in enumerate('०१२३४५६७८९'):
			s = s.replace(str(i), d)
		return s
	return '\n' + "। ".join([re.sub(r"[0-9\.]*", '', b) for b in a[0]]) + '॥ ' + snkya(re.findall(r'[0-9\.]*', a[0][0])[0].replace('.', '॰')) + '॥\n'


print(f'ए॒का॒किन॑ उ॒दात्ता॑न्ता उत्तमा॒र्धाः\t{len(u)}' + nidrsnm(u))
print(f'ए॒का॒किनोऽनु॑दात्तान्ता उत्तमा॒र्धाः\t{len(a)}' + nidrsnm(a))
print(f'उ॒दात्ते॑ पूर्व्या॒र्धे स॒त्यु१॒॑दात्ता॑ उत्तमा॒र्धाः\t{len(Uu)}' + nidrsnm(Uu))
print(f'उ॒दात्ते॑ पूर्व्या॒र्धे स॒त्यनु॑दात्ता उत्तमा॒र्धाः\t{len(Ua)}' + nidrsnm(Ua))
print(f'अनु॑दात्ते पूर्व्या॒र्धे स॒त्यु१॒॑दात्ता॑ उत्तमा॒र्धाः\t{len(Au)}' + nidrsnm(Au))
print(f'अनु॑दात्ते पूर्व्या॒र्धे स॒त्यनु॑दात्ता उत्तमा॒र्धाः\t{len(Aa)}' + nidrsnm(Aa))
print(f'उ॒दात्ता॑न्त उत्तमा॒र्धे स॒त्यु१॒॑दात्ता॑न्ता उत्तमेतरा॒र्धाः\t{len(uU)}' + nidrsnm([r for r in rch if [a for a in uU if a in r]]))
print(f'उ॒दात्ता॑न्त उत्तमा॒र्धे स॒त्यनु॑दात्तान्ता उत्तमेतरा॒र्धाः\t{len(aU)}' + nidrsnm([r for r in rch if [a for a in aU if a in r]]))
print(f'अनु॑दात्त उत्तमा॒र्धे स॒त्यु१॒॑दात्ता॑न्ता उत्तमेतरा॒र्धाः\t{len(uA)}' + nidrsnm([r for r in rch if [a for a in uA if a in r]]))
print(f'अनु॑दात्त उत्तमा॒र्धे स॒त्यनु॑दात्तान्ता उत्तमेतरा॒र्धाः\t{len(aA)}' + nidrsnm([r for r in rch if [a for a in aA if a in r]]))

print(f'\nउ॒दात्ते॑ पूर्व्या॒र्धे स॒त्यु॑त्तमा॒र्धाना॑म् उदात्त॒त्वम्\t{len(Uu)/(len(Uu)+len(Ua))}')
print(f'अनु॑दात्ते पूर्व्या॒र्धे स॒त्यु॑त्तमा॒र्धाना॑म् उदात्त॒त्वम्\t{len(Au)/(len(Au)+len(Aa))}')
print(f'उ॒दात्त॑ उत्तमा॒र्धे स॒त्यु॑त्तमेतरा॒र्धाना॑म् उदात्त॒त्वम्\t{len(uU)/(len(uU)+len(aU))}')
print(f'अनु॑दात्त उत्तमा॒र्धे स॒त्यु॑त्तमेतरा॒र्धाना॑म् उदात्त॒त्वम्\t{len(uA)/(len(uA)+len(aA))}')