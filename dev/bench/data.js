window.BENCHMARK_DATA = {
  "lastUpdate": 1634516605864,
  "repoUrl": "https://github.com/paritytech/jsonrpsee",
  "entries": {
    "Benchmark": [
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
          "id": "0b4355560b9a21985375660934da20ace19d2c36",
          "message": "benches: add option to run benchmarks against jsonrpc crate servers (#527)\n\n* fix http client bench with request limit\r\n\r\n* benches for jsonrpc servers\r\n\r\n* workaround; dont use max request limit\r\n\r\n* add subscriptions\r\n\r\n* revert unintentional change\r\n\r\n* ignore batch request bench for ws\r\n\r\n* fmt\r\n\r\n* log -> tracing\r\n\r\n* test bench CI\r\n\r\n* test bench v0.3\r\n\r\n* wtf; run CI\r\n\r\n* work plz\r\n\r\n* remove test CI bench\r\n\r\n* fix compile warn on macos",
          "timestamp": "2021-10-17T14:41:49Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/0b4355560b9a21985375660934da20ace19d2c36"
        },
        "date": 1634516605336,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 169,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 193,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 161511,
            "range": "± 26974",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 159389,
            "range": "± 23316",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 135846,
            "range": "± 12633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 220422,
            "range": "± 9754",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 378509,
            "range": "± 20793",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1790251,
            "range": "± 68444",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3383314,
            "range": "± 129906",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6591846,
            "range": "± 359324",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12995126,
            "range": "± 608494",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 26516980,
            "range": "± 1187932",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 51103898,
            "range": "± 2062491",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 100306,
            "range": "± 9983",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 105,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 110120,
            "range": "± 9435",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 131534,
            "range": "± 11266",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 151594,
            "range": "± 11406",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 177249,
            "range": "± 21377",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 157910,
            "range": "± 10589",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 258350,
            "range": "± 39453",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 458735,
            "range": "± 80137",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 927977,
            "range": "± 95405",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1777058,
            "range": "± 145454",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3328414,
            "range": "± 370756",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 154876,
            "range": "± 10457",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 102,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 156405,
            "range": "± 35516",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 165210,
            "range": "± 37468",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 268476,
            "range": "± 35359",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 432566,
            "range": "± 45279",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1967756,
            "range": "± 156332",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3677814,
            "range": "± 187467",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7555786,
            "range": "± 672530",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14282953,
            "range": "± 852998",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28055130,
            "range": "± 1844115",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56102010,
            "range": "± 3085865",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 107527,
            "range": "± 11156",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 113,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 117911,
            "range": "± 16828",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 139447,
            "range": "± 29094",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 162732,
            "range": "± 16467",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 186426,
            "range": "± 17422",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 173733,
            "range": "± 55857",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 278066,
            "range": "± 39109",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 476372,
            "range": "± 40736",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 961170,
            "range": "± 102510",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1882152,
            "range": "± 157227",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3499903,
            "range": "± 528001",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 121351,
            "range": "± 8043",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 17832,
            "range": "± 2397",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1490,
            "range": "± 235",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}