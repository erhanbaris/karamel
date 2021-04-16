---
description: Türkçe tabanlı yazılım dili.
---

# Karamel Programlama Dili \(KPD\)

**Karamel Programlama Dili**, Türkçe tabanlı yazılım dilidir. Kendi sanal düzeneği üzerinde çalışır. Ayrıca birçok düzlemde bile çalıştırılabilir. Başlıca amacı, bilişime ve yazılım alanına yeni başlayanlar için kolay kullanımlı ve kolay öğrenmeli bir geliştirme ortamı sağlamaktır.  
  
Anadili Türkçe olanlarla birlikte tüm Türkçe konuşanlara yönelik kullanılabilir bir kodlama dili, programlama dili başka bir deyişle yazılım dili olarak karşınızdadır.  
Dinamik olarak değişken tanımlayabilir ve dahasını yapabilirsiniz.  
  
Gözde dillerde bulunan özelliklerden esinlenerek geliştirilmeye başlanan **Karamel dili**, bilinen iki işletim sistemi **Windows** ve **MacOsX** üzerinde sorunsuzca çalışmaktadır.  
  
Bütün dil **Rust Programlama Dili** kullanılarak geliştirilmektedir. Kendi sanal düzeneği üzerinde çalışan dil olması yanısıra, WebAssembly yardımı ile ağ üzerinde de kullanılabilir.

### İndirilebilir Durumda Mı?

Üzgünüz, henüz indirilebilir bir örneği yok. Ancak en kısa sürede ağ üzerinden çalışabilir bir sürümü yayınlamayı düşünüyoruz. Denemelik sürüme bir süreliğine şuradan erişebilirsiniz.  
[https://erhanbaris.github.io/karamel/karamelweb/src/www/](https://erhanbaris.github.io/karamel/karamelweb/src/www/)

### Karamel Hangi Düzeyde?

* Geliştirilme aşamasında olup emekleme sürecindedir.
* Dört bir yandan gelecek düşünce, görüş ve kodlama gibi türlü yardımlara gerek duymaktadır.
* Terim dağarcığında oturmamış konuları bulunmaktadır.
* Sözdizimi üzerinde esnek tutum sergilense bile kömek yani yardım gerekmektedir.

### Karamel Hangi Ölçünlere Göre Geliştiriliyor?

Türkçe tabanlı yazılım dillerinde kullanılmak üzere oluşturulan kararlı ölçünler henüz bulunmuyor. Bu nedenle şimdilik böyle bir durum söz konusu değil. Ancak [Türkçe Yazılım Konatı](https://github.com/turkce-yazilim-konati/) altında geliştirilmekte olan [**YAZILIMCA**](https://github.com/turkce-yazilim-konati/yazilimca) ****ölçünlerini izliyor, uygun bulduğu yanlarını değerlendirip kullanıyor.

### Takıma Katıl

Karamel, Türkçe Yazılım Konatı altında birleşen kişilerle birlikte geliştiriliyor. Bu takımda yer almak için aşağıdaki yerlere başvurman yeterlidir.

* **Github**'da [Türkçe Yazılım Konatı](https://github.com/turkce-yazilim-konati)'na katıl.
* **Discord**'da [Türkçe Yazılım Konatı](%20https://discord.gg/8ymtm9XPyQ)'na katıl.
* **Facebook**'ta [Türkçe Yazılım Konatı](https://www.facebook.com/groups/815710512519539)'na katıl.

### 



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

