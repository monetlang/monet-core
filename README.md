# monet-rust

This is an experimental language parser for native FVM WASM.

```ruby

when Deposit {
    from: "alice_addr",
    token: {
      name: "Filecoin",
      ticker: "FIL",
      amount: 100
    }
  } then

    pay {
      to: "alice_addr",
      token: {
        name: "Wrapped Filecoin",
        ticker: "WFIL",
        amount: 90
      }
    }

    pay {
      to: "bob_addr",
      token: {
        name: "Wrapped Filecoin,
        ticker: "WFIL",
        amount: 10
      }
    }
  }

```