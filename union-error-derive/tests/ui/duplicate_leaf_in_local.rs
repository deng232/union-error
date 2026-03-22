use union_error::located_error;

#[located_error]
enum LocalErrors {
    Parse(std::num::ParseIntError),
    ParseAgain(std::num::ParseIntError),
}

fn main() {}
