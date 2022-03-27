
## Test feature disabling in `solana-test-validator`

### Setup
Assumes using vscode, adjust as necessary for other...

0. Git clone [solana-gadgets](https://github.com/FrankC01/solana-gadgets)
1. Use `solana-install ...` to update to 1.9.6 or install from scratch
2. In `solana-gadgets` go to `rust/scfs-program`
3. Run `cargo build-bpf`

### Test from command line

`cargo test -- --test-threads=1 --nocapture`

### Test in vscode editor
1. `code .`
2. Open `tests/cu_tests.rs`
3. Run `test_base_pass()` -> This has Tx wide CU feature enabled (by default)
4. Run `test_deactivate_tx_cu_pass()` -> This disables the Tx wide CU feature
5. Run `test_devnet_parity_pass()` -> Demonstrates using the scfs engine to get the deactivated feature list from
devnet to feed into the test validator. In effect, emulating devnet from a feature perspective.

You should see similar results. Note that the first test consumes down from Tx wide ComputeBudget whereas
the 2nd and 3rd tests start each instruction with new ComputeBudget of 200_000 CU

Without removing feature: 5ekBxc8itEnPv4NzGJtr8BVVQLNMQuLMNQQj7pHoLNZ9 from genesis_config.accounts on 1.9.6:
```
[2022-01-30T20:58:50.041063000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:58:50.041297000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:58:50.041327000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 187069 units remaining
[2022-01-30T20:58:50.041348000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 200000 compute units
[2022-01-30T20:58:50.041455000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:58:50.050247000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:58:50.050378000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:58:50.050400000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 174125 units remaining
[2022-01-30T20:58:50.050415000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 187056 compute units
[2022-01-30T20:58:50.050509000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:58:50.077803000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:58:50.077961000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:58:50.077980000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 187069 units remaining
[2022-01-30T20:58:50.077992000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 200000 compute units
[2022-01-30T20:58:50.078050000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:58:50.086386000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:58:50.086483000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:58:50.086500000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 174125 units remaining
[2022-01-30T20:58:50.086511000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 187056 compute units
[2022-01-30T20:58:50.086569000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:58:50.174242000Z DEBUG solana_runtime::message_processor::stable_log] Program Vote111111111111111111111111111111111111111 invoke [1]
[2022-01-30T20:58:50.174939000Z DEBUG solana_runtime::message_processor::stable_log] Program Vote111111111111111111111111111111111111111 success
```

With removing feature: 5ekBxc8itEnPv4NzGJtr8BVVQLNMQuLMNQQj7pHoLNZ9 from genesis_config.accounts on 1.9.6:

```bash
[2022-01-30T20:51:35.402343000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:51:35.402625000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:51:35.402660000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 187069 units remaining
[2022-01-30T20:51:35.402683000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 200000 compute units
[2022-01-30T20:51:35.402801000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:51:35.412038000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:51:35.412179000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:51:35.412202000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 187069 units remaining
[2022-01-30T20:51:35.412218000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 200000 compute units
[2022-01-30T20:51:35.412431000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:51:35.440901000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:51:35.441068000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:51:35.441087000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 187069 units remaining
[2022-01-30T20:51:35.441100000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 200000 compute units
[2022-01-30T20:51:35.441160000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:51:35.449623000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc invoke [1]
[2022-01-30T20:51:35.449703000Z DEBUG solana_runtime::message_processor::stable_log] Program log: process_instruction: PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc: 0 accounts, data=[0]
[2022-01-30T20:51:35.449719000Z DEBUG solana_runtime::message_processor::stable_log] Program consumption: 187069 units remaining
[2022-01-30T20:51:35.449730000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc consumed 12944 of 200000 compute units
[2022-01-30T20:51:35.449785000Z DEBUG solana_runtime::message_processor::stable_log] Program PWDnx8LkjJUn9bAVzG6Fp6BuvB41x7DkBZdo9YLMGcc success
[2022-01-30T20:51:35.538520000Z DEBUG solana_runtime::message_processor::stable_log] Program Vote111111111111111111111111111111111111111 invoke [1]
[2022-01-30T20:51:35.539185000Z DEBUG solana_runtime::message_processor::stable_log] Program Vote111111111111111111111111111111111111111 success
```