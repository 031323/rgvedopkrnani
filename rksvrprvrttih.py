import re

rch = [r.split('।') for r in re.findall(r'(?<=[0-9])[^\n॥]+?(?=॥)', open('tmp').read())]

def udattanth(s):
	return not re.findall(r'॑[^॒\n]*$', s)

def anudattanth(s):
	return not udattanth(s)

Uu = len([r for r in rch if len(r) > 1 and udattanth(r[-2]) and udattanth(r[-1])])
Ua = len([r for r in rch if len(r) > 1 and udattanth(r[-2]) and anudattanth(r[-1])])
Au = len([r for r in rch if len(r) > 1 and anudattanth(r[-2]) and udattanth(r[-1])])
Aa = len([r for r in rch if len(r) > 1 and anudattanth(r[-2]) and anudattanth(r[-1])])
uU = len([a for r in rch for a in r[:-1] if udattanth(a) and udattanth(r[-1])])
aU = len([a for r in rch for a in r[:-1] if udattanth(a) and anudattanth(r[-1])])
uA = len([a for r in rch for a in r[:-1] if anudattanth(a) and udattanth(r[-1])])
aA = len([a for r in rch for a in r[:-1] if anudattanth(a) and anudattanth(r[-1])])

print(f'ए॒का॒किन॑ उ॒दात्ता॑न्ता उत्तमा॒र्धाः\t{len([r for r in rch if len(r) == 1 and udattanth(r[-1])])}')
print(f'ए॒का॒किनोऽनु॑दात्तान्ता उत्तमा॒र्धाः\t{len([r for r in rch if len(r) == 1 and anudattanth(r[-1])])}')
print(f'उ॒दात्ते॑ पूर्व्या॒र्धे स॒त्यु१॒॑दात्ता॑ उत्तमा॒र्धाः\t{Uu}')
print(f'उ॒दात्ते॑ पूर्व्या॒र्धे स॒त्यनु॑दात्ता उत्तमा॒र्धाः\t{Ua}')
print(f'अनु॑दात्ते पूर्व्या॒र्धे स॒त्यु१॒॑दात्ता॑ उत्तमा॒र्धाः\t{Au}')
print(f'अनु॑दात्ते पूर्व्या॒र्धे स॒त्यनु॑दात्ता उत्तमा॒र्धाः\t{Aa}')
print(f'उ॒दात्ता॑न्त उत्तमा॒र्धे स॒त्यु१॒॑दात्ता॑न्ता उत्तमेतरा॒र्धाः\t{uU}')
print(f'उ॒दात्ता॑न्त उत्तमा॒र्धे स॒त्यनु॑दात्तान्ता उत्तमेतरा॒र्धाः\t{aU}')
print(f'अनु॑दात्त उत्तमा॒र्धे स॒त्यु१॒॑दात्ता॑न्ता उत्तमेतरा॒र्धाः\t{uA}')
print(f'अनु॑दात्त उत्तमा॒र्धे स॒त्यनु॑दात्तान्ता उत्तमेतरा॒र्धाः\t{aA}')

print(f'\nउ॒दात्ते॑ पूर्व्या॒र्धे स॒त्यु॑त्तमा॒र्धाना॑म् उदात्त॒त्वम्\t{Uu/(Uu+Ua)}')
print(f'अनु॑दात्ते पूर्व्या॒र्धे स॒त्यु॑त्तमा॒र्धाना॑म् उदात्त॒त्वम्\t{Au/(Au+Aa)}')
print(f'उ॒दात्त॑ उत्तमा॒र्धे स॒त्यु॑त्तमेतरा॒र्धाना॑म् उदात्त॒त्वम्\t{uU/(uU+aU)}')
print(f'अनु॑दात्त उत्तमा॒र्धे स॒त्यु॑त्तमेतरा॒र्धाना॑म् उदात्त॒त्वम्\t{uA/(uA+aA)}')