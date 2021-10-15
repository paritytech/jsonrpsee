window.BENCHMARK_DATA = {
  "lastUpdate": 1634324678804,
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
          "id": "a2e368c53a2870cc6f47b92f9bba210a5be45098",
          "message": "testme",
          "timestamp": "2021-10-15T13:13:38Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/529/commits/a2e368c53a2870cc6f47b92f9bba210a5be45098"
        },
        "date": 1634324677649,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 177,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 201,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 169681,
            "range": "± 19486",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 92,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 165227,
            "range": "± 9034",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 140786,
            "range": "± 18300",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 220522,
            "range": "± 14638",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 378623,
            "range": "± 19278",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1829294,
            "range": "± 158317",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3500373,
            "range": "± 257164",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6593371,
            "range": "± 407873",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13422520,
            "range": "± 587031",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 26367078,
            "range": "± 1074229",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 54041847,
            "range": "± 2012373",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 104255,
            "range": "± 7360",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 99,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 120138,
            "range": "± 5705",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 130240,
            "range": "± 9150",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 151071,
            "range": "± 18447",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 185537,
            "range": "± 10679",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 135977,
            "range": "± 15075",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 211024,
            "range": "± 24692",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 362302,
            "range": "± 29580",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 746549,
            "range": "± 241791",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1508511,
            "range": "± 167302",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2990323,
            "range": "± 322177",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 169463,
            "range": "± 12055",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 89,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 166462,
            "range": "± 9393",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 148322,
            "range": "± 20473",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 227452,
            "range": "± 15717",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 382615,
            "range": "± 27364",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1896407,
            "range": "± 109007",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3543021,
            "range": "± 184980",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6870428,
            "range": "± 310623",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13475874,
            "range": "± 644732",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 26233745,
            "range": "± 1461790",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 54780947,
            "range": "± 2408268",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 109304,
            "range": "± 6589",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 121720,
            "range": "± 10690",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 133992,
            "range": "± 13150",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 157626,
            "range": "± 11260",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 192289,
            "range": "± 37590",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 136520,
            "range": "± 14174",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 217539,
            "range": "± 141770",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 378511,
            "range": "± 62751",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 799975,
            "range": "± 101337",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1589138,
            "range": "± 163383",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3079601,
            "range": "± 352554",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 120607,
            "range": "± 9360",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 23332,
            "range": "± 4245",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1487,
            "range": "± 624",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}