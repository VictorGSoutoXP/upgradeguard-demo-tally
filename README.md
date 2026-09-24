# tally: UpgradeGuard demo program

A small Anchor program in the state most legacy Solana programs are in today: Anchor 0.30.1, Solana CLI 1.18.26 pinned in `Anchor.toml`, built as **sBPF v0**. Once SIMD-0500 is active, loader-v3 rejects its next upgrade.

UpgradeGuard takes it to sBPF v3 through a pull request, with evidence:

1. **Scan** the deployed program: `upgradeguard scan <PROGRAM_ID> -c devnet` reports sBPF v0, `NOT READY`.
2. **Migrate**: run the *UpgradeGuard migration* workflow. It opens a PR that moves Anchor to 0.31.2 and the toolchain to Agave 4.2.2 + cargo-build-sbf 4.2.0, and updates `Cargo.lock`. It never merges.
3. **Evidence**: the same run builds the PR branch twice for sBPF v3, checks the ELF, replays `tests/upgradeguard/fixtures.json` against the deployed v0 program and the v3 candidate in LiteSVM, and publishes `SIMD-0500 COMPATIBILITY: PASSED` (or what failed).
4. A human reviews, merges and upgrades the program. UpgradeGuard never holds keys or deploys.

This is compatibility evidence, not a security audit.

## Program

| Instruction | Does |
|---|---|
| `initialize(limit)` | creates a tally owned by the signer |
| `increment(amount)` | adds to the count, fails past `limit`, returns the new count |
| `reset` | sets the count to zero; only the authority |

## Setting up the demo

1. Create a GitHub repository with the contents of this folder at its root.
2. In the repository, enable **Settings → Actions → General → Workflow permissions → "Allow GitHub Actions to create and approve pull requests"**.
3. The program address is `C6AC67E3kLs1iK7Wj7G7AzVFEkjyf14uXuBFFt6V49mp`; its `tally-keypair.json` stays with the maintainer and is never committed (`.gitignore` covers it). To use another address, create one with `solana-keygen new -o tally-keypair.json`, replace the address in `programs/tally/src/lib.rs`, `Anchor.toml`, `tests/upgradeguard/fixtures.json` and both workflows, and rebuild the v0 binary with the legacy toolchain (`cargo build-sbf` from Agave 4.2.2 builds v0 by default) into `tests/upgradeguard/tally-v0.so`.
4. Check that devnet still accepts v0 deployments: `upgradeguard status -c devnet`. If `disable_sbpf_v0_v1_v2_deployment` is active there, a v0 program cannot be deployed; use the committed binary as the baseline instead (`baseline-so: tests/upgradeguard/tally-v0.so` in place of `program-id`).
5. Deploy the v0 binary with your own wallet: `solana program deploy tests/upgradeguard/tally-v0.so --program-id tally-keypair.json -u devnet`.
6. Run **Actions → UpgradeGuard migration → Run workflow**.

`tests/upgradeguard/tally-v0.so` is the v0 build of this source, so the baseline is reproducible without the chain.
