use crate::server::error::Error;
use crate::http::error::Error as HttpError;
use crate::http::response::Response;
use crate::http::status::StatusCode;
use crate::server::resource;
// use crate::server::context::Context;

// call in handler function
pub fn response(status: u16, path: Option<&str>) -> Result<(), Error> {
    let builder = Response::builder()
        .status(StatusCode::from_u16(status)
            .map_err(|e| Error::from(HttpError::from(e)))?);
    // let res: Response<String> = match path {
    //     Some(p) => {
    //         let path = resource::get_path(ctx.root.to_str().unwrap(), p);
    //         let body = resource::read(path.to_str().unwrap())?;
    //         // Response::from_parts(builder.parts(), body)
    //         builder.response(body)
    //     },
    //     None => builder.empty_response(),
    // };
    Ok(())
}