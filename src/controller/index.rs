#[get("/")]
pub fn index_router() -> &'static str {
    r#"three API: /mount /umount /ready
        API  URI: /mount
             BODY: {}

    body is json, and for cubefs client config.json content
"#
}
