# governance — On-Chain Governance

Purpose: Proposal-based on-chain governance. Proposals carry an executable action (target contract + method + args), require a deposit, pass through a voting period, then a timelock before execution. Supports proposal cancellation/expiration, config updates with a hardcoded 48h timelock, and delegation of vote power. Vote power is one-address-one-vote (quorum expressed as an absolute vote count).

## Entry points

- `init(admin, config)` — initialize with GovernanceConfig
- `create_proposal(proposer, deposit_amount, action, description)` — create a proposal; returns u64 proposal id
- `cast_vote(voter, proposal_id, vote)` — cast For/Against/Abstain vote
- `finalize_proposal(proposal_id)` — finalize after voting ends
- `execute_proposal(proposal_id)` — execute a succeeded proposal after timelock
- `cancel_proposal(caller, proposal_id)` — cancel a proposal
- `expire_proposal(proposal_id)` — expire a proposal
- `queue_config_update(admin, new_config)` — queue a config update (executable after 48h)
- `execute_config_update()` — execute a queued config update
- `cancel_config_update(admin)` — cancel a pending config update

Reads:
- `get_proposal(id)` — Result<Proposal>
- `get_proposals(status, limit)` — Vec<Proposal> filtered by status
- `get_proposal_metadata_page(cursor, limit)` — paginated ProposalMetadataPage
- `get_vote(proposal_id, voter)` — Option<VoteRecord>
- `get_vote_power(voter)` — i128 vote power
- `get_deposit(id)` — Option<i128> proposal deposit
- `get_pending_config_update()` — Option<PendingConfigUpdate>
- `get_config()` — Result<GovernanceConfig>

Staking integration:
- `set_staking_contract(admin, staking)` — link staking contract for vote power

Pause:
- `pause(admin)` / `unpause(admin)` — emergency stop

Delegation:
- `delegate(delegator, delegatee)` — delegate vote power
- `revoke_delegation(delegator)` — revoke delegation

## Storage keys (DataKey)

| Key | Meaning |
|-----|---------|
| `Admin` | governance admin |
| `Config` | GovernanceConfig |
| `ProposalCount` | u64 next proposal id |
| `Proposal(u64)` | per-proposal Proposal |
| `Vote(u64, Address)` | per-proposal-per-voter VoteRecord |
| `Deposit(u64)` | per-proposal deposit |
| `PendingConfig` | queued config update |
| `ProposalsByStatus(ProposalStatus)` | index of proposal ids by status |
| `StakingContract` | linked staking contract address |
| `Delegation(Address)` | delegatee per delegator |
| `Delegators(Address)` | delegators per delegatee |

## Events

| Event | Topics | Meaning |
|-------|--------|---------|
| `ProposalCreated { id, proposer, deposit_amount, voting_ends_at }` | id, proposer | proposal created |
| `VoteCast { id, voter, vote, vote_power }` | id, voter | vote cast |
| `ProposalStatusChanged { id, status }` | id, status | proposal status changed |
| `ProposalExecuted { id, executed_by }` | id, executed_by | proposal executed |
| `ProposalExpired { id, proposer, deposit_refunded }` | id, proposer | proposal expired |
| `ProposalCancelled { id, cancelled_by }` | id, cancelled_by | proposal cancelled |
| `ConfigUpdated { updated_by }` | updated_by | config updated |
| `ConfigUpdateQueued { queued_by, executable_at }` | queued_by | config update queued |
| `ConfigUpdateCancelled { cancelled_by }` | cancelled_by | config update cancelled |
| `DepositRefunded { id, proposer, amount }` | id, proposer | deposit refunded |
| `DepositForfeited { id, proposer, amount }` | id, proposer | deposit forfeited |
| `Delegated { delegator, delegatee }` | delegator, delegatee | delegation created |
| `DelegationRevoked { delegator }` | delegator | delegation revoked |

## Invariants
- Proposal deposit must meet `min_proposal_deposit` (`InsufficientDeposit`).
- Voting only while proposal is active (`VotingNotActive`, `VotingEnded`).
- One vote per voter per proposal (`AlreadyVoted`).
- Execution requires quorum met and pass threshold reached (`QuorumNotMet`); timelock must have elapsed (`TimelockNotElapsed`).
- Config update executable only after the hardcoded 48h delay (cannot be shortened by admin).
- Circular delegation rejected (`CircularDelegation`).
- Mutating functions rejected while paused (`ContractPaused`).

## Test command
```bash
# From repo root
cargo test -p governance
cargo test -p governance --doc
# Full workspace
cargo test --workspace
```
