# TASLAK

**Karamel Programlama Dili** kısa adıyla **KPD**; ikilik, sekizlik, onluk ve onaltılık sayı tabanlarını desteklemektedir.

_Sozdizimi:_  
  _Sayi ::_  
    _2lik_  
    _8lik_  
    _10luk_  
    _16lik_

  _2lik ::_  
   **0b** _2lik sayilar_  
   **0B** _2lik sayilar_

  _8lik ::_  
   **0o** _8lik sayilar_  
   **0O** _8lik sayilar_

  _10luk ::_  
   _10luk sayilar_  
   _10luk sayilar_

  _16lik ::_  
   **0x** _16lik sayilar_  
   **0X** _16lik sayilar_

  _2lik sayilar ::_  
    _2lik sayi_  
    _2lik sayilar 2lik sayi_

  _2lik sayi ::_ **biri**  
   **0 1**

  _8lik sayilar ::_  
    _8lik sayi_  
    _8lik sayilar 8lik sayi_

  _8lik sayi ::_ **biri**  
   **0 1 2 3 4 5 6 7**

  _10luk sayilar ::_  
    _10luk sayi_  
    _10luk sayilar 10luk sayi_

  _10luk sayi ::_ **biri**  
   **0 1 2 3 4 5 6 7 8 9**

  _16lik sayilar ::_  
    _16lik sayi_  
    _16lik sayilar 16lik sayi_

  _16lik sayi ::_ **biri**  
   **0 1 2 3 4 5 6 7 8 9 a b c d e f A B C D E F**

## Onluk Sayilar

Gunluk kullanimda olan onluk sayi sistemi kendi icerinde gercek sayilar ve tam sayilar olarak ikiye ayrilmaktadir. Her iki tur sayilarda kullanilabilmektedir.

_0 1 2 3 4 5 6 7 8 9_ karakterlerinden olusmaktadir.

### Tam sayilar

En buyuk sayi **9007199254740991** ve en kucuk sayi **-9007199254740991** kullanilabilmektedir.

Ornek kullanimi:  
_2020_  
_0123456789_  
_6699770000000_

### Noktali sayilar

Gunluk hayatta kullandigimiz noktali sayilar AAA dili icerinde ki karsiligi bircok diger dilde oldugu gibi **nokta** ile iki kisima ayrilmaktadir. En kucuk noktali sayi **1.7976931348623157e+308** en kucuk noktali sayi **5e-324** olarak tanimlanmistir. Tam sayilardan farkli olarak ust bilgisi kullanilabilmektedir. Fakat us bilgisi kullanilmis olan sayinin ciktisi noktali yada tam sayi olabilmektedir.

Ornek kullanimi:  
_1.23456789_  
_-123.456_  
_-123.4e-4_  
_123.4e+4_

\_\_

### Temel Tipler

* Tam Sayı \(_1024_, _1\_204_, _2048_\)
* Noktalı Sayı \(_1.234_, _1\_234.56789_, _123.4e+4_, _1\_230.4e+4_\)
* Yazı \(_"Merhaba Dünya"_, _'Merhaba Dünya'_\)
* Bool \(_doğru_, _yanlış_\)
* Atom \(_:bilgi_, _:başarılı_, _:hatalı_\)
* Liste \(_\[1,2,3\]_, _\[\]_, _\[:kayıt\_başarılı, 'Kullanıcı Bilgisi'\]_\)
* Sözlük \(_{'ad':'erhan', 'soyad':'barış'}_\)

### Döngü

```text
kayıt = 10
toplam = 0
döngü kayıt iken:
    gç::satıryaz(kayıt)
    kayıt -= 1
    toplam += 1
hataayıklama::doğrula(toplam, 10)
hataayıklama::doğrula(kayıt, 0)
```

```text
sonsuz:
    gç::satıryaz("Sonsuza kadar devam")
```

Döngü kontrolü için _devam_, _continue_, _kır_, _break_.

### Sorgulama

```text
eğer a == b:  
    gç::satıryaz('a eşittir b')
yoksa a == c:  
    gç::satıryaz('a eşittir c')
yoksa:  
    gç::satıryaz('a hiçbirine eşit değil')
```

## Fonksiyon tanımlama

```text
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

