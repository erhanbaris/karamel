
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaErrorType {
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
    MissingIf = 132
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
    let lines = data.split(|c| c == '\n').collect::<Vec<_>>();
    let line = lines.iter().skip(error.line as usize).next().unwrap();
    return format!("...\r\n{}\r\n{:>width$} [{}:{}] {}", line, "^", error.line, error.column, error.error_type.as_text(),  width=error.column as usize);
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
            BramaErrorType::MultipleElseUsageNotValid => "Birden fazla 'yoksa' ifadesi kullanılamaz",
            BramaErrorType::BreakAndContinueBelongToLoops => "'kır' ve 'devam' ifadeleri döngü içinde kullanılabilir",
            BramaErrorType::FunctionConditionBodyNotFound => "Fonksiyon içi kodlar bulunamadı",
            BramaErrorType::ColonMarkMissing => "':' eksik",
            BramaErrorType::ElseIsUsed => "'yoksa' zaten kullanıldı",
            BramaErrorType::IndentationIssue => "Girinti sorunu",
            BramaErrorType::DictNotClosed => "Sözlük düzgün kapatılmamış",
            BramaErrorType::ArrayNotClosed => "Dizi düzgün kapatılmadı",
            BramaErrorType::InvalidListItem => "Dizi elemanı geçersiz",
            BramaErrorType::DictionaryKeyNotValid => "Sözlük anahtarı geçersiz",
            BramaErrorType::DictionaryValueNotValid => "Sözlük geçeri geçersiz",
            BramaErrorType::CommentNotFinished => "Yorum bilgisi düzgün kapatılmadı",
            BramaErrorType::WhileStatementNotValid => "Döngü düzgün tanımlanmamış",
            BramaErrorType::FunctionDefinationNotValid => "Fonksiyon tanımlaması hatalı",
            BramaErrorType::MissingIf => "'ise' sözcüğü eksik"
        };
        format!("(#{}) {}", *self as u8, message)
    }
}