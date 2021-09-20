use std::env;

lazy_static! {
    // Application Settings
    pub static ref HOST: String =
        env::var("JANUARY_HOST").expect("Missing JANUARY_HOST environment variable.");
}
