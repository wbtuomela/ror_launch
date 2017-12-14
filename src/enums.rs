#[derive(Debug)]
pub enum ELanguage {
    English,
    French,
    German,
    Italian,
    Spanish,
    Korean,
    Chinese,
    Japanese,
    Russian,
}

#[derive(Debug)]
pub enum EOpCode {
    ClCheck = 1,
    LcrCheck,
    ClStart,
    LcrStart,
    ClCreate,
    LcrCreate,
    ClInfo,
    LcrInfo,
}

#[derive(Debug)]
pub enum ECheckResult {
    OK,
    Version,
    File,
}
