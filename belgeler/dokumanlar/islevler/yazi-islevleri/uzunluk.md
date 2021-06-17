# YAZI.uzunluk\(\)

Değme türlü yazıların kaç birim uzunlukta yer tuttuğunu sayıp döker ve sayılı sonuç olarak çıktı olarak verir.

### Örnek

> yazı\_uzunluğu değişkenine "Sarartamadıklarımızdan geriye kalan." yazısının kaç birim yazıdan oluştuğunu sayıp sağışlayalım.

{% code title="sınama.krml" %}
```bash
örnek_yazı = "Çekoslavakyalılaştıramadıklarımızdan mısınız?"
gç::satıryaz('Yazı uzunluğu: ', örnek_yazı.uzunluk())
```
{% endcode %}

