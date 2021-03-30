# Karamel Programlama Dili (KPD)

Türkçe Programlama Dili (kısaca TPD) sıfırdan kendi sanal makinesi üzerinde çalışan, birden fazla platformda çalışan, dinamik olarak değişkenlerin tanımlandığı, Türkçe konuşanlara yönelik geliştirilmekte olan bir programlama dilidir.
Python dilinde bulunan özelliklerden ilham alınarak geliştirilmeye başlanan dil, şu anda windows ve macosx üzerinde sorunsuz olarak çalışmaktadır.
Asıl amacı yeni başlayanlara kullanımı ve öğrenmesi kolay bir geliştirme ortamı sağlamaktadır.
Dilin tamamı Rust Programlama Dili kullanılarak geliştirilmektedir.
Kendi sanal makinesi üzerinde çalışan dil ve WebAssembly yardımı ile web üzerinde de kullanılabilir.

### Peki hangi ne tip sanal makne kullanıyor?
TPD, stack machine isimli sanal makine mimarisini kullanıyoruz. Bunu kullanmamızın nedeni yeni özelliklerin daha hızlı bir şekilde entegre edebilmemizden dolayı. Diğer Register Machine yaklaşımına kıyasla daha yavaş olsada ilk amacımız performanstan ziyade özellik ekleyip, stabil hale getirmek.

### Peki Stack Machine tam olarak nasıl çalışıyor?
Bu mimaride kullanılacak olan değişkenler bir yığın olarak üst üste istiflenir ve sonrasında LIFO (Last In First Out) yaklaşımına göre değişkenler istiflerden geri alınıp işleme tabii tutulur.
Bu yapının avantajı kodlar en basit haline dönüştürülerek daha performanslı olarak çalışması sağlanmaktadır. Yazılımcının yazdığı yüksek seviyeli olan kodlar işlenerek ara kodlara dönüştürülmektedir. Dönüştürülen ara kodlar TPD sanal makinesinde çalıştırılmaktadır. Aslında Üst düzey yazmış olduğunuz kodlar ve sanal makinenin işledi kodlar olarak iki farklı programlama dili içermektedir.

### Peki bunu başka hangi diller kullanıyor?
Python, PHP, Ruby gibi oldukça popüler olan diller Stack Machine yaklaşımını kullanmaktadırlar.

### Dilin şu anda ki durumu nedir?
Halen geliştirme aşamasında olup, yardımlara her zaman açığız. Mutlaka kodlama yapmanıza gerek yok. Fikirleriniz ilede gelip destek olabilirsiniz.

### İndirilebilir durumda mı?
Şu anda indirilebilir bir örneği yok ama en kısa zamanda web üzerinden çalışabilir bir versiyonu yapıp yayınlamayı düşünüyoruz.

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
