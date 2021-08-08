isim = "erhan"
isim[0] = "E"
hataayıklama::doğrula(isim, "Erhan")

isim[5] = "E"
hataayıklama::doğrula(isim, "Erhan")

soyisim = "barış"
soyisim[0] = "B"
soyisim[3] = "i"
soyisim[4] = "s"
hataayıklama::doğrula(soyisim, "Baris")

soyisim[-1] = "!"
hataayıklama::doğrula(soyisim, "Baris")

soyisim[10] = "s"
hataayıklama::doğrula(soyisim, "Baris")

hataayıklama::doğrula("1024".sayi(), 1024)
hataayıklama::doğrula("1024.1".sayi(), 1024.1)
hataayıklama::doğrula("a1024.1".sayi(), boş)
hataayıklama::doğrula("erhan".sayi(), boş)
hataayıklama::doğrula("+123".sayi(), 123)
hataayıklama::doğrula("-123".sayi(), -123)

hataayıklama::doğrula("1 Ocak\"ta işlerim var".uzunluk(), 22)
hataayıklama::doğrula('1 Ocak\'ta işlerim var'.uzunluk(), 22)

cümle1 = "1 Ocak\"ta işlerim var"
hataayıklama::doğrula(cümle1[cümle1.uzunluk() -1], 'r')

cümle2 = '1 Ocak\'ta işlerim var'
hataayıklama::doğrula(cümle2[cümle2.uzunluk() -1], 'r')

