use crate::jwt;
use actix_web::middleware::identity::*;
use actix_web::middleware::{Middleware, Started};
use actix_web::{HttpRequest, Result};

use crate::controllers::*;
use db::models::*;

pub struct JWTMiddleWare; // <- Our middleware

/// Middleware implementation, middlewares are generic over application state,
/// so you can access state with `HttpRequest::state()` method.
impl<S> Middleware<S> for JWTMiddleWare {
    /// Method is called when request is ready. It may return
    /// future, which should resolve before next middleware get called.
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        req.identity().and_then(|to| {
            let token = jwt::decode::<JWTClaims>(&to, "666".as_ref(), &jwt::Validation::default());
            println!("{:?}", token);
            if let Ok(t) = token {
                let refresh = t.claims.refresh;
                let now =
                    NaiveDateTime::from_timestamp(Local::now().timestamp(), 6).timestamp() as usize;
                if now > refresh {
                    let jwt = JWT {
                        user_id: t.claims.user_id.to_owned(),
                    };
                    req.remember(jwt.token().unwrap());
                };
                Some(())
            } else {
                req.forget();
                None
            }
        });
        Ok(Started::Done)
    }
}
