use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotionToObsidianError {
    #[error("APIトークンが設定されていません")]
    NoToken,
    #[error("Notionのブロックの取得に失敗しました: {0}")]
    BlockRetrievalError(String),
    #[error("Notionのページの取得に失敗しました: {0}")]
    PageRetrievalError(String),
    #[error("変換処理に失敗しました: {0}")]
    ConversionError(String),
    #[error("ファイルの書き込みに失敗しました: {0}")]
    FileWriteError(String),
    #[error("環境変数が設定されていません: {0}")]
    EnvVarError(String),
}

pub type Result<T> = std::result::Result<T, NotionToObsidianError>;
