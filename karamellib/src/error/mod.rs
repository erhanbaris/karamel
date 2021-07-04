
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum KaramelErrorType {
    SyntaxError = 100,
    InvalidExpression = 101,
    MoreThan1ArgumentPassed = 102,
    RightParanthesesMissing = 103,
    AssertFailed = 104,
    NumberNotParsed = 105,
    MissingStringDeliminator = 106,
    CharNotValid = 107,
    RightSideOfExpressionNotFound = 108,
    ReturnMustBeUsedInFunction = 109,
    FunctionCallSyntaxNotValid = 110,
    FunctionNameNotDefined = 111,
    ArgumentMustBeText = 112,
    IfConditionBodyNotFound = 113,
    ParenthesesNotClosed = 114,
    InvalidUnaryOperation = 115,
    UnaryWorksWithNumber = 116,
    ArgumentNotFound = 117,
    MultipleElseUsageNotValid = 118,
    BreakAndContinueBelongToLoops = 119,
    FunctionConditionBodyNotFound = 120,
    ColonMarkMissing = 121,
    ElseIsUsed = 122,
    IndentationIssue = 123,
    DictNotClosed = 124,
    ArrayNotClosed = 125,
    InvalidListItem = 126,
    DictionaryKeyNotValid = 127,
    DictionaryValueNotValid = 128,
    CommentNotFinished = 129,
    WhileStatementNotValid = 130,
    FunctionDefinationNotValid = 131,
    MissingIf = 132,
    KeywordCouldNotBeUsed = 133
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

pub fn generate_error_message(data: &String, error: &KaramelError) -> String {
    let lines = data.split(|c| c == '\n').collect::<Vec<_>>();
    let line = lines.iter().skip(error.line as usize).next().unwrap();
    return format!("...\r\n{}\r\n{:>width$} [{}:{}] {}", line, "^", error.line, error.column, error.error_type.as_text(),  width=error.column as usize);
}

impl KaramelErrorType {
    pub fn as_text(&self) -> String {
        let message = match self {
            KaramelErrorType::SyntaxError => "Sozdizimi hatasi",
            KaramelErrorType::MoreThan1ArgumentPassed => "Birden fazla degisken kullanilamaz",
            KaramelErrorType::RightParanthesesMissing => "Sağ parantaz eksik",
            KaramelErrorType::AssertFailed => "Doğrulanamadı",
            KaramelErrorType::NumberNotParsed => "Sayı ayrıştırılamadı",
            KaramelErrorType::MissingStringDeliminator => "Yazı sonlandırıcısı bulunamadı",
            KaramelErrorType::CharNotValid => "Karakter geçerli değil",
            KaramelErrorType::RightSideOfExpressionNotFound => "İfadenin sağ tarafı bulunamadı",
            KaramelErrorType::ReturnMustBeUsedInFunction => "Döndür komutu fonksiyon içinde kullanılmalıdır",
            KaramelErrorType::FunctionCallSyntaxNotValid => "Fonksiyon çağırma sözdizimi geçerli değil",
            KaramelErrorType::FunctionNameNotDefined => "Fonksiyon adı tanımlanmamış",
            KaramelErrorType::ArgumentMustBeText => "Değişken yazı olmalıdır",
            KaramelErrorType::IfConditionBodyNotFound => "Koşul gövdesi eksik",
            KaramelErrorType::ParenthesesNotClosed => "Parantez kapatılmamış",
            KaramelErrorType::InvalidUnaryOperation => "Geçersiz tekli işlem",
            KaramelErrorType::UnaryWorksWithNumber => "Tekli numara ile çalışmaktadır",
            KaramelErrorType::InvalidExpression => "Geçersiz ifade",
            KaramelErrorType::ArgumentNotFound => "Parametre bulunamadı",
            KaramelErrorType::MultipleElseUsageNotValid => "Birden fazla 'yoksa' ifadesi kullanılamaz",
            KaramelErrorType::BreakAndContinueBelongToLoops => "'kır' ve 'devam' ifadeleri döngü içinde kullanılabilir",
            KaramelErrorType::FunctionConditionBodyNotFound => "Fonksiyon içi kodlar bulunamadı",
            KaramelErrorType::ColonMarkMissing => "':' eksik",
            KaramelErrorType::ElseIsUsed => "'yoksa' zaten kullanıldı",
            KaramelErrorType::IndentationIssue => "Girinti sorunu",
            KaramelErrorType::DictNotClosed => "Sözlük düzgün kapatılmamış",
            KaramelErrorType::ArrayNotClosed => "Dizi düzgün kapatılmadı",
            KaramelErrorType::InvalidListItem => "Dizi elemanı geçersiz",
            KaramelErrorType::DictionaryKeyNotValid => "Sözlük anahtarı geçersiz",
            KaramelErrorType::DictionaryValueNotValid => "Sözlük geçeri geçersiz",
            KaramelErrorType::CommentNotFinished => "Yorum bilgisi düzgün kapatılmadı",
            KaramelErrorType::WhileStatementNotValid => "Döngü düzgün tanımlanmamış",
            KaramelErrorType::FunctionDefinationNotValid => "Fonksiyon tanımlaması hatalı",
            KaramelErrorType::MissingIf => "'ise' sözcüğü eksik",
            KaramelErrorType::KeywordCouldNotBeUsed => "Anahtar kelimeler kullanılamaz"
        };
        format!("(#{}) {}", *self as u8, message)
    }
}