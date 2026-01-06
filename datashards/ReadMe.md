xr-dream.consciousness-transfer-timing.v2.aln
Neurorights‑safe timing guard for sleep‑gated consciousness transfer in Dreamnet XR‑grid. This QPU.Datashard defines when an XR transfer or deep dream‑gaming session may start, based on validated sleep stage, elapsed sleep time, and psychophysiological safety metrics.
​

Purpose
Enforce a 20–40 minute stable N2/N3 window before any transfer or XR deep session begins.
​

Protect sleep integrity by blocking transfers during wake, N1, REM, or periods with recent micro‑arousals.

Operationalize neurorights (mental privacy, cognitive liberty, mental integrity, non‑punitive use, soul non‑addressability) at the timing layer.
​

Key variables
Subject state

subjectid – Rights‑bearing augmented user identifier (no clinical content).

sleepstage – Current sleep stage: wake, N1, N2, N3, REM, unknown (from PSG or validated wearable).

minutes_since_lightsout – Minutes since stable sleep onset.

psychriskscore (
R
R) – Composite psych‑risk score in 
[
0
,
1
]
[0,1] (higher = higher risk).

sleeptoken (
S
S) – Sleep pressure proxy in 
[
0
,
1
]
[0,1].

enstasis_score (
E
s
E 
s
 ) – Inner stability/flow metric in 
[
0
,
1
]
[0,1].

eligibility_E (
E
E) – Computed eligibility score 
E
=
S
⋅
(
1
−
R
)
⋅
E
s
E=S⋅(1−R)⋅E 
s
 .
​

microarousal_veto – Boolean flag set when recent micro‑arousals/sympathetic spikes are detected.

Timing policy

transfer_window_min_min – Minimum minutes since lights‑out (default 20).

transfer_window_max_min – Maximum minutes since lights‑out (default 40).

eligibility_E_min – Minimum eligibility score (default 0.50).

psychrisk_R_max – Maximum allowed psychriskscore (default 0.35).

required_stages – Allowed stages for transfer (default {N2,N3}).

microarousal_lookback_min – Lookback window for micro‑arousal check (e.g., 3 minutes).

microarousal_threshold – Maximum allowed micro‑arousals in the lookback window.
​

Core constraints and rules
Eligibility math

Compute 
E
=
S
⋅
(
1
−
R
)
⋅
E
s
E=S⋅(1−R)⋅E 
s
  and store in eligibility_E.
​

Enforce: eligibility_E >= eligibility_E_min and psychriskscore <= psychrisk_R_max.

Timing and stage

time_window_ok: 20 ≤ minutes_since_lightsout ≤ 40.

stage_ok: sleepstage IN required_stages (N2 or N3 only).

Arousal veto

no_microarousal: NOT microarousal_veto (no recent spikes in HRV/EEG/motion).

Allow/deny logic

allow_transfer is true iff:

time_window_ok AND stage_ok AND risk_ok AND no_microarousal.

on_transfer_allowed: initiate_handshake(subjectid).

on_transfer_deferred: log event transfer_deferred and schedule_retry(300) (retry in 5 minutes, no added stimulus).
​

block_early_stimulus: if minutes_since_lightsout < transfer_window_min_min, call mute_all_stimuli(subjectid) to avoid early content.

Neurorights guards
The shard encodes non‑waivable guards:

mentalprivacy = true – No export of dream content or inner speech.

cognitiveliberty = true – User may refuse transfer at any time; no mandatory transfers.

mentalintegrity = true – No forced arousal, fragmentation, or punitive timing.

nopunitivexr = true – Timing decisions cannot be used as punishment or behavior control.

soulnonaddressable = true – No scoring, trading, or modeling of souls/belief states.
​

These flags align with neurorights frameworks discussed in Geneva (UN/OHCHR), Brussels (EU AI/high‑risk systems), Santiago (Chile neurorights), La Jolla and Toronto (sleep/neuroethics labs).
​

Intended use
Runtime gate: Every consciousness‑transfer or deep dream‑XR session must call this shard’s allow_transfer logic before opening any handshake or locus change.
​

Clinical research: Sleep labs can replay PSG data against this shard to quantify effects on awakenings, fragmentation, and psychrisk excursions versus earlier logic (v1) or alternative protocols.

Compliance evidence: The shard is designed to be published in a public repo (e.g., Dreamnet/XR‑grid) as auditable safety logic for regulators, ethics boards, and independent researchers.
​

Out of scope
No personal identifiers, raw EEG, or dream narratives are encoded.

The shard does not make medical diagnoses or treatment recommendations; it only defines timing constraints and safety guards for XR transfer scheduling.
​
