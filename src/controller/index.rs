#[get("/")]
pub fn index() -> &'static str {
    r#"three API: /mount /umount /ready
        API  URI: /mount
             BODY: {}

    body is json, and for cubefs client config.json content
"#
}
