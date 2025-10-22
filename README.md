# Observing PHP for fun and profit!

## Preparations

> [!IMPORTANT]
> Please do these steps before attending the workshop

```bash
git clone https://github.com/realFlowControl/observing-php-for-fun-and-profit.git
cd observing-php-for-fun-and-profit
make build
make prep
```

These steps:
- clone the repository which gets you kick started
- pulls all docker images that we need
- builds the dev container with Clang, Rust and PHP in it
- builds the extensions and prewarms the cargo cache

Once done you can run `make test` which should show you a passing test:

```
// ...
PASS Basic function and extension being loaded [tests/phpt/basic_001.phpt] 
=====================================================================
Number of tests :     1                 1
Tests skipped   :     0 (  0.0%) --------
Tests warned    :     0 (  0.0%) (  0.0%)
Tests failed    :     0 (  0.0%) (  0.0%)
Tests passed    :     1 (100.0%) (100.0%)
---------------------------------------------------------------------
Time taken      : 0.010 seconds
=====================================================================
```
