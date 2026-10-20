# escrow-swap — Hash-Time-Locked Escrow Swap

Purpose: Atomic hash-time-locked token swap between two parties. An initiator locks token_a and the responder must reveal a preimage (secret) matching `hash_lock` to claim token_b within `time_lock`, otherwise the swap can be cancelled/refunded. Enables trustless peer-to-peer token exchange without a counterparty risk window beyond the time lock.

## Entry points

- `__constructor(admin)` — initialize with admin address
- `create_swap(initiator, responder, token_a, token_b, initiator_amount, responder_amount, hash_lock, time_lock)` — create a swap; returns u64 swap id
- `accept_swap(id, responder, secret)` — responder accepts by revealing the secret preimage
- `complete_swap(id, caller)` — complete the swap after secret is revealed
- `cancel_swap(id, caller)` — cancel a swap (e.g. after time lock expiry)

Reads:
- `get_swap(id)` — Result<SwapRequest> swap details
- `get_swaps()` — Vec<SwapRequest> all swaps

Pause:
- `pause(admin)` / `unpause(admin)` — emergency stop

## Storage keys (DataKey)

| Key | Meaning |
|-----|---------|
| `Admin` | admin address |
| `SwapRequests` | stored swap requests |
| `NextSwapId` | u64 next swap id counter |
| `Paused` | pause flag |

## Events

| Event | Topics | Meaning |
|-------|--------|---------|
| `SwapCreated { id, initiator, ... }` | id, initiator | new swap created |
| `SwapAccepted { id, responder }` | id, responder | swap accepted |
| `SwapCompleted { id, initiator, responder }` | id | swap completed |
| `SwapCancelled { id }` | id | swap cancelled |

## Invariants
- Secret must hash to `hash_lock` (`HashLockMismatch`).
- Swap must be active to accept/complete (`SwapNotActive`).
- Cannot accept an already-accepted swap (`AlreadyAccepted`).
- Cannot complete/cancel after time lock expiry without following the correct path (`TimeLockExpired`).
- Sufficient balances required on both sides (`InsufficientBalance`).
- Mutating functions rejected while paused (`ContractPaused`).

## Test command
```bash
# From repo root
cargo test -p escrow-swap
cargo test -p escrow-swap --doc
# Full workspace
cargo test --workspace
```
