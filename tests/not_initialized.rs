mod util;

mod not_init {
    use crate::util;

    #[test]
    fn tasks_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
        util::assert_fails_if_not_initialized(&["tasks"])
    }

    #[test]
    fn log_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
        util::assert_fails_if_not_initialized(&["log"])
    }

    #[test]
    fn details_not_initialized() -> Result<(), Box<dyn std::error::Error>> {
        util::assert_fails_if_not_initialized(&["details", "123"])
    }
}
