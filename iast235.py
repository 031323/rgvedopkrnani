from indic_transliteration import sanscript
import re, sys
import regex

def visrgh(s):
    t = s
    t=re.sub(r'ḥ(?= ?s)', 's', t)
    t=re.sub(r'ḥ(?= ?ṣ)', 'ṣ', t)
    t=re.sub(r'ḥ(?= ?ś)', 'ś', t)
    return t

def iast235(s):
    t = s
    for c in "'()*3:[]~+@":
        t=t.replace(c, '-')
    for p in 'á:a3 à:a5 í:i3 ì:i5 ú:u3 ù:u5 ŕ̥:r̥3 é:e3 è:e5 ó:o3 ò:o5 ́:3 \\:6 ̀:5 ṁ:ṃ Å:-u'.split(' '):
        ps = p.split(':')
        t=t.replace(ps[0], ps[1])
    t=t.replace('-', '')
    t=visrgh(t)
    t=sanscript.transliterate(t, sanscript.IAST, sanscript.SLP1).replace('x', 'L')
    t=re.sub(r'(?<=[aiuf][35] )C', 'cC', t)
    t=re.sub(r'(?<=[aiuf] )C', 'cC', t)
    t=re.sub(r'(?<=[aiuf][35])C', 'cC', t)
    t=re.sub(r'(?<=[aiuf])C', 'cC', t)
    t=re.sub(r'H(?= ?[kK])', 'Z', t)
    t=re.sub(r'H(?= ?[pP])', 'V', t)
    return t

def iast2dev(s):
    t=iast235(s)
    t=t.replace('5', '')
    t=t.replace('6', '5')
    t=t.replace('Z', 'H')
    t=t.replace('V', 'H')
    t=t.replace(' ', '')
    
    t=t.split('\n\n')
    ut=[]
    for tt in t:
        tt=tt.split('\n')
        u=[tt[0]]
        for i in range(1, len(tt)):
            if not i%2:
                u.append('। ')
            u.append(tt[i])
        ut.append(''.join(u))
    t = '।\n'.join(ut) + '।'

    t=re.sub(r'o3(?=[aAiIuUfFeEoO])', 'a3v', t)
    t=re.sub(r'o5(?=[aAiIuUfFeEoO])', 'a5v', t)
    t=re.sub(r'o(?=[aAiIuUfFeEoO])', 'av', t)
    t=re.sub(r'e(?=[aAiIuUfFeEoO])', 'ay', t)
    t=re.sub(r'e3(?=[aAiIuUfFeEoO])', 'a3y', t)
    t=re.sub(r'e5(?=[aAiIuUfFeEoO])', 'a5y', t)
    t=re.sub(r'(?<=[aAiIuUfFeEoO])(?![35])', '@', t)

    t=re.sub(r'@(?=[^aAiIuUfFeEoO।]*[aAiIuUfFeEoO][35])', '॒', t) #snntrh
    t=regex.sub(r'(?<=3[^aAiIuUfFeEoO।]*[aAiIuUfFeEoO])@', '5', t) #udattadnudattsysvrith
    s=t
    while True:
        t=regex.sub(r'(?<=[5%][^aAiIuUfFeEoO।]*[aAiIuUfFeEoO])@', '%', t)
        if t!=s:
            s=t
        else:
            break
    t=t.replace('%', '')
    t=t.replace('@', '॒')
    t=t.replace('3', '')
    #t=t.replace('॒H', 'H॒')
    #t=t.replace('॒M', 'M॒')
    t=re.sub(r'5(?=[^aAiIuUfFeEoO।]*[aAiIuUfFeEoO][35])', '२॒॑', t)
    t=t.replace('5', '॑')
    return sanscript.transliterate(t, sanscript.SLP1, sanscript.DEVANAGARI)

def main():
    print(iast2dev(open(sys.argv[1]).read()))

if __name__ == "__main__":
    main()
