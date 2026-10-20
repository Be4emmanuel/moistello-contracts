# treasury — Protocol Fee Treasury

Purpose: Collects and holds the 0.5% protocol fee on every circle payout distribution. Admin-controlled withdrawals and token rescue. Circles deposit fees into this contract; the admin can withdraw accumulated funds or rescue errant tokens.

## Entry points

- `init(admin, token)` — initialize with admin address and token contract address
- `deposit_fee(from, amount, circle_id)` — deposit a fee (called by circles on payout)
- `withdraw(admin, to, amount)` — admin withdrawal of accumulated funds
- `rescue_tokens(admin, to, token, amount)` — rescue tokens sent to the contract in error

Reads:
- `get_balance()` — i128 current treasury holdings
- `get_deposits()` — Vec<Deposit> history of all fee deposits

Pause:
- `pause(admin)` / `unpause(admin)` — emergency stop

## Storage keys (DataKey)

| Key | Meaning |
|-----|---------|
| `Admin` | treasury admin |
| `Balance` | i128 current balance |
| `Deposits` | Vec<Deposit> fee deposit history |
| `Withdrawals` | Vec<Withdrawal> withdrawal history |
| `Token` | token contract address |

## Events

| Event | Topics | Meaning |
|-------|--------|---------|
| `FeeDeposited { from, amount, circle_id }` | — | fee deposited |
| `FundsWithdrawn { to, amount }` | — | funds withdrawn |
| `TokensRescued { to, token, amount }` | — | tokens rescued |

## Invariants
- Only the initialized admin may withdraw or rescue tokens (`Unauthorized`).
- Withdrawals cannot exceed the current balance (`InsufficientBalance`).
- Amount must be positive (`InvalidAmount`).
- Mutating functions rejected while paused (`ContractPaused`).
- Cannot initialize twice (`AlreadyInitialized`); cannot pause an already-unpaused / unpause an already-paused contract beyond idempotent no-ops.

## Test command
```bash
# From repo root
cargo test -p treasury
cargo test -p treasury --doc
# Full workspace
cargo test --workspace
```
