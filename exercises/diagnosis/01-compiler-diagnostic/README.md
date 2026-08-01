# Compiler diagnostic: ownership at an install boundary

The artifact is the complete first report. The service count log was added during
an observability change; compilation then failed.

## Contract

Installation commits the parsed configuration to the registry. On success, the
log reports the number of installed services. Avoid duplicating a potentially
large configuration merely for logging.

## Investigation

Use the labels to reconstruct ownership transfer. Inspect the installer's needs and
the caller's last uses before accepting any compiler suggestion. There are several
valid API shapes; justify which layer should own the data after success and failure.

Run the compile-failure exercise with `make ex N=27` only after completing
[WORKSHEET.md](WORKSHEET.md).

Reference: [`E0382`](https://doc.rust-lang.org/error_codes/E0382.html) and
[understanding ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html).
