# YAZI.harfleriküçült\(\)

Abecelerde küçük ve büyük türde yazımları olan bütün büyük harfleri küçük harflere çevirir.

{% code title="sınama.krml" %}
```bash
küçültülmüş_harfler = "Arkasında Kalan Gölge.".harfleriküçült()
gç::satıryaz('Küçültülmüş yazı: ', küçültülmüş_harfler)

# GİRDİ
# Arkasında Kalan Gölge
# 
# ÇIKTI
# arkasında kalan gölge
```
{% endcode %}

{% hint style="warning" %}
Bu işlev, küçültme işini bir süreliğine yalnızca Türk abecesi üzerinde işlemektedir.
{% endhint %}

