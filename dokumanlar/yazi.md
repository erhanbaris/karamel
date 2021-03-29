# Yazı

## Fonksiyonlar

### uzunluk()

### küçükharf()

### büyükharf()

### içeriyormu(aranan)

### satırlar()

### parçala(bununla)

### değiştir(bunu, bununla)
Değişkenlerin değerlerini birbirleri ile yer değiştirmek için kullanılır. A ve B adında iki değişkenimiz olsun. A değişkeninde bulunan değeri B değişkeni değeri ile değiştirir ve B değişkeninde bulunan değeri A değişkeni değeri ile değiştirir.

**Örnek**
```
A = "Ak"
B = "Kara"

gç::satıryaz("A değişkeni değeri: ", A) 
# Ak yazacak
gç::satıryaz("B değişkeni değeri: ", B) 
# Kara yazacak

değiştir(A,B)

gç::satıryaz("değiştir işlevinden sonra")
gç::satıryaz("A değişkeni yeni değeri: ", A) 
# Kara yazacak
gç::satıryaz("B değişkeni yeni değeri: ", B) 
# Ak yazacak
```
