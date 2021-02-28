from indic_transliteration import sanscript
import re

def visrgh(s):
    t = s
    t=re.sub(r'ḥ(?= ?s)', 's', t)
    t=re.sub(r'ḥ(?= ?ṣ)', 'ṣ', t)
    t=re.sub(r'ḥ(?= ?ś)', 'ś', t)
    return t

def iast235(s):
    t = s
    for c in "'()*3:[]~":
        t=t.replace(c, '-')
    for p in 'á:a3 à:a5 í:i3 ì:i5 ú:u3 ù:u5 ŕ̥:r̥3 é:e3 è:e5 ó:o3 ò:o5 ́:3 ̀:5 ṁ:ṃ Å:-u'.split(' '):
        ps = p.split(':')
        t=t.replace(ps[0], ps[1])
    t=t.replace('-', '')
    t=visrgh(t)
    t=sanscript.transliterate(t, sanscript.IAST, sanscript.SLP1).replace('x', 'L')
    t=re.sub(r'(?=[aiuf][35]? ?)C', 'cC', t)
    t=re.sub(r'H(?= ?[kK])', 'Z', t)
    t=re.sub(r'H(?= ?[pP])', 'V', t)
    return t

