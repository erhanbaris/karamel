---
description: Karamel Programlama Dili Hakkında
---

# Hakkında

### Peki hangi ne tip sanal makine kullanıyor?

TPD, stack machine isimli sanal makine mimarisini kullanıyoruz. Bunu kullanmamızın nedeni yeni özelliklerin daha hızlı bir şekilde entegre edebilmemizden dolayı. Diğer Register Machine yaklaşımına kıyasla daha yavaş olsada ilk amacımız performanstan ziyade özellik ekleyip, stabil hale getirmek.

### Peki Stack Machine tam olarak nasıl çalışıyor?

Bu mimaride kullanılacak olan değişkenler bir yığın olarak üst üste istiflenir ve sonrasında LIFO \(Last In First Out\) yaklaşımına göre değişkenler istiflerden geri alınıp işleme tabii tutulur. Bu yapının avantajı kodlar en basit haline dönüştürülerek daha performanslı olarak çalışması sağlanmaktadır. Yazılımcının yazdığı yüksek seviyeli olan kodlar işlenerek ara kodlara dönüştürülmektedir. Dönüştürülen ara kodlar TPD sanal makinesinde çalıştırılmaktadır. Aslında Üst düzey yazmış olduğunuz kodlar ve sanal makinenin işledi kodlar olarak iki farklı programlama dili içermektedir.

### Peki bunu başka hangi diller kullanıyor?

Python, PHP, Ruby gibi oldukça popüler olan diller Stack Machine yaklaşımını kullanmaktadırlar.

