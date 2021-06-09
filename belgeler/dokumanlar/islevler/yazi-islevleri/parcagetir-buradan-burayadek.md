# parçagetir\(buradan, burayadek\)

Bir _Yazı_ içerisinden bir parçayı almak için kullanılır. Eğer _buradan_ değeri 0'dan küçük olursa, başlangıç noktası 0 olarak kabul edilir. Eğer _burayakadar_ değeri _Yazı_'nın uzunluğundan büyük olursa, bitiş değeri _Yazı_'ının uzunluğu olarak kabul edilir.

**Örnek**

```text
değişkenim = "merhaba dünya"
gç::satıryaz(değişkenim) // merhaba dünya
gç::satıryaz(değişkenim.parçagetir(0, 7)) // merhaba
gç::satıryaz(değişkenim.parçagetir(8, 14)) // dünya
```

