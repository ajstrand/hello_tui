use regex::Regex;
use serde_json;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

#[derive(Debug, Clone)]
pub struct LintIssue {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: LintSeverity,
    pub rule_name: String,
}

pub struct Linter {
    enabled: bool,
    regex_cache: HashMap<String, Regex>,
}

impl Linter {
    pub fn new() -> Self {
        Self {
            enabled: true,
            regex_cache: HashMap::new(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn lint(&mut self, content: &str, filename: Option<&str>) -> Vec<LintIssue> {
        if !self.enabled {
            return Vec::new();
        }

        let mut issues = Vec::new();

        // Add universal linting rules
        issues.extend(self.lint_universal(content));

        // Language-specific linting based on file extension
        if let Some(filename) = filename {
            if filename.ends_with(".rs") {
                issues.extend(self.lint_rust(content));
            } else if filename.ends_with(".js") || filename.ends_with(".ts") {
                issues.extend(self.lint_javascript(content));
            } else if filename.ends_with(".py") {
                issues.extend(self.lint_python(content));
            } else if filename.ends_with(".json") {
                issues.extend(self.lint_json(content));
            }
        }

        issues
    }

    fn get_regex(&mut self, key: &str, pattern: &str) -> Option<&Regex> {
        if !self.regex_cache.contains_key(key) {
            if let Ok(regex) = Regex::new(pattern) {
                self.regex_cache.insert(key.to_string(), regex);
            }
        }
        self.regex_cache.get(key)
    }

    fn lint_universal(&mut self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        for (line_idx, line) in content.lines().enumerate() {
            let line_number = line_idx + 1;

            // Check for trailing whitespace
            if line.ends_with(' ') || line.ends_with('\t') {
                issues.push(LintIssue {
                    line: line_number,
                    column: line.trim_end().len() + 1,
                    message: "Trailing whitespace".to_string(),
                    severity: LintSeverity::Info,
                    rule_name: "trailing-whitespace".to_string(),
                });
            }

            // Check for long lines (>100 characters)
            if line.len() > 100 {
                issues.push(LintIssue {
                    line: line_number,
                    column: 101,
                    message: "Line too long (>100 characters)".to_string(),
                    severity: LintSeverity::Warning,
                    rule_name: "long-line".to_string(),
                });
            }

            // Check for mixed indentation (tabs and spaces)
            if line.starts_with(' ') && line.contains('\t') {
                issues.push(LintIssue {
                    line: line_number,
                    column: 1,
                    message: "Mixed indentation (tabs and spaces)".to_string(),
                    severity: LintSeverity::Warning,
                    rule_name: "mixed-indentation".to_string(),
                });
            }
        }

        issues
    }

    fn lint_rust(&mut self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        for (line_idx, line) in content.lines().enumerate() {
            let line_number = line_idx + 1;

            // Check for .unwrap() usage - comprehensive pattern
            if let Some(regex) = self.get_regex("unwrap_call", r"\.unwrap\(\)") {
                for mat in regex.find_iter(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Avoid using .unwrap(), consider using .expect() with a descriptive message or proper error handling".to_string(),
                        severity: LintSeverity::Warning,
                        rule_name: "avoid-unwrap".to_string(),
                    });
                }
            }

            // Check for missing semicolons in statements
            if let Some(regex) = self.get_regex("missing_semicolon", r"^\s*(println!|print!|return\s+[^;]+|let\s+.*=\s*[^;]+)\s*$") {
                if regex.is_match(line) && !line.trim().ends_with('{') && !line.trim().ends_with(',') {
                    issues.push(LintIssue {
                        line: line_number,
                        column: line.len() + 1,
                        message: "Missing semicolon".to_string(),
                        severity: LintSeverity::Error,
                        rule_name: "missing-semicolon".to_string(),
                    });
                }
            }

            // Check for unused variables
            if let Some(regex) = self.get_regex("unused_variable", r"let\s+_*([a-zA-Z_][a-zA-Z0-9_]*)\s*=") {
                if let Some(captures) = regex.captures(line) {
                    if let Some(var_name) = captures.get(1) {
                        let var = var_name.as_str();
                        // Simple check: if variable name starts with underscore, it's intentionally unused
                        if !var.starts_with('_') && !content.contains(&format!("{}", var)) {
                            issues.push(LintIssue {
                                line: line_number,
                                column: var_name.start() + 1,
                                message: format!("Variable '{}' may be unused. Consider prefixing with '_' if intentional", var),
                                severity: LintSeverity::Info,
                                rule_name: "unused-variable".to_string(),
                            });
                        }
                    }
                }
            }

            // Check for panic!() usage
            if let Some(regex) = self.get_regex("panic_call", r"panic!\s*\(") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Consider using Result<T, E> or expect() instead of panic!()".to_string(),
                        severity: LintSeverity::Warning,
                        rule_name: "avoid-panic".to_string(),
                    });
                }
            }
        }

        issues
    }

    fn lint_javascript(&mut self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        // Enhanced JavaScript linting with Biome-style rules
        issues.extend(self.lint_javascript_enhanced(content));

        issues
    }

    fn lint_javascript_enhanced(&mut self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        for (line_idx, line) in content.lines().enumerate() {
            let line_number = line_idx + 1;

            // Check for console.log usage
            if let Some(regex) = self.get_regex("console_log", r"console\.log\s*\(") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Avoid console.log in production code (biome-style: no-console-log)".to_string(),
                        severity: LintSeverity::Warning,
                        rule_name: "biome/no-console-log".to_string(),
                    });
                }
            }

            // Check for == usage (suggest ===)
            if let Some(regex) = self.get_regex("loose_equality", r"\s==\s") {
                if let Some(mat) = regex.find(line) {
                    if !line.contains("===") {
                        issues.push(LintIssue {
                            line: line_number,
                            column: mat.start() + 1,
                            message: "Use '===' instead of '==' for strict equality (biome-style: use-strict-equality)".to_string(),
                            severity: LintSeverity::Error,
                            rule_name: "biome/use-strict-equality".to_string(),
                        });
                    }
                }
            }

            // Check for var usage
            if let Some(regex) = self.get_regex("var_usage", r"\bvar\s+") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Use 'let' or 'const' instead of 'var' (biome-style: no-var)".to_string(),
                        severity: LintSeverity::Error,
                        rule_name: "biome/no-var".to_string(),
                    });
                }
            }

            // Check for debugger statements
            if let Some(regex) = self.get_regex("debugger_usage", r"\bdebugger\s*;?") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Remove debugger statements (biome-style: no-debugger)".to_string(),
                        severity: LintSeverity::Error,
                        rule_name: "biome/no-debugger".to_string(),
                    });
                }
            }

            // Check for unused variables (simplified)
            if let Some(regex) = self.get_regex("js_unused_var", r"(let|const|var)\s+([a-zA-Z_$][a-zA-Z0-9_$]*)\s*=") {
                if let Some(captures) = regex.captures(line) {
                    if let Some(var_name) = captures.get(2) {
                        let var = var_name.as_str();
                        if !var.starts_with('_') && !content[line_idx..].contains(var) {
                            issues.push(LintIssue {
                                line: line_number,
                                column: var_name.start() + 1,
                                message: format!("Variable '{}' is declared but never used (biome-style: no-unused-variables)", var),
                                severity: LintSeverity::Warning,
                                rule_name: "biome/no-unused-variables".to_string(),
                            });
                        }
                    }
                }
            }

            // Check for double negation
            if let Some(regex) = self.get_regex("double_negation", r"!!\s*\w") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Use Boolean() instead of double negation (!!) (biome-style: no-double-negation)".to_string(),
                        severity: LintSeverity::Info,
                        rule_name: "biome/no-double-negation".to_string(),
                    });
                }
            }

            // Check for empty blocks
            if let Some(regex) = self.get_regex("empty_block", r"\{\s*\}") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Empty block statement (biome-style: no-empty-block)".to_string(),
                        severity: LintSeverity::Warning,
                        rule_name: "biome/no-empty-block".to_string(),
                    });
                }
            }

            // Check for function spacing
            if let Some(regex) = self.get_regex("function_spacing", r"function\s*\(\s*\)\s*\{") {
                if regex.is_match(line) && !line.contains("function ()") {
                    issues.push(LintIssue {
                        line: line_number,
                        column: 1,
                        message: "Consider proper spacing around function declarations (biome-style: formatting)".to_string(),
                        severity: LintSeverity::Info,
                        rule_name: "biome/function-spacing".to_string(),
                    });
                }
            }

            // Check for semicolon consistency
            if let Some(regex) = self.get_regex("statement_without_semicolon", r"^\s*[a-zA-Z_$].*[^;{}\s]\s*$") {
                if regex.is_match(line) && !line.trim().ends_with(',') && !line.trim().starts_with("//") && !line.trim().starts_with("/*") {
                    issues.push(LintIssue {
                        line: line_number,
                        column: line.len() + 1,
                        message: "Missing semicolon (biome-style: use-semicolons)".to_string(),
                        severity: LintSeverity::Warning,
                        rule_name: "biome/use-semicolons".to_string(),
                    });
                }
            }
        }

        issues
    }

    fn lint_python(&mut self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        for (line_idx, line) in content.lines().enumerate() {
            let line_number = line_idx + 1;

            // PEP 8: Line length should be <= 79 characters
            if line.len() > 79 {
                issues.push(LintIssue {
                    line: line_number,
                    column: 80,
                    message: "Line too long (PEP 8 recommends ≤79 characters)".to_string(),
                    severity: LintSeverity::Info,
                    rule_name: "pep8-line-length".to_string(),
                });
            }

            // Check for improper indentation (should be 4 spaces)
            if line.starts_with(' ') && !line.starts_with("    ") {
                let leading_spaces = line.len() - line.trim_start().len();
                if leading_spaces % 4 != 0 && leading_spaces > 0 {
                    issues.push(LintIssue {
                        line: line_number,
                        column: 1,
                        message: "PEP 8: Use 4 spaces per indentation level".to_string(),
                        severity: LintSeverity::Warning,
                        rule_name: "pep8-indentation".to_string(),
                    });
                }
            }

            // Check for missing space after comma
            if let Some(regex) = self.get_regex("comma_spacing", r",[^\s]") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 2,
                        message: "PEP 8: Missing whitespace after ','".to_string(),
                        severity: LintSeverity::Info,
                        rule_name: "pep8-comma-spacing".to_string(),
                    });
                }
            }

            // Check for print statements (suggest logging)
            if let Some(regex) = self.get_regex("print_statement", r"\bprint\s*\(") {
                if let Some(mat) = regex.find(line) {
                    issues.push(LintIssue {
                        line: line_number,
                        column: mat.start() + 1,
                        message: "Consider using logging instead of print for production code".to_string(),
                        severity: LintSeverity::Hint,
                        rule_name: "prefer-logging".to_string(),
                    });
                }
            }
        }

        issues
    }

    fn lint_json(&self, content: &str) -> Vec<LintIssue> {
        let mut issues = Vec::new();

        match serde_json::from_str::<serde_json::Value>(content) {
            Ok(_) => {
                // Valid JSON, check for common style issues
                for (line_idx, line) in content.lines().enumerate() {
                    let line_number = line_idx + 1;
                    
                    // Check for trailing commas (not allowed in JSON)
                    if line.trim().ends_with(',') && (line.contains('}') || line.contains(']')) {
                        issues.push(LintIssue {
                            line: line_number,
                            column: line.rfind(',').unwrap_or(0) + 1,
                            message: "Trailing comma not allowed in JSON".to_string(),
                            severity: LintSeverity::Error,
                            rule_name: "no-trailing-comma".to_string(),
                        });
                    }
                }
            }
            Err(e) => {
                // Parse the error to get line information if possible
                let error_msg = e.to_string();
                let line = if error_msg.contains("line") {
                    // Try to extract line number from error message
                    error_msg
                        .split("line ")
                        .nth(1)
                        .and_then(|s| s.split(' ').next())
                        .and_then(|s| s.parse::<usize>().ok())
                        .unwrap_or(1)
                } else {
                    1
                };

                issues.push(LintIssue {
                    line,
                    column: 1,
                    message: format!("JSON syntax error: {}", e),
                    severity: LintSeverity::Error,
                    rule_name: "json-syntax".to_string(),
                });
            }
        }

        issues
    }

    pub fn get_issue_counts(&self, issues: &[LintIssue]) -> (usize, usize, usize, usize) {
        let mut errors = 0;
        let mut warnings = 0;
        let mut infos = 0;
        let mut hints = 0;

        for issue in issues {
            match issue.severity {
                LintSeverity::Error => errors += 1,
                LintSeverity::Warning => warnings += 1,
                LintSeverity::Info => infos += 1,
                LintSeverity::Hint => hints += 1,
            }
        }

        (errors, warnings, infos, hints)
    }
}
