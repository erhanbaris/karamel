# harfleribüyült\(\)

Abecelerde küçük ve büyük türde yazımları olan bütün küçük harfleri büyük harflere çevirir.

{% code title="sınama.krml" %}
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
Bu işlev, büyültme işini bir süreliğine yalnızca Türk abecesi üzerinde işlemektedir.
{% endhint %}

