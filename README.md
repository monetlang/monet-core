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
  }

```