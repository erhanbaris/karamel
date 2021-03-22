# Karamel Programlama Dili (KPD)

### Temel Tipler
 - Tam Sayı (*1024*, *1_204*, *2048*)
 - Noktalı Sayı (*1.234*, *1_234.56789*, *123.4e+4*, *1_230.4e+4*)
 - Yazı (*"Merhaba Dünya"*, *'Merhaba Dünya'*)
 - Bool (*doğru*, *yanlış*)
 - Atom (*:bilgi*, *:başarılı*, *:hatalı*)
 - Liste (*[1,2,3]*, *[]*, *[:kayıt_başarılı, 'Kullanıcı Bilgisi']*)
 - Sözlük (*{'ad':'erhan', 'soyad':'barış'}*)

### Döngü
```
kayıt = 10
toplam = 0
döngü kayıt iken:
    gç::satıryaz(kayıt)
    kayıt -= 1
    toplam += 1
hataayıklama::doğrula(toplam, 10)
hataayıklama::doğrula(kayıt, 0)
```

```
sonsuz:
    gç::satıryaz("Sonsuza kadar devam")
```

Döngü kontrolü için *devam*, *continue*, *kır*, *break*.

### Sorgulama
```
eğer a == b:  
    gç::satıryaz('a eşittir b')
yoksa a == c:  
    gç::satıryaz('a eşittir c')
yoksa:  
    gç::satıryaz('a hiçbirine eşit değil')
```

## Fonksiyon tanımlama
```
fonk metod_1(a):
    gç::yaz(a)

fonk merhaba_dünya:
    gç::yaz('Merhaba dünya')

fonk metod_1(a):
    gç::yaz(a)

fonk faktoriyel(sayı):    
    eğer sayı==1 veya sayı==0:
        döndür 1
    yoksa:
        döndür sayı * faktoriyel(sayı - 1)

faktoriyel_sonucu = faktoriyel(10)
gç::satıryaz('faktoriyel 10 => ', faktoriyel_sonucu)
```
