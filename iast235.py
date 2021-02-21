def iast235(s):
    t = s
    for c in "'()*3:[]~":
        t=t.replace(c, '-')
    for p in 'á:a3 à:a5 í:i3 ì:i5 ú:u3 ù:u5 ŕ̥:r̥3 é:e3 è:e5 ó:o3 ò:o5 ́:3 ̀:5 ṁ:ṃ Å:-u ḷh:| ḷ:ḍ'.split(' '):
        ps = p.split(':')
        t=t.replace(ps[0], ps[1])
    t=t.replace('-', '')
    t=t.replace('C', 'cC')
    return t