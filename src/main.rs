mod tournament;

use std::io::Error;


#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    Ok(())
}
