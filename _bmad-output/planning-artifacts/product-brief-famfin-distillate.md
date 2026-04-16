---
title: "Product Brief Distillate: famfin"
type: llm-distillate
source: "product-brief-famfin-2026-03-24.md"
created: "2026-04-07"
purpose: "Token-efficient context for downstream PRD creation"
inputs:
  - "_bmad-output/brainstorming/brainstorming-session-2026-03-22-001.md"
  - "_bmad-output/planning-artifacts/research/domain-finances-familiales-partage-budget-research-2026-03-22.md"
---

# famfin — Detail Pack for PRD Creation

## Product Identity

- **Name:** famfin
- **Type:** Personal household finance tool — not a commercial product, not SaaS
- **Distribution:** Open source; built for one household; no monetization intent
- **Architecture:** Local-first — runs on a home server (NAS, Raspberry Pi, local machine); browser access; zero cloud dependency
- **Origin:** Built by a household that used Bankin until upsell pressure (credit products pushed inside the app) killed trust and usage — famfin is the conflict-free alternative
- **Core principle:** "Finance in service of life — not the other way around." Wealth is a side effect; freedom to make life choices is the goal.

---

## Persona: The Constrained Aspirationalist

- Stable household income: ~€3,500–6,000/month net
- Values-driven spenders: quality food, culture, sport, child's education — aspirations slightly above income
- Not in debt, not irresponsible — just financially invisible to themselves
- Failed at budgeting apps before because apps were working against them (upselling, bad UX, tedious categorization)
- Key emotional state: financial anxiety as background noise — especially when unexpected expenses hit (car repair, medical)
- Life goals: build an emergency fund, save for home purchase, work with less fear
- Couple household: two partners with different desires and risk tolerances; financial conversations tend to be emotionally charged
- Prior tool used: Bankin — abandoned due to commercial conflicts

---

## Core Emotional Design Constraints

Three prohibitions — every design decision is evaluated against these:
1. No sense of loss of control
2. No feeling of overwhelm
3. No sensation of helplessness

Additional non-choices (deliberate):
- No push notifications
- No streaks
- No FOMO mechanics
- No engagement optimization

Target UX feeling: opening the app produces calm. Designer for serenity, not retention.

---

## The Autonomy Pyramid (Core Framework)

Three sequential stages — the user progresses through them linearly:
1. **Informed** — full clarity on current financial state; can identify immediate actions
2. **Confident** — emergency fund built, first investments launched, no panic on unexpected expense
3. **In Control** — partial financial independence, freedom to make life choices without financial coercion

The app knows which stage the household is at and actively guides them to the next. The pyramid is the product — not a dashboard feature.

**Non-negotiable sequencing:** Emergency fund (matelas de sécurité) must be complete before any other goals activate. The app enforces this and explains why, without judgment.

---

## Versioned Feature Roadmap

### V1 — MVP: Clarity
- CSV/OFX import from household bank(s) with per-bank import guides (banks deliberately obscure export — guides are a UX feature)
- Transaction categorization via ML model + optional LLM integration (user chooses provider: Ollama for local/private, OpenAI, Anthropic, etc.)
- Learning engine: remembers every manual correction; never re-asks
- Batch correction by merchant name: "recategorize all BIOCOOP LYON → Quality Food in one tap"; applies to full history
- Revenue auto-detection from recurring credits; presents inference to user for confirmation
- 3–6 month spending projection
- Home server deployment (local web app); browser access for all household members on local network
- Monthly calendar event generation: recurring "Financial Review — 30 min" block in household calendar; frames import as intentional couple ritual

### V2 — Trajectory
- Autonomy Pyramid UI with current stage indicator and guidance toward next stage
- Traffic-light main screen: green (on track), orange (slight drift), red (significant drift)
- Life goal decomposition: input "buy home in Lyon in 5 years, €300K" → auto-calculates required monthly savings, down payment, notary fees, intermediate milestones; recalibrates in real time
- Emergency fund as locked prerequisite: other goals visible but frozen until matelas complete
- Reality recalibrator: when goal is unreachable, offers 3 paths — (1) extend timeline, (2) identify savings delta, (3) alternative scenarios
- Couple alignment: both partners express wants with priority levels (essential / nice-to-have / bonus); app crosses with resources, proposes fact-based arbitration
- Smart goal suspension: after a financial shock (auto-detected), app proactively proposes pausing ambitious goals for 1–2 months rather than waiting for the user to feel like a failure

### V3 — Intelligence
- Contextual benchmarking: compare household spending to INSEE anonymized data filtered by city, income bracket, family composition — statistical comparison, not social
- City cost-of-life normalization: €3,000/month in Lyon ≠ €3,000/month in Paris
- Annual expense calendar: during first 30 days, maps predictable expenses for 12 months (vacances, Christmas, rentrée scolaire, car service) — distinguished from true unexpected expenses
- Values-aligned recommendations: advice framed against user's declared values, not external norms ("you told us health matters — here's how current choices align")
- Subscription audit: detect recurring payments, cross with declared usage, surface alternatives
- Inaction cost display: each month without savings shows real future wealth cost via compound interest over 20 years

### V4 — Freedom
- Freelance / variable income module: revenue smoothing ("salary lissé"), social charges provisioning, monthly virement recommendations
- Fiscal optimization via structure: identify household expenses eligible for professional deduction
- Long-term freedom simulator: "at this investment rate, in X years your passive income covers Y% of your expenses"
- Bank fee negotiator: detect abnormal fees, generate contestation letter templates
- International life simulator: "what if your family lived in Lisbon?" — real purchasing power comparison

---

## Technical Architecture Decisions

- **Local-first:** All financial data stays on home server; no cloud sync; no user account
- **Multi-device access:** Browser-based local web app accessible by both partners on home network — inherently handles couple access without separate account system
- **Data resilience:** Code on GitHub; annual export of last 3 years in CSV/JSON local backup; if device fails: reinstall + reimport; zero cloud dependency
- **Categorization stack:** ML model as primary; LLM as optional enhancement — user wires their preferred provider (local model via Ollama = fully private; cloud API = user's choice and consent)
- **Bank data input:** CSV/OFX manual import (MVP); no real-time bank sync in V1 — deliberate: monthly import ritual is a feature, not a limitation
- **Import friction as feature:** Monthly import is reframed as a 30-min intentional couple ritual with calendar blocking; friction = mindfulness, not failure
- **Open source:** Full codebase public; makes "no conflicts of interest" claim auditable

---

## Competitive Intelligence

| App | Why famfin is different |
|---|---|
| **Bankin** | Upsells financial products inside the app — origin story of famfin |
| **Honeydue** | Only dedicated couple app; free; technically stagnant since 2022; no AI |
| **YNAB** | Best methodology; ~€100/year; US-centric; no local option |
| **Monarch Money** | Strong general app; collaboration features; not family-first; subscription |
| **Copilot** | AI-native; Apple ecosystem only; subscription |
| **Splitwise** | Expense splitting only; not a budgeting tool |

- Market gap: no cross-platform, AI-native, couple/family-first app with modern UX in EU/French market
- Honeydue holds mindshare but is stagnant — open door
- Commercial apps are structurally unable to be conflict-free (their revenue depends on financial product referrals)

---

## Regulatory Context (for future reference if scope expands)

- **Current V1 scope (CSV import, local):** No banking API integration → no PSD2 AISP license required; minimal regulatory exposure
- **If real-time bank sync added (V2+):** AISP license required OR integration via licensed aggregator (Tink, Budget Insight/Powens — they carry the license)
- **RGPD:** Local-first architecture largely sidesteps RGPD exposure; no cloud data processing, no third-party data sharing
- **Multi-member consent (RGPD):** Each household member must individually consent to data visibility by other members — design this from day 1
- **AI Act:** Applies if AI classification used — categorize the system at design time

---

## Rejected / Deprioritized Ideas

- **Real-time bank sync (V1):** Rejected for MVP — CSV import is sufficient and transforms into a meaningful ritual; sync adds PSD2 complexity
- **Social comparison (users vs. users):** Rejected entirely — famfin uses INSEE statistical data only; no social graph, no user community
- **Mobile app (V1):** Out of scope; local browser access sufficient for household use
- **Notifications / streaks / engagement mechanics:** Deliberately excluded — conflict with serenity-first design philosophy
- **Meal planning module:** Adjacent opportunity flagged in brainstorming but explicitly out of scope (food spend optimization is in-scope; meal planning is not)
- **Commercial monetization:** Not a goal; explicitly excluded to preserve conflict-free positioning

---

## Success Criteria for the Household (in priority order)

1. **Serenity** — Opening the app produces calm, not dread. Financial anxiety no longer the default emotional state.
2. **Emergency fund built** — 3 months of expenses in liquid savings, number known and tracked.
3. **Home purchase savings active** — Monthly savings target set, funded, timeline visible.

Secondary: 10+ consecutive monthly imports maintained; both partners reference the app in financial conversations; at least one life goal tracked for 6+ months.

---

## Open Questions / Unresolved

- **Tech stack for home server:** No preference stated — any web framework works (Python/FastAPI, Node/NestJS, etc.)
- **ML categorization model:** Training approach not defined — requires French transaction label data; may start with rule-based + LLM fallback in V1
- **LLM integration design:** UI/UX for provider selection (settings page, onboarding step?) not defined
- **Partner onboarding flow:** How does the second partner first access the app? Simple URL on local network, or some form of invite/setup?
- **Data import history depth:** How many months of historical CSVs should V1 support on first import?
- **Gamification timing:** Badges and milestone rewards are in V3 — confirm if any micro-gamification belongs in V1 (e.g., "first import complete" acknowledgment)
