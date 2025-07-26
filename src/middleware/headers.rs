use poem::{
    http::header::{
        CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
    },
    middleware::SetHeader,
};

/// Return a middleware that sets some static security headers.
pub fn security_headers() -> SetHeader {
    SetHeader::new()
        .appending(
            CONTENT_SECURITY_POLICY,
            "default-src 'self'; \
                frame-ancestors 'self'; \
                form-action 'self'; \
                img-src 'self' data: www.gravatar.com s.gravatar.com; \
                style-src 'self' 'unsafe-inline'; \
                script-src 'self' static.cloudflareinsights.com;",
        )
        .appending(
            "permissions-policy",
            "geolocation=(), microphone=(), camera=()",
        )
        .appending(REFERRER_POLICY, "strict-origin-when-cross-origin")
        .appending(X_CONTENT_TYPE_OPTIONS, "nosniff")
        .appending(X_FRAME_OPTIONS, "DENY")
}
