# YAZI.değiştir\(ESKİ\_DEĞER, YENİ\_DEĞER\)

Bir **Yazı**'nın tüm eşleşmelerini başka bir **Yazı** ile değiştirir. Bu fonksiyon yeni bir _Yazı_ oluşturur ve asıl _Yazı_ içeriğini kopyalar sonrasında değiştirme işlemi yapar. Orjinal _Yazı_ içeriği değişmez.

**Örnek**

"_merhaba dünya_" elimizde olan bir **Yazı**'mız olsun. "_dünya_" sözcüğünü silip yerine "_karamel_" sözcüğü yazarak "_merhaba karamel_" **Yazı**'sını elde etmeye çalışalım.

```text
söz = "merhaba dünya"
gç::satıryaz(söz.değiştir("dünya", "karamel"))
```

> ## Girdi:
>
> merhaba dünya  
>
> ## Çıktı:
>
> merhaba karamel

