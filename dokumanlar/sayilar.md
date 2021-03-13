# TASLAK

**Karamel Programlama Dili** kısa adıyla **KPD**; ikilik, sekizlik, onluk ve onaltılık sayı tabanlarını desteklemektedir.

*Sozdizimi:*  
&emsp;&emsp;*Sayi ::*  
&emsp;&emsp;&emsp;&emsp;*2lik*  
&emsp;&emsp;&emsp;&emsp;*8lik*  
&emsp;&emsp;&emsp;&emsp;*10luk*  
&emsp;&emsp;&emsp;&emsp;*16lik*  

&emsp;&emsp;*2lik ::*  
&emsp;&emsp;&emsp;**0b** *2lik sayilar*  
&emsp;&emsp;&emsp;**0B** *2lik sayilar*    

&emsp;&emsp;*8lik ::*  
&emsp;&emsp;&emsp;**0o** *8lik sayilar*  
&emsp;&emsp;&emsp;**0O** *8lik sayilar*  

&emsp;&emsp;*10luk ::*  
&emsp;&emsp;&emsp;*10luk sayilar*  
&emsp;&emsp;&emsp;*10luk sayilar*  

&emsp;&emsp;*16lik ::*  
&emsp;&emsp;&emsp;**0x** *16lik sayilar*  
&emsp;&emsp;&emsp;**0X** *16lik sayilar*  

&emsp;&emsp;*2lik sayilar ::*  
&emsp;&emsp;&emsp;&emsp;*2lik sayi*  
&emsp;&emsp;&emsp;&emsp;*2lik sayilar&emsp;2lik sayi*  

&emsp;&emsp;*2lik sayi ::* **biri**  
&emsp;&emsp;&emsp;**0 1**  

&emsp;&emsp;*8lik sayilar ::*  
&emsp;&emsp;&emsp;&emsp;*8lik sayi*  
&emsp;&emsp;&emsp;&emsp;*8lik sayilar&emsp;8lik sayi*  

&emsp;&emsp;*8lik sayi ::* **biri**  
&emsp;&emsp;&emsp;**0 1 2 3 4 5 6 7**  

&emsp;&emsp;*10luk sayilar ::*  
&emsp;&emsp;&emsp;&emsp;*10luk sayi*  
&emsp;&emsp;&emsp;&emsp;*10luk sayilar&emsp;10luk sayi*  

&emsp;&emsp;*10luk sayi ::* **biri**  
&emsp;&emsp;&emsp;**0 1 2 3 4 5 6 7 8 9**  

&emsp;&emsp;*16lik sayilar ::*  
&emsp;&emsp;&emsp;&emsp;*16lik sayi*  
&emsp;&emsp;&emsp;&emsp;*16lik sayilar&emsp;16lik sayi*  

&emsp;&emsp;*16lik sayi ::* **biri**  
&emsp;&emsp;&emsp;**0 1 2 3 4 5 6 7 8 9 a b c d e f A B C D E F**  


### Onluk Sayilar
Gunluk kullanimda olan onluk sayi sistemi kendi icerinde gercek sayilar ve tam sayilar olarak ikiye ayrilmaktadir. Her iki tur sayilarda kullanilabilmektedir.

*0 1 2 3 4 5 6 7 8 9* karakterlerinden olusmaktadir. 

##### Tam sayilar
En buyuk sayi **9007199254740991** ve en kucuk sayi **-9007199254740991** kullanilabilmektedir.

Ornek kullanimi:  
*2020*  
*0123456789*  
*6699770000000*  

##### Noktali sayilar
Gunluk hayatta kullandigimiz noktali sayilar AAA dili icerinde ki karsiligi bircok diger dilde oldugu gibi **nokta** ile iki kisima ayrilmaktadir. En kucuk noktali sayi **1.7976931348623157e+308** en kucuk noktali sayi **5e-324** olarak tanimlanmistir. Tam sayilardan farkli olarak ust bilgisi kullanilabilmektedir. Fakat us bilgisi kullanilmis olan sayinin ciktisi noktali yada tam sayi olabilmektedir.

Ornek kullanimi:  
*1.23456789*  
*-123.456*  
*-123.4e-4*  
*123.4e+4*  
