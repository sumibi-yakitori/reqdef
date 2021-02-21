#[macro_use]
extern crate lazy_static;

use reqwest::blocking::Client as BlockingClient;
use reqwest::Client;
pub use reqwest::*;

lazy_static! {
  static ref CLIENT: Client = Client::new();
  static ref BLOCKING_CLIENT: BlockingClient = BlockingClient::new();
}

pub fn client() -> &'static Client {
  &CLIENT
}

pub fn blocking_client() -> &'static BlockingClient {
  &BLOCKING_CLIENT
}
