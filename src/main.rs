use std::io;
use std::process::ExitCode;

use rs_str2alnum::bind;
use rs_str2alnum::lift;

fn envkey2val(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(io::Error::other)
}

fn allow_under_score_string() -> Result<String, io::Error> {
    envkey2val("ENV_ALLOW_UNDER_SCORE")
}

fn string2bool(s: String) -> Result<bool, io::Error> {
    str::parse(s.as_str()).map_err(io::Error::other)
}

fn allow_under_score() -> Result<bool, io::Error> {
    bind!(allow_under_score_string, lift!(string2bool))()
}

fn sub() -> Result<(), io::Error> {
    let allow_us: bool = allow_under_score().unwrap_or(false);
    rs_str2alnum::str2alnum::stdin2stdout(allow_us)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
