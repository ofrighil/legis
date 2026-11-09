pub(crate) fn canonical_url(base: &str, path: &str, mut query: Vec<(&str, String)>) -> String {
    query.sort_unstable();

    // assumption: query is a non-empty, unique vector
    let params: String = query
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<String>>()
        .join("&");
    format!("{base}/{path}?{params}")
}
