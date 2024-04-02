#[derive(Clone,Default)]
pub enum ErrorList{
    CheckSumErr,
    RepoCheckSumErr,
    FrequencyValueOutOfRange ,
    ResponesErr,
    OverLimit,
    NonResponse,
    CRCError,
    DeviceSNErr,
    StandByMode,
    BoardErr,
    #[default]
    None
}