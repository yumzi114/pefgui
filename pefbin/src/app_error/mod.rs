#[derive(Clone,Default)]
pub enum ErrorList{
    CheckSumErr,
    FrequencyValueOutOfRange ,
    ResponesErr,
    OverLimit,
    NonResponse,
    CRCError,
    #[default]
    None
}