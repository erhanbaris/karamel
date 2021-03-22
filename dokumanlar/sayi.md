# Sayı

## Fonksiyonlar

### hex()
Sayıyı dexadecimal formatında yazar.
```
gç::satıryaz(1.2.yuvarla())    //  1
```

### yuvarla()
Noktalı sayıyı en yakın sayıya yuvarlar.
```
gç::satıryaz(1.2.yuvarla())    //  1
gç::satıryaz(1.5.yuvarla())    //  2
gç::satıryaz(1.51.yuvarla())   //  2
gç::satıryaz(-1.2.yuvarla())   // -1
gç::satıryaz(-1.5.yuvarla())   // -2
gç::satıryaz(-1.51.yuvarla())  // -2
```

### tavan()
Noktalı sayıyı üst sayıya tamamlar.

```
gç::satıryaz(1.2.tavan())    //  2
gç::satıryaz(1.5.tavan())    //  2
gç::satıryaz(1.51.tavan())   //  2
gç::satıryaz(-1.2.tavan())   // -1
gç::satıryaz(-1.5.tavan())   // -1
gç::satıryaz(-1.51.tavan())  // -1
```

### taban()
Noktalı sayıyı alt sayıya tamamlar.

```
gç::satıryaz(1.2.tavan())    //  1
gç::satıryaz(1.5.tavan())    //  1
gç::satıryaz(1.51.tavan())   //  1
gç::satıryaz(-1.2.tavan())   // -2
gç::satıryaz(-1.5.tavan())   // -2
gç::satıryaz(-1.51.tavan())  // -2
```