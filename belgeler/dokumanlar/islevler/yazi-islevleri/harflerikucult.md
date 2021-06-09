# harfleriküçült\(\)



## harfleriküçült\(\)

Abecelerde küçük ve büyük türde yazımları olan bütün büyük harfleri küçük harflere çevirir.

{% code title="sınama.krml" %}
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
Bu işlev, küçültme işini bir süreliğine yalnızca Türk abecesi üzerinde işlemektedir.
{% endhint %}

