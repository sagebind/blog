use std::{net::IpAddr, str::FromStr};

use poem::{web::RemoteAddr, FromRequest, Request, RequestBody, Result};

#[derive(Clone, Debug)]
pub struct ClientIp(pub IpAddr);

#[poem::async_trait]
impl<'a> FromRequest<'a> for ClientIp {
    async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self> {
        let forwarded_client = req
            .headers()
            .get("X-Forwarded-For")
            .and_then(|value| value.to_str().ok())
            .and_then(|s| s.split(',').next())
            .map(|s| s.trim())
            .and_then(|s| IpAddr::from_str(s).ok());

        if let Some(addr) = forwarded_client {
            Ok(Self(addr))
        } else {
            <&RemoteAddr>::from_request(req, body)
                .await
                .map(|remote_addr| Self(remote_addr.as_socket_addr().unwrap().ip()))
        }
    }
}
