window.BENCHMARK_DATA = {
  "lastUpdate": 1611166396377,
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
      }
    ]
  }
}