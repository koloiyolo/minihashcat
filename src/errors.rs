#[derive(Debug, Clone)]
pub enum MiniHashcatError {
    FileNotFound { path: String },
}

impl MiniHashcatError {
    pub(crate) fn fine_not_found(path: String) -> Self {
        Self::FileNotFound { path }
    }
}
