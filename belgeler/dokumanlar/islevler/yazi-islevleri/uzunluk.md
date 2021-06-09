# uzunluk\(\)

Değme türlü yazıların kaç birim uzunlukta yer tuttuğunu sağışlar ve çıktı olarak verir.

### Örnek

> yazı\_uzunluğu değişkenine "Sarartamadıklarımızdan geriye kalan." yazısının kaç birim yazıdan oluştuğunu sayıp sağışlayalım.

{% code title="sınama.krml" %}
```bash
yazı_uzunluğu = uzunluk("Sarartamadıklarımızdan geriye kalan.")
gç::satıryaz('Yazı uzunluğu: ', yazı_uzunluğu)
```
{% endcode %}

