//! This module contains generic re-usable responses with a 2xx response code.
//!

use poem_extensions::OneResponse;

#[derive(OneResponse)]
#[oai(status = 200)]
pub(crate) struct EmptyOK;

#[derive(OneResponse)]
#[oai(status = 204)]
/// # NO CONTENT
///
/// The operation completed successfully, but there is no data to return.
///
/// ## NO DATA BODY IS RETURNED FOR THIS RESPONSE
pub(crate) struct NoContent;
