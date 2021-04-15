# Yazı İşlevleri

## uzunluk\(\)

Değme türlü yazıların kaç birim uzunlukta yer tuttuğunu sağışlar ve çıktı olarak verir.

#### Örnek

> yazı\_uzunluğu değişkenine "Sarartamadıklarımızdan geriye kalan." yazısının kaç birim yazıdan oluştuğunu sayıp sağışlayalım.

{% code title="sına.krml" %}
```bash
yazı_uzunluğu = uzunluk("Sarartamadıklarımızdan geriye kalan.")
gç::satıryaz('Yazı uzunluğu: ', yazı_uzunluğu)
```
{% endcode %}

## harfleriküçült\(\)

Abecelerde küçük ve büyük türde yazımları olan bütün büyük harfleri küçük harflere çevirir. 

{% code title="sına.krml" %}
```bash
küçültülmüş_harfler = harfleriküçült("Arkasında Kalan Gölge.")
gç::satıryaz('Küçültülmüş yazı: ', küçültülmüş_harfler)
# 
# Girdi (İşlem öncesi)
# Arkasında Kalan Gölge
# 
# Çıktı (İşlem sonrası)
# arkasında kalan gölge
```
{% endcode %}

{% hint style="warning" %}
Bu işlev, küçültme işini bir süreliğine yalnızca Türk abecesi üzerinde uygulamaktadır.
{% endhint %}

## harfleribüyült\(\)

Abecelerde küçük ve büyük türde yazımları olan bütün küçük harfleri büyük harflere çevirir. 

{% code title="sına.krml" %}
```bash
büyültülmüş_harfler = harfleribüyült("Arkasında Kalan Gölge.")
gç::satıryaz('Küçültülmüş yazı: ', büyültülmüş_harfler)
# 
# Girdi (İşlem öncesi)
# Arkasında Kalan Gölge
# 
# Çıktı (İşlem sonrası)
# ARKASINDA KALAN GÖLGE
```
{% endcode %}

{% hint style="warning" %}
Bu işlev, büyültme işini bir süreliğine yalnızca Türk abecesi üzerinde uygulamaktadır.
{% endhint %}

