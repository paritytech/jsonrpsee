window.BENCHMARK_DATA = {
  "lastUpdate": 1611165546426,
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
      }
    ]
  }
}