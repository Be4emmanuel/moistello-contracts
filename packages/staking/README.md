# staking — Token Staking with Voting Power

Purpose: Token staking contract that backs governance vote power. Users stake tokens for a fixed period (1/3/6/12 months) with time-weighted voting power multipliers (1x/2x/3x/5x). Supports top-up, unstake with an unbonding period (14 days), claim after unbonding, and slashing with a notice period. Integrates with the governance contract for vote-power queries.

## Entry points

- `init(admin, token)` — initialize with admin and token contract
- `stake(user, amount, period_months)` — stake tokens; returns voting power position
- `top_up(user, amount)` — add tokens to an active stake
- `unstake(user)` — initiate unstake (starts unbonding period)
- `claim(user)` — claim tokens after unbonding completes
- `slash(admin, user, shortfall)` — #523 slash collateral proportional to shortfall; returns amount actually slashed
- `create_slash_notice(admin, user, amount)` — #467 open a slashing notice period
- `cancel_slash(admin, user)` — cancel a slash during the notice period
- `execute_slash(admin, user)` — execute a slash after notice period elapses
- `update_admin(current_admin, new_admin)` — transfer admin role
- `set_slash_notice_period(admin, period_seconds)` — set global slash notice period (default 24h)

Reads:
- `get_voting_power(user)` — i128 current voting power
- `get_stake(user)` — Option<StakePosition>
- `get_stake_amount(user)` — i128 raw staked amount
- `get_unbonding(user)` — Option<UnbondingPosition>
- `get_total_staked()` — i128 total staked across all users
- `get_all_stakers()` — Vec<Address> all active stakers
- `get_time_weighted_amount(user)` — i128 time-weighted amount
- `get_slash_notice_period()` — u64 global notice period
- `get_slash_notice(user)` — Option<(i128, u64, Address)> active slash notice

Pause:
- `pause(admin)` / `unpause(admin)` — emergency stop

## Storage keys (DataKey)

| Key | Meaning |
|-----|---------|
| `Admin` | staking admin |
| `Token` | token contract address |
| `Paused` | pause flag |
| `Stake(Address)` | per-user active StakePosition |
| `Unbonding(Address)` | per-user unbonding position |
| `TotalStaked` | i128 aggregate staked |
| `StakerList` | ordered list of active staker addresses |
| `SlashNotice(Address)` | active slash notice per user |
| `SlashNoticePeriod` | u64 global notice period seconds |

## Events

| Event | Topics | Meaning |
|-------|--------|---------|
| `Staked { user, amount, period, multiplier, voting_power }` | user | tokens staked |
| `UnstakeInitiated { user, amount, claimable_time }` | user | unstake started |
| `Claimed { user, amount }` | user | tokens claimed |
| `VotingPowerQueried { user, voting_power }` | user | vp query (governance integration) |
| `StakeToppedUp { user, added_amount, new_total_amount, unlock_time, new_voting_power }` | user | stake topped up |
| `SlashNoticeCreated { executor, user, amount, notice_until }` | executor, user | slash notice opened |
| `SlashCancelled { executor, user, amount }` | executor, user | slash notice cancelled |
| `SlashExecuted { executor, user, amount }` | executor, user | slash executed |
| `Slashed { user, amount, shortfall }` | user | #523 stake slashed |

## Invariants
- Minimum stake `MIN_STAKE_AMOUNT` (1_0000000) rejects dust positions.
- Period must be one of 1/3/6/12 months (`InvalidPeriod`).
- Cannot stake twice while an active position exists (`AlreadyStaked`).
- Cannot claim before unbonding completes (`UnbondingNotComplete`).
- Slash amount capped at `min(shortfall, stake)`; insufficient contract balance returns `InsufficientContractBalance` as a typed preflight error.
- Mutating functions rejected while paused (`ContractPaused`); reentrancy rejected (`ReentrantCall`).

## Test command
```bash
# From repo root
cargo test -p staking
cargo test -p staking --doc
# Full workspace
cargo test --workspace
```
