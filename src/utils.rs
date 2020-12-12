// handy utilities


use std::str::FromStr;


/// Read lines from a file and parse them into a vector; blows up if the file is not found.
/// The vector is a vector of Results.
/// Use like
///     let things = read_file::<Thing>("filename");
pub fn read_file<T: FromStr>(file_name: &str) -> Vec<Result<T, <T as FromStr>::Err>> {
    std::fs::read_to_string(file_name)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse())
        .collect()
}



