use std::{fs, path::PathBuf};
use utoipa::openapi::OpenApi;

fn main() {
    for spec in [
        PathBuf::new().join("specs/charted.json"),
        PathBuf::new().join("specs/stripe.json"),
        PathBuf::new().join("specs/youtrack.json"),
    ] {
        println!("--> {}", spec.display());

        let contents = fs::read_to_string(&spec).expect("to read the contents");
        match serde_json::from_str::<OpenApi>(&contents) {
            Ok(_) => panic!("this should never happen"),
            Err(e) => {
                eprintln!(
                    "[{}] failed in category {:?}, line {}, column {}: {e}",
                    spec.display(),
                    e.classify(),
                    e.line(),
                    e.column(),
                );
            }
        }
    }
}
