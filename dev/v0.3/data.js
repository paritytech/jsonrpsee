window.BENCHMARK_DATA = {
  "lastUpdate": 1634315567378,
  "repoUrl": "https://github.com/paritytech/jsonrpsee",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "name": "paritytech",
            "username": "paritytech"
          },
          "committer": {
            "name": "paritytech",
            "username": "paritytech"
          },
          "id": "1348ff93be9cf7ebea00a2508ee89a946eb9a78a",
          "message": "benches: add option to run benchmarks again jsonrpc crate servers",
          "timestamp": "2021-10-15T13:13:38Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/527/commits/1348ff93be9cf7ebea00a2508ee89a946eb9a78a"
        },
        "date": 1634315566751,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 146,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 173,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 130277,
            "range": "± 2897",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 109946,
            "range": "± 1514",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 95703,
            "range": "± 1473",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 160363,
            "range": "± 4437",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 272896,
            "range": "± 8466",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1688779,
            "range": "± 23005",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3298950,
            "range": "± 12398",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6491032,
            "range": "± 31357",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12915099,
            "range": "± 53944",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 25584452,
            "range": "± 205087",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 50983649,
            "range": "± 472215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 64819,
            "range": "± 2086",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 83029,
            "range": "± 1513",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 91114,
            "range": "± 1215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 110529,
            "range": "± 1641",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 158277,
            "range": "± 10318",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 111001,
            "range": "± 3286",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 183649,
            "range": "± 4133",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 325597,
            "range": "± 9026",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 709709,
            "range": "± 15892",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1356558,
            "range": "± 59066",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2769896,
            "range": "± 189621",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 131182,
            "range": "± 4249",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 109449,
            "range": "± 1051",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 95720,
            "range": "± 1721",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 161894,
            "range": "± 3915",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 274673,
            "range": "± 3346",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1696345,
            "range": "± 23796",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3296596,
            "range": "± 13653",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6516590,
            "range": "± 23787",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12900478,
            "range": "± 56597",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 25733182,
            "range": "± 226412",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 51193609,
            "range": "± 359859",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65137,
            "range": "± 2075",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 82247,
            "range": "± 4784",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 91429,
            "range": "± 933",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 112501,
            "range": "± 1451",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 159770,
            "range": "± 1835",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 112662,
            "range": "± 2129",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 181935,
            "range": "± 2273",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 324086,
            "range": "± 9008",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 678049,
            "range": "± 16799",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1312521,
            "range": "± 136273",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2641402,
            "range": "± 202313",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 80779,
            "range": "± 2046",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 14929,
            "range": "± 2624",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 834,
            "range": "± 144",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}