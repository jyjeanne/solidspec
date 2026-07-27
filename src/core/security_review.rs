//! OWASP Top 10 heuristic audit for the `security-first` schema's
//! `security-review` artifact.
//!
//! Runs deterministic, no-LLM pattern checks against `plan.md` (and
//! `spec.md` for context) so `solidspec security-review` — and therefore
//! `solidspec pipeline --schema security-first --no-agent` — succeeds
//! without requiring an AI agent. An agent can still be layered on top via
//! the registered `/solidspec-security-review` slash command to go deeper
//! than these heuristics allow.

use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;

use super::errors::SolidSpecError;
use super::review::Severity;

/// One OWASP-flavored finding surfaced by the heuristic audit.
#[derive(Debug)]
pub struct SecurityFinding {
    pub category: &'static str,
    pub severity: Severity,
    pub message: String,
    pub remediation: String,
}

/// Full security-review result for one feature.
#[derive(Debug)]
pub struct SecurityReviewReport {
    pub feature_id: String,
    pub findings: Vec<SecurityFinding>,
}

struct OwaspCheck {
    category: &'static str,
    /// Terms whose presence in spec+plan text signal the concern applies.
    trigger: &'static Regex,
    /// Terms in plan.md whose presence means the concern already has a
    /// documented mitigation, so no finding is raised.
    mitigation: &'static Regex,
    severity: Severity,
    message: &'static str,
    remediation: &'static str,
}

static AUTHN_TRIGGER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(auth(entication|enticate)?|login|log[- ]?in|sign[- ]?in|passwords?|session|jwt|oauth|sso)\b")
        .expect("invalid authn trigger regex")
});
static AUTHN_MITIGATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(authentication|mfa|multi-factor|password hash|bcrypt|argon2|session\s+(expiry|timeout|invalidation))\b")
        .expect("invalid authn mitigation regex")
});

static AUTHZ_TRIGGER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(role|permission|admin|authoriz|access\s+control|rbac)\b")
        .expect("invalid authz trigger regex")
});
static AUTHZ_MITIGATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(authorization|rbac|role-based|least\s+privilege|access\s+control\s+(list|check))\b")
        .expect("invalid authz mitigation regex")
});

static INJECTION_TRIGGER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(sql|query|database|db\b|orm|raw\s+query|shell\s+command|exec\()")
        .expect("invalid injection trigger regex")
});
static INJECTION_MITIGATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(parameteriz|prepared\s+statement|sanitiz|input\s+valid|escap(e|ing)|orm\b)\b",
    )
    .expect("invalid injection mitigation regex")
});

static SENSITIVE_DATA_TRIGGER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(pii|personal\s+data|credit\s+card|payment|ssn|social\s+security|health\s+record|credential)\b")
        .expect("invalid sensitive data trigger regex")
});
static SENSITIVE_DATA_MITIGATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(encrypt|tls|https|at\s+rest|hashing|tokeniz|pci[- ]dss|redact)\b")
        .expect("invalid sensitive data mitigation regex")
});

static RATE_LIMIT_TRIGGER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(public\s+api|endpoint|webhook|external\s+request)\b")
        .expect("invalid rate limit trigger regex")
});
static RATE_LIMIT_MITIGATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(rate\s+limit|throttl|quota|backoff)\b")
        .expect("invalid rate limit mitigation regex")
});

static LOGGING_TRIGGER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(log(ging|s)?|audit\s+trail|monitor)\b")
        .expect("invalid logging trigger regex")
});
static LOGGING_MITIGATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\b(audit\s+log|structured\s+log|log\s+retention|no\s+sensitive\s+data\s+in\s+logs)\b",
    )
    .expect("invalid logging mitigation regex")
});

static OWASP_CHECKS: LazyLock<Vec<OwaspCheck>> = LazyLock::new(|| {
    vec![
        OwaspCheck {
            category: "A07:2021 – Identification and Authentication Failures",
            trigger: &AUTHN_TRIGGER,
            mitigation: &AUTHN_MITIGATION,
            severity: Severity::High,
            message: "Feature involves authentication/session concepts but plan.md documents no authentication mechanism, password handling, or session lifecycle.",
            remediation: "Document the authentication mechanism, password hashing algorithm, and session expiry/invalidation strategy in plan.md.",
        },
        OwaspCheck {
            category: "A01:2021 – Broken Access Control",
            trigger: &AUTHZ_TRIGGER,
            mitigation: &AUTHZ_MITIGATION,
            severity: Severity::High,
            message: "Feature involves roles/permissions but plan.md documents no authorization or access-control strategy.",
            remediation: "Document the authorization model (RBAC, ownership checks, least privilege) in plan.md.",
        },
        OwaspCheck {
            category: "A03:2021 – Injection",
            trigger: &INJECTION_TRIGGER,
            mitigation: &INJECTION_MITIGATION,
            severity: Severity::Critical,
            message: "Feature involves database/query/shell operations but plan.md documents no input validation, parameterization, or sanitization strategy.",
            remediation: "Use parameterized queries or an ORM, and document input validation/sanitization in plan.md.",
        },
        OwaspCheck {
            category: "A02:2021 – Cryptographic Failures",
            trigger: &SENSITIVE_DATA_TRIGGER,
            mitigation: &SENSITIVE_DATA_MITIGATION,
            severity: Severity::Critical,
            message: "Feature handles sensitive data (PII/payment/credentials) but plan.md documents no encryption-at-rest or in-transit strategy.",
            remediation: "Document encryption at rest and in transit (TLS), and data minimization/redaction, in plan.md.",
        },
        OwaspCheck {
            category: "A04:2021 – Insecure Design (Rate Limiting)",
            trigger: &RATE_LIMIT_TRIGGER,
            mitigation: &RATE_LIMIT_MITIGATION,
            severity: Severity::Medium,
            message: "Feature exposes a public endpoint/API but plan.md documents no rate limiting or throttling strategy.",
            remediation: "Document rate limiting, throttling, or quota enforcement for public-facing endpoints in plan.md.",
        },
        OwaspCheck {
            category: "A09:2021 – Security Logging and Monitoring Failures",
            trigger: &LOGGING_TRIGGER,
            mitigation: &LOGGING_MITIGATION,
            severity: Severity::Low,
            message: "Feature discusses logging/monitoring but plan.md doesn't document audit-log retention or sensitive-data exclusion from logs.",
            remediation: "Document audit log retention and confirm sensitive data (passwords, tokens, PII) is excluded from logs in plan.md.",
        },
    ]
});

/// Run the OWASP Top 10 heuristic audit against `spec.md` + `plan.md`.
///
/// Read-only — no file modifications. Requires `plan.md` to exist (the
/// `security-review` artifact's schema dependency); `spec.md` is read for
/// additional context if present.
pub fn run_security_review(feature_dir: &Path) -> Result<SecurityReviewReport> {
    let feature_id = feature_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let plan_path = feature_dir.join("plan.md");
    if !plan_path.exists() {
        return Err(SolidSpecError::Spec {
            feature_id,
            message: "plan.md not found".into(),
            fix: "Run 'solidspec plan' first.".into(),
        }
        .into());
    }
    let plan_content = std::fs::read_to_string(&plan_path)?;

    let spec_path = feature_dir.join("spec.md");
    let spec_content = std::fs::read_to_string(&spec_path).unwrap_or_default();

    let combined = format!("{spec_content}\n{plan_content}");
    let plan_lower = plan_content.to_lowercase();

    let mut findings = Vec::new();
    for check in &*OWASP_CHECKS {
        if check.trigger.is_match(&combined) && !check.mitigation.is_match(&plan_lower) {
            findings.push(SecurityFinding {
                category: check.category,
                severity: check.severity.clone(),
                message: check.message.to_string(),
                remediation: check.remediation.to_string(),
            });
        }
    }

    Ok(SecurityReviewReport {
        feature_id,
        findings,
    })
}

/// Render a `SecurityReviewReport` as the Markdown written to `security-review.md`.
pub fn format_security_review(report: &SecurityReviewReport) -> String {
    let mut out = format!("# Security Review: {}\n\n", report.feature_id);
    out.push_str("OWASP Top 10 heuristic audit of plan.md (and spec.md for context).\n\n");

    let critical = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::Critical)
        .count();
    let high = report
        .findings
        .iter()
        .filter(|f| f.severity == Severity::High)
        .count();

    out.push_str(&format!(
        "**Findings**: {} total ({} critical, {} high)\n\n",
        report.findings.len(),
        critical,
        high
    ));

    if report.findings.is_empty() {
        out.push_str(
            "No OWASP Top 10 concerns detected by the heuristic audit. \
             Run a deeper agent-driven review with `/solidspec-security-review` \
             before treating this as a full security sign-off.\n",
        );
        return out;
    }

    let severity_order = [
        Severity::Critical,
        Severity::High,
        Severity::Medium,
        Severity::Low,
        Severity::Info,
    ];

    out.push_str("## Findings by Severity\n\n");
    for sev in &severity_order {
        let sev_findings: Vec<_> = report
            .findings
            .iter()
            .filter(|f| &f.severity == sev)
            .collect();
        if sev_findings.is_empty() {
            continue;
        }

        out.push_str(&format!("### {sev}\n\n"));
        for finding in sev_findings {
            out.push_str(&format!(
                "- **[{}]** {}\n",
                finding.category, finding.message
            ));
            out.push_str(&format!("  *Mitigation task*: {}\n\n", finding.remediation));
        }
    }

    out.push_str(
        "## Next Steps\n\n\
         Every finding above must have a corresponding mitigation task in tasks.md \
         (see the security-first schema: tasks cannot be generated until this file \
         exists, and every finding should map to a task).\n",
    );

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_feature(dir: &Path, spec: &str, plan: &str) {
        std::fs::write(dir.join("spec.md"), spec).unwrap();
        std::fs::write(dir.join("plan.md"), plan).unwrap();
    }

    #[test]
    fn missing_plan_returns_error() {
        let dir = TempDir::new().unwrap();
        let err = run_security_review(dir.path()).unwrap_err();
        assert!(err.to_string().contains("plan.md not found"));
    }

    #[test]
    fn clean_plan_with_no_sensitive_topics_has_no_findings() {
        let dir = TempDir::new().unwrap();
        write_feature(
            dir.path(),
            "# Spec\nA static marketing page with no user data.\n",
            "# Plan\nServe pre-rendered HTML from a CDN.\n",
        );

        let report = run_security_review(dir.path()).unwrap();
        assert!(report.findings.is_empty());
    }

    #[test]
    fn auth_without_mitigation_raises_high_finding() {
        let dir = TempDir::new().unwrap();
        write_feature(
            dir.path(),
            "# Spec\nUsers log in with a password.\n",
            "# Plan\nStore the login form state in memory.\n",
        );

        let report = run_security_review(dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.category.contains("Authentication") && f.severity == Severity::High)
        );
    }

    #[test]
    fn auth_with_documented_mitigation_has_no_authn_finding() {
        let dir = TempDir::new().unwrap();
        write_feature(
            dir.path(),
            "# Spec\nUsers log in with a password.\n",
            "# Plan\nPasswords are hashed with argon2. Sessions expire after 30 minutes.\n",
        );

        let report = run_security_review(dir.path()).unwrap();
        assert!(
            !report
                .findings
                .iter()
                .any(|f| f.category.contains("Authentication"))
        );
    }

    #[test]
    fn sql_without_parameterization_raises_critical_finding() {
        let dir = TempDir::new().unwrap();
        write_feature(
            dir.path(),
            "# Spec\nSearch products by name.\n",
            "# Plan\nBuild a SQL query string from the search term and run it against the database.\n",
        );

        let report = run_security_review(dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.category.contains("Injection") && f.severity == Severity::Critical)
        );
    }

    #[test]
    fn payment_data_without_encryption_raises_critical_finding() {
        let dir = TempDir::new().unwrap();
        write_feature(
            dir.path(),
            "# Spec\nAccept credit card payment for checkout.\n",
            "# Plan\nSend the payment form to the backend for processing.\n",
        );

        let report = run_security_review(dir.path()).unwrap();
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.category.contains("Cryptographic") && f.severity == Severity::Critical)
        );
    }

    #[test]
    fn spec_missing_is_tolerated_plan_only_still_runs() {
        let dir = TempDir::new().unwrap();
        std::fs::write(
            dir.path().join("plan.md"),
            "# Plan\nStore passwords in plaintext.\n",
        )
        .unwrap();

        let report = run_security_review(dir.path()).unwrap();
        assert!(!report.findings.is_empty());
    }

    #[test]
    fn format_report_renders_markdown_with_severity_sections() {
        let report = SecurityReviewReport {
            feature_id: "001-checkout".into(),
            findings: vec![SecurityFinding {
                category: "A03:2021 – Injection",
                severity: Severity::Critical,
                message: "raw SQL".into(),
                remediation: "use parameterized queries".into(),
            }],
        };

        let md = format_security_review(&report);
        assert!(md.contains("# Security Review: 001-checkout"));
        assert!(md.contains("### CRITICAL"));
        assert!(md.contains("use parameterized queries"));
    }

    #[test]
    fn format_report_clean_says_no_concerns_detected() {
        let report = SecurityReviewReport {
            feature_id: "001-static".into(),
            findings: vec![],
        };
        let md = format_security_review(&report);
        assert!(md.contains("No OWASP Top 10 concerns detected"));
    }
}
