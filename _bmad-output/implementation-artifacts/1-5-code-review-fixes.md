# Story 1.5: Code Review Fixes — Epic 1 Hardening

**Status:** done  
**Epic:** 1 — Secure Local Service & Foundation  
**Story ID:** 1.5  
**Created:** 2026-04-12  
**Implemented:** 2026-04-12  

---

## User Story

As a developer,
I want to harden the Epic 1 implementation based on code review findings,
So that session authentication, deployment, and health checks are production-ready.

---

## Implementation Tasks

- [x] T1: Fix Session Token Tampering — Require HMAC Signature for All Tokens
  - [x] S1.1: Remove unsigned token acceptance path (middleware.rs line 70)
  - [x] S1.2: Ensure all tokens require HMAC verification
  - [x] S1.3: Test that unsigned tokens are rejected

- [x] T2: Externalize HMAC Key — Load from Environment
  - [x] S2.1: Add `get_session_hmac_key()` function to load from `SESSION_HMAC_KEY` env var
  - [x] S2.2: Update `generate_signed_session_cookie()` to use externalized key
  - [x] S2.3: Update `verify_signed_session_cookie()` to use externalized key
  - [x] S2.4: Provide fallback with deprecation warning for development
  - [x] S2.5: Update famfin.service to include SESSION_HMAC_KEY in EnvironmentFile

- [x] T3: Validate Session ID Format
  - [x] S3.1: Add UUID validation function
  - [x] S3.2: Validate session_id in `verify_signed_session_cookie()`
  - [x] S3.3: Return descriptive error if format invalid

- [x] T4: Improve Health Endpoint Response Validation
  - [x] S4.1: Update justfile deploy target to use JSON grep instead of substring
  - [x] S4.2: Validate JSON structure, not just substring match
  - [x] S4.3: Graceful fallback for grep-based validation

- [x] T5: Add Database Lock Poisoning Logging
  - [x] S5.1: Log error when database mutex is poisoned
  - [x] S5.2: Include context in error message

- [x] T6: Differentiate Session Validation Errors
  - [x] S6.1: Distinguish expired sessions (401) from server errors (500)
  - [x] S6.2: Log actual validation errors for debugging
  - [x] S6.3: Return specific error messages to client

- [x] T7: Fix Health Check Race Condition — Check Dependencies
  - [x] S7.1: Update `health_check()` to validate database connectivity
  - [x] S7.2: Return SERVICE_UNAVAILABLE (503) if dependencies not ready
  - [x] S7.3: Update deploy target to handle 503 responses

- [x] T8: Clarify AC5 Spec (Story 1.4)
  - [x] S8.1: Decide on RUST_LOG behavior: restart-based configuration
  - [x] S8.2: Update story 1.4 AC5 with chosen behavior
  - [x] S8.3: Document in systemd service file

---

## Dev Notes

**Security Focus:**
- T1 & T2: Session token tampering is highest priority (security impact)
- T3: Session ID validation prevents confusion between formats
- T5 & T6: Error handling prevents information leakage

**Operational Focus:**
- T4: Health check robustness ensures deploy reliability
- T7: Dependency checking prevents "false healthy" states
- T8: Spec clarity prevents future misimplementations

**Implementation Strategy:**
- Fix T1 first (session tampering) — blocker for auth safety
- Fix T2 next (externalize key) — depends on T1 working
- Fix T3-T7 in parallel (no dependencies)
- Resolve T8 (spec) as decision point for T7

---

## Dev Agent Record

### Implementation Summary
- ✅ T1-T8 completed: 8 security and robustness fixes applied
- ✅ Initial code review identified 9 findings; 7 classified as "patch"
- ✅ All 7 patches implemented and validated
- ✅ Final code review identified 2 additional improvements
- ✅ Both improvements applied and tested

### Code Review Findings & Resolution

**First Review (Pre-fix):**
- 2 HIGH findings: Session tampering, HMAC key hardcoded → Fixed T1, T2
- 5 MEDIUM findings: Health check, error handling, race condition → Fixed T4-T7
- 2 MINOR findings: AC5 ambiguity, operational concerns → Fixed T8, deferred
- 1 MINOR: Acceptable (Bearer token flexibility) → Rejected as noise

**Final Review (Post-fix):**
- UUID validation strengthened: 8-4-4-4-12 segment validation added
- Health check timeout: 5-second timeout on database query added
- All acceptance criteria passed
- No remaining blockers

### Completion Notes
- ✅ All 8 implementation tasks completed and verified
- ✅ Both final review recommendations applied
- ✅ Code compiles cleanly (pre-existing warnings only)
- ✅ Production-ready hardening complete

---

## File List
- [x] `famfin-backend/src/auth/middleware.rs` — Removed unsigned token acceptance; added logging for lock poisoning and validation errors
- [x] `famfin-backend/src/auth/mod.rs` — Externalized HMAC key via `get_session_hmac_key()`; added UUID validation; improved error handling
- [x] `famfin-backend/src/main.rs` — Updated health_check to validate database connectivity; returns 503 if unhealthy
- [x] `justfile` — Updated health check to validate JSON structure and handle 503 responses
- [x] `famfin.service` — Added SESSION_HMAC_KEY environment variable
- [x] `1-4-pi-3b-deployment-and-systemd-service.md` — Clarified AC5 to specify RUST_LOG via EnvironmentFile with restart requirement

---

## Change Log
- [x] 2026-04-12: Story created from code review findings
- [x] 2026-04-12: T1 Complete — Session token tampering fixed (require HMAC for all tokens)
- [x] 2026-04-12: T2 Complete — HMAC key externalized from environment; UUID validation added
- [x] 2026-04-12: T4 Complete — Health check JSON validation improved
- [x] 2026-04-12: T5 Complete — Database lock poisoning now logged
- [x] 2026-04-12: T6 Complete — Session validation errors differentiated (401 vs 500)
- [x] 2026-04-12: T7 Complete — Health check now validates database; returns 503 if unhealthy
- [x] 2026-04-12: T8 Complete — AC5 spec clarified (RUST_LOG via EnvironmentFile with restart)
- [x] 2026-04-12: All 8 tasks complete; code review fixes applied

---
