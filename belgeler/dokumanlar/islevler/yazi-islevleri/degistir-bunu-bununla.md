# değiştir\(bunu, bununla\)

Bir _Yazı_'nın tüm eşleşmelerini başka bir _Yazı_ ile değiştirir. Bu fonksiyon yeni bir _Yazı_ oluşturur ve asıl _Yazı_ içeriğini kopyalar sonrasında değiştirme işlemi yapar. Orjinal _Yazı_ içeriği değişmez.

**Örnek**

"**merhaba dünya**" elimizde olan bir _Yazı_'mız olsun. "**dünya**" sözcüğünü silip yerine "**karamel**" sözcüğü yazarak "**merhaba karamel**" _Yazı_'sını elde etmeye çalışalım.

```text
değişkenim = "merhaba dünya"
gç::satıryaz("Orjinal içerik : ", değişkenim) // merhaba dünya
gç::satıryaz("Değiştirilmiş içerik : ", değişkenim.değiştir("dünya", "karamel")) // merhaba karamel
```

