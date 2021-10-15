window.BENCHMARK_DATA = {
  "lastUpdate": 1634318909764,
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
          "id": "2895c769db2cbfeaf4ae49b43194a63284ba6fb0",
          "message": "testme",
          "timestamp": "2021-10-15T13:13:38Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/529/commits/2895c769db2cbfeaf4ae49b43194a63284ba6fb0"
        },
        "date": 1634318908744,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 169,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 203,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 185672,
            "range": "± 17233",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 100,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 178595,
            "range": "± 16848",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 144593,
            "range": "± 10476",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 220669,
            "range": "± 19391",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 385395,
            "range": "± 47174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1766499,
            "range": "± 110198",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3325833,
            "range": "± 248519",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6156048,
            "range": "± 503582",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11954971,
            "range": "± 769218",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 25150881,
            "range": "± 1972762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 48830321,
            "range": "± 3028295",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 97801,
            "range": "± 7209",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 86,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 114451,
            "range": "± 10503",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 123519,
            "range": "± 16654",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 161253,
            "range": "± 23065",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 188738,
            "range": "± 193653",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 163555,
            "range": "± 80020",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 224597,
            "range": "± 20661",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 392803,
            "range": "± 45960",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 790696,
            "range": "± 93585",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1569193,
            "range": "± 174977",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3205700,
            "range": "± 402181",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 163362,
            "range": "± 22015",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 177566,
            "range": "± 14718",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 137139,
            "range": "± 16512",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 225491,
            "range": "± 46832",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 365561,
            "range": "± 28667",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1639949,
            "range": "± 141776",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3276764,
            "range": "± 243278",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6104292,
            "range": "± 429972",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12344398,
            "range": "± 929115",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 24299067,
            "range": "± 1310432",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48485397,
            "range": "± 2761580",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 98811,
            "range": "± 7432",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 113433,
            "range": "± 11114",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 132218,
            "range": "± 10576",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 158028,
            "range": "± 11432",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 188803,
            "range": "± 17049",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 151065,
            "range": "± 16560",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 241401,
            "range": "± 26086",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 506666,
            "range": "± 55292",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 811637,
            "range": "± 76143",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1565780,
            "range": "± 159733",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3278626,
            "range": "± 454366",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 135749,
            "range": "± 22231",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 25608,
            "range": "± 5413",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1598,
            "range": "± 749",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}