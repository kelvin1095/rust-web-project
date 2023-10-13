

fn add_tracing(router: Router) -> Router {
  return router
      .layer(
          TraceLayer::new_for_http()
          .make_span_with(|request: &Request<_>| {

              // let matched_path = request
              //     .extensions()
              //     .get::<MatchedPath>()
              //     .map(MatchedPath::as_str);

              info_span!(
                  "http_request ",
                  // method = ?request.method(),
                  uri = ?request.uri().path(),
                  // user_agent = ?request.headers().get("User-Agent"),
                  // matched_path,
                  // some_other_field = tracing::field::Empty,
              )

          })
          .on_request(|request: &Request<_>, _span: &Span| {
              tracing::info!("request: {} {}", request.method(), request.uri().path())
          })
          .on_response(|response: &Response, latency: Duration, _span: &Span| {
              tracing::info!("response: {} in {:?}", response.status(), latency)
          })
          .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
              tracing::info!("sending {} bytes in {:?}", chunk.len(), latency)
          })
          .on_eos(|_trailers: Option<&HeaderMap>, stream_duration: Duration, _span: &Span| {
              tracing::info!("stream closed after {:?}", stream_duration)
          })
          .on_failure(|error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
              tracing::error!("error: {}", error)
          })
      )
}