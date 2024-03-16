#[derive(Clone,Default)]
pub enum ErrorList{
    CheckSumErr,
    RepoCheckSumErr,
    FrequencyValueOutOfRange ,
    ResponesErr,
    OverLimit,
    NonResponse,
    CRCError,
    #[default]
    None
}