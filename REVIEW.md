# Code Review Instructions

These instructions guide the review process for pull requests in this repository.

## What Not to Review

Please do not review pull requests if:
- They contain only changes in the `monitoring-manifests` directory.
- The pull request description contains the phrase: "AI, PLEASE DO NOT REVIEW THIS PR".

## Goals

The primary goal is to find real, actionable issues that could lead to:
- Bugs
- Security problems
- Data inconsistencies
- Incorrect behavior

## Scope and Constraints

- **Focus on the Diff:** Base your review entirely on the information visible in the pull request diff.
- **Changed Lines Only:** Restrict your comments to lines that were actually changed or added in the PR.
- **No Assumptions:** Do not make assumptions about behaviors in other files, external systems, or runtime environments.
- **High Signal-to-Noise:** Prefer making zero comments over leaving low-confidence or low-value remarks.

## What to Avoid Commenting On

- **Style and Refactoring:** Avoid commenting on naming conventions, documentation, or refactoring opportunities unless they are actively preventing a tangible bug.
- **Preferences:** Do not leave comments based purely on code style or architectural preferences.
- **Automated Checks:** Assume that automated linters and formatters are already enforcing style rules.
- **Speculation:** Do not speculate, guess the author's intent, or suggest "consider" or "might be" improvements unless an issue is directly provable from the diff.

## Formatting Comments

Every review comment must follow this structure:

1. **Quote:** Include the exact line(s) from the diff that you are referring to.
2. **Issue:** State the problem clearly in one single sentence.
3. **Fix:** Propose a concrete fix or a safer alternative (limit to 1-3 sentences).
4. **Severity:** Assign one of the following severities:
   - 🔴 `blocker`
   - 🟠 `major`
   - 🟡 `minor`
5. **Confidence:** Assign a confidence level:
   - ✅ `high`
   - ⚠️ `medium`
   - *Note: Do not post comments if your confidence is `low`.*
