pub const SERVER_URL: &str = "baca.ii.uj.edu.pl";
pub const PERMUTATION: &str = "5A4AE95C27260DF45F17F9BF027335F6";
pub const EMPTY_RESPONSE: &str = "//OK[0,[],0,7]";

pub fn permutation() -> String {
    PERMUTATION.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_function_should_return_same_string() {
        assert_eq!(permutation(), PERMUTATION);
    }
}
