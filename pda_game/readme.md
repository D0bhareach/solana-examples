## Run tests
Run `anchor build`, then start `solana-test-validator -q` in new terminal, then
in previous terminal run `anchor test --skip-local-validator`.

## Error send transaction
Tests are running on localnet.   
I try to run `anchor test` without steps described in [Run tests](#run-tests),
but get this error: Error: failed to send transaction: Transaction simulation failed: This program may not be used for executing instructions
This [answer from stackexchange](https://solana.stackexchange.com/questions/6034/anchor-failed-transaction-transaction-simulation-failed-this-program-may-not) 
has the solutiont to this error. 