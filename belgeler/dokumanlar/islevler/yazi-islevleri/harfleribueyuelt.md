# YAZI.harfleribüyült\(\)

Abecelerde küçük ve büyük türde yazımları olan bütün küçük harfleri büyük harflere çevirir.

{% code title="sınama.krml" %}
```bash
büyültülmüş_harfler = "Arkasında Kalan Gölge.".harfleribüyült()
gç::satıryaz('Büyültülmüş yazı: ', büyültülmüş_harfler)
 
# GİRDİ
# Arkasında Kalan Gölge
# 
# ÇIKTI
# ARKASINDA KALAN GÖLGE
```
{% endcode %}

{% hint style="warning" %}
Bu işlev, büyültme işini bir süreliğine yalnızca Türk abecesi üzerinde işlemektedir.
{% endhint %}

