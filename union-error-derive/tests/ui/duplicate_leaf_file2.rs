use union_error::located_error;

#[located_error]
pub enum LocalErrors {
    ParseAgain(std::num::ParseIntError),
}
