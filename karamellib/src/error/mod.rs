
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaErrorType {
    SyntaxError = 100,
    InvalidExpression,
    MoreThan1ArgumentPassed,
    RightParanthesesMissing,
    AssertFailed,
    NumberNotParsed,
    MissingStringDeliminator,
    CharNotValid,
    RightSideOfExpressionNotFound,
    ReturnMustBeUsedInFunction,
    FunctionCallSyntaxNotValid,
    FunctionNameNotDefined,
    ArgumentMustBeText,
    IfConditionBodyNotFound,
    ParenthesesNotClosed,
    InvalidUnaryOperation,
    UnaryWorksWithNumber,
    ArgumentNotFound,
    MultipleElseUsageNotValid,
    BreakAndContinueBelongToLoops,
    FunctionConditionBodyNotFound,
    ColonMarkMissing,
    ElseIsUsed,
    IndentationIssue,
    DictNotClosed,
    ArrayNotClosed,
    InvalidListItem,
    DictionaryKeyNotValid,
    DictionaryValueNotValid,
    CommentNotFinished,
    WhileStatementNotValid
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct BramaError {
    pub error_type: BramaErrorType,
    pub line: u32,
    pub column: u32
}

pub fn generate_error_message(data: &String, error: &BramaError) -> String {
    let line = data.lines().skip(error.line as usize).next().unwrap();
    return format!("...\r\n[{}:{}] {}\r\n...\r\n{}", error.line + 1, error.column, line, error.error_type.as_text());
    /*
gç::sa tıryaz("Sonsuza kadar devam")
      ^ ASas */
}

impl BramaErrorType {
    pub fn as_text(&self) -> String {
        let message = match self {
            BramaErrorType::SyntaxError => "Sozdizimi hatasi",
            BramaErrorType::MoreThan1ArgumentPassed => "Birden fazla degisken kullanilamaz",
            BramaErrorType::RightParanthesesMissing => "Sağ parantaz eksik",
            BramaErrorType::AssertFailed => "Doğrulanamadı",
            BramaErrorType::NumberNotParsed => "Sayı ayrıştırılamadı",
            BramaErrorType::MissingStringDeliminator => "Yazı sonlandırıcısı bulunamadı",
            BramaErrorType::CharNotValid => "Karakter geçerli değil",
            BramaErrorType::RightSideOfExpressionNotFound => "İfadenin sağ tarafı bulunamadı",
            BramaErrorType::ReturnMustBeUsedInFunction => "Döndür komutu fonksiyon içinde kullanılmalıdır",
            BramaErrorType::FunctionCallSyntaxNotValid => "Fonksiyon çağırma sözdizimi geçerli değil",
            BramaErrorType::FunctionNameNotDefined => "Fonksiyon adı tanımlanmamış",
            BramaErrorType::ArgumentMustBeText => "Değişken yazı olmalıdır",
            BramaErrorType::IfConditionBodyNotFound => "Koşul gövdesi eksik",
            BramaErrorType::ParenthesesNotClosed => "Parantez kapatılmamış",
            BramaErrorType::InvalidUnaryOperation => "Geçersiz tekli işlem",
            BramaErrorType::UnaryWorksWithNumber => "Tekli numara ile çalışmaktadır",
            BramaErrorType::InvalidExpression => "Geçersiz ifade",
            BramaErrorType::ArgumentNotFound => "Parametre bulunamadı",
            BramaErrorType::MultipleElseUsageNotValid => "Birden fazla yada ifadesi kullanılamaz",
            BramaErrorType::BreakAndContinueBelongToLoops => "'kır' ve 'devamet' ifadeleri döngü içinde kullanılabilir",
            BramaErrorType::FunctionConditionBodyNotFound => "Fonksiyon içi kodlar bulunamadı",
            BramaErrorType::ColonMarkMissing => "':' eksik",
            BramaErrorType::ElseIsUsed => "'yada' zaten kullanıldı",
            BramaErrorType::IndentationIssue => "Girinti sorunu",
            BramaErrorType::DictNotClosed => "Sözlük düzgün kapatılmamış",
            BramaErrorType::ArrayNotClosed => "Dizi düzgün kapatılmadı",
            BramaErrorType::InvalidListItem => "Dizi elemanı geçersiz",
            BramaErrorType::DictionaryKeyNotValid => "Sözlük anahtarı geçersiz",
            BramaErrorType::DictionaryValueNotValid => "Sözlük geçeri geçersiz",
            BramaErrorType::CommentNotFinished => "Yorum bilgisi düzgün kapatılmadı",
            BramaErrorType::WhileStatementNotValid => "Döngü düzgün tanımlanmamış"
        };
        format!("(#{}) {}", *self as u8, message)
    }
}