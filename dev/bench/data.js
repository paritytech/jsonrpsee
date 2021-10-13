window.BENCHMARK_DATA = {
  "lastUpdate": 1634084620466,
  "repoUrl": "https://github.com/paritytech/jsonrpsee",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "niklasadolfsson1@gmail.com",
            "name": "Niklas",
            "username": "niklasad1"
          },
          "committer": {
            "email": "niklasadolfsson1@gmail.com",
            "name": "Niklas",
            "username": "niklasad1"
          },
          "distinct": true,
          "id": "f0235c05be9c52161c8abe5f4b863101d3825777",
          "message": "[ci]: fix benchmark workflow",
          "timestamp": "2021-01-20T18:51:04+01:00",
          "tree_id": "093d677a6dca6f893bbab2496720cf00b663e8ca",
          "url": "https://github.com/paritytech/jsonrpsee/commit/f0235c05be9c52161c8abe5f4b863101d3825777"
        },
        "date": 1611165545761,
        "tool": "cargo",
        "benches": [
          {
            "name": "synchronous_http_round_trip",
            "value": 162746,
            "range": "± 2236",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/0",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/1",
            "value": 144945,
            "range": "± 10356",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/2",
            "value": 137309,
            "range": "± 2783",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/4",
            "value": 183388,
            "range": "± 4676",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/8",
            "value": 322238,
            "range": "± 12592",
            "unit": "ns/iter"
          },
          {
            "name": "synchronous_WebSocket_round_trip",
            "value": 647850,
            "range": "± 392066",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/0",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/1",
            "value": 683807,
            "range": "± 434675",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/2",
            "value": 992206,
            "range": "± 1278550",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/4",
            "value": 1161617,
            "range": "± 843059",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/8",
            "value": 1917106,
            "range": "± 850998",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "f0235c05be9c52161c8abe5f4b863101d3825777",
          "message": "[ci]: fix benchmark workflow",
          "timestamp": "2021-01-18T09:37:20Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/190/commits/f0235c05be9c52161c8abe5f4b863101d3825777"
        },
        "date": 1611165555469,
        "tool": "cargo",
        "benches": [
          {
            "name": "synchronous_http_round_trip",
            "value": 163361,
            "range": "± 9944",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/0",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/1",
            "value": 148407,
            "range": "± 2830",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/2",
            "value": 138922,
            "range": "± 3324",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/4",
            "value": 192514,
            "range": "± 6119",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/8",
            "value": 326230,
            "range": "± 5791",
            "unit": "ns/iter"
          },
          {
            "name": "synchronous_WebSocket_round_trip",
            "value": 721677,
            "range": "± 456884",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/0",
            "value": 4,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/1",
            "value": 657803,
            "range": "± 506646",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/2",
            "value": 979109,
            "range": "± 776652",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/4",
            "value": 1244881,
            "range": "± 718537",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/8",
            "value": 2116362,
            "range": "± 823326",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "e032dc3885ad414b985ac1ad625c725a5a27dc1a",
          "message": "[ci]: fix benchmark workflow",
          "timestamp": "2021-01-18T09:37:20Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/190/commits/e032dc3885ad414b985ac1ad625c725a5a27dc1a"
        },
        "date": 1611166395785,
        "tool": "cargo",
        "benches": [
          {
            "name": "synchronous_http_round_trip",
            "value": 167930,
            "range": "± 13011",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/0",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/1",
            "value": 170856,
            "range": "± 4730",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/2",
            "value": 166736,
            "range": "± 3145",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/4",
            "value": 222910,
            "range": "± 6564",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_http_round_trip/8",
            "value": 365403,
            "range": "± 5521",
            "unit": "ns/iter"
          },
          {
            "name": "synchronous_WebSocket_round_trip",
            "value": 928709,
            "range": "± 846221",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/0",
            "value": 5,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/1",
            "value": 913449,
            "range": "± 535363",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/2",
            "value": 1355129,
            "range": "± 4405953",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/4",
            "value": 1738628,
            "range": "± 2269087",
            "unit": "ns/iter"
          },
          {
            "name": "concurrent_WebSocket_round_trip/8",
            "value": 2775213,
            "range": "± 1050408",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Niklas Adolfsson",
            "username": "niklasad1",
            "email": "niklasadolfsson1@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "2891ca11f7da6be8022a9e165eaa9a90017d3d43",
          "message": "chore(release 0.4.1) (#513)",
          "timestamp": "2021-10-12T17:04:56Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/2891ca11f7da6be8022a9e165eaa9a90017d3d43"
        },
        "date": 1634084619291,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 197,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 229,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 185063,
            "range": "± 15069",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 113,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 200767,
            "range": "± 15058",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 186197,
            "range": "± 17116",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 259798,
            "range": "± 10853",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 449624,
            "range": "± 23620",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1931850,
            "range": "± 51733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3710238,
            "range": "± 143235",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7226515,
            "range": "± 236273",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14120558,
            "range": "± 314479",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27667816,
            "range": "± 609309",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56019117,
            "range": "± 2017882",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 111479,
            "range": "± 10775",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 124658,
            "range": "± 6985",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 146340,
            "range": "± 15984",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 171555,
            "range": "± 16768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 202903,
            "range": "± 10763",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 180233,
            "range": "± 12989",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 292386,
            "range": "± 25221",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 517401,
            "range": "± 54112",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1080651,
            "range": "± 168062",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2058063,
            "range": "± 148735",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3826794,
            "range": "± 359541",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 181991,
            "range": "± 13708",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 111,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 177180,
            "range": "± 14988",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 161363,
            "range": "± 25080",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 261245,
            "range": "± 29436",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 449862,
            "range": "± 24577",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1969144,
            "range": "± 107537",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3747837,
            "range": "± 190969",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7272838,
            "range": "± 282686",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14252038,
            "range": "± 455696",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28322099,
            "range": "± 1450969",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56269228,
            "range": "± 1678899",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 114682,
            "range": "± 11241",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 102,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 127535,
            "range": "± 6423",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 156547,
            "range": "± 23082",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 175941,
            "range": "± 19073",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 205550,
            "range": "± 8776",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 183313,
            "range": "± 9212",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 298256,
            "range": "± 19743",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 530863,
            "range": "± 31199",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1104499,
            "range": "± 114518",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2072102,
            "range": "± 162385",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3902460,
            "range": "± 489338",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 140092,
            "range": "± 10863",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 23874,
            "range": "± 2139",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2036,
            "range": "± 942",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}