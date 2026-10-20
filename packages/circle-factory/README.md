# circle-factory — Circle Deployment Factory

Purpose: Deploys new Circle instances and maintains the on-chain circle registry. The single deployment contract for all circles; propagates protocol-wide configuration (treasury, reputation registry, fee bps) into every circle it deploys. Enforces organizer rate limiting so a single organizer cannot spam circle deployments.

## Entry points

- `init(admin, fee_bps, circle_wasm_hash, organizer_rate_limit, rate_limit_period_secs)` — initialize factory
- `init_with_config(admin, fee_bps, treasury, reputation_registry, circle_wasm_hash)` — initialize with protocol config
- `deploy_circle(config)` — deploy a new Circle; returns the circle contract address
- `set_fee_config(admin, fee_bps)` — update protocol fee basis points
- `set_factory_config(admin, treasury, reputation_registry, fee_bps)` — update factory-wide config
- `pause(admin)` / `unpause(admin)` — emergency stop

Reads:
- `get_circles()` — CircleRegistry (all deployed circles)
- `get_circle_config(cid)` — CircleConfig for a deployed circle
- `get_circle_count()` — u32 count of deployed circles
- `get_fee_config()` — FeeConfig
- `get_rate_limit_config()` — RateLimitConfig
- `get_factory_config()` — Option<FactoryConfig>

## Storage keys (DataKey)

| Key | Meaning |
|-----|---------|
| `Admin` | factory admin |
| `FeeConfig` | protocol fee config |
| `CircleList` / `CircleCount` | deployed circle registry |
| `WasmHash` | circle implementation WASM hash |
| `CircleConfig(Address)` | per-circle config map |
| `CanonicalDeployment(BytesN<32>)` | canonical deployment tracking |
| `RateLimitConfig` | organizer rate limit settings |
| `OrganizerPeriodCount(Address, u64)` | per-organizer deployment counts per period |
| `FactoryConfig` | treasury / reputation_registry / fee_bps |

## Events

| Event | Topics | Meaning |
|-------|--------|---------|
| `CircleDeployed { creator, circle_id, name }` | creator, circle_id | new circle deployed |
| `FeeConfigUpdated { old_fee_bps, new_fee_bps, updated_by }` | updated_by | fee config changed |
| `FactoryConfigUpdated { treasury, reputation_registry, fee_bps, updated_by }` | updated_by | factory config changed |

## Invariants
- `WasmHash` must be set before any circle can be deployed (`WasmHashNotSet`).
- Fee bps must be valid (`InvalidFeeBps`).
- Organizer deployments are rate-limited per configured window (`RateLimitExceeded`).
- Factory config must be set before circles can be configured on deploy (`FactoryConfigNotSet`).
- Paused factory rejects mutating calls (`ContractPaused`).

## Test command
```bash
# From repo root
cargo test -p circle-factory
cargo test -p circle-factory --doc
# Full workspace
cargo test --workspace
```
