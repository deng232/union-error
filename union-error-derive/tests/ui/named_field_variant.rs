use union_error::located_error;

#[located_error]
enum LocalErrors {
    Parse { source: std::num::ParseIntError },
}

fn main() {}
