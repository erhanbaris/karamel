listem = ['merhaba', 'dünya']

hataayıklama::doğrula(listem.guncelle(1, 'erhan'), doğru)
hataayıklama::doğrula(listem.getir(0), "merhaba")
hataayıklama::doğrula(listem.getir(1), "erhan")
hataayıklama::doğrula(listem.uzunluk(), 2)
hataayıklama::doğrula(listem.guncelle(10, 'erhan'), yanlış)
