# Yazı

## Fonksiyonlar

### uzunluk()

### küçükharf()

### büyükharf()

### içeriyormu(aranan)

### satırlar()

### parçala(bununla)

### kırp()

### sonukırp()

### başıkırp()

### parçagetir(buradan, burayakadar)
Bir _Yazı_ içerisinden bir parçayı almak için kullanılır. Eğer _buradan_ değeri 0'dan küçük olursa, başlangıç noktası 0 olarak kabul edilir. Eğer _burayakadar_ değeri _Yazı_'nın uzunluğundan büyük olursa, bitiş değeri _Yazı_'ının uzunluğu olarak kabul edilir.

**Örnek**
```
değişkenim = "merhaba dünya"
gç::satıryaz(değişkenim) // merhaba dünya
gç::satıryaz(değişkenim.parçagetir(0, 7)) // merhaba
gç::satıryaz(değişkenim.parçagetir(8, 14)) // dünya
```

### değiştir(bunu, bununla)
Ele alınan bir **veri bütünlüğü**nde değiştirilmek istenen veriyi silip yerine yeni veri koyma yoluyla veri üzerinde değiştirme yapar. Bu görevle yeni veri, kaldırılmak istenen veri ile değiştirilir. Bir başka deyişle yazının tüm eşleşmelerini başka bir yazı ile değiştirir. Bu fonksiyon yeni bir yazı _Yazı_ oluşturur ve asıl _Yazı_ içeriğini kopyalayıp içeriği değiştirir.

**Örnek**  
"**merhaba dünya**" elimizde olan bir yazımız olsun. "**dünya**" sözcüğünü silip yerine "**karamel**" sözcüğü yazarak "**merhaba karamel**" yazısını elde etmeye çalışalım.
```
değişkenim = "merhaba dünya"
gç::satıryaz("Orjinal içerik : ", değişkenim) // merhaba dünya
gç::satıryaz("Değiştirilmiş içerik : ", değişkenim.değiştir("dünya", "karamel")) // merhaba karamel
```

### ????(bunu, buna, buna, buna, buna, değer)
Bir değişkendeki değeri edilgen değişkenler içine koyar. Bu işlem yapılırken ana değişkendeki değer eksilir. Fonksiyon değeri girilmediyse ana değişkendeki değer edilgen değişkenlerin içine olduğu gibi katılır.

```
anaDğşkn = "Ak"
edilgenDğşkn1 = "Kara"
edilgenDğşkn2 = "Yeşil"
edilgenDğşkn3 = "Sarı"

gç::satıryaz(anaDğşkn, edilgenDğşkn1, edilgenDğşkn2, edilgenDğşkn3) 
# AkKaraYeşilSarı yazacak 

????(anaDğşkn, edilgenDğşkn1, edilgenDğşkn2, edilgenDğşkn3)
gç::satıryaz(anaDğşkn, edilgenDğşkn1, edilgenDğşkn2, edilgenDğşkn3) 
# KaraAkYeşilAkSarıAk yazacak
```
