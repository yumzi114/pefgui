#[derive(Clone,Default)]
pub enum ErrorList{
    CheckSumErr,
    ResponesErr,
    OverLimit,
    NonResponse,
    CRCError,
    #[default]
    None
}