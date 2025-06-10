use url::form_urlencoded::Serializer;

#[derive(Debug)]
pub struct BugReportStep {
    pub message: String,
    pub reasoning: usize,
    pub tool_calls: usize,
}

pub fn build_bug_report_url(
    steps: &[BugReportStep],
    version: &str,
    model: &str,
    platform: &str,
) -> String {
    let mut serializer = Serializer::new(String::new());
    serializer.append_pair("template", "2-bug-report.yml");
    serializer.append_pair("labels", "bug");
    serializer.append_pair("version", version);
    serializer.append_pair("model", model);
    serializer.append_pair("platform", platform);

    if !steps.is_empty() {
        let mut bullets = String::new();
        for step in steps {
            if !bullets.is_empty() {
                bullets.push('\n');
            }
            let msg = step.message.replace('\n', " ");
            bullets.push_str(&format!(
                "- ```\n  {}\n  ```\n  - `{}` reasoning | `{}` tool`",
                msg, step.reasoning, step.tool_calls
            ));
        }
        serializer.append_pair("steps", &bullets);
    }

    format!(
        "https://github.com/openai/codex/issues/new?{}",
        serializer.finish()
    )
}
