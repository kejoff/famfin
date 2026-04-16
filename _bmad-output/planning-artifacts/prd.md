---
stepsCompleted: ["step-01-init", "step-02-discovery", "step-02b-vision", "step-02c-executive-summary", "step-03-success", "step-04-journeys", "step-05-domain", "step-06-innovation", "step-07-project-type", "step-08-scoping", "step-09-functional", "step-10-nonfunctional", "step-11-polish"]
classification:
  projectType: web_app
  domain: fintech
  complexity: medium
  projectContext: greenfield
  notes: "Import historique multi-années au setup pour bootstrapper le ML sans cold start. Complexité principale : comportementale (adoption rituel) + UX deux profils couple."
inputDocuments:
  - "_bmad-output/planning-artifacts/product-brief-famfin-2026-03-24.md"
  - "_bmad-output/planning-artifacts/product-brief-famfin-distillate.md"
  - "_bmad-output/planning-artifacts/research/domain-finances-familiales-partage-budget-research-2026-03-22.md"
workflowType: 'prd'
briefCount: 2
researchCount: 1
brainstormingCount: 0
projectDocsCount: 0
---

# Product Requirements Document - famfin

**Author:** kj
**Date:** 2026-04-08

## Executive Summary

famfin is a local-first household finance tool built for one family. It runs on a home server, stores nothing in the cloud, and has no commercial agenda. Its purpose: give a household full clarity over their financial situation, align that clarity with their life goals, and progressively move them from financial anxiety to financial autonomy.

The target user is the **Constrained Aspirationalist household** — two working adults, one or two children, stable income (€3,500–6,000/month net), values-driven spenders who have tried commercial finance apps and quit because the tools were working against them. Their problem is not income. It is the absence of a trusted, conflict-free picture of where they stand and where they're headed. A €900 car repair triggers a week of anxiety. A home purchase conversation ends in vague discomfort. They're not irresponsible — they're financially invisible to themselves.

The guiding principle: **finance in service of life, not the other way around.**

**Project context:** Greenfield web application (local SPA, home server deployment). Domain: personal finance with ML categorization. Complexity: medium — ML integration and dual LLM provider flexibility are the core technical challenges; local-first architecture sidesteps PSD2/RGPD regulatory constraints in V1. Solo developer with data engineering and AI background.

### What Makes This Special

Every commercial finance app is funded by the financial products it is supposed to help you evaluate objectively. This is a structural conflict of interest — not a bug, but the business model. famfin has no such model. It recommends nothing it earns commission on. The codebase is open source: the "no conflicts" claim is auditable, not just declared.

Three additional differentiators compound this:
- **Privacy by architecture** — all financial data stays on the household's home server; no cloud sync, no account creation, no data broker risk.
- **Designed for serenity, not engagement** — no notifications, no streaks, no FOMO mechanics. Every design decision is evaluated against three prohibitions: no sense of loss of control, no feeling of overwhelm, no sensation of helplessness.
- **ML trained on household data** — the categorization model bootstraps from the household's own multi-year transaction history at first import. It learns *this* family's spending patterns, not a generic average. Cold start is eliminated. The product becomes uniquely theirs over time.

## User Journeys

### Journey 1: The First Import — "Finally Seeing the Map"

**Persona:** Sophie, 34. Works in communications. Manages most of the household's financial logistics by default — not by choice, but because her partner Julien tends to avoid the subject. They earn €5,200/month net combined. They used Bankin for eight months, then quietly stopped when the third credit card offer appeared. Since then: nothing. Sophie knows, roughly, that they're not in trouble. She doesn't know where the money actually goes.

**Opening Scene:** It's a Sunday afternoon. Julien is with their daughter. Sophie has 45 minutes. She installs famfin on the home server, navigates to it in her browser. The app asks one question first: *"What do you want?"* Not her income. Not her bank details. Her intention. She types: "Stop feeling like we're behind."

**Rising Action:** She follows the per-bank guide for Société Générale — four clicks she didn't know existed. She exports 18 months of transactions. The import takes 40 seconds. The app presents its first inference: *"I see recurring credits of €5,200/month — is that your household income?"* One click to confirm. Then: categories appear. Most are right. Three are wrong. She selects all 14 "BIOCOOP LYON" entries, recategorizes them in one tap to "Quality Food." The app remembers. She won't be asked again.

**Climax:** The projection appears. For the first time, Sophie sees — not feels, not suspects, but *sees* — that they spend €680/month on food, €340 on subscriptions (three of which she doesn't recognize), and that they have effectively €0 left over at month's end. It's not as bad as the vague dread suggested. It's not good either. But it's *real*. The fog lifts.

**Resolution:** Sophie closes the laptop. She doesn't feel overwhelmed. She feels oriented. She knows the number now. She texts Julien: *"I found something. Let's look at it together tonight."* That text is the product working.

**Requirements revealed:** CSV import engine, per-bank guides, revenue auto-detection, batch correction by merchant, categorization engine, 3-month projection display, intentional onboarding flow (intention-first, not data-first).

---

### Journey 2: The Monthly Ritual — "The First of the Month"

**Persona:** Sophie and Julien, together. Third month using famfin. The calendar event fires at 8pm on the 1st: "Financial Review — 30 min."

**Opening Scene:** They sit at the kitchen table, laptop open. Their daughter is asleep. Julien makes tea. This used to be a conversation they avoided. Now it has a container — a time, a place, a structure.

**Rising Action:** Sophie imports this month's CSV. Thirty seconds. The app shows what changed: food spend is down €80 from last month (the meal planning they discussed). One anomaly flagged at the top of the screen: a €340 charge from a car garage — higher than usual. Julien remembers: the winter tire swap. The app asks: *"Is this a one-time expense or recurring?"* One-time. It's logged, contextualized, not treated as a trend.

**Climax:** The projection updates. With the food reduction held, they're €120 positive this month — the first time in the three months they've been tracking. The app doesn't celebrate with confetti. It simply shows the trajectory moving, quietly, in the right direction. Julien says: *"That's actually something."* It is.

**Resolution:** They close the laptop at 8:28pm. Two minutes to spare. They didn't argue. They made two decisions: cancel one subscription they'd forgotten, redirect €100 to the emergency fund. The ritual worked. They'll do it again next month — not because the app reminded them, but because they want to.

**Requirements revealed:** Month-over-month delta display, anomaly detection with context prompt, trajectory indicator, subscription detection, savings redirect action, ritual framing (no clutter, clean review flow).

---

### Journey 3: The Unexpected Expense — "The Car Repair"

**Persona:** Julien, 37. Monday morning. A warning light on the dashboard. The mechanic calls at noon: €870 for a brake system repair. Julien's immediate reaction is a familiar tightening in his chest — the "can we afford this?" dread that has no answer because there's never been a real number attached to it.

**Opening Scene:** Julien opens famfin on his phone browser during his lunch break. He hasn't touched it since the ritual four days ago. He navigates to the dashboard.

**Rising Action:** The main screen shows their current absorption capacity — the amount the household can absorb in unexpected expenses without derailing any active goal. The number is €1,240. The repair is €870. Julien stares at the number for a moment.

**Climax:** The tightening in his chest releases. Not because €870 is nothing — it isn't. But because he can see, for the first time, that it's *handleable*. The app hasn't minimized the expense. It's given him the context to hold it correctly. He calls the mechanic back and says yes.

**Resolution:** That evening, at no scheduled moment, Julien opens famfin and logs the repair as a one-time transport expense. The projection adjusts. The emergency fund target shifts by one month. The app notes: *"This month absorbed a €870 shock. Your trajectory remains intact."* He closes the app. He doesn't think about it again.

**Requirements revealed:** Absorption capacity indicator (always visible on main screen), one-time expense logging, trajectory recalibration after shock, contextual reassurance messaging, mobile browser access.

---

### Journey 4: The Partner's Awakening — "Julien's Turn"

**Persona:** Julien, three months in. He's watched Sophie run the rituals. He understands the app exists. He hasn't felt it was *for him* — finance has always felt like Sophie's domain, and he's been comfortable with that, and quietly guilty about it.

**Opening Scene:** Sophie has a work trip. She asks Julien to run the monthly import. He opens famfin alone for the first time. He's mildly anxious — not about the finances, but about doing it wrong.

**Rising Action:** The import guide is step-by-step. His bank is CIC. The guide shows exactly which menu, which button, which file format. He exports the CSV without difficulty. He drags it into the app. The import processes. Categories appear — most already learned from Sophie's corrections over three months. Two transactions are uncategorized. The app surfaces them with a simple choice: *"What is this?"* He picks from the list. Done.

**Climax:** He looks at the dashboard Sophie has been looking at for three months. He sees the emergency fund at 34%. He sees the home purchase goal with a timeline. He sees the projection showing they're on track. Something clicks — this isn't Sophie's project. It's theirs. He pulls out his phone and takes a photo of the screen to show her.

**Resolution:** When Sophie returns, Julien mentions two things he noticed on his own: a streaming subscription they're both paying for separately, and that their grocery spend went up this month because of a birthday dinner they'd forgotten to account for. Sophie raises an eyebrow. *"You looked at it,"* she says. He shrugs. *"It made sense."* That's the product working for the second partner.

**Requirements revealed:** Per-bank import guide (multiple banks), learning engine retaining previous corrections, dual-access via home network browser, clear goal progress indicators, minimal cognitive load for infrequent users, no account/login required.

---

### Journey Requirements Summary

| Capability | Journeys |
|---|---|
| CSV import engine with per-bank guides | 1, 2, 4 |
| Categorization + batch correction + learning engine | 1, 4 |
| Revenue auto-detection | 1 |
| 3–6 month projection | 1, 2, 3 |
| Absorption capacity indicator | 3 |
| Month-over-month delta + anomaly detection | 2 |
| Trajectory indicator (main screen signal) | 2, 3 |
| One-time expense logging + recalibration | 3 |
| Subscription detection | 2 |
| Goal progress display (emergency fund, home purchase) | 4 |
| Intention-first onboarding | 1 |
| Mobile browser access (home network) | 3 |
| No login required on local network | 4 |
| Contextual reassurance messaging | 3 |

## Success Criteria

### User Success

The household experiences famfin as a trustworthy, fair companion — not a tool that judges or overwhelms. Success is felt before it is measured:

- **No financial panic** — when an unexpected expense hits, the household absorbs it without anxiety because the app has shown them their real absorption capacity. *Measurable proxy:* the household has not drawn from its emergency fund in the past 12 months.
- **No apprehension opening the app** — opening famfin produces reassurance, not dread. The household opens it willingly as part of a ritual, not under obligation or anxiety.
- **A fair companion** — the app never judges spending choices. It reflects reality, aligns advice with the household's declared values, and leaves decisions to the humans.
- **Emergency fund built** — 3 months of household expenses held in liquid savings; the exact target amount is known, tracked, and reached.
- **Home purchase savings active** — a monthly savings target toward the down payment is set, funded, and displayed with a live timeline.
- **Still in active use after 12 months** — both partners open the app and reference it in financial decisions.

### Project Success

Since famfin has no commercial metrics, project success is defined by household impact:

- The monthly import ritual is maintained consistently (>10 consecutive cycles)
- **Push notifications for positive events only** — income increase detected, milestone reached, goal ahead of schedule. Behavioral drift alerts (spending pattern change, unexpected category spike) are reserved for the monthly ritual screen, displayed at first open — not pushed mid-day, which would create the anxiety the app is designed to prevent.
- The household adjusts at least one spending behavior based on app-surfaced insight within 6 months

### Technical Success

- **Categorization:** High accuracy on French bank transaction labels from first import; learning engine retains every manual correction and applies it across historical transactions; accuracy improves visibly over the first 3 import cycles
- **Import:** Frictionless for the major French banks (BNP Paribas, Société Générale, Crédit Agricole, La Banque Postale, CIC, LCL); per-bank guides resolve the deliberate UX friction banks create around CSV export
- **UX — 5-star simplicity:** Either partner, on their first solo import session, can identify the 3 largest spending categories and correctly interpret their absorption capacity without external explanation — in under 15 minutes. Both partners are technically proficient; the UX test is emotional clarity and financial signal quality, not technical accessibility.
- **Push notifications (V2+):** Positive event alerts delivered reliably to mobile via PWA or lightweight companion; behavioral alerts displayed in-app only

### Measurable Outcomes

| Outcome | Target | Timeline |
|---|---|---|
| Monthly import ritual maintained | >10 consecutive months | Month 12 |
| Emergency fund reached | 100% of 3-month target | Month 18 |
| No emergency fund withdrawal | 0 unplanned draws | Month 12 |
| Both partners active | Both reference app in decisions | Month 3 |
| First solo import autonomy | Full import + correct interpretation in <15 min unassisted | Month 1 |
| At least one behavior change from app guidance | 1 confirmed adjustment | Month 6 |

## Product Scope

### MVP — V1: Clarity

- CSV/OFX import with per-bank guides for major French banks
- ML categorization + optional LLM provider (Ollama local, Claude API)
- Learning engine with batch correction by merchant name (applies to full history)
- Revenue auto-detection from recurring credits, presented for confirmation
- 3–6 month spending projection — this IS the budget orientation in V1; no separate feature needed; the projection naturally surfaces which categories consume disproportionate resources
- Home server deployment (local web app, browser access on home network)
- Monthly calendar ritual: recurring 30-min "Financial Review" event auto-generated in household calendar

### Growth — V2: Trajectory

- Autonomy Pyramid with stage indicator (Informed → Confident → In Control)
- Traffic-light trajectory screen (on track / drifting / alert)
- Life goal decomposition with live recalibration (home purchase, emergency fund, other)
- Emergency fund as non-negotiable prerequisite — gates all other goals
- Reality recalibrator: 3 paths when goal is unreachable (extend timeline / increase savings delta / alternative scenario)
- Couple alignment: shared wants prioritization with fact-based arbitration
- Smart goal suspension after auto-detected financial shock
- Push notification infrastructure (PWA Service Worker) — positive events only; behavioral drift surfaced in-app at ritual open

### Vision — V3 & V4

- INSEE contextual benchmarking (city + income bracket + family composition — statistical, not social)
- Annual expense calendar (predictable peaks mapped 12 months ahead)
- Values-aligned recommendations
- Inaction cost display (compound interest visualization)
- Subscription audit
- Freelance/variable income module with revenue smoothing
- Long-term freedom simulator
- Bank fee negotiator

## Domain-Specific Requirements

### Regulatory Scope — V1

famfin V1 operates entirely outside regulated fintech perimeters by design:

- **No PSD2/AISP required** — CSV/OFX import is a user-initiated file transfer; no banking API access; no license required
- **No PCI-DSS** — no payment processing of any kind
- **No AML/KYC** — no commercial financial activity
- **No SOX** — no corporate reporting obligations

The local-first architecture is a deliberate regulatory bypass: financial data never leaves the household's home server, eliminating cloud data processing obligations under RGPD for V1.

### AI Act — Applicable from V1

The ML categorization engine qualifies as a limited-risk AI system under EU AI Act. Mitigation is already built into the product: human override is mandatory and always accessible — manual correction of any categorization is a core product constraint, not an edge case. Required actions at design time:
- Formally classify the system as limited risk (financial categorization for personal use, no automated financial decisions without human review)
- Document the model's training data, decision logic, and correction mechanisms
- Human override availability must be preserved in all future versions

### Data Privacy — Multi-Member Consent

Even in a local-first context, two household members accessing each other's financial data requires informed individual consent:
- Each partner explicitly acknowledges shared financial visibility via a dedicated consent screen on first access from their device
- Consent is active (requires affirmative action), not implied by app usage
- UX implementation: full-screen consent step shown once per device on first access, before any financial data is visible

### Security — All Versions

- Home server accessible on local network only — no external port exposure by default
- **Authentication active by default:** simple session password required to access the app on the local network; both partners use the same shared household password; disabling auth is possible but not the default
- **Database encryption by default:** app encrypts its local database at rest (SQLCipher or equivalent); disk-level encryption (FileVault, LUKS) recommended in setup guide as complementary layer
- Setup guide explicitly advises against exposing the server port externally (no port forwarding, no reverse proxy without authentication)

### V2 — Bank API Sync

Automatic bank sync removes the manual import friction — the single largest adoption barrier in V1. Approach for a personal, non-commercial, open source project:

- **No commercial aggregator required** — famfin accesses the PSD2 APIs of the household's own banks directly (1–3 banks maximum in practice)
- **No AISP license required** in this context: a developer accessing their own accounts for personal non-commercial use is not the target of PSD2 commercial regulation; risk of enforcement is negligible for an open source project with no users beyond the household
- **CSV remains the permanent fallback** — if a bank blocks direct API access, the manual import ritual continues unchanged
- **Implementation:** one dedicated connector module per bank (Crédit Agricole, La Banque Postale, etc.); each connector is isolated behind a stable interface, making future additions or replacements straightforward
- **SCA (Strong Customer Authentication):** required by PSD2 — bank re-authentication every 90 days; must be handled gracefully in the UX
- **Architecture principle:** the bank integration layer is isolated behind a stable internal interface; provider changes (new bank, new API version) do not require changes to core application logic

## Innovation & Novel Patterns

### Detected Innovation Areas

**1. Structural Conflict-of-Interest Elimination**
Every personal finance app on the market is funded by the financial products it evaluates. This creates an irresolvable bias — the tool cannot be fully honest about whether a loan, insurance product, or investment is a good idea, because its revenue depends on that recommendation. famfin eliminates this structurally: no commercial model means no conflict, and the open source codebase makes this claim auditable rather than declarative. This is not a feature — it is an architectural property that commercial competitors cannot replicate without destroying their business model.

**2. The Inverse Engagement Model**
Commercial apps optimize for sessions opened, streaks maintained, notifications clicked. famfin optimizes for the opposite: a household that opens the app once a month for 30 minutes and lives well the rest of the time has succeeded. The explicit goal is to transfer financial competence until the app is no longer needed for basic decisions. No engagement mechanic, no streak, no FOMO. This inverts the standard product success model: the "death" of the app (household autonomy achieved) is the victory condition.

**3. Absorption Capacity as Primary Dashboard Metric**
Existing apps show balances, categories, budgets. famfin's primary screen signal is different: *how much unexpected expense can this household absorb right now without derailing any active goal?* This number converts financial anxiety directly into actionable clarity. It answers the question people actually ask in a crisis ("can we afford this?") rather than the question apps typically answer ("where did the money go?").

**4. Friction as Ritual Design**
The manual CSV import is typically framed as a limitation to be solved by automatic bank sync. famfin reframes it deliberately: monthly import is a 30-minute intentional couple ritual, blocked in the calendar, designed to create shared financial awareness. The friction is not a bug to eliminate — it is the mechanism by which the household stays conscious of their finances. Bank sync (V2) will automate the data, but preserve the ritual.

**5. Household-Specific ML from Day One**
The categorization engine bootstraps on the household's own multi-year transaction history at first import. There is no generic model trained on anonymous data — the model learns *this family's* spending patterns: their specific merchants, their specific categories, their specific correction preferences. By month 3, the engine is a household artifact, not a generic tool. This eliminates cold start and creates a genuine moat: the model becomes uniquely theirs over time.

### Market Context & Competitive Landscape

No competitor combines all five innovation dimensions:
- **Local-first + no commercial agenda:** zero competitors (all apps require accounts, cloud sync, and revenue models)
- **Inverse engagement model:** zero competitors (all apps maximize engagement by design)
- **Absorption capacity as primary metric:** zero competitors (all apps center on spending categories or budget balances)
- **Ritual friction:** one indirect parallel — YNAB's zero-based budgeting methodology requires intentional monthly reconciliation, but it's framed as a discipline, not a couple ritual
- **Household-specific ML:** no direct parallel; commercial apps use shared models by necessity

The structural gap: Honeydue is the only dedicated couple finance app and is technically stagnant. The "collaborative + local-first + AI-native + no conflicts" combination has no precedent in the market.

### Validation Approach

| Innovation | How to validate |
|---|---|
| Conflict-free trust | Household opens the app without apprehension after 3 months — no Bankin-style abandonment trigger |
| Inverse engagement model | App still in active use at month 12 despite zero engagement mechanics |
| Absorption capacity metric | Household references the absorption number in a real financial decision within 6 months |
| Ritual friction | Monthly ritual maintained for >10 consecutive months without calendar reminder needed |
| Household ML | Uncategorized transactions <5% of imports by month 4 |

### Risk Mitigation

| Risk | Mitigation |
|---|---|
| Ritual abandoned when novelty fades | Calendar event auto-generated and persistent; couple commitment built into onboarding |
| ML accuracy insufficient for trust | LLM fallback always available; human override always visible and one-tap; accuracy shown transparently |
| Absorption capacity misunderstood | Clear in-context explanation on first display; onboarding explains the concept before showing the number |
| "No conflicts" claim disbelieved | Open source codebase is the proof — link to code visible from the app |

## Web Application Specific Requirements

### Project-Type Overview

famfin is a local-first Single Page Application (SPA) served from a home server and accessed via browser. Remote access is provided by a household VPN — the VPN perimeter is the security boundary; the app assumes a trusted network. Both users are data engineers with AI backgrounds — the UX challenge is emotional clarity and financial signal quality, not technical accessibility.

### Access Model

- **Primary access:** Home network (local server URL)
- **Remote access:** VPN — provides secure tunnel to home server; app behavior is identical whether accessed locally or via VPN
- **Security perimeter:** VPN handles authentication and encryption in transit; app-level session password handles in-app access control
- **No cloud dependency:** All data stays on the home server; VPN connects the user to the server, not to a cloud intermediary

### Browser Matrix

- **Supported:** Chrome 120+, Firefox 120+, Safari 17+, Edge 120+
- **Not supported:** IE11, legacy mobile browsers, polyfills not required
- **No app store dependency** — URL access only

### Interface Strategy — Dual-First Design

Two distinct interface modes sharing the same data layer:

**Desktop Interface (Ritual Mode)**
- Full dashboard: spending breakdown, projection, category analysis, goal progress
- Import flow: CSV drag-and-drop, categorization review, batch correction
- Monthly ritual experience: complete financial review in 30 minutes

**Mobile Interface (Consultation Mode)**
- Single primary metric visible above the fold: absorption capacity in €
- Trajectory status: one-color signal (green / orange / red)
- Secondary access: month-to-date summary, active goal progress
- Goal: answer "can we afford this?" in under 3 seconds
- No data entry on mobile — read-only consultation only

### Progressive Web App (PWA) Strategy

- **V1:** PWA manifest + installable to mobile home screen; **no offline caching** — VPN access makes offline mode unnecessary and caching financial data on device introduces unnecessary exposure risk
- **V2:** Service Worker push notification infrastructure activated — positive financial events only (milestone reached, income increase, goal ahead of schedule); behavioral drift surfaced in-app at ritual open, never pushed

### SEO Strategy

- **None required** — local/VPN app, not publicly indexed

### Accessibility Level

- **Target: WCAG 2.1 AA** — standard practice
- **Real UX challenge:** Emotional clarity and financial signal quality; information density calibrated for technically proficient users; no hand-holding copy; clear financial signal hierarchy

### Implementation Considerations

- **Data layer:** Local database (SQLCipher-encrypted); served via API from home server to browser
- **LLM integration:** Two-provider config — Ollama (local model of user's choice, fully private) and Claude API (Anthropic); provider selected in settings with one-click connection test; API keys stored locally on server
- **Calendar integration:** CalDAV push to Radicale (household self-hosted calendar server) — monthly ritual event created and maintained via CalDAV protocol; no Google/Apple Calendar API dependency

## Project Scoping & Phased Development

### MVP Strategy & Philosophy

**MVP Approach:** Problem-solving MVP — the minimum that delivers the core value proposition: full clarity over the household's financial situation after one import cycle.

**Resource Profile:** Solo developer with data engineering and AI background — ideal for ML integration and architecture; single contributor requires strict sequential scoping with no parallel workstreams.

**MVP Philosophy:** The first import must produce a revelation. If Sophie closes the laptop after her first session without feeling oriented, V1 has failed. Everything else is secondary to that moment.

### MVP Feature Set — V1: Clarity

**Core User Journeys Supported:**
- Journey 1: The First Import (primary value delivery)
- Journey 2: The Monthly Ritual (retention mechanism)
- Journey 3: The Unexpected Expense (partial — absorption capacity indicator)
- Journey 4: The Partner's Awakening (inherent via shared server access)

**Must-Have Capabilities:** See Product Scope — V1: Clarity above for the complete capability list. All V1 capabilities are in scope for MVP; nothing is deferred within V1.

**Explicitly Out of V1:**
- Autonomy Pyramid / trajectory indicator
- Life goal decomposition
- Real-time bank sync
- Push notifications
- Couple alignment features
- INSEE benchmarking
- Subscription audit
- Any V3/V4 features

### Post-MVP Features

**Phase 2 — V2: Trajectory**
Autonomy Pyramid with stage indicator; traffic-light trajectory screen; life goal decomposition with live recalibration; emergency fund prerequisite gate; reality recalibrator; couple alignment and shared wants arbitration; smart goal suspension after financial shock; PWA push notifications (positive events only); direct PSD2 bank API connectors for household banks (1–3 max).

**Phase 3 — V3 & V4: Intelligence & Freedom**
INSEE contextual benchmarking; annual expense calendar; values-aligned recommendations; inaction cost display; subscription audit; freelance/variable income module; long-term freedom simulator; bank fee negotiator.

### Risk Mitigation Strategy

| Risk | Severity | Mitigation |
|---|---|---|
| ML cold start on French bank labels | Medium | Bootstrap from multi-year historical import at setup; LLM fallback for uncategorized transactions; manual correction always one tap away |
| Ritual adoption without engagement mechanics | High | CalDAV calendar event created at setup; value delivered on first import creates intrinsic motivation; monthly delta comparison makes each ritual rewarding |
| Bank API instability in V2 | Medium | CSV import is permanent fallback; bank connectors isolated behind stable interface; one connector per bank — failure is contained |
| Solo dev bandwidth | Medium | Strict V1 scope, no parallel workstreams; V2 gated on V1 household adoption confirmation |

## Functional Requirements

### Data Import & Ingestion

- **FR1:** User can import transaction data from CSV or OFX files via drag-and-drop
- **FR2:** System can provide step-by-step export guides for each supported French bank (BNP Paribas, Société Générale, Crédit Agricole, La Banque Postale, CIC, LCL)
- **FR3:** System can detect recurring credit patterns and infer household net monthly income, presenting the inference to the user for single-action confirmation
- **FR4:** User can import up to 36 months of historical transaction data in a single session
- **FR5:** User can export all household financial data (transactions, categories, settings) as a portable file

### Transaction Categorization

- **FR6:** System can automatically assign a category to each imported transaction using an ML model trained on the household's own transaction history
- **FR7:** User can manually reassign a category to any individual transaction
- **FR8:** User can reassign all transactions from a given merchant to a category in a single action, with the change applied to the full transaction history
- **FR9:** System can retain all manual category corrections and apply them automatically to future transactions from the same merchant without re-prompting
- **FR10:** User can request LLM-assisted categorization for uncategorized transactions using a configured AI provider
- **FR11:** User can mark any expense as one-time to exclude it from trend and projection calculations

### Financial Analysis & Projection

- **FR12:** System can generate a 3–6 month forward spending projection based on categorized transaction history
- **FR13:** System can calculate and display the household's current absorption capacity — the maximum unexpected expense absorbable without derailing any active goal
- **FR14:** System can detect month-over-month spending anomalies by category and surface them with a one-time vs. trend classification prompt
- **FR15:** System can detect recurring subscription payments within transaction history
- **FR16:** User can review detected recurring subscriptions and mark each as active, cancelled, or under review
- **FR17:** System can display month-over-month spending delta by category

### Dashboard & Visualization

- **FR18:** User can view a desktop dashboard presenting full financial summary: spending by category, 3–6 month projection, absorption capacity, and active goal progress
- **FR19:** User can view a mobile consultation interface presenting absorption capacity and trajectory status as the primary signal, accessible in under 3 seconds
- **FR20:** System can display a trajectory status signal (on track / drifting / alert) based on current spending versus projected plan
- **FR21:** System can display contextual reassurance messaging after an unexpected expense is logged, confirming whether the household trajectory remains intact
- **FR22:** System can maintain consistent financial state across desktop and mobile interfaces without requiring manual refresh

### Onboarding & Setup

- **FR23:** System can present an intention-first onboarding flow in which the user states their financial goal before providing any financial data
- **FR24:** System can present a consent screen to each household member on first device access, before any financial data is visible, requiring affirmative acknowledgment of shared financial visibility
- **FR25:** User can complete full initial setup and first import following guided per-bank instructions

### Security & Access Control

- **FR26:** System can require session password authentication before granting access to any financial data (active by default, opt-out available)
- **FR27:** System can store all financial data in an encrypted local database
- **FR28:** System can display clear, non-alarming error messages when import, AI provider connection, or calendar sync operations fail

### Configuration & Integration

- **FR29:** User can configure and validate an AI provider connection via a settings interface
- **FR30:** System can create and maintain a recurring monthly calendar event on the household's self-hosted calendar server
- **FR31:** System can be installed as a PWA on mobile devices for home-screen access
- **FR32:** System can display a direct link to the open source codebase from within the application

### Goals & Autonomy Pyramid *(V2)*

- **FR33:** User can define a life goal with target amount and timeline; system decomposes it into required monthly savings with intermediate milestones and live recalibration *(V2)*
- **FR34:** System can display the household's current stage on the Autonomy Pyramid (Informed / Confident / In Control) and active guidance toward the next stage *(V2)*
- **FR35:** System can gate non-emergency goals as inactive until the emergency fund prerequisite target is met *(V2)*
- **FR36:** System can detect a financial shock and proactively propose temporary suspension of non-essential goals *(V2)*

### Couple Alignment & Notifications *(V2)*

- **FR37:** Each household member can declare financial wants with priority levels; system can display a fact-based comparison against available resources *(V2)*
- **FR38:** System can send push notifications to household members for positive financial events only — milestone reached, income increase, goal ahead of schedule *(V2)*

### Bank Sync *(V2)*

- **FR39:** System can connect to household bank accounts via direct PSD2 API and import transactions automatically, with CSV import available as permanent fallback *(V2)*

## Non-Functional Requirements

### Performance

- **NFR-P1 — Absorption mobile :** Le Time To Interactive du tableau de bord mobile est ≤ 3 secondes sur réseau local. Sur accès VPN distant, l'objectif est atteint si la latence réseau est ≤ 50ms (précondition d'environnement, hors responsabilité applicative).
- **NFR-P2 — Import et catégorisation :** Le traitement d'un import CSV/OFX de 12 mois (≤ 1 500 transactions), catégorisation ML incluse, se termine en moins de 30 secondes sur hardware de référence (ARM Cortex-A72 quad-core, 4 Go RAM — équivalent Raspberry Pi 4).
- **NFR-P3 — Actions utilisateur :** Les opérations de lecture (navigation, consultation, filtrage) répondent en moins de 500ms (p95) sur réseau local. Les opérations d'écriture (correction, recatégorisation batch) répondent en moins de 2 secondes.
- **NFR-P4 — Projection :** Le calcul de projection 3–6 mois se met à jour en moins de 3 secondes après une modification de catégorie ou de revenu.
- **NFR-P5 — Chargement progressif :** Un indicateur de chargement s'affiche dans les 200ms suivant la navigation. Les dernières données consultées sont affichées immédiatement depuis le stockage local du navigateur pendant le chargement ; le fetch complet s'effectue en arrière-plan sans écran blanc.

### Sécurité

- **NFR-S1 — Chiffrement au repos :** L'intégralité de la base de données est chiffrée avec SQLCipher (AES-256). Aucune donnée financière n'est stockée en clair sur le serveur.
- **NFR-S2 — Chiffrement en transit :** Toutes les communications client-serveur se font via HTTPS (TLS 1.2 minimum). L'accès HTTP non chiffré est bloqué ou redirigé.
- **NFR-S3 — Authentification :** L'authentification par mot de passe de session est activée par défaut. La durée de session est configurable (défaut : 8 heures). La réinitialisation du mot de passe s'effectue uniquement via accès direct au serveur — aucun mécanisme de récupération en ligne.
- **NFR-S4 — Isolation des données LLM :** Les payloads envoyés au fournisseur LLM contiennent uniquement le libellé brut de la transaction après suppression automatique des patterns IBAN, numéros de carte, et montants. Les champs compte, titulaire, solde, et tout agrégat identifiant ne sont jamais transmis. Un mode diagnostic permet de logger les payloads LLM sortants pour vérification — en production, cette vérification est couverte par des tests d'intégration.
- **NFR-S5 — Consentement données couple :** Chaque membre du foyer doit explicitement confirmer la visibilité partagée des données lors de la première connexion depuis un nouveau navigateur.

### Fiabilité

- **NFR-R1 — Résilience locale :** Les fonctionnalités core (consultation, catégorisation, projection) fonctionnent sans dépendance à un service externe. La perte de connectivité vers Ollama, Claude API, ou Radicale n'interrompt pas le fonctionnement principal.
- **NFR-R2 — Dégradation gracieuse — LLM :** Si le fournisseur LLM configuré est indisponible, la catégorisation bascule automatiquement sur le modèle ML local. L'utilisateur est informé du fallback via un message en langue naturelle, sans jargon ni alarme.
- **NFR-R3 — Dégradation gracieuse — CalDAV :** Si Radicale est indisponible lors de la génération de l'événement calendrier mensuel, l'opération est retentée à la prochaine ouverture de l'application. L'import n'est pas bloqué.
- **NFR-R4 — Idempotence d'import :** Réimporter un fichier CSV/OFX déjà importé ne crée pas de transactions dupliquées. La détection s'appuie sur une empreinte composite : date + montant + libellé normalisé (lowercase, trim, suppression de ponctuation) + index ordinal dans le fichier source.
- **NFR-R5 — Export de résilience :** L'intégralité des données (transactions, catégories, corrections manuelles, configuration) est exportable en CSV/JSON à tout moment. L'export d'un historique de 5 ans (≈ 7 500 transactions) se termine en moins de 10 secondes.
- **NFR-R6 — Démarrage à froid :** Le service est disponible (health check HTTP 200) dans les 60 secondes suivant le démarrage du processus sur hardware de référence ARM quad-core.

### Intégration

- **NFR-I1 — CalDAV :** La création d'événements calendrier est compatible avec le protocole CalDAV standard (RFC 4791). La connexion Radicale est configurable via URL et credentials dans les paramètres.
- **NFR-I2 — LLM :** Les deux fournisseurs (Ollama local, Claude API) sont configurables indépendamment. Le changement de fournisseur ne nécessite pas de redémarrage du serveur.
- **NFR-I3 — Formats d'import :** Les fichiers CSV et OFX issus de Crédit Agricole et La Banque Postale sont pris en charge nativement. L'ajout d'un nouveau profil de banque ne nécessite pas de modification du code core.
- **NFR-I4 (V2) — PSD2 :** Les connecteurs PSD2 directs sont implémentés par banque de façon isolée. La défaillance d'un connecteur n'affecte ni les autres banques ni le fallback CSV.

### Maintenabilité

- **NFR-M1 — Déploiement reproductible :** Un script d'installation (`install.sh`) est fourni et documenté. Une installation fraîche sur Debian/Ubuntu doit produire un service opérationnel en moins de 30 minutes sur hardware de référence.
- **NFR-M2 — Observabilité :** Toutes les opérations de fond (import, appel LLM, sync CalDAV) génèrent des logs structurés consultables depuis l'interface d'administration. Les erreurs persistantes (≥ 3 échecs consécutifs) sont visibles sur le tableau de bord sans formulation alarmante.
- **NFR-M3 — Migrations de schéma :** Les migrations de base de données sont appliquées automatiquement au démarrage. Un retour à la version précédente est possible sans perte de données via restauration du backup pré-migration.

### Accessibilité

- **NFR-A1 — Contraste traffic-light :** Les indicateurs visuels traffic-light (vert / orange / rouge) respectent le ratio de contraste WCAG AA (4.5:1) et sont complétés par un label textuel ou icône — la couleur n'est jamais le seul signal.
- **NFR-A2 — Mode sombre :** L'interface respecte la préférence système `prefers-color-scheme: dark` du navigateur. Les indicateurs critiques restent lisibles et distinguables en mode sombre sans dégradation de contraste.

### Localisation

- **NFR-L1 — Langue :** L'interface est entièrement en français. Aucune chaîne UI n'est affichée en anglais dans la configuration par défaut.

### UX & Tonalité

- **NFR-UX1 — Messages d'erreur :** Tous les messages d'erreur visibles par l'utilisateur sont formulés en langue naturelle, sans jargon technique. Ils expliquent ce qui s'est passé et proposent une action concrète ou confirment que le système a géré la situation automatiquement.
