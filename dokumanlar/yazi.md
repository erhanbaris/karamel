# Yazı

## Fonksiyonlar

### uzunluk()

_Yazı_'nın uzunluğunu döndürür.

### harfleriküçült()

Bütün harfleri küçük harfe çevirir. Şu an için sadece türkçe karakterlere yönelik olarak küçültme işlemi yapmaktadır.

### harfleribüyült()

Bütün harfleri büyük harfe çevirir. Şu an için sadece türkçe karakterlere yönelik olarak büyütme işlemi yapmaktadır.

### içeriyormu(aranan)

_Yazı_ içerisinde bir kelime var mı diye kontrol eder. Geriye _Bool_ veri çevirir.

### satırlar()

_Yazı_'yı satırlara göre bölüp _Liste_ geri çevirir.

### parçala(bununla)

Verilen _Yazı_'ya göre parçalara ayırır. Geriye _Liste__ döndürür.

### kırp()

_Yazı__'nın sonunda ki ve başında ki _BeyazBoşluk_'ları temizler.

### sonukırp()

_Yazı__'nın sonunda ki _BeyazBoşluk_'ları temizler.

### başıkırp()

_Yazı__'nın başında ki _BeyazBoşluk_'ları temizler.

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

Bir _Yazı_'nın tüm eşleşmelerini başka bir _Yazı_ ile değiştirir. Bu fonksiyon yeni bir _Yazı_ oluşturur ve asıl _Yazı_ içeriğini kopyalar sonrasında değiştirme işlemi yapar. Orjinal _Yazı_ içeriği değişmez.

**Örnek**

"**merhaba dünya**" elimizde olan bir _Yazı_'mız olsun. "**dünya**" sözcüğünü silip yerine "**karamel**" sözcüğü yazarak "**merhaba karamel**" _Yazı_'sını elde etmeye çalışalım.

```
değişkenim = "merhaba dünya"
gç::satıryaz("Orjinal içerik : ", değişkenim) // merhaba dünya
gç::satıryaz("Değiştirilmiş içerik : ", değişkenim.değiştir("dünya", "karamel")) // merhaba karamel
```

****_BeyazBoşluk_*******

- U+0009 (yatay sekme, '\t')
- U+000A (yeni satır, '\n')
- U+000B (dikey sekme)
- U+000C (form besleme)
- U+000D (satırbaşı, '\r')
- U+0020 (boşluk, ' ')
- U+0085 (sonraki satır)
- U+200E (soldan sağa işareti)
- U+200F (sağdan sola işareti)
- U+2028 (satır ayırıcı)
- U+2029 (paragraf ayırıcı)
  