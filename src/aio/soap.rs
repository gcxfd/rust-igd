use crate::errors::RequestError;

#[derive(Clone, Debug)]
pub struct Action(String);

impl Action {
  pub fn new(action: &str) -> Action {
    Action(action.into())
  }
}

const HEADER_NAME: &str = "SOAPAction";

pub async fn send_async(url: &str, action: Action, body: &str) -> Result<String, RequestError> {
  Ok(
    surf::post(url)
      .header(HEADER_NAME, action.0)
      .content_type("text/xml")
      .body(body)
      .recv_string()
      .await?,
  )
}
