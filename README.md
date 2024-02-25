# monet-rust

This is an experimental language parser for native FVM WASM.

```ruby

when Deposit {
    from: "some_addr",
    token: {
      name: "Filecoin",
      ticker: "FIL",
      amount: 100
    }
  } then pay {
    to: "some_addr",
    token: {
      name: "Wrapped Filecoin",
      ticker: "WFIL",
      amount: 90
    }

    end_on {
      duration: 10000000
    }
  }

```