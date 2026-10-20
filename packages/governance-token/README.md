# governance-token — Governance Token (ST-20-like)

Purpose: Fungible governance token with admin controls: mint/burn/clawback, freeze/unfreeze accounts, allowlist mode, and admin transfer. Standard ERC-20-style read functions (balance, allowance, total_supply, name, symbol, decimals) plus transfer/transfer_from/approve with expirations. Used for governance voting power and membership-weighted actions across the Moistello ecosystem.

## Entry points

- `initialize(admin, name, symbol, decimals)` — initialize token metadata and admin
- `transfer(from, to, amount)` — transfer tokens
- `transfer_from(spender, from, to, amount)` — spender transfers on owner's behalf
- `approve(owner, spender, amount, expiration_ledger)` — approve spender with expiration
- `mint(admin, to, amount)` — admin mints tokens
- `burn(from, amount)` — burn tokens (owner)
- `clawback(admin, from, amount)` — admin clawback
- `burn_from(admin, from, amount)` — #518 admin-authorized burn from any account
- `freeze(admin, account)` / `unfreeze(admin, account)` — freeze/unfreeze an account
- `set_admin(admin, new_admin)` — transfer admin role

Reads:
- `balance(account)` — i128 balance
- `allowance(owner, spender)` — AllowanceData
- `total_supply()` — i128 total supply
- `name()` / `symbol()` / `decimals()` — metadata
- `get_admin()` — Result<Address> current admin
- `is_frozen(account)` — bool
- `is_allowlist_mode_enabled()` — bool
- `is_allowlisted(account)` — bool

Allowlist management:
- `set_allowlist_mode(admin, enabled)` — enable/disable allowlist mode
- `add_to_allowlist(admin, account)` / `remove_from_allowlist(admin, account)`

Pause:
- `pause(admin)` / `unpause(admin)` — emergency stop

## Storage keys (DataKey)
Defined in contract/src/types.rs and contract.rs (standard token storage: balances, allowances, metadata, admin, paused, allowlist).

## Events

| Event | Topics | Meaning |
|-------|--------|---------|
| `Transfer { from, to, amount }` | — | tokens transferred |
| `Approve { owner, spender, amount, expiration_ledger }` | — | approval granted |
| `Mint { to, amount }` | — | tokens minted |
| `Burn { from, amount }` | — | tokens burned |
| `Clawback { from, amount }` | — | tokens clawed back |
| `AccountFrozen { account }` | — | account frozen |
| `AccountUnfrozen { account }` | — | account unfrozen |
| `AllowlistModeChanged { enabled }` | — | allowlist mode toggled |
| `AllowlistAdded { account }` | — | account added to allowlist |
| `AllowlistRemoved { account }` | — | account removed from allowlist |

## Invariants
- Transfer/burn amount must be positive and not exceed balance (`InsufficientBalance`, `InvalidAmount`).
- Only admin may mint, burn_from, clawback, freeze, set_admin (`NotAdmin`/`Unauthorized`).
- Frozen accounts cannot transfer (`Frozen`).
- Allowance cannot exceed max / go negative (`AllowanceExceeded`, `NegativeAllowance`); expired allowances are invalid (`AllowanceExpired`).
- Arithmetic overflow/underflow returns typed errors (`Overflow`, `Underflow`), never panics.
- Cannot transfer to self (`CannotTransferToSelf`).
- Mutating functions rejected while paused (`ContractPaused`).

## Test command
```bash
# From repo root
cargo test -p governance-token
cargo test -p governance-token --doc
# Full workspace
cargo test --workspace
```
