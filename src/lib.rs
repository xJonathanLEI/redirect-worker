use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn handle_fetch(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Prints error message to console on panic
    utils::set_panic_hook();

    let response_code = match env.var("RESPONSE_CODE") {
        Ok(response_code) => response_code.to_string().parse::<u16>().unwrap_or(302),
        Err(_) => 302,
    };

    if let Ok(target_full_url) = env.var("TARGET_FULL_URL") {
        let target_full_url = target_full_url.to_string();
        if !target_full_url.is_empty() {
            return Response::redirect_with_status(target_full_url.parse::<Url>()?, response_code);
        }
    }

    if let Ok(target_base_url) = env.var("TARGET_BASE_URL") {
        let mut target_url = target_base_url.to_string();
        if !target_url.is_empty() {
            if target_url.ends_with('/') {
                target_url.pop();
            }

            let req_url = req.url()?;
            target_url.push_str(req_url.path());
            if let Some(query) = req_url.query() {
                target_url.push('?');
                target_url.push_str(query);
            }

            return Response::redirect_with_status(target_url.parse::<Url>()?, response_code);
        }
    }

    Response::error("Redirect URL not set", 500)
}
