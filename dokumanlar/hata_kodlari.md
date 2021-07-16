# Hata Kodları

Karamel Programlama dili derleme ve çalışmazamanında hatalar üretmektedir. Bu hataların çoğunluğu derleme zamanında üretilmnektedir ve bu hatanın çalışma zamanında dil içerisinden kontrolü yapılamaz. Bağlı hataya göre kodun tekrardan düzenlenmesi gerekmektedir. Bazı hatalarda dinamik bilgiler ile birlikte gelmektedir. 

## Geçersiz ifade
Kodu: 101  
InvalidExpression

## Birden fazla degisken kullanilamaz
Kodu: 102  
Tanımlaması: MoreThan1ArgumentPassed

## Sağ parantaz eksik
Kodu: 103  
Tanımlaması: RightParanthesesMissing

## Doğrulanama  başarısız
Kodu: 104  
Tanımlaması: AssertFailed

## Sayı ayrıştırılamadı
Kodu: 105  
Tanımlaması: NumberNotParsed

## Yazı sonlandırıcısı bulunamadı
Kodu: 106  
Tanımlaması: MissingStringDeliminator

## Karakter geçerli değil
Kodu: 107  
Tanımlaması: CharNotValid

## İfadenin sağ tarafı bulunamadı
Kodu: 108  
Tanımlaması: RightSideOfExpressionNotFound

## Döndür komutu fonksiyon içinde kullanılmalıdır
Kodu: 109  
Tanımlaması: ReturnMustBeUsedInFunction

## Fonksiyon çağırma sözdizimi geçerli değil
Kodu: 110  
Tanımlaması: FunctionCallSyntaxNotValid

## Fonksiyon adı tanımlanmamış
Kodu: 111  
Tanımlaması: FunctionNameNotDefined

## Değişken yazı olmalıdır
Kodu: 112  
Tanımlaması: ArgumentMustBeText

## Koşul gövdesi eksik
Kodu: 113  
Tanımlaması: IfConditionBodyNotFound

## Parantez kapatılmamış
Kodu: 114  
Tanımlaması: ParenthesesNotClosed

## Geçersiz tekli işlem
Kodu: 115  
Tanımlaması: InvalidUnaryOperation

## Tekli numara ile çalışmaktadır
Kodu: 116  
Tanımlaması: UnaryWorksWithNumber

## Parametre bulunamadı
Kodu: 117  
Tanımlaması: ArgumentNotFound

## Birden fazla 'yoksa' ifadesi kullanılamaz
Kodu: 118  
Tanımlaması: MultipleElseUsageNotValid

## 'kır' ve 'devam' ifadeleri döngü içinde kullanılabilir
Kodu: 119  
Tanımlaması: BreakAndContinueBelongToLoops

## Fonksiyon içi kodlar bulunamadı
Kodu: 120  
Tanımlaması: FunctionConditionBodyNotFound

## ':' eksik
Kodu: 121  
Tanımlaması: ColonMarkMissing

## 'yoksa' zaten kullanıldı
Kodu: 122  
Tanımlaması: ElseIsUsed

## Girinti sorunu
Kodu: 123  
Tanımlaması: IndentationIssue

## Sözlük düzgün kapatılmamış
Kodu: 124  
Tanımlaması: DictNotClosed

## Dizi düzgün kapatılmadı
Kodu: 125  
Tanımlaması: ArrayNotClosed

## Dizi elemanı geçersiz
Kodu: 126  
Tanımlaması: InvalidListItem

## Sözlük anahtarı geçersiz
Kodu: 127  
Tanımlaması: DictionaryKeyNotValid

## Sözlük geçeri geçersiz
Kodu: 128  
Tanımlaması: DictionaryValueNotValid

## Yorum bilgisi düzgün kapatılmadı
Kodu: 129  
Tanımlaması: CommentNotFinished

## Döngü düzgün tanımlanmamış
Kodu: 130  
Tanımlaması: WhileStatementNotValid

## Fonksiyon tanımlaması hatalı
Kodu: 131  
Tanımlaması: FunctionDefinationNotValid

## 'ise' sözcüğü eksik
Kodu: 132  
Tanımlaması: MissingIf

## Anahtar kelimeler kullanılamaz
Kodu: 133  
Tanımlaması: KeywordCouldNotBeUsed

## '{dosya adı}' okunamadi. Hata: '{hata}'
Kodu: 134  
Tanımlaması: FileReadError  
Parametreler:  
 - dosya adı  
 - hata  

## '{bilgi}' bulunamadi
Kodu: 135  
Tanımlaması: FileNotFound  
Parametreler:  
 - bilgi  

## {bilgi}
Kodu: 136  
Tanımlaması: GeneralError  
Parametreler:  
 - bilgi  

## '{bilgi}' fonksiyonu önceden tanımlanmış
Kodu: 137  
Tanımlaması: FunctionAlreadyDefined  
Parametreler:  
 - bilgi  

## '{bilgi}' fonksiyonu bulunamadı
Kodu: 138  
Tanımlaması: FunctionNotFound  
Parametreler:  
 - bilgi  

## '{fonksiyon}' fonksiyon parametreleri eşleşmiyor. {beklenen} adet beklenirken {bulunan} adet bulundu
Kodu: 139  
Tanımlaması: FunctionArgumentNotMatching  
Parametreler:  
 - fonksiyon  
 - beklenen  
 - bulunan  

## '{fonksiyon}' fonksiyonu sadece {beklenen} parametresini kabul ediyor
Kodu: 140  
Tanımlaması: FunctionExpectedThatParameterType  
Parametreler:  
 - fonksiyon  
 - beklenen  

## Doğrulama başarısız (Sol: {sol} sağ: {sağ})
Kodu: 141  
Tanımlaması: AssertFailedWithArgument  
Parametreler:  
 - sol  
 - sağ  

## Tekli ifade geçerli değil
Kodu: 142  
Tanımlaması: UnaryExpressionNotValid  

## Tekli operatör bulunamadi
Kodu: 143  
Tanımlaması: UnaryOperatorNotFound

## Depoda değer bulunamadı
Kodu: 144  
Tanımlaması: ValueNotFoundInStorage

## '{bilgi}' reserv edilmiş kelimedir kullanılamaz
Kodu: 145  
Tanımlaması: ReservedName  
Parametreler:  
 - bilgi  

## '{isim}' modül okuma sırasında hata ile karşılaşıldı. Hata {hata}
Kodu: 146  
Tanımlaması: ModuleParseError  
Parametreler:  
 - isim  
 - hata  

## Depoda fonksiyon({bilgi}) bulunamadı
Kodu: 147  
Tanımlaması: FunctionNotFoundInStorage  
Parametreler:  
 - bilgi  

## '{bilgi}' fonksiyon olarak çağrılabilir değil
Kodu: 148  
Tanımlaması: NotCallable  
Parametreler:  
 - bilgi  

## '{bilgi}' geçerli bir sıralayıcı değil sayı olması gerekiyor
Kodu: 149  
Tanımlaması: IndexerMustBeNumber  
Parametreler:  
 - bilgi  

## '{bilgi}' geçerli bir sıralayıcı değil yazı olması gerekiyor
Kodu: 150  
Tanımlaması: IndexerMustBeString  
Parametreler:  
 - bilgi  

## Döngü ile sadece atama öperatörü kullanılabilir
Kodu: 151  
Tanımlaması: AssignOperatorRequiredForLoop

## virgül eksik
Kodu: 152  
Tanımlaması: CommaIsMissing

## Öperatör geçerli değil
Kodu: 153  
Tanımlaması: OperatorNotValid
