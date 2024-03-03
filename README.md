# monet-rust

This is an experimental language parser for native FVM WASM.

```ruby

when
  Deposit {
    from: "alice_addr",
    token: {
      name: "Filecoin",
      ticker: "FIL",
      amount: 100
    }
  } with {
    to: "alice_addr",
    name: "Wrapped Filecoin",
    comm: 0.1
  } then
    pay {
      to,
      token: {
        name,
        ticker: "WFIL",
        amount: 100 - 100 * comm
      }
    }

    pay {
      to: "bob_addr",
      token: {
        name: "Wrapped Filecoin,
        ticker: "WFIL",
        amount: 100 * comm
      }
    }

    propose {
      deal_request: {
        piece_cid: "Qmx",
        piece_size: 123,
        verified_deal: true,
        label: "label",
        start_epoch: 123,
        end_epoch: 123,
        storage_price_per_epoch: 123,
        provider_collateral: 123,
        extra_params_version: 123
      }
    }

    close {
      duration: 1000000
    }
  }

  Deposit { from: "bob_addr" } then close

  Deposit { 
    from: "alice_addr",
    token: with {
      token
    }
  } then
    pay {
      to: "alice_addr",
      token
    }

```