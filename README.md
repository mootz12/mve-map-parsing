# MVE Map Parsing

This contract serves as an example of a parsing bug when a Map is used as an input to a contract. It appears that the decoding of an ScVal Map argument into a host value fails due to DepthLimiter.

## Deployment

A version of this contract is deployed on testnet:

WASM Hash: `14743fa0e529eb05888b8b40df4b1c34b26e9e3dd3c9214df9c897b05429ed2b`
Contract ID: `CALD2RDMV7O3ZKW3SQGE4ANNLHN5AQCAIFFVDF5LMI2W3ORKLQNFFBKE`

## Tests

The contract exposes two functions "parse_map" and "parse_vec" that convert a map to a vec and vice versa. These tests can be reproduced by simulating the given Transaction Envelope XDRs.

The tests were conducted against: `https://soroban-testnet.stellar.org`

### Test 1: Map arg with 3 entries

XDR:
```
AAAAAgAAAACwiY6jGXPgYefPF2qg3pHpaANmt8buDyj6PKvE1vdtigABhqAAFwkJAAAAAwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAGAAAAAAAAAABFj1EbK/dvKrblAxOAa1Z29BAQEFLUZerYjVtuipcGlIAAAAJcGFyc2VfbWFwAAAAAAAAAQAAABEAAAABAAAAAwAAABIAAAAAAAAAAJ0hDvFbpeMeW6Ehe3q7E3GPSZcgYzyuvxb/jVFc/FPgAAAACgAAAAAAAAAAAAAAAAAAAHsAAAASAAAAAAAAAACv5VaSAF/ak6Hw6OJHQE85YrwSdaawPRhde4NaRYWuaAAAAAoAAAAAAAAAAAAAAAAAAATSAAAAEgAAAAAAAAAAyiKZ/UzlVwDqi8HAzoyfLS++fZomtOO5c/YKw2XfuGQAAAAKAAAAAAAAAAAAAAAAAAAwOQAAAAAAAAAAAAAAAA==
```

Successful:
```
[
  [ 'GCOSCDXRLOS6GHS3UEQXW6V3CNYY6SMXEBRTZLV7C37Y2UK47RJ6A6N2', 123n ],
  [ 'GCX6KVUSABP5VE5B6DUOER2AJ44WFPASOWTLAPIYLV5YGWSFQWXGRPMR', 1234n ],
  [
    'GDFCFGP5JTSVOAHKRPA4BTUMT4WS7PT5TITLJY5ZOP3AVQ3F364GJEXW',
    12345n
  ]
]
```

### Test 2: Map arg with 3 entries

XDR:
```
AAAAAgAAAACwiY6jGXPgYefPF2qg3pHpaANmt8buDyj6PKvE1vdtigABhqAAFwkJAAAAAwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAGAAAAAAAAAABFj1EbK/dvKrblAxOAa1Z29BAQEFLUZerYjVtuipcGlIAAAAJcGFyc2VfbWFwAAAAAAAAAQAAABEAAAABAAAABAAAABIAAAAAAAAAAJ0hDvFbpeMeW6Ehe3q7E3GPSZcgYzyuvxb/jVFc/FPgAAAACgAAAAAAAAAAAAAAAAAAAHsAAAASAAAAAAAAAACv5VaSAF/ak6Hw6OJHQE85YrwSdaawPRhde4NaRYWuaAAAAAoAAAAAAAAAAAAAAAAAAATSAAAAEgAAAAAAAAAAyiKZ/UzlVwDqi8HAzoyfLS++fZomtOO5c/YKw2XfuGQAAAAKAAAAAAAAAAAAAAAAAAAwOQAAABIAAAAAAAAAADwKyzoHp3c+9qYQtLLsIdKzVAo6kAjk1guoCGrQGkJsAAAACgAAAAAAAAAAAAAAAAAABNIAAAAAAAAAAAAAAAA=
```

Failure:
```
{
  _parsed: true,
  id: undefined,
  latestLedger: 1510580,
  events: [ ChildStruct { _attributes: [Object] } ],
  error: 'HostError: Error(Object, InvalidInput)\n' +
    '\n' +
    'Event log (newest first):\n' +
    '   0: [Diagnostic Event] topics:[error, Error(Object, InvalidInput)], data:"failed to convert ScVal to host value"\n' +
    '\n' +
    'Backtrace (newest first):\n' +
    '   0: soroban_env_host::budget::limits::DepthLimiter::with_limited_depth\n' +
    '   1: <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold\n' +
    '   2: <alloc::vec::Vec<T> as alloc::vec::spec_from_iter::SpecFromIter<T,I>>::from_iter\n' +
    '   3: soroban_env_host::host::metered_clone::MeteredIterator::metered_collect\n' +
    '   4: soroban_env_host::host::frame::<impl soroban_env_host::host::Host>::invoke_function\n' +
    '   5: soroban_env_host::e2e_invoke::invoke_host_function_in_recording_mode\n' +
    '   6: soroban_simulation::simulation::simulate_invoke_host_function_op\n' +
    '   7: preflight::preflight_invoke_hf_op::{{closure}}\n' +
    '   8: core::ops::function::FnOnce::call_once{{vtable.shim}}\n' +
    '   9: preflight::catch_preflight_panic\n' +
    '  10: _cgo_f11f55888060_Cfunc_preflight_invoke_hf_op\n' +
    '             at tmp/go-build/cgo-gcc-prolog:105:11\n' +
    '  11: runtime.asmcgocall\n' +
    '             at ./runtime/asm_amd64.s:918\n' +
    '\n'
}
```

### Test 3: Vec arg with 6 entries

XDR:
```
AAAAAgAAAACwiY6jGXPgYefPF2qg3pHpaANmt8buDyj6PKvE1vdtigABhqAAFwkJAAAAAwAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAGAAAAAAAAAABFj1EbK/dvKrblAxOAa1Z29BAQEFLUZerYjVtuipcGlIAAAAJcGFyc2VfdmVjAAAAAAAAAQAAABAAAAABAAAABgAAABAAAAABAAAAAgAAABIAAAAAAAAAAJ0hDvFbpeMeW6Ehe3q7E3GPSZcgYzyuvxb/jVFc/FPgAAAACgAAAAAAAAAAAAAAAAAAAHsAAAAQAAAAAQAAAAIAAAASAAAAAAAAAACv5VaSAF/ak6Hw6OJHQE85YrwSdaawPRhde4NaRYWuaAAAAAoAAAAAAAAAAAAAAAAAAATSAAAAEAAAAAEAAAACAAAAEgAAAAAAAAAAyiKZ/UzlVwDqi8HAzoyfLS++fZomtOO5c/YKw2XfuGQAAAAKAAAAAAAAAAAAAAAAAAAwOQAAABAAAAABAAAAAgAAABIAAAAAAAAAADwKyzoHp3c+9qYQtLLsIdKzVAo6kAjk1guoCGrQGkJsAAAACgAAAAAAAAAAAAAAAAAABNIAAAAQAAAAAQAAAAIAAAASAAAAAAAAAACTFTg8OYqh4DU09XGi7B3ayuT0wIb9LPJNKE0hfS+XCAAAAAoAAAAAAAAAAAAAAAAAADA5AAAAEAAAAAEAAAACAAAAEgAAAAAAAAAAfJQG64a66WKGgU0Cv82pOGYNpvxe5sX+Nhnp1QkVG3gAAAAKAAAAAAAAAAAAAAAAAAAADAAAAAAAAAAAAAAAAA==
```

Successful:
```
{
  GA6AVSZ2A6TXOPXWUYILJMXMEHJLGVAKHKIARZGWBOUAQ2WQDJBGYY2G: 1234n,
  GB6JIBXLQ25OSYUGQFGQFP6NVE4GMDNG7RPONRP6GYM6TVIJCUNXR5TW: 12n,
  GCJRKOB4HGFKDYBVGT2XDIXMDXNMVZHUYCDP2LHSJUUE2IL5F6LQQ45Y: 12345n,
  GCOSCDXRLOS6GHS3UEQXW6V3CNYY6SMXEBRTZLV7C37Y2UK47RJ6A6N2: 123n,
  GCX6KVUSABP5VE5B6DUOER2AJ44WFPASOWTLAPIYLV5YGWSFQWXGRPMR: 1234n,
  GDFCFGP5JTSVOAHKRPA4BTUMT4WS7PT5TITLJY5ZOP3AVQ3F364GJEXW: 12345n
}
```



