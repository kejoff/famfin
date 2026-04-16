---
stepsCompleted: ["step-01-validate-prerequisites", "step-02-design-epics", "step-03-create-stories", "step-04-final-validation"]
inputDocuments:
  - "_bmad-output/planning-artifacts/prd.md"
  - "_bmad-output/planning-artifacts/architecture.md"
---

# famfin - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for famfin, decomposing the requirements from the PRD and Architecture into implementable stories.

## Requirements Inventory

### Functional Requirements

FR1: User can import transaction data from CSV or OFX files via drag-and-drop
FR2: System can provide step-by-step export guides for each supported French bank (BNP Paribas, Société Générale, Crédit Agricole, La Banque Postale, CIC, LCL)
FR3: System can detect recurring credit patterns and infer household net monthly income, presenting the inference to the user for single-action confirmation
FR4: User can import up to 36 months of historical transaction data in a single session
FR5: User can export all household financial data (transactions, categories, settings) as a portable file
FR6: System can automatically assign a category to each imported transaction using an ML model trained on the household's own transaction history
FR7: User can manually reassign a category to any individual transaction
FR8: User can reassign all transactions from a given merchant to a category in a single action, with the change applied to the full transaction history
FR9: System can retain all manual category corrections and apply them automatically to future transactions from the same merchant without re-prompting
FR10: User can request LLM-assisted categorization for uncategorized transactions using a configured AI provider
FR11: User can mark any expense as one-time to exclude it from trend and projection calculations
FR12: System can generate a 3–6 month forward spending projection based on categorized transaction history
FR13: System can calculate and display the household's current absorption capacity — the maximum unexpected expense absorbable without derailing any active goal
FR14: System can detect month-over-month spending anomalies by category and surface them with a one-time vs. trend classification prompt
FR15: System can detect recurring subscription payments within transaction history
FR16: User can review detected recurring subscriptions and mark each as active, cancelled, or under review
FR17: System can display month-over-month spending delta by category
FR18: User can view a desktop dashboard presenting full financial summary: spending by category, 3–6 month projection, absorption capacity, and active goal progress
FR19: User can view a mobile consultation interface presenting absorption capacity and trajectory status as the primary signal, accessible in under 3 seconds
FR20: System can display a trajectory status signal (on track / drifting / alert) based on current spending versus projected plan
FR21: System can display contextual reassurance messaging after an unexpected expense is logged, confirming whether the household trajectory remains intact
FR22: System can maintain consistent financial state across desktop and mobile interfaces without requiring manual refresh
FR23: System can present an intention-first onboarding flow in which the user states their financial goal before providing any financial data
FR24: System can present a consent screen to each household member on first device access, before any financial data is visible, requiring affirmative acknowledgment of shared financial visibility
FR25: User can complete full initial setup and first import following guided per-bank instructions
FR26: System can require session password authentication before granting access to any financial data (active by default, opt-out available)
FR27: System can store all financial data in an encrypted local database
FR28: System can display clear, non-alarming error messages when import, AI provider connection, or calendar sync operations fail
FR29: User can configure and validate an AI provider connection via a settings interface
FR30: System can create and maintain a recurring monthly calendar event on the household's self-hosted calendar server
FR31: System can be installed as a PWA on mobile devices for home-screen access
FR32: System can display a direct link to the open source codebase from within the application

### NonFunctional Requirements

NFR-P1: Mobile dashboard Time To Interactive ≤ 3s on local network; target met when VPN latency ≤ 50ms (environmental precondition)
NFR-P2: CSV/OFX import of 12 months (≤ 1,500 transactions) including ML categorization completes in < 30s on reference hardware (ARM Cortex-A72 quad-core, 4 GB RAM — Pi 4 equivalent). Note: must revalidate on Pi 3B (Cortex-A53, 1 GB shared); T460 test assertions calibrated at ×4–6 factor (import 1,500 tx < 6s, export 7,500 tx < 2s)
NFR-P3: Read operations (navigation, consultation, filtering) respond in < 500ms p95 on local network; write operations (correction, batch recategorization) respond in < 2s
NFR-P4: 3–6 month projection recalculates in < 3s after any category or income change
NFR-P5: Loading indicator displayed within 200ms of navigation; last-known values shown immediately from browser localStorage while full fetch runs in background (no white screen)
NFR-S1: Entire database encrypted with SQLCipher (AES-256); no financial data stored in plaintext on server
NFR-S2: All client-server communications via HTTPS (TLS 1.2 minimum); unencrypted HTTP blocked or redirected
NFR-S3: Session password authentication active by default; session duration configurable (default 8h); password reset via direct server access only — no online recovery mechanism
NFR-S4: LLM payloads contain only raw transaction label after automatic removal of IBAN patterns, card numbers, and amounts; account, holder, balance, and identifying aggregate fields never transmitted; diagnostic mode available to log outgoing LLM payloads for verification
NFR-S5: Each household member must explicitly confirm shared financial data visibility on first access from a new browser
NFR-R1: Core functions (consultation, categorization, projection) operate without external services; loss of connectivity to Ollama, Claude API, or Radicale does not interrupt main functionality
NFR-R2: If configured LLM provider is unavailable, categorization automatically falls back to local ML model; user informed via plain-language message without jargon or alarm
NFR-R3: If Radicale is unavailable during monthly calendar event generation, operation is retried on next app open; import is not blocked
NFR-R4: Re-importing an already-imported CSV/OFX file creates no duplicate transactions; deduplication uses composite fingerprint: date + amount + normalized label (lowercase, trim, remove punctuation) + ordinal index in source file
NFR-R5: All data (transactions, categories, manual corrections, configuration) exportable as CSV/JSON at any time; export of 5-year history (≈ 7,500 transactions) completes in < 10s
NFR-R6: Service available (HTTP 200 health check) within 60s of process start on reference ARM quad-core hardware
NFR-I1: Calendar event creation compatible with CalDAV standard (RFC 4791); Radicale connection configurable via URL and credentials in settings
NFR-I2: Both LLM providers (Ollama local, Claude API) independently configurable; provider switch requires no server restart
NFR-I3: CSV and OFX files from Crédit Agricole and La Banque Postale natively supported; adding a new bank profile requires no modification to core code
NFR-M1: Reproducible deployment: install.sh script provided and documented; fresh install on Debian/Ubuntu produces operational service in < 30 minutes on reference hardware
NFR-M2: Observability: all background operations (import, LLM call, CalDAV sync) generate structured logs viewable from admin interface; persistent errors (≥ 3 consecutive failures) visible on dashboard without alarming language
NFR-M3: Database migrations applied automatically at startup; rollback to previous version possible without data loss via pre-migration backup restore
NFR-A1: Traffic-light visual indicators (green/orange/red) meet WCAG AA contrast ratio (4.5:1) and are supplemented by text label or icon — color is never the sole signal
NFR-A2: Interface respects browser prefers-color-scheme: dark preference; critical indicators remain readable and distinguishable in dark mode without contrast degradation
NFR-L1: Interface entirely in French; no UI string displayed in English in default configuration
NFR-UX1: All user-visible error messages written in natural language without technical jargon; they explain what happened and propose a concrete action or confirm the system handled the situation automatically

### Additional Requirements

- **Stack:** Rust (Axum 0.8) backend + SvelteKit (Svelte 5) frontend; target deployment binary `aarch64-unknown-linux-gnu` (Pi 3B); development on T460 x86_64
- **Project scaffold & build pipeline:** justfile with `build-arm`, `deploy`, `train` targets; cross-compilation via `cross` (Docker-based); first implementation story must set up this pipeline
- **Database migrations:** `refinery` crate — versioned `.sql` files, embedded in binary, applied at startup with mandatory timeout (prevents systemd restart loop on long migration)
- **Async model:** rusqlite synchronous in Tokio `spawn_blocking`; `worker_threads = 2` configured in Rust code (not justfile) to limit memory contention on Pi 3B
- **Session auth:** HMAC-signed httpOnly cookie; server-side state in SQLite `sessions` table; flags: `HttpOnly; Secure; SameSite=Strict; Max-Age=28800`; generic auth error messages (no enumeration)
- **LLM API key storage:** stored in SQLCipher DB (not plaintext config file); accessible only via app settings interface
- **LLM abstraction:** `LlmProvider` trait with `SanitizedPayload` type — compiler enforces sanitization before any LLM call; `MockLlmProvider` for integration tests
- **ML pipeline:** training on T460 (Python/scikit-learn), exported to ONNX; inference on Pi via `ort` crate loaded at Axum startup; ONNX model must be compiled on T460 before first Pi deployment
- **localStorage cache:** derived non-sensitive metrics only — absorption capacity (€), trajectory signal (green/orange/red), last-updated timestamp; never: transactions, amounts, merchant names, balances, session tokens, API keys
- **Logging:** `tracing` crate → stdout → captured by journald; `journalctl -u famfin -f`; default INFO in production, DEBUG via env var
- **Testing environment:** T460 x86_64, SQLite in-memory (not SQLCipher in tests); `axum::test` harness + `MockLlmProvider`; performance assertions calibrated with ×4–6 factor for Pi 3B
- **systemd:** unit file `famfin.service` — dedicated non-root user, restart on failure, auto-start at boot
- **Pi 3B benchmark:** baseline benchmark to run at first deployment to calibrate actual T460→Pi3B performance multiplier

### UX Design Requirements

No UX Design document found. UI requirements are derived from PRD interface strategy (FR18, FR19, dual-mode desktop/mobile design) and NFR-A1, NFR-A2, NFR-L1, NFR-UX1.

### FR Coverage Map

FR1: Epic 2 — CSV/OFX drag-and-drop import
FR2: Epic 2 — Per-bank export guides
FR3: Epic 2 — Revenue auto-detection and confirmation
FR4: Epic 2 — 36-month historical import
FR5: Epic 5 — Full data export (CSV/JSON)
FR6: Epic 2 — ML auto-categorization (ONNX inference)
FR7: Epic 2 — Individual transaction recategorization
FR8: Epic 2 — Batch merchant recategorization applied to full history
FR9: Epic 2 — Learning engine retention of manual corrections
FR10: Epic 2 — LLM-assisted categorization for uncategorized transactions
FR11: Epic 2 — One-time expense tagging (excluded from projections)
FR12: Epic 3 — 3–6 month forward spending projection
FR13: Epic 3 — Absorption capacity calculation and display
FR14: Epic 3 — Month-over-month anomaly detection with one-time/trend prompt
FR15: Epic 3 — Recurring subscription detection
FR16: Epic 3 — Subscription review interface (active / cancelled / under review)
FR17: Epic 3 — Month-over-month spending delta by category
FR18: Epic 4 — Desktop dashboard (full ritual mode)
FR19: Epic 4 — Mobile consultation interface (absorption capacity primary signal)
FR20: Epic 4 — Trajectory status signal (on track / drifting / alert)
FR21: Epic 4 — Contextual reassurance messaging after unexpected expense
FR22: Epic 4 — Consistent financial state across desktop and mobile
FR23: Epic 2 — Intention-first onboarding flow
FR24: Epic 2 — Per-device consent screen (shared financial visibility)
FR25: Epic 2 — Guided per-bank first import setup
FR26: Epic 1 — Session password authentication (active by default)
FR27: Epic 1 — SQLCipher-encrypted local database
FR28: Epic 5 — Clear, non-alarming error messages for all failure modes
FR29: Epic 2 — LLM provider configuration and validation
FR30: Epic 4 — Recurring monthly CalDAV ritual event (Radicale)
FR31: Epic 4 — PWA installable to mobile home screen
FR32: Epic 5 — Direct link to open source codebase

## Epic List

### Epic 1: Secure Local Service & Foundation
A running famfin service is accessible on the home network, protected by session authentication and backed by an AES-256 encrypted database. The build and deployment pipeline to the Raspberry Pi 3B is operational.
**FRs covered:** FR26, FR27
**NFRs addressed:** NFR-S1, NFR-S2, NFR-S3, NFR-R6, NFR-M1, NFR-M3
**Architecture prerequisites:** Rust/SvelteKit scaffold, justfile cross-compilation pipeline (aarch64), refinery migrations, HMAC httpOnly cookie session, SQLCipher, systemd unit file

### Epic 2: First Import & ML Categorization
A household member can import up to 36 months of bank transactions (CSV/OFX) following guided per-bank instructions, complete the intention-first onboarding and per-device consent flow, and immediately see transactions automatically categorized by the household's own ML model. They can correct categories individually or by batch merchant, the system learns from every correction, and an LLM provider can fill gaps. One-time expenses are taggable.
**FRs covered:** FR1, FR2, FR3, FR4, FR6, FR7, FR8, FR9, FR10, FR11, FR23, FR24, FR25, FR29
**NFRs addressed:** NFR-P2, NFR-R4, NFR-S4, NFR-S5, NFR-R2, NFR-I2, NFR-I3
**Architecture prerequisites:** ONNX model trained on household data (T460) and deployed to Pi before first import; LlmProvider trait + SanitizedPayload; MockLlmProvider for tests
**Epic validation criterion (FR25):** A household member with no prior famfin experience can complete the full flow — intention-first onboarding → per-device consent → per-bank guide → CSV import → categorized transactions visible — without external assistance.

### Epic 3: Financial Analysis & Projection
The system generates a 3–6 month spending projection, calculates and displays the household's absorption capacity, detects month-over-month anomalies and recurring subscriptions, and shows spending deltas by category. All calculations update within 3 seconds of any data change.
**FRs covered:** FR12, FR13, FR14, FR15, FR16, FR17
**NFRs addressed:** NFR-P4, NFR-R1

### Epic 4: Monthly Ritual — Dashboard, Mobile & CalDAV
Both partners have a complete desktop dashboard for the monthly financial review and a mobile interface that answers "can we afford this?" in under 3 seconds. The trajectory signal is always visible. A recurring CalDAV ritual event is auto-generated on the household's Radicale server. The app is installable as a PWA.
**FRs covered:** FR18, FR19, FR20, FR21, FR22, FR30, FR31
**NFRs addressed:** NFR-P1, NFR-P3, NFR-P5, NFR-A1, NFR-A2, NFR-L1, NFR-UX1, NFR-R3, NFR-I1

### Epic 5: Data Resilience & Configuration
The household can export all financial data at any time (CSV/JSON), all error messages are written in natural language without technical jargon, and the app displays a direct link to its open source codebase.
**FRs covered:** FR5, FR28, FR32
**NFRs addressed:** NFR-R5, NFR-M2

---

## Epic 5: Data Resilience & Configuration

The household can export all financial data at any time (CSV/JSON), all error messages are written in natural language without technical jargon, and the app displays a direct link to its open source codebase.

### Story 5.1: Full Data Export (CSV/JSON)

As a household member,
I want to export all my financial data — transactions, categories, manual corrections, and settings — as a portable file,
So that I am never locked into famfin and can restore or migrate my data at any time.

**Acceptance Criteria:**

**Given** the user navigates to the export section
**When** they trigger a full data export
**Then** a ZIP archive is generated containing:
- all transactions in CSV format (date, amount, label, category, one-time flag)
- all merchant correction rules in CSV format
- all settings and configuration in JSON format (excluding LLM API keys and session secrets)

**Given** a household with 5 years of transaction history (≈ 7,500 transactions)
**When** the export is triggered
**Then** the archive is ready for download in under 10 seconds (NFR-R5)

**Given** the export is triggered
**When** the archive is generated
**Then** no LLM API keys, session tokens, or SQLCipher encryption keys are included in the export

**Given** the exported CSV files
**When** opened in a standard spreadsheet application
**Then** the columns are correctly delimited, encoded in UTF-8, and human-readable without transformation

### Story 5.2: Operational Observability & Persistent Error Display

As a household member,
I want background operation failures surfaced on my dashboard in plain language, and structured logs available when I need to diagnose a problem,
So that I know when something is wrong without being alarmed by technical noise.

**Acceptance Criteria:**

**Given** any background operation (import, LLM call, CalDAV sync) runs
**When** it completes — success or failure
**Then** a structured log entry is written via the `tracing` crate and captured by journald
**And** the log entry includes: operation type, duration, outcome, and any error detail (server-side only — never shown to user)

**Given** a background operation fails
**When** it has failed 3 or more consecutive times
**Then** a non-alarming indicator is displayed on the dashboard (e.g. "Calendar sync hasn't succeeded recently — check your Radicale connection in settings")
**And** the message uses plain natural language without error codes, stack traces, or technical jargon (NFR-UX1)

**Given** the admin log interface is accessed
**When** recent background operations are listed
**Then** import, LLM call, and CalDAV sync events are visible with their outcome and timestamp
**And** the list is filterable by operation type and outcome (success / failure)

**Given** import, LLM provider connection, or calendar sync fails
**When** the error is displayed to the user
**Then** the message explains what happened, whether it was handled automatically, and what action (if any) the user can take (FR28, NFR-UX1)

### Story 5.3: Open Source Codebase Link

As a household member,
I want to see a direct link to famfin's open source codebase from within the application,
So that the "no conflicts of interest" claim is auditable, not just declared.

**Acceptance Criteria:**

**Given** the user is on any page of the application
**When** they navigate to the About or Settings section
**Then** a clearly labelled link to the public source code repository is displayed

**Given** the link is displayed
**When** the user clicks it
**Then** it opens the repository in a new browser tab

**Given** the link text and surrounding copy
**When** rendered
**Then** the text is in French and explains the purpose of the link in one sentence (e.g. "famfin est open source — le code est consultable ici") (NFR-L1)

---

## Epic 4: Monthly Ritual — Dashboard, Mobile & CalDAV

Both partners have a complete desktop dashboard for the monthly financial review and a mobile interface that answers "can we afford this?" in under 3 seconds. The trajectory signal is always visible. A recurring CalDAV ritual event is auto-generated on the household's Radicale server. The app is installable as a PWA.

### Story 4.1: Desktop Dashboard — Full Ritual View

As a household member on desktop,
I want a complete financial summary dashboard showing spending by category, projection, absorption capacity, and goal progress,
So that I can conduct a full 30-minute monthly financial review with my partner without switching between screens.

**Acceptance Criteria:**

**Given** the user is authenticated and has imported transaction data
**When** the desktop dashboard loads
**Then** the following are visible without scrolling: spending by category (current month), 3–6 month projection, absorption capacity in euros, and active goal progress indicators

**Given** the desktop dashboard is displayed
**When** a category is selected
**Then** the transactions contributing to that category are shown inline with their individual amounts and labels

**Given** the desktop dashboard loads
**When** previously cached metric values exist in localStorage (absorption capacity, trajectory signal, last-updated timestamp)
**Then** those values are displayed immediately while the full data fetch runs in the background (NFR-P5 stale-while-revalidate)
**And** the display updates silently when the fresh data arrives — no white screen or loading spinner blocking the view

**Given** the desktop dashboard
**When** any text, label, or UI element is rendered
**Then** all strings are in French (NFR-L1)
**And** error states use plain-language natural language messages without technical jargon (NFR-UX1)

**Given** visual indicators on the dashboard (trajectory, deltas)
**When** color is used as a signal
**Then** a text label or icon supplements the color so color is never the sole distinguishing signal (NFR-A1)
**And** all indicators remain readable in dark mode (`prefers-color-scheme: dark`) without contrast degradation (NFR-A2)

### Story 4.2: Mobile Consultation Interface

As a household member on mobile,
I want to see my absorption capacity and trajectory status as the primary signal in under 3 seconds,
So that I can answer "can we afford this?" during the day without opening a laptop.

**Acceptance Criteria:**

**Given** the user opens famfin on a mobile browser on the local network or via VPN
**When** the page loads
**Then** the absorption capacity in euros is visible above the fold within 3 seconds Time To Interactive (NFR-P1)
**And** the trajectory status signal (green / orange / red + text label) is visible above the fold

**Given** the mobile interface loads
**When** cached metric values exist in localStorage
**Then** absorption capacity and trajectory signal are shown immediately from cache while the background fetch completes (NFR-P5)

**Given** the mobile interface is displayed
**When** the user scrolls below the fold
**Then** month-to-date spending summary and active goal progress are accessible as secondary information

**Given** the mobile interface
**When** any action that would modify data is attempted (import, correction, category change)
**Then** the action is not available — mobile is read-only consultation only

**Given** the desktop and mobile interfaces both show the absorption capacity
**When** data changes on one interface
**Then** the other interface reflects the updated value on next load without requiring a manual cache clear (FR22)

### Story 4.3: Trajectory Status Signal

As a household member,
I want a clear visual signal indicating whether my household spending is on track, drifting, or in alert territory,
So that I can assess our financial direction at a glance without interpreting raw numbers.

**Acceptance Criteria:**

**Given** categorized transaction history and a projection exist
**When** the trajectory signal is calculated
**Then** the signal is one of three states: On Track (green), Drifting (orange), Alert (red)
**And** the state is determined by the ratio of current month spending pace to the projected monthly baseline:
- **On Track**: current pace ≤ 100% of projected baseline
- **Drifting**: current pace between 101% and 120% of projected baseline
- **Alert**: current pace > 120% of projected baseline

**Given** the trajectory signal is displayed
**When** rendered in any interface (desktop or mobile)
**Then** the color signal is always accompanied by a text label ("On track", "Drifting", "Alert") — color is never the sole signal (NFR-A1)
**And** the signal remains distinguishable in dark mode (NFR-A2)

**Given** the trajectory signal is shown on the mobile interface
**When** the page loads
**Then** the signal is visible above the fold alongside the absorption capacity

**Given** a category correction or new import changes the spending data
**When** the trajectory is recalculated
**Then** the signal updates within 3 seconds (NFR-P4)

### Story 4.4: Contextual Reassurance Messaging After Unexpected Expense

As a household member,
I want the app to confirm whether my household trajectory remains intact after I log an unexpected expense,
So that I can absorb a financial shock with clarity rather than anxiety.

**Acceptance Criteria:**

**Given** a transaction is marked as a one-time expense (Story 2.7)
**When** the projection and absorption capacity recalculate
**Then** a contextual message is displayed confirming the trajectory status: e.g. "This month absorbed a €870 shock. Your trajectory remains intact."

**Given** the unexpected expense exceeds the current absorption capacity
**When** the message is displayed
**Then** the message acknowledges the impact plainly without alarming language and indicates what has changed in the projection

**Given** the reassurance message is displayed
**When** rendered in any interface
**Then** the message is written in natural French without technical jargon (NFR-UX1, NFR-L1)

**Given** no one-time expense has been logged in the current session
**When** the dashboard is viewed
**Then** no reassurance message is shown — the message only appears contextually after a relevant action

### Story 4.5: Monthly CalDAV Ritual Event (Radicale)

As a household,
I want a recurring monthly "Financial Review" calendar event auto-generated on our Radicale calendar server,
So that the 30-minute ritual is blocked in both partners' calendars without manual setup.

**Acceptance Criteria:**

**Given** Radicale CalDAV URL and credentials are configured in settings
**When** the configuration is saved and validated
**Then** a recurring monthly "Revue financière — 30 min" event is created on the Radicale server via CalDAV (RFC 4791, NFR-I1)
**And** the event recurs on the 1st of each month at the configured time

**Given** the CalDAV event has been created
**When** it is viewed in the household's calendar client (Nextcloud, Thunderbird, etc.)
**Then** the event appears as a recurring event with the correct title, duration, and recurrence rule

**Given** Radicale is unavailable when the event creation is attempted
**When** the failure occurs
**Then** the operation is queued and retried on the next app open (NFR-R3)
**And** the import or onboarding flow is not blocked by the CalDAV failure

**Given** the CalDAV connection test in settings
**When** the user triggers the test
**Then** a plain-language success or failure message is returned within 5 seconds (NFR-UX1)

### Story 4.6: PWA Installation on Mobile Home Screen

As a household member,
I want to install famfin as a PWA on my phone's home screen,
So that I can open it with one tap like a native app without navigating to a URL.

**Acceptance Criteria:**

**Given** the user opens famfin in a supported mobile browser (Chrome 120+, Safari 17+)
**When** the PWA manifest is served correctly
**Then** the browser presents a native "Add to Home Screen" prompt or the option is available in the browser menu

**Given** famfin is installed as a PWA
**When** the user taps the home screen icon
**Then** the app opens in standalone mode (no browser chrome) and displays the mobile consultation interface

**Given** the PWA manifest
**When** inspected
**Then** it includes the app name ("famfin"), icons, `display: standalone`, and the correct `start_url`

**Given** the PWA installation (V1 scope)
**When** the device is offline or VPN is disconnected
**Then** no offline caching is active — the app shows a connection error (V1 explicitly excludes offline caching; VPN access makes it unnecessary)

---

## Epic 3: Financial Analysis & Projection

The system generates a 3–6 month spending projection, calculates and displays the household's absorption capacity, detects month-over-month anomalies and recurring subscriptions, and shows spending deltas by category. All calculations update within 3 seconds of any data change.

### Story 3.1: 3–6 Month Spending Projection

As a household member,
I want to see a 3–6 month forward spending projection based on my categorized transaction history,
So that I can understand where my money is headed and identify which categories consume disproportionate resources.

**Acceptance Criteria:**

**Given** at least 2 months of categorized transaction history exists
**When** the projection view is loaded
**Then** a 3–6 month forward projection is displayed broken down by spending category
**And** one-time expenses (Story 2.7) are excluded from the category trends used in the projection

**Given** the projection is displayed
**When** a manual category correction or income update is made
**Then** the projection recalculates and updates within 3 seconds (NFR-P4)

**Given** fewer than 2 months of transaction history exist
**When** the projection view is loaded
**Then** a plain-language message explains that more data is needed and indicates how many months are available

**Given** the projection calculation
**When** no LLM or external service is available
**Then** the projection runs entirely from local data without any external dependency (NFR-R1)

### Story 3.2: Absorption Capacity Calculation & Display

As a household member,
I want to see my household's current absorption capacity — the maximum unexpected expense I can absorb without derailing any active goal,
So that I can answer "can we afford this?" with a real number instead of anxiety.

**Acceptance Criteria:**

**Given** categorized transaction history and household income are available
**When** the dashboard loads
**Then** the absorption capacity is displayed in euros as the primary metric
**And** it is calculated as: average monthly surplus (income minus average monthly spending) × 3 months horizon
**And** in V1 there are no active goals — "committed goal contributions" = €0 (goals are a V2 feature)

**Given** the absorption capacity is displayed
**When** a one-time expense is logged or a category correction changes the monthly surplus
**Then** the absorption capacity recalculates and updates within 3 seconds (NFR-P4)

**Given** the absorption capacity is shown for the first time to a user
**When** the value is displayed
**Then** a brief in-context explanation is available on demand (not always visible) describing what the number means and how it is calculated

**Given** the absorption capacity is displayed on both desktop and mobile
**When** either interface is loaded
**Then** the same value is shown without requiring manual refresh (FR22 dependency)

### Story 3.3: Month-over-Month Anomaly Detection

As a household member,
I want the system to detect unusual spending in a category compared to prior months and ask me whether it's a one-time event or a new trend,
So that I can contextualize anomalies without them distorting my projections.

**Acceptance Criteria:**

**Given** at least 2 months of categorized transaction history
**When** a new import is processed
**Then** the system identifies categories where spending deviates significantly from the prior month average
**And** each anomaly is surfaced with a prompt: "Is this a one-time expense or a new recurring pattern?"

**Given** an anomaly is surfaced
**When** the user selects "one-time"
**Then** the transaction(s) driving the anomaly are flagged as one-time (same mechanism as Story 2.7)
**And** the anomaly is dismissed from the review queue

**Given** an anomaly is surfaced
**When** the user selects "recurring"
**Then** the anomaly is dismissed and the new spending level is incorporated into the projection baseline

**Given** an anomaly detection run
**When** no external service is available
**Then** detection runs entirely on local data (NFR-R1)

### Story 3.4: Recurring Subscription Detection & Review

As a household member,
I want the system to detect recurring subscription payments and let me review their status,
So that I can identify forgotten subscriptions and make conscious decisions about each one.

**Acceptance Criteria:**

**Given** transaction history contains repeating charges of the same amount from the same merchant at regular intervals (monthly, annual)
**When** subscription detection runs after an import
**Then** each detected subscription is listed with merchant name, amount, frequency, and last charge date

**Given** the subscription list is displayed
**When** the user reviews a detected subscription
**Then** they can mark it as: Active (keep), Cancelled (remove from projections), or Under Review (flag for discussion)

**Given** a subscription is marked as Cancelled
**When** the projection recalculates
**Then** the cancelled subscription amount is removed from the forward projection (NFR-P4 — within 3 seconds)

**Given** a subscription is marked as Under Review
**When** the dashboard is viewed
**Then** the subscription appears in a "pending decision" list until it is resolved

**Given** subscription detection runs
**When** no external service is available
**Then** detection runs entirely on local transaction data (NFR-R1)

### Story 3.5: Month-over-Month Spending Delta by Category

As a household member,
I want to see how my spending in each category changed compared to the previous month,
So that I can immediately spot progress (food spend down €80) or unexpected increases without manual calculation.

**Acceptance Criteria:**

**Given** at least 2 months of categorized transaction history
**When** the spending delta view is loaded
**Then** each spending category displays the current month total, the previous month total, and the difference in euros and percentage

**Given** the delta view is displayed
**When** a category shows a positive delta (spending increased)
**Then** the increase is visually distinguished from a decrease — using a non-color-only signal (NFR-A1: label or icon in addition to color)

**Given** a new import is processed
**When** the delta view is refreshed
**Then** the deltas update to reflect the new data within 500ms (NFR-P3 read operation)

**Given** a category has only one month of data
**When** the delta view is displayed
**Then** that category shows the current month total with an indicator that no prior month comparison is available

---

## Epic 2: First Import & ML Categorization

A household member can import up to 36 months of bank transactions (CSV/OFX) following guided per-bank instructions, complete the intention-first onboarding and per-device consent flow, and immediately see transactions automatically categorized by the household's own ML model. They can correct categories individually or by batch merchant, the system learns from every correction, and an LLM provider can fill gaps. One-time expenses are taggable.

### Story 2.1: ML Training Pipeline & ONNX Model Deployment

As a developer,
I want a Python training pipeline that exports a scikit-learn model to ONNX and a Rust inference layer that loads it at startup,
So that transaction categorization is available from the very first import without cold-start.

**Acceptance Criteria:**

**Given** a CSV of historical household transactions with known categories
**When** `just train` is run on the T460
**Then** a `models/model.onnx` file is produced and a training report (accuracy, category distribution) is printed to stdout

**Given** the `models/model.onnx` file exists on the Pi at the configured path
**When** the famfin binary starts
**Then** the ONNX model is loaded into memory via the `ort` crate before the HTTP server begins accepting requests
**And** a startup log entry confirms model loading with the model file path and load time

**Given** the ONNX model file is missing at startup
**When** the binary starts
**Then** the service starts successfully but logs a WARNING that ML categorization is unavailable
**And** the health check endpoint returns `200 OK` (service is degraded, not broken)

**Given** the `just deploy` target
**When** run after a model retrain
**Then** `models/model.onnx` is included in the scp transfer alongside the binary

### Story 2.2: Intention-First Onboarding, Consent Screen & Per-Bank Guides

As a household member accessing famfin for the first time,
I want to state my financial intention before seeing any data and explicitly consent to shared financial visibility,
So that the app feels aligned with my goals from the start and both partners have given informed consent.

**Acceptance Criteria:**

**Given** a browser that has never accessed famfin before
**When** the user navigates to the app after authentication
**Then** a full-screen intention prompt is shown before any financial data is visible
**And** the prompt asks the user to describe their financial goal in free text (e.g. "Stop feeling like we're behind")
**And** the intention is saved and displayed as context on the dashboard

**Given** the intention has been submitted
**When** the consent screen is shown for the first time on this device
**Then** a full-screen consent step requires affirmative acknowledgment of shared financial visibility before proceeding
**And** consent is stored per browser (not per user account) — re-shown on a new device

**Given** the consent has been confirmed
**When** the user reaches the import screen for the first time
**Then** a per-bank guide selector is shown listing all supported French banks (BNP Paribas, Société Générale, Crédit Agricole, La Banque Postale, CIC, LCL)
**And** selecting a bank displays step-by-step instructions for exporting a CSV/OFX from that bank's online interface

**Given** a returning user on a previously consented device
**When** the user navigates to the app after authentication
**Then** the intention prompt and consent screen are skipped and the dashboard is shown directly

### Story 2.3: CSV/OFX Import Engine with Revenue Detection & Deduplication

As a household member,
I want to import my bank transactions via drag-and-drop and have the system detect my household income automatically,
So that I can see a complete picture of my finances after a single import session.

**Acceptance Criteria:**

**Given** the import screen is displayed
**When** a CSV or OFX file is dragged and dropped onto the import zone
**Then** the file is parsed and a preview of the first 10 transactions is shown with detected columns (date, amount, label)
**And** a progress indicator appears within 200ms of drop (NFR-P5)

**Given** a valid CSV/OFX file from a supported French bank
**When** the import is confirmed
**Then** up to 36 months of transactions are imported and stored in the encrypted database (NFR-S1)
**And** the import of 1,500 transactions completes in under 6 seconds on the T460 (NFR-P2 T460-calibrated assertion)
**And** duplicate transactions are not created if the same file is imported again (NFR-R4 composite fingerprint: date + amount + normalized label + ordinal index)

**Given** the imported transactions contain recurring credits matching a consistent monthly amount
**When** the import completes
**Then** the system surfaces a single-action confirmation prompt: "I see recurring credits of €X/month — is that your household income?"
**And** confirming sets the household net monthly income used in projections (FR3)
**And** declining allows the user to set income manually

**Given** a CSV file from an unsupported bank format
**When** the file is dropped
**Then** a plain-language error message is shown explaining the format is not recognized and suggesting the per-bank guide (NFR-UX1)
**And** the import is not partially committed

### Story 2.4: Automatic ML Categorization via ONNX Inference

**Depends on:** Story 2.1 (ONNX model must be trained and deployed to Pi before this story can be tested end-to-end)

As a household member,
I want my imported transactions to be automatically categorized by a model trained on my household's own history,
So that I immediately see meaningful spending categories without manual work.

**Acceptance Criteria:**

**Given** transactions have been imported and the ONNX model is loaded
**When** the import pipeline runs
**Then** every transaction receives a predicted category and a confidence score
**And** transactions with confidence below the defined threshold are flagged as "needs review" rather than assigned a low-confidence category

**Given** categorized transactions are displayed
**When** the user views the transaction list
**Then** each transaction shows its assigned category
**And** transactions flagged "needs review" are visually distinguished and surfaced at the top of the review queue

**Given** the ONNX model is unavailable at categorization time
**When** transactions are imported
**Then** all transactions are marked "needs review" and a plain-language notice informs the user that automatic categorization is unavailable (NFR-R2 degraded mode)
**And** the import itself is not blocked

**Given** the feature label preprocessing
**When** a transaction label is sent to the ONNX model for inference
**Then** the raw label is normalized (lowercase, trim, punctuation removed) before inference — matching the normalization applied during training

### Story 2.5: Manual & Batch Merchant Recategorization with Learning Engine

As a household member,
I want to correct a transaction's category individually or reassign all transactions from a merchant in one action,
So that the system learns my household's spending patterns and never asks me twice for the same merchant.

**Acceptance Criteria:**

**Given** a transaction is displayed with an assigned category
**When** the user selects a different category from the category picker
**Then** the transaction's category is updated immediately
**And** a merchant correction rule is saved: all future transactions from this merchant will be auto-assigned to the new category without prompting (FR9)

**Given** multiple transactions from the same merchant exist
**When** the user selects "Reassign all [merchant] transactions" and chooses a category
**Then** all transactions from that merchant across the full transaction history are updated to the new category (FR8)
**And** a confirmation shows the count of updated transactions

**Given** a merchant correction rule exists
**When** a new import includes transactions from that merchant
**Then** those transactions are assigned the corrected category directly without appearing in the "needs review" queue

**Given** a batch recategorization is applied to historical transactions
**When** the projection and analysis metrics are viewed
**Then** they reflect the updated categories immediately (recalculation within 3 seconds — NFR-P4)

### Story 2.6: LLM Provider Configuration & LLM-Assisted Categorization

As a household member,
I want to configure an LLM provider and use it to categorize transactions my ML model couldn't classify confidently,
So that uncategorized transactions are handled automatically without sacrificing privacy.

**Acceptance Criteria:**

**Given** the settings interface
**When** the user selects an LLM provider (Ollama or Claude API) and enters the required configuration
**Then** a connection test can be triggered and returns a plain-language success or failure message within 5 seconds

**Given** an LLM provider is configured and connected
**When** the user requests LLM-assisted categorization for "needs review" transactions (FR10)
**Then** each transaction label is sanitized before sending: IBAN patterns, card numbers, and amounts are stripped from the raw label (NFR-S4)
**And** only the sanitized label is transmitted to the LLM provider — no account names, balances, or identifying aggregates

**Given** a sanitized label is sent to the LLM provider
**When** a category response is received
**Then** the suggested category is applied to the transaction and added as a merchant correction rule (same learning engine as Story 2.5)
**And** the user can override the LLM suggestion using the same manual correction flow

**Given** the configured LLM provider is unreachable
**When** LLM-assisted categorization is requested
**Then** the system falls back to marking transactions as "needs review" without crashing (NFR-R2)
**And** a plain-language message informs the user that the LLM provider is currently unavailable

**Given** the provider switch setting is changed from Ollama to Claude API (or vice versa)
**When** the change is saved
**Then** the new provider is used for subsequent categorization requests without restarting the server (NFR-I2)

### Story 2.7: One-Time Expense Tagging

As a household member,
I want to mark any transaction as a one-time expense,
So that exceptional costs (car repair, medical bill) do not distort my spending trends or projections.

**Acceptance Criteria:**

**Given** a transaction is displayed in the transaction list
**When** the user marks it as "one-time expense"
**Then** the transaction is flagged and visually distinguished from recurring transactions

**Given** a transaction is marked as one-time
**When** the 3–6 month spending projection is calculated
**Then** that transaction's amount is excluded from the category trend used in the projection (FR12 dependency)

**Given** a transaction is marked as one-time
**When** month-over-month anomaly detection runs
**Then** the transaction is excluded from trend calculations and does not trigger a recurring anomaly alert (FR14 dependency)

**Given** a transaction previously marked as one-time
**When** the user removes the one-time flag
**Then** the transaction is re-included in trend and projection calculations
**And** the projection recalculates within 3 seconds (NFR-P4)

---

## Epic 1: Secure Local Service & Foundation

A running famfin service is accessible on the home network, protected by session authentication and backed by an AES-256 encrypted database. The build and deployment pipeline to the Raspberry Pi 3B is operational.

### Story 1.1: Project Scaffold & Build Pipeline

As a developer,
I want a working Rust/SvelteKit project scaffold with a cross-compilation pipeline to aarch64,
So that I can build and deploy a famfin binary to the Raspberry Pi 3B from the T460 development machine.

**Acceptance Criteria:**

**Given** the T460 development machine with `cross` and `just` installed
**When** `just build-arm` is run
**Then** a release binary `famfin` targeting `aarch64-unknown-linux-gnu` is produced in `target/aarch64-unknown-linux-gnu/release/`
**And** the binary starts successfully on the Pi 3B and serves an HTTP health check endpoint returning `200 OK` at `GET /health`

**Given** the SvelteKit frontend project is initialized
**When** `just build-frontend` is run
**Then** a compiled `dist/` directory is produced and served as static files by the Axum backend in production mode

**Given** the justfile at project root
**When** `just` is run without arguments
**Then** available targets (`build-arm`, `build-frontend`, `deploy`, `train`) are listed with descriptions

**Given** the Tokio runtime configuration in main.rs
**When** the binary starts
**Then** `worker_threads = 2` is configured via `#[tokio::main(worker_threads = 2)]` (not via environment variable or justfile)

### Story 1.2: Encrypted Database & Schema Migrations

As a developer,
I want SQLCipher-encrypted database initialization with versioned refinery migrations applied at startup,
So that all financial data is encrypted at rest from the first run and the schema evolves safely without manual intervention.

**Acceptance Criteria:**

**Given** the famfin binary starts for the first time
**When** the database file does not yet exist
**Then** a new SQLCipher-encrypted SQLite database is created at the configured path
**And** all pending refinery migrations are applied automatically before the HTTP server begins accepting requests

**Given** the famfin binary starts with an existing database
**When** pending migrations exist
**Then** migrations are applied in version order within a startup timeout of 30 seconds
**And** if the migration exceeds 30 seconds, the process exits with a non-zero status code (prevents systemd restart loop)

**Given** the test environment
**When** tests run on the T460
**Then** an in-memory SQLite database is used (no SQLCipher) so no encryption key is required in CI or test runs

**Given** the production binary
**When** inspecting the database file on disk with a standard SQLite tool
**Then** the file content is not readable (binary/encrypted) — no financial data is accessible without the SQLCipher key

### Story 1.3: Session Authentication

As a household member,
I want to authenticate with a session password before accessing any financial data,
So that famfin is protected from unauthorized access on the local network.

**Acceptance Criteria:**

**Given** session authentication is enabled (default)
**When** any protected route is accessed without a valid session cookie
**Then** the response is HTTP 401 and the client is redirected to the login page

**Given** the login page is displayed
**When** the correct household password is submitted
**Then** an HMAC-signed httpOnly session cookie is set with flags `HttpOnly; Secure; SameSite=Strict; Max-Age=28800`
**And** the user is redirected to the dashboard

**Given** the login page is displayed
**When** an incorrect password is submitted
**Then** a generic error message is shown without indicating whether the password was wrong or the account does not exist (no enumeration)
**And** no session cookie is set

**Given** an active session
**When** the session cookie is absent or the HMAC signature is invalid
**Then** the request is rejected with HTTP 401 regardless of the route

**Given** session authentication is disabled via configuration
**When** any route is accessed without a session cookie
**Then** access is granted (opt-out mode functional)

**Given** the `sessions` table in SQLite
**When** a new session is created
**Then** the session record is stored server-side with its expiry timestamp
**And** expired sessions are not accepted even if the cookie HMAC is valid

### Story 1.4: Pi 3B Deployment & systemd Service

As a developer,
I want `just deploy` to push the binary to the Pi and the service to start automatically on boot,
So that famfin is accessible on the home network without manual intervention after each reboot.

**Acceptance Criteria:**

**Given** a built aarch64 binary and ONNX model at the expected paths
**When** `just deploy` is run from the T460
**Then** the binary and model are copied to the Pi via scp
**And** `systemctl restart famfin` is executed on the Pi
**And** the health check endpoint `GET /health` returns `200 OK` within 60 seconds of restart (NFR-R6)

**Given** the `famfin.service` systemd unit file
**When** the Pi reboots
**Then** the famfin service starts automatically
**And** the service runs as a dedicated non-root user
**And** the service restarts automatically on failure (`Restart=on-failure`)

**Given** the install.sh script on a fresh Debian/Ubuntu system
**When** the script is executed on reference hardware
**Then** all dependencies are installed, the service is configured, and famfin is operational in under 30 minutes (NFR-M1)

**Given** the famfin service is running
**When** `journalctl -u famfin -f` is run on the Pi
**Then** structured INFO-level logs from the `tracing` crate are visible
**And** the log level can be set to DEBUG via environment variable without restarting the service binary
