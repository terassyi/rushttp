use crate::server::error::Error;
use crate::http::error::Error as HttpError;
use crate::http::response::Response;
use crate::http::status::StatusCode;
use crate::server::resource;
use crate::http::header::{Header, content_length};
// use crate::server::context::Context;

// call in handler function
pub fn response(status: u16, data: Option<&str>) -> Result<String, Error> {
    let builder = Response::builder()
        .status(StatusCode::from_u16(status)
            .map_err(|e| Error::from(HttpError::from(e)))?);

    let mut header = Header::new();
    header.parse("Content-Type: text/html");
    header.parse(&match data {
        Some(d) => content_length(d.len()),
        None => content_length(0)
    });

    let builder = builder.header(header);
    let res: Response<String> = match data {
        Some(d) => {
            // Response::from_parts(builder.parts(), body)
            builder.response(d.to_string())
        },
        None => builder.empty_response(),
    };
    let buf = res.format().map_err(|e| Error::from(HttpError::from(e)))?;
    Ok(buf)
}