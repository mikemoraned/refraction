# Fix Later

## imap-proto

```
cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
warning: the following packages contain code that will be rejected by a future version of Rust: imap-proto v0.10.2
note: to see what the problems were, use the option `--future-incompat-report`, or run `cargo report future-incompatibilities --id 1`
```

```
cargo build --future-incompat-report
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
warning: the following packages contain code that will be rejected by a future version of Rust: imap-proto v0.10.2
note:
To solve this problem, you can try the following approaches:


- Some affected dependencies have newer versions available.
You may want to consider updating them to a newer version to see if the issue has been fixed.

imap-proto v0.10.2 has the following newer versions available: 0.11.0, 0.12.0, 0.12.1, 0.12.2, 0.13.0, 0.14.0, 0.14.1, 0.14.2, 0.14.3, 0.15.0, 0.16.0, 0.16.1, 0.16.2


- If the issue is not solved by updating the dependencies, a fix has to be
implemented by those dependencies. You can help with that by notifying the
maintainers of this problem (e.g. by creating a bug report) or by proposing a
fix to the maintainers (e.g. by creating a pull request):

  - imap-proto@0.10.2
  - Repository: https://github.com/djc/tokio-imap
  - Detailed warning command: `cargo report future-incompatibilities --id 2 --package imap-proto@0.10.2`

- If waiting for an upstream fix is not an option, you can use the `[patch]`
section in `Cargo.toml` to use your own version of the dependency. For more
information, see:
https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section

note: this report can be shown with `cargo report future-incompatibilities --id 1`
```
