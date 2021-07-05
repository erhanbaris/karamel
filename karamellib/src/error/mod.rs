use std::borrow::Borrow;

use strum::EnumMessage;
use strum_macros::EnumIter;
use strum_macros::EnumMessage;
use thiserror::Error;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Error)]
#[derive(EnumIter)]
#[derive(EnumMessage)]
pub enum KaramelErrorType {
    #[error("Sozdizimi hatasi")]
    #[strum(message = "100")]
    SyntaxError,
    
    #[error("Geçersiz ifade")]
    #[strum(message = "101")]
    InvalidExpression,

    #[error("Birden fazla degisken kullanilamaz")]
    #[strum(message = "102")]
    MoreThan1ArgumentPassed,
    
    #[error("Sağ parantaz eksik")]
    #[strum(message = "103")]
    RightParanthesesMissing,
    
    #[error("Doğrulanamadı")]
    #[strum(message = "104")]
    AssertFailed,
    
    #[error("Sayı ayrıştırılamadı")]
    #[strum(message = "105")]
    NumberNotParsed,
    
    #[error("Yazı sonlandırıcısı bulunamadı")]
    #[strum(message = "106")]
    MissingStringDeliminator,
    
    #[error("Karakter geçerli değil")]
    #[strum(message = "107")]
    CharNotValid,
    
    #[error("İfadenin sağ tarafı bulunamadı")]
    #[strum(message = "108")]
    RightSideOfExpressionNotFound,
    
    #[error("Döndür komutu fonksiyon içinde kullanılmalıdır")]
    #[strum(message = "109")]
    ReturnMustBeUsedInFunction,
    
    #[error("Fonksiyon çağırma sözdizimi geçerli değil")]
    #[strum(message = "110")]
    FunctionCallSyntaxNotValid,
    
    #[error("Fonksiyon adı tanımlanmamış")]
    #[strum(message = "111")]
    FunctionNameNotDefined,
    
    #[error("Değişken yazı olmalıdır")]
    #[strum(message = "112")]
    ArgumentMustBeText,
    
    #[error("Koşul gövdesi eksik")]
    #[strum(message = "113")]
    IfConditionBodyNotFound,
    
    #[error("Parantez kapatılmamış")]
    #[strum(message = "114")]
    ParenthesesNotClosed,
    
    #[error("Geçersiz tekli işlem")]
    #[strum(message = "115")]
    InvalidUnaryOperation,
    
    #[error("Tekli numara ile çalışmaktadır")]
    #[strum(message = "116")]
    UnaryWorksWithNumber,
    
    #[error("Parametre bulunamadı")]
    #[strum(message = "117")]
    ArgumentNotFound,
    
    #[error("Birden fazla 'yoksa' ifadesi kullanılamaz")]
    #[strum(message = "118")]
    MultipleElseUsageNotValid,
    
    #[error("'kır' ve 'devam' ifadeleri döngü içinde kullanılabilir")]
    #[strum(message = "119")]
    BreakAndContinueBelongToLoops,
    
    #[error("Fonksiyon içi kodlar bulunamadı")]
    #[strum(message = "120")]
    FunctionConditionBodyNotFound,
    
    #[error("':' eksik")]
    #[strum(message = "121")]
    ColonMarkMissing,
    
    #[error("'yoksa' zaten kullanıldı")]
    #[strum(message = "122")]
    ElseIsUsed,
    
    #[error("Girinti sorunu")]
    #[strum(message = "123")]
    IndentationIssue,
    
    #[error("Sözlük düzgün kapatılmamış")]
    #[strum(message = "124")]
    DictNotClosed,
    
    #[error("Dizi düzgün kapatılmadı")]
    #[strum(message = "125")]
    ArrayNotClosed,
    
    #[error("Dizi elemanı geçersiz")]
    #[strum(message = "126")]
    InvalidListItem,
    
    #[error("Sözlük anahtarı geçersiz")]
    #[strum(message = "127")]
    DictionaryKeyNotValid,
    
    #[error("Sözlük geçeri geçersiz")]
    #[strum(message = "128")]
    DictionaryValueNotValid,
    
    #[error("Yorum bilgisi düzgün kapatılmadı")]
    #[strum(message = "129")]
    CommentNotFinished,
    
    #[error("Döngü düzgün tanımlanmamış")]
    #[strum(message = "130")]
    WhileStatementNotValid,
    
    #[error("Fonksiyon tanımlaması hatalı")]
    #[strum(message = "131")]
    FunctionDefinationNotValid,
    
    #[error("'ise' sözcüğü eksik")]
    #[strum(message = "132")]
    MissingIf,
    
    #[error("Anahtar kelimeler kullanılamaz")]
    #[strum(message = "133")]
    KeywordCouldNotBeUsed
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct KaramelError {
    pub error_type: KaramelErrorType,
    pub line: u32,
    pub column: u32
}

pub fn generate_error_message<T: AsRef<str>, E: Borrow<KaramelError>>(data: T, error: E) -> String {
    let error_ref = error.borrow();
    let lines = data.as_ref().split(|c| c == '\n').collect::<Vec<_>>();
    let line = lines.iter().skip(error_ref.line as usize).next().unwrap();
    return format!("...\r\n{}\r\n{:>width$} [{}:{}] (#{}) {}", line, "^", error_ref.line, error_ref.column, error_ref.error_type.get_message().unwrap(), error_ref.error_type,  width=error_ref.column as usize);
}

#[cfg(test)]
mod test {
    use strum::IntoEnumIterator;
    use strum::EnumMessage;

    #[test]
    fn test_all_error_has_number() {
        for error_enum in super::KaramelErrorType::iter() {
            let error_message = format!("{}", error_enum);

            if error_message.len() == 0 {
                assert!(false, "'{:?}' hata mesaji yok", error_enum)
            }

            match error_enum.get_message() {
                Some(error_message) => assert!(error_message.len() > 0, "'{:?}' sira numarasi bos olamaz", error_enum),
                None => assert!(false, "{} sira numarasi yok", error_enum)
            }
        }
    }
    #[test]
    fn test_error_message_generator() {
        let error_info = super::KaramelError {
            error_type: super::KaramelErrorType::SyntaxError,
             line: 0,
             column: 0
         };
        let error_message = super::generate_error_message("merhaba dunya", &error_info);

        assert!(error_message.len() > 0, "Hata mesaji bos uretilemez");
        assert!(error_message.contains("merhaba dunya"), "Hata mesaji icerisinde kaynak kod bilgisi yok");
        assert!(error_message.contains(error_info.error_type.get_message().unwrap()), "Mesaj icerisinde hata kodu yok");

        let error_message = format!("{}", error_info.error_type);
        assert!(error_message.contains(&error_message), "Mesaj icerisinde hata kodu mesaji");
    }
}