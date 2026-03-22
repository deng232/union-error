use union_error::located_error;

#[located_error]
pub enum LocalErrors {
    Parse(std::num::ParseIntError),
}
