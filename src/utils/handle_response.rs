use anyhow::Result;
use elasticsearch::http::response::Response;
use serde_json::Value;

use super::output::{output_error_table, output_json, print_error, print_success, Output};

pub async fn handle_response(
    output: &Output,
    response: Response,
    success_msg: String,
    error_msg: String,
    pretty: bool,
) -> Result<()> {
    if !response.status_code().is_success() {
        let ex = response.exception().await?.unwrap();
        let reason = ex.error().reason().unwrap_or("");
        let status_code = ex.status().unwrap_or(0).to_string();

        match output {
            Output::Default => {
                print_error(error_msg);
                output_error_table(reason, &status_code);
            }
            Output::Json => output_json(ex.error(), pretty)?,
        }
    } else {
        match output {
            Output::Default => print_success(success_msg),
            Output::Json => {
                let response_body: Value = response.json().await?;
                output_json(&response_body, pretty)?
            }
        };
    }
    Ok(())
}
