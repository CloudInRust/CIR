use actix_web::{middleware::{Middleware, Response}, HttpResponse, HttpRequest, Error, Body};

pub struct ErrorTemplateHandler;

impl<S> Middleware<S> for ErrorTemplateHandler {
    fn response(&self, _: &HttpRequest<S>, resp: HttpResponse) -> Result<Response, Error> {
        let status_code = resp.status().as_u16();
        if status_code >= 400 && status_code <= 599 {
            if resp.body() == &Body::Empty {
                trace!("Replacing empty error response with error template");

                // TODO: Replace empty error pages with a simple template based on frontend design
            }
        }

        Ok(Response::Done(resp))
    }
}