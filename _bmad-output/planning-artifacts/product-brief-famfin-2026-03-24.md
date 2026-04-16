---
title: "Product Brief: famfin"
status: "complete"
created: "2026-03-24"
updated: "2026-04-07"
inputs:
  - "_bmad-output/brainstorming/brainstorming-session-2026-03-22-001.md"
  - "_bmad-output/planning-artifacts/research/domain-finances-familiales-partage-budget-research-2026-03-22.md"
---

# Product Brief: famfin

## Executive Summary

Most families with stable incomes are quietly losing ground — not from lack of money, but from lack of clarity. They don't know what they can afford, they don't have a plan, and the tools they turn to for help are actively working against them. Bankin upsells loans. Linxo pushes credit cards. Mint was shut down. YNAB costs €100/year and speaks American. The entire market for personal finance software is funded by the same financial products it's supposed to help you evaluate objectively. One household tried Bankin, found it genuinely useful — then got tired of being sold credit products every time they opened it. They stopped using it. The clarity disappeared. This tool is the answer they were looking for.

famfin is a family finance tool built with no commercial agenda. It is designed for one household, runs on a home server, stores nothing in the cloud, and is paid for by no one. Its purpose is simple: give a family full clarity over their financial situation, align that clarity with their life goals, and progressively move them from financial anxiety to financial autonomy. The richness is a side effect. The freedom to make choices — to take the job that matters instead of the job that pays most — is the point.

The guiding principle is: **finance in service of life, not the other way around.**

## The Problem

A family earns €4,500/month net. They know roughly what comes in. They have only a blurry sense of what goes out. They feel like they should be saving, but somehow never do. A car repair for €900 triggers a week of anxiety. When they talk about buying a home, the conversation ends in vague discomfort. They're not in debt. They're not irresponsible. They're just — floating.

They've tried apps. Bankin worked for a while, but it wanted to sell them a credit card. The categories were off. Fixing them was tedious. They gave up.

This is the "constrained aspirationalist" household: values-driven, educated, intentional about life choices — but financially invisible to themselves. Their problem is not income. It is the absence of a trusted, conflict-free picture of where they stand and where they're headed.

The cost of this status quo is real: decisions made from anxiety rather than clarity, life goals that stay vague because they've never been quantified, savings that never happen because the system never makes them automatic.

## The Solution

famfin solves the clarity problem first, then the planning problem, then the optimization problem — in that order.

**Phase 1: Clarity (MVP)**
Monthly CSV/OFX import from the household bank(s). Automatic transaction categorization via an ML model, with an optional LLM integration the household can wire to any provider — a local model (Ollama, fully private) or a cloud API (OpenAI, Anthropic) depending on their preference. The engine remembers every manual correction. Batch correction ("recategorize all BIOCOOP LYON transactions as Quality Food in one tap"). A 3–6 month spending projection. Revenue detected automatically from recurring credits. After one import cycle, the household knows exactly where their money is going.

**Phase 2: Trajectory**
The Autonomy Pyramid: three sequential stages — *Informed* (clarity on current state), *Confident* (emergency fund built, first investments launched), *In Control* (financial independence partial, free to choose). The app knows which stage the household is at and guides them toward the next. The main screen shows a trajectory — "you're on track" or "you've drifted" — with a single traffic light indicator. Not a dashboard to study. A signal to trust.

Life goals become managed projects: "buy a home in Lyon in 5 years, €300K" decomposes automatically into a required monthly savings amount, a timeline, intermediate milestones. When finances shift, the projection recalibrates in real time. The emergency fund is non-negotiable — it must be complete before other goals activate.

**Phase 3: Intelligence**
Contextual benchmarking from public INSEE data: how does this household's spending compare to similar households in the same city, same income bracket, same family composition? The comparison is statistical, not social. Subscriptions audited. Annual expected expenses mapped (Christmas, school rentrée, car service). A household calendar of predictable financial events, so "exceptional" expenses stop being surprises.

The app's voice is warm and non-judgmental. It never says "you spend too much." It says "here's how this aligns with what you told us you care about."

## What Makes This Different

**No conflicts of interest — verifiable.** famfin is not funded by a bank, an insurer, or a lender. It recommends nothing it earns commission on. It is open source: anyone can read the code and confirm there is no hidden agenda. The "no conflicts" claim is auditable, not just declared.

**Privacy by architecture.** All data lives on a home server (NAS, Raspberry Pi, local machine). No cloud sync, no account creation, no data broker risk. The household's financial life stays inside their home.

**Designed for serenity, not engagement.** Every design decision is evaluated against three prohibitions: no sense of loss of control, no feeling of overwhelm, no sensation of helplessness. The goal is for the user to open the app and feel calm. famfin has no notifications, no streaks, no FOMO mechanics. This is a deliberate non-choice — and it is genuinely rare in a market where commercial apps are optimized for sessions opened, not lives improved.

**The coach that makes itself useless.** The explicit objective of famfin is to transfer financial competence to the household until the app is no longer needed for basic decisions. Success is measured in autonomy gained, not in sessions opened.

**Built for the couple.** Both partners can express wants, priorities, and constraints. The app mediates — not emotionally, but factually: "this goal is reachable in 3 months, this one in 8 if you do X." Financial conversations between partners shift from negotiations to informed choices.

## Who This Serves

**Primary: The Constrained Aspirationalist Household**
Two working adults. One or two children. Stable income in the €3,500–6,000/month net range. Values-driven spenders — quality food, culture, a good school — with aspirations that slightly outpace their income. They've tried budgeting apps and quit because the tools felt adversarial or useless. They feel financial anxiety as background noise. They want to buy a home. They want to stop panicking when something breaks. They want to feel like they're going somewhere.

**Secondary: The Family Getting Serious**
Single income, or recently combined finances after moving in together. Just realized that "we should probably have a budget" and ready to act on it. Needs a frictionless first experience that delivers value within the first import cycle.

## Success Criteria

famfin has worked when the household can say:

1. **Serenity** — Opening the app produces calm, not dread. Financial anxiety is no longer the default emotional state.
2. **Emergency fund built** — The household has 3 months of expenses in liquid savings and knows exactly what that number is.
3. **Home purchase savings active** — A monthly savings target toward the down payment is set, funded, and tracked with a visible timeline.

Secondary signals:
- Monthly import ritual is maintained (>10 consecutive months)
- Both partners use the app and reference it in financial conversations
- At least one life goal has been decomposed, funded, and tracked for 6+ months

## Scope

**In for V1:**
- CSV/OFX import with per-bank import guides
- Automatic categorization with a learning engine
- Batch correction by merchant name
- 3–6 month spending projection
- Revenue auto-detection from recurring credits
- Home server deployment (local web app, browser access)
- Monthly calendar event generation: a recurring "Financial Review — 30 min" block added automatically to the household calendar, framing the monthly import as an intentional couple ritual rather than a chore

**Explicitly out for V1:**
- Real-time bank sync (Open Banking / PSD2)
- Life goal decomposition
- Autonomy Pyramid / trajectory indicator
- Couple features
- INSEE benchmarking
- Investment recommendations
- Mobile app

**V2 and beyond:**
The full Autonomy Pyramid, life goal management, couple alignment, trajectory indicator, and INSEE contextual benchmarking. Later: freelance/variable income module, subscription auditing, long-term freedom simulator.

## Vision

In three years, famfin is the financial nervous system of the household. Import is automatic. The trajectory is trusted. The emergency fund is built. The home purchase savings are ahead of schedule. The household makes career decisions based on what they want, not what they need. One partner turned down a promotion. Not because they couldn't afford not to — but because famfin showed them they could afford to say no.

The app doesn't celebrate this. It expected it. That was always the plan.
