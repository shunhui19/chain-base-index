pub struct Coiner {
    log_file: String,
    run_coin: String,
    max_threads: u32,
}

impl Coiner {
    fn new<'a>(run_coin: String) -> &'a Self {
        &Self {
            log_file: Default::default(),
            run_coin,
            max_threads: 8,
        }
    }
}
