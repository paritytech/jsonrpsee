window.BENCHMARK_DATA = {
  "lastUpdate": 1665842691967,
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
          "id": "c0f952dc234044a6f32ff5e0a166c007505084c0",
          "message": "fix gha gitlab bench again",
          "timestamp": "2022-10-05T12:19:16Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/897/commits/c0f952dc234044a6f32ff5e0a166c007505084c0"
        },
        "date": 1664972773434,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 316,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 451,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 322,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 459,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 172551,
            "range": "± 28676",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 191086,
            "range": "± 5976",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 196179,
            "range": "± 10117",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 225112,
            "range": "± 38614",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 396984,
            "range": "± 26225",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 489746,
            "range": "± 28370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 708689,
            "range": "± 20398",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1482776,
            "range": "± 34466",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 196872,
            "range": "± 10088",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 11142275,
            "range": "± 637997",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1269836,
            "range": "± 28560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 215759,
            "range": "± 11026",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 244870,
            "range": "± 7808",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 281961,
            "range": "± 16760",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 658744,
            "range": "± 69604",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1138738,
            "range": "± 24046",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 319172,
            "range": "± 18344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 297867,
            "range": "± 22089",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 306577,
            "range": "± 20156",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 301989,
            "range": "± 14793",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 702156,
            "range": "± 66378",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 872460,
            "range": "± 43884",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 1017186,
            "range": "± 24105",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 141130,
            "range": "± 4188",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6153098,
            "range": "± 275257",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1218497,
            "range": "± 9882",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 174545,
            "range": "± 13826",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 210716,
            "range": "± 4509",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 256597,
            "range": "± 11248",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 672360,
            "range": "± 11526",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1161625,
            "range": "± 20207",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2836901,
            "range": "± 111144",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5558575,
            "range": "± 212209",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 10656333,
            "range": "± 425930",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1394819,
            "range": "± 39516",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2579154,
            "range": "± 102053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 4957402,
            "range": "± 418186",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2427521,
            "range": "± 39865",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 4258802,
            "range": "± 81041",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 7610435,
            "range": "± 220533",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 18942813,
            "range": "± 927245",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 32385493,
            "range": "± 1349443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 56157063,
            "range": "± 2851931",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 104117401,
            "range": "± 5214099",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10145795,
            "range": "± 366340",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 20272051,
            "range": "± 797429",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 37001181,
            "range": "± 1819782",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 53245632,
            "range": "± 5378726",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 14545102,
            "range": "± 169731",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 27082763,
            "range": "± 341013",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 47003154,
            "range": "± 954290",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78458255,
            "range": "± 853713",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 196073,
            "range": "± 6331",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 191571,
            "range": "± 12495",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 210253,
            "range": "± 13808",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 223026,
            "range": "± 4102",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 398750,
            "range": "± 14182",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 489164,
            "range": "± 17719",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 719521,
            "range": "± 19198",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1473691,
            "range": "± 57282",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 196765,
            "range": "± 1451",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6789688,
            "range": "± 300475",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1271921,
            "range": "± 14254",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 203357,
            "range": "± 981",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 244760,
            "range": "± 9938",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 292375,
            "range": "± 7514",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 669979,
            "range": "± 12204",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1148983,
            "range": "± 20711",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 300671,
            "range": "± 13263",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 295325,
            "range": "± 17631",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 297162,
            "range": "± 16173",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 300544,
            "range": "± 9093",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 678229,
            "range": "± 15447",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 831966,
            "range": "± 44863",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 988396,
            "range": "± 27067",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 142815,
            "range": "± 1912",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6035820,
            "range": "± 798092",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1209481,
            "range": "± 6028",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 165187,
            "range": "± 6579",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 198630,
            "range": "± 1437",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 249366,
            "range": "± 9542",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 659102,
            "range": "± 9542",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1156702,
            "range": "± 33654",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2803354,
            "range": "± 51936",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5608163,
            "range": "± 161476",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 10804706,
            "range": "± 386226",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1270696,
            "range": "± 42728",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2294141,
            "range": "± 77888",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 4246282,
            "range": "± 313881",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2430055,
            "range": "± 34991",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 4250138,
            "range": "± 114929",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 7610460,
            "range": "± 281737",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 19105441,
            "range": "± 538531",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 32204556,
            "range": "± 1233101",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 56108627,
            "range": "± 2688421",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 103036463,
            "range": "± 5167356",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 9580678,
            "range": "± 507748",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 19369758,
            "range": "± 1057854",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 35032296,
            "range": "± 2945320",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 50122496,
            "range": "± 4712926",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 14600623,
            "range": "± 156846",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 27244929,
            "range": "± 247808",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 47923196,
            "range": "± 960675",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78877023,
            "range": "± 834933",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 233814,
            "range": "± 2067",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 10723,
            "range": "± 2444",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5723,
            "range": "± 1889",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexander Samusev",
            "username": "alvicsam",
            "email": "41779041+alvicsam@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "cf3723535ae6777406a4e4cfbb6a32b980cd8b0b",
          "message": "[ci][benchmarks] Change output.txt file path in gitlab ci (#898)",
          "timestamp": "2022-10-07T12:45:22Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/cf3723535ae6777406a4e4cfbb6a32b980cd8b0b"
        },
        "date": 1665152136169,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 312,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 446,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 313,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 452,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 212454,
            "range": "± 1787",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 213323,
            "range": "± 3798",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 217858,
            "range": "± 1018",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 245351,
            "range": "± 1798",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 413592,
            "range": "± 51201",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 520577,
            "range": "± 6692",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 780738,
            "range": "± 15510",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1540162,
            "range": "± 30504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 211408,
            "range": "± 3083",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10898935,
            "range": "± 903200",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1285104,
            "range": "± 3959",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 225322,
            "range": "± 6966",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 254913,
            "range": "± 3074",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 298224,
            "range": "± 2612",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 663251,
            "range": "± 14397",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1118326,
            "range": "± 12830",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 274473,
            "range": "± 42833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 298787,
            "range": "± 37505",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 298311,
            "range": "± 12251",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 299045,
            "range": "± 10323",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 631297,
            "range": "± 6264",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 684899,
            "range": "± 8965",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 857043,
            "range": "± 38569",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 136417,
            "range": "± 774",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6071496,
            "range": "± 429165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1208339,
            "range": "± 4504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 162286,
            "range": "± 730",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 194783,
            "range": "± 2024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 246059,
            "range": "± 3240",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 652246,
            "range": "± 2392",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1156917,
            "range": "± 5192",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2674920,
            "range": "± 60743",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5167817,
            "range": "± 211032",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9872551,
            "range": "± 482047",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1436686,
            "range": "± 66680",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2653353,
            "range": "± 58291",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5136710,
            "range": "± 125673",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2633457,
            "range": "± 72109",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5061455,
            "range": "± 103231",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9751408,
            "range": "± 270550",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17579407,
            "range": "± 652316",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30307964,
            "range": "± 1138724",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 51711377,
            "range": "± 2891867",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 96652116,
            "range": "± 5454811",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10042370,
            "range": "± 187887",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18314714,
            "range": "± 348311",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30628413,
            "range": "± 422902",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48923884,
            "range": "± 774272",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17141419,
            "range": "± 161597",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 27953210,
            "range": "± 545504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 47018215,
            "range": "± 548719",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78677894,
            "range": "± 861246",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 190213,
            "range": "± 2063",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 191525,
            "range": "± 3798",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 195998,
            "range": "± 2774",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 223665,
            "range": "± 2508",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 389162,
            "range": "± 6558",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 488905,
            "range": "± 10489",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 705249,
            "range": "± 12115",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1406716,
            "range": "± 38047",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 189781,
            "range": "± 1413",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6495633,
            "range": "± 516945",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1265463,
            "range": "± 7668",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 204386,
            "range": "± 9281",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 232893,
            "range": "± 1139",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 279622,
            "range": "± 2653",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 644605,
            "range": "± 11644",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1106270,
            "range": "± 9888",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 291685,
            "range": "± 13969",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 296038,
            "range": "± 19750",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 298215,
            "range": "± 10868",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 299182,
            "range": "± 8780",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 631763,
            "range": "± 6316",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 684074,
            "range": "± 12858",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 856508,
            "range": "± 24008",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 137301,
            "range": "± 1825",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6155874,
            "range": "± 108571",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1208877,
            "range": "± 12055",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 163356,
            "range": "± 1833",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 196036,
            "range": "± 2646",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 246677,
            "range": "± 5560",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 653202,
            "range": "± 4274",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1153029,
            "range": "± 57802",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2670679,
            "range": "± 31181",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5189789,
            "range": "± 151779",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9958410,
            "range": "± 349233",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1437285,
            "range": "± 59985",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2662227,
            "range": "± 97384",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5149647,
            "range": "± 110017",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2626415,
            "range": "± 68641",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5047430,
            "range": "± 100923",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9719441,
            "range": "± 239216",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17552076,
            "range": "± 650229",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30558879,
            "range": "± 1336149",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 51458091,
            "range": "± 2568455",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 95729981,
            "range": "± 5345435",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10094897,
            "range": "± 129705",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18309743,
            "range": "± 300955",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30500192,
            "range": "± 467836",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48806499,
            "range": "± 638638",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17122702,
            "range": "± 188994",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 27859937,
            "range": "± 656208",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 46854580,
            "range": "± 590983",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78218617,
            "range": "± 767505",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 239053,
            "range": "± 1964",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 7305,
            "range": "± 652",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 6714,
            "range": "± 501",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexander Samusev",
            "username": "alvicsam",
            "email": "41779041+alvicsam@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "cf3723535ae6777406a4e4cfbb6a32b980cd8b0b",
          "message": "[ci][benchmarks] Change output.txt file path in gitlab ci (#898)",
          "timestamp": "2022-10-07T12:45:22Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/cf3723535ae6777406a4e4cfbb6a32b980cd8b0b"
        },
        "date": 1665237962570,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 310,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 444,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 312,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 455,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 192364,
            "range": "± 2378",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 193176,
            "range": "± 9160",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 198295,
            "range": "± 4040",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 225590,
            "range": "± 2968",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 393787,
            "range": "± 3386",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 488668,
            "range": "± 8822",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 705561,
            "range": "± 12661",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1409477,
            "range": "± 72827",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 191674,
            "range": "± 16446",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10302298,
            "range": "± 723482",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1268571,
            "range": "± 3308",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 205521,
            "range": "± 1179",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 235607,
            "range": "± 1484",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 280951,
            "range": "± 1481",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 643741,
            "range": "± 4503",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1113188,
            "range": "± 36375",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 288026,
            "range": "± 49578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 294380,
            "range": "± 16378",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 294176,
            "range": "± 9105",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 304170,
            "range": "± 8096",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 627873,
            "range": "± 9666",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 684915,
            "range": "± 11092",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 855383,
            "range": "± 21273",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 136091,
            "range": "± 1960",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6215301,
            "range": "± 82877",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1207614,
            "range": "± 5402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 162026,
            "range": "± 2588",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 194665,
            "range": "± 12491",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 245767,
            "range": "± 599",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 653672,
            "range": "± 1741",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1155526,
            "range": "± 6792",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2675248,
            "range": "± 47562",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5141165,
            "range": "± 157903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9931799,
            "range": "± 379463",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1434112,
            "range": "± 146366",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2656646,
            "range": "± 83227",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5132489,
            "range": "± 92305",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2639043,
            "range": "± 55258",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5085202,
            "range": "± 97223",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9798719,
            "range": "± 200473",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17548008,
            "range": "± 839596",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30691851,
            "range": "± 1355939",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 51837720,
            "range": "± 3264578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 95725974,
            "range": "± 5576867",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10044143,
            "range": "± 161441",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18344770,
            "range": "± 334045",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30547611,
            "range": "± 536142",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48665573,
            "range": "± 843654",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17145896,
            "range": "± 153642",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 27937702,
            "range": "± 661041",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 46925814,
            "range": "± 532172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78719900,
            "range": "± 901104",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 191757,
            "range": "± 2399",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 191730,
            "range": "± 3373",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 197888,
            "range": "± 2910",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 224609,
            "range": "± 1414",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 388542,
            "range": "± 27267",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 491026,
            "range": "± 7241",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 703761,
            "range": "± 9932",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1411104,
            "range": "± 17683",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 190635,
            "range": "± 1761",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6567284,
            "range": "± 324227",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1265221,
            "range": "± 6039",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 205796,
            "range": "± 2627",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 234259,
            "range": "± 2100",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 280236,
            "range": "± 7538",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 650506,
            "range": "± 6843",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1115929,
            "range": "± 15452",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 286321,
            "range": "± 23845",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 292655,
            "range": "± 16008",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 293593,
            "range": "± 10927",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 305374,
            "range": "± 5660",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 630075,
            "range": "± 20292",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 684243,
            "range": "± 12153",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 853723,
            "range": "± 47223",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 136085,
            "range": "± 6532",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6228068,
            "range": "± 69781",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1207672,
            "range": "± 4988",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 161845,
            "range": "± 1624",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 194553,
            "range": "± 10035",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 245587,
            "range": "± 1849",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 653250,
            "range": "± 47116",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1156028,
            "range": "± 47924",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2670306,
            "range": "± 98759",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5160325,
            "range": "± 165691",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9955042,
            "range": "± 383665",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1428685,
            "range": "± 65519",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2662339,
            "range": "± 41497",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5146582,
            "range": "± 105391",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2637115,
            "range": "± 51669",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5107032,
            "range": "± 81433",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9838415,
            "range": "± 197341",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17757439,
            "range": "± 569419",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30421891,
            "range": "± 1214561",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 51200472,
            "range": "± 2966714",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 96682668,
            "range": "± 4985641",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10059971,
            "range": "± 122345",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18353324,
            "range": "± 356281",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30519004,
            "range": "± 427853",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48689040,
            "range": "± 782924",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17088033,
            "range": "± 252861",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 28033496,
            "range": "± 701002",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 47017489,
            "range": "± 597328",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 79035841,
            "range": "± 741729",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 238751,
            "range": "± 1748",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 8598,
            "range": "± 1606",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 7578,
            "range": "± 621",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexander Samusev",
            "username": "alvicsam",
            "email": "41779041+alvicsam@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "cf3723535ae6777406a4e4cfbb6a32b980cd8b0b",
          "message": "[ci][benchmarks] Change output.txt file path in gitlab ci (#898)",
          "timestamp": "2022-10-07T12:45:22Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/cf3723535ae6777406a4e4cfbb6a32b980cd8b0b"
        },
        "date": 1665324274969,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 311,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 444,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 313,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 454,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 189863,
            "range": "± 5666",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 191105,
            "range": "± 1702",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 196041,
            "range": "± 4771",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 223407,
            "range": "± 4109",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 391216,
            "range": "± 6423",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 487809,
            "range": "± 8997",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 703730,
            "range": "± 9798",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1412940,
            "range": "± 35174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 188620,
            "range": "± 10857",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10719213,
            "range": "± 657808",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1266220,
            "range": "± 10022",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 203670,
            "range": "± 3974",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 232847,
            "range": "± 8139",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 279314,
            "range": "± 6530",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 644198,
            "range": "± 3671",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1112034,
            "range": "± 8518",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 293981,
            "range": "± 12152",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 298685,
            "range": "± 6549",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 291105,
            "range": "± 16495",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 306060,
            "range": "± 6246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 632416,
            "range": "± 11998",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 684736,
            "range": "± 15669",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 856225,
            "range": "± 24608",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 138191,
            "range": "± 549",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6254674,
            "range": "± 843051",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1209155,
            "range": "± 11804",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 163800,
            "range": "± 16353",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 196186,
            "range": "± 1903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 247461,
            "range": "± 3224",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 655614,
            "range": "± 4783",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1157853,
            "range": "± 110339",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2651034,
            "range": "± 88684",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5124991,
            "range": "± 241277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9774213,
            "range": "± 517420",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1429168,
            "range": "± 79608",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2652554,
            "range": "± 54999",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5135061,
            "range": "± 120675",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2617458,
            "range": "± 87001",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5063695,
            "range": "± 82027",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9779801,
            "range": "± 257947",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17514938,
            "range": "± 750592",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30160774,
            "range": "± 989933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 52054031,
            "range": "± 2710241",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 96147318,
            "range": "± 6433126",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10137448,
            "range": "± 115364",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18671277,
            "range": "± 239589",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30662530,
            "range": "± 421995",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48948815,
            "range": "± 770168",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17158834,
            "range": "± 187365",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 28175069,
            "range": "± 733329",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 46993294,
            "range": "± 564837",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78963573,
            "range": "± 951397",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 189482,
            "range": "± 1939",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 191055,
            "range": "± 1915",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 196333,
            "range": "± 1336",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 221915,
            "range": "± 5992",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 390675,
            "range": "± 14731",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 489297,
            "range": "± 7271",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 702546,
            "range": "± 9100",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1406623,
            "range": "± 33885",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 187752,
            "range": "± 7620",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6602605,
            "range": "± 297676",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1262638,
            "range": "± 17050",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 202704,
            "range": "± 6994",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 232595,
            "range": "± 5189",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 278330,
            "range": "± 1636",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 644215,
            "range": "± 4520",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1117630,
            "range": "± 13869",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 295093,
            "range": "± 14586",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 297938,
            "range": "± 20250",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 295172,
            "range": "± 8850",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 305065,
            "range": "± 10159",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 632423,
            "range": "± 7256",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 687038,
            "range": "± 9171",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 858117,
            "range": "± 61016",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 138356,
            "range": "± 748",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6331869,
            "range": "± 135389",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1209509,
            "range": "± 8168",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 163805,
            "range": "± 1454",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 197353,
            "range": "± 1701",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 247867,
            "range": "± 9062",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 658517,
            "range": "± 5510",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1158045,
            "range": "± 112316",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2661846,
            "range": "± 68593",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5127892,
            "range": "± 107514",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9729486,
            "range": "± 450996",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1439179,
            "range": "± 54166",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2679118,
            "range": "± 64389",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5168526,
            "range": "± 105601",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2642130,
            "range": "± 42757",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5093606,
            "range": "± 147393",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9782231,
            "range": "± 320837",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17421728,
            "range": "± 759613",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30090539,
            "range": "± 1150356",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 51329612,
            "range": "± 2614056",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 95235124,
            "range": "± 6360464",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10125037,
            "range": "± 137308",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18550720,
            "range": "± 419386",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30780465,
            "range": "± 535723",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48849944,
            "range": "± 669543",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17116685,
            "range": "± 244705",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 28153074,
            "range": "± 618012",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 47047158,
            "range": "± 600346",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78782751,
            "range": "± 909245",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 236306,
            "range": "± 5840",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 8397,
            "range": "± 955",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 7188,
            "range": "± 469",
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
          "id": "264e0a8d4f3308db880d60b563eaf3410754eef9",
          "message": "update readme (#899)",
          "timestamp": "2022-10-10T09:55:26Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/264e0a8d4f3308db880d60b563eaf3410754eef9"
        },
        "date": 1665411436142,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 312,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 448,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 314,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 459,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 189909,
            "range": "± 2405",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 190894,
            "range": "± 31216",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 195730,
            "range": "± 2947",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 223674,
            "range": "± 921",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 389060,
            "range": "± 4973",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 485589,
            "range": "± 7339",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 702138,
            "range": "± 13201",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1416590,
            "range": "± 23299",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 188384,
            "range": "± 1622",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10129810,
            "range": "± 793984",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1263421,
            "range": "± 15513",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 203907,
            "range": "± 8674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 232462,
            "range": "± 2194",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 277954,
            "range": "± 31816",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 644066,
            "range": "± 73185",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 727744,
            "range": "± 190838",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 292891,
            "range": "± 21215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 295667,
            "range": "± 18779",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 295961,
            "range": "± 11492",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 297064,
            "range": "± 12136",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 630788,
            "range": "± 15039",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 680881,
            "range": "± 12847",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 854155,
            "range": "± 32617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 136981,
            "range": "± 4500",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6216306,
            "range": "± 100798",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1210220,
            "range": "± 12768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 163303,
            "range": "± 1457",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 195977,
            "range": "± 26064",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 246667,
            "range": "± 28562",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 655865,
            "range": "± 6869",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1155703,
            "range": "± 24289",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2683407,
            "range": "± 44142",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5165969,
            "range": "± 126959",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9844693,
            "range": "± 429496",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1436351,
            "range": "± 86053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2673838,
            "range": "± 77291",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5169615,
            "range": "± 95757",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2633761,
            "range": "± 72080",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5069414,
            "range": "± 88892",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9773903,
            "range": "± 191906",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17800663,
            "range": "± 549471",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30770407,
            "range": "± 1180874",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 52025125,
            "range": "± 2663947",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 96343201,
            "range": "± 5875133",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10098774,
            "range": "± 203256",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18398265,
            "range": "± 292340",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30612995,
            "range": "± 497834",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48938214,
            "range": "± 665972",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17143184,
            "range": "± 165235",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 27932122,
            "range": "± 582911",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 46960626,
            "range": "± 545024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78860997,
            "range": "± 884285",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 189088,
            "range": "± 1231",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 191502,
            "range": "± 5397",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 195807,
            "range": "± 1479",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 223695,
            "range": "± 1133",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 393019,
            "range": "± 7765",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 486538,
            "range": "± 9530",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 706464,
            "range": "± 8663",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1413275,
            "range": "± 29002",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 188514,
            "range": "± 1142",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6537652,
            "range": "± 197174",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1265246,
            "range": "± 9455",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 204106,
            "range": "± 2069",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 231679,
            "range": "± 3276",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 278436,
            "range": "± 3960",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 648634,
            "range": "± 6320",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1119835,
            "range": "± 77279",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 293018,
            "range": "± 17746",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 295230,
            "range": "± 11667",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 297423,
            "range": "± 9875",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 298398,
            "range": "± 10434",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 629646,
            "range": "± 9601",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 684132,
            "range": "± 12761",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 857618,
            "range": "± 62372",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 138220,
            "range": "± 404",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6321684,
            "range": "± 67445",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1209725,
            "range": "± 4548",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 162916,
            "range": "± 465",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 195966,
            "range": "± 5904",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 246240,
            "range": "± 7129",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 656351,
            "range": "± 6073",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1154754,
            "range": "± 9343",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2689587,
            "range": "± 39237",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5199414,
            "range": "± 106411",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9948917,
            "range": "± 302621",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1451525,
            "range": "± 81936",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2670771,
            "range": "± 82253",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5162170,
            "range": "± 124492",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2622497,
            "range": "± 109307",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5073397,
            "range": "± 111357",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9766305,
            "range": "± 224302",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17726745,
            "range": "± 582843",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30639290,
            "range": "± 1353519",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 53069593,
            "range": "± 2595823",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 97530217,
            "range": "± 5840872",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10127271,
            "range": "± 143276",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18411753,
            "range": "± 376089",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30622533,
            "range": "± 504399",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48958434,
            "range": "± 733304",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17098335,
            "range": "± 213674",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 27916599,
            "range": "± 711451",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 46941061,
            "range": "± 495991",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78727563,
            "range": "± 978302",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 236869,
            "range": "± 8912",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 9312,
            "range": "± 1316",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 9147,
            "range": "± 739",
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
          "id": "264e0a8d4f3308db880d60b563eaf3410754eef9",
          "message": "update readme (#899)",
          "timestamp": "2022-10-10T09:55:26Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/264e0a8d4f3308db880d60b563eaf3410754eef9"
        },
        "date": 1665497677732,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 308,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 447,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 309,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 453,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 211494,
            "range": "± 3888",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 212129,
            "range": "± 3593",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 215826,
            "range": "± 5522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 244118,
            "range": "± 2271",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 415590,
            "range": "± 29691",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 513047,
            "range": "± 8650",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 773984,
            "range": "± 25483",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1513957,
            "range": "± 57879",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 210301,
            "range": "± 1362",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 11026424,
            "range": "± 699029",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1283827,
            "range": "± 5120",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 224267,
            "range": "± 6662",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 252196,
            "range": "± 11354",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 279952,
            "range": "± 11500",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 653775,
            "range": "± 18232",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1120473,
            "range": "± 111998",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 293910,
            "range": "± 13696",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 296354,
            "range": "± 19002",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 297622,
            "range": "± 13953",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 298641,
            "range": "± 9792",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 632097,
            "range": "± 13026",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 685018,
            "range": "± 14055",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 850020,
            "range": "± 39630",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 138502,
            "range": "± 1636",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6158564,
            "range": "± 105188",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1209567,
            "range": "± 10152",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 163612,
            "range": "± 22040",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 196503,
            "range": "± 4008",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 246356,
            "range": "± 3129",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 654023,
            "range": "± 17553",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1159147,
            "range": "± 7170",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2657576,
            "range": "± 90246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5128025,
            "range": "± 95834",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9867745,
            "range": "± 471561",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1432263,
            "range": "± 88693",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2680069,
            "range": "± 88836",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5173321,
            "range": "± 305599",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2636149,
            "range": "± 60361",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5096706,
            "range": "± 107949",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9793184,
            "range": "± 234946",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17478287,
            "range": "± 595716",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30112232,
            "range": "± 1124897",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 51322201,
            "range": "± 2816658",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 94694895,
            "range": "± 5080440",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10119735,
            "range": "± 164356",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18518523,
            "range": "± 318808",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30574586,
            "range": "± 432825",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48874724,
            "range": "± 775179",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17050936,
            "range": "± 263262",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 28009196,
            "range": "± 655297",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 47148449,
            "range": "± 599370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78603349,
            "range": "± 977218",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 161625,
            "range": "± 19414",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 191529,
            "range": "± 8429",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 196530,
            "range": "± 1997",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 224068,
            "range": "± 12820",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 390322,
            "range": "± 5090",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 482005,
            "range": "± 6718",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 703164,
            "range": "± 9770",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1407173,
            "range": "± 23945",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 189636,
            "range": "± 2946",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6535443,
            "range": "± 181446",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1263642,
            "range": "± 17434",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 203517,
            "range": "± 9159",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 233519,
            "range": "± 9608",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 278915,
            "range": "± 1212",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 648652,
            "range": "± 4504",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1126707,
            "range": "± 11637",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 291920,
            "range": "± 17220",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 296804,
            "range": "± 14252",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 290892,
            "range": "± 30777",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 300095,
            "range": "± 11503",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 633956,
            "range": "± 11602",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 687594,
            "range": "± 8153",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 858365,
            "range": "± 14905",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 138166,
            "range": "± 1271",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6174584,
            "range": "± 95798",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1210095,
            "range": "± 4432",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 164114,
            "range": "± 8081",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 196305,
            "range": "± 5653",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 251378,
            "range": "± 7956",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 656859,
            "range": "± 4079",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1158251,
            "range": "± 20441",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2657463,
            "range": "± 172891",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5127717,
            "range": "± 198496",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9838680,
            "range": "± 392355",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1441029,
            "range": "± 100716",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2690167,
            "range": "± 106613",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5179399,
            "range": "± 130613",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2644128,
            "range": "± 47310",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5102498,
            "range": "± 75228",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9838981,
            "range": "± 206127",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17631670,
            "range": "± 534271",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30366633,
            "range": "± 1076502",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 51549999,
            "range": "± 3221217",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 95548141,
            "range": "± 5481091",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10175227,
            "range": "± 113386",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18559756,
            "range": "± 335601",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30695022,
            "range": "± 440665",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48973287,
            "range": "± 839392",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17067899,
            "range": "± 190393",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 28100363,
            "range": "± 646363",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 47185198,
            "range": "± 582779",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78896062,
            "range": "± 881436",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 239263,
            "range": "± 4194",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 7803,
            "range": "± 859",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 6881,
            "range": "± 481",
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
          "id": "264e0a8d4f3308db880d60b563eaf3410754eef9",
          "message": "update readme (#899)",
          "timestamp": "2022-10-10T09:55:26Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/264e0a8d4f3308db880d60b563eaf3410754eef9"
        },
        "date": 1665584219077,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 311,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 446,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 311,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 455,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 213054,
            "range": "± 5802",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 213144,
            "range": "± 5621",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 218418,
            "range": "± 1306",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 245815,
            "range": "± 829",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 414147,
            "range": "± 6836",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 517406,
            "range": "± 8488",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 778804,
            "range": "± 12548",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1511183,
            "range": "± 47043",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 212615,
            "range": "± 9308",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10835951,
            "range": "± 772459",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1287834,
            "range": "± 4614",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 226264,
            "range": "± 1982",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 254637,
            "range": "± 2087",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 301783,
            "range": "± 5593",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 673718,
            "range": "± 6225",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1149391,
            "range": "± 115488",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 276301,
            "range": "± 41406",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 293068,
            "range": "± 38016",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 300941,
            "range": "± 33609",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 325803,
            "range": "± 24512",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 810205,
            "range": "± 21628",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 1036156,
            "range": "± 14164",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 1687629,
            "range": "± 91885",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 154746,
            "range": "± 9533",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6269907,
            "range": "± 828496",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1225157,
            "range": "± 1805",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 180230,
            "range": "± 939",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 213159,
            "range": "± 1389",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 261410,
            "range": "± 1633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 672196,
            "range": "± 4129",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1169719,
            "range": "± 24763",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2957875,
            "range": "± 94740",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5702715,
            "range": "± 141971",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 10621736,
            "range": "± 709859",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 2849821,
            "range": "± 64793",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 5440077,
            "range": "± 103073",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 10381407,
            "range": "± 264958",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5050151,
            "range": "± 105042",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8914357,
            "range": "± 390268",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 12042250,
            "range": "± 555063",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 18668246,
            "range": "± 782689",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 32147391,
            "range": "± 1728670",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 51876437,
            "range": "± 2888074",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 96769906,
            "range": "± 6129852",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10211913,
            "range": "± 153289",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18469508,
            "range": "± 440501",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30790684,
            "range": "± 470890",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 49152594,
            "range": "± 775164",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17211120,
            "range": "± 193325",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 28027078,
            "range": "± 651886",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 47318517,
            "range": "± 586014",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 79235151,
            "range": "± 1004359",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 191739,
            "range": "± 2176",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 192408,
            "range": "± 2116",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 197122,
            "range": "± 2880",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 224283,
            "range": "± 3127",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 389405,
            "range": "± 5977",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 487251,
            "range": "± 9489",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 701494,
            "range": "± 12764",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1399957,
            "range": "± 21414",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 191705,
            "range": "± 2100",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6534301,
            "range": "± 218139",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1265987,
            "range": "± 6541",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 206598,
            "range": "± 2390",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 235348,
            "range": "± 2610",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 282312,
            "range": "± 3647",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 654141,
            "range": "± 13207",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1133095,
            "range": "± 13215",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 290409,
            "range": "± 20410",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 295243,
            "range": "± 11520",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 298439,
            "range": "± 7618",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 297818,
            "range": "± 10348",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 637482,
            "range": "± 14868",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 692671,
            "range": "± 9104",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 859532,
            "range": "± 17531",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 139289,
            "range": "± 418",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6651447,
            "range": "± 121691",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1209627,
            "range": "± 1672",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 163703,
            "range": "± 16138",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 197513,
            "range": "± 2194",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 248138,
            "range": "± 2462",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 655757,
            "range": "± 6447",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1154919,
            "range": "± 28877",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2656010,
            "range": "± 27395",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5130086,
            "range": "± 88339",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9775482,
            "range": "± 354692",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1443401,
            "range": "± 112066",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2688122,
            "range": "± 61667",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5188872,
            "range": "± 118181",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2637841,
            "range": "± 59912",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5093339,
            "range": "± 60726",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9812240,
            "range": "± 193902",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17604298,
            "range": "± 867496",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30493374,
            "range": "± 1253686",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 51523383,
            "range": "± 2915111",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 97093564,
            "range": "± 5750136",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10178105,
            "range": "± 122557",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18568661,
            "range": "± 327341",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30706139,
            "range": "± 456803",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 49043453,
            "range": "± 705612",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17253874,
            "range": "± 171688",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 28026288,
            "range": "± 577697",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 47166629,
            "range": "± 590942",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 79069973,
            "range": "± 808867",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 241802,
            "range": "± 5176",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 6972,
            "range": "± 1944",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5885,
            "range": "± 210",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexandru Vasile",
            "username": "lexnv",
            "email": "60601340+lexnv@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "d390823aa4f4c8792d874aa25bb34632903af81b",
          "message": "server: Expose the subscription ID (#900)\n\n* server: Expose the subscription ID\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Check subscription ID is exposed correctly\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* server: Dedicated method for exposing the sub ID\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix clippy\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-10-13T09:50:13Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/d390823aa4f4c8792d874aa25bb34632903af81b"
        },
        "date": 1665670216721,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 311,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 446,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 312,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 455,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 212809,
            "range": "± 845",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 212507,
            "range": "± 1337",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 218947,
            "range": "± 1384",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 244630,
            "range": "± 1913",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 414396,
            "range": "± 7773",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 514197,
            "range": "± 8333",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 785376,
            "range": "± 11417",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1552728,
            "range": "± 36388",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 210601,
            "range": "± 2184",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10515486,
            "range": "± 907244",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1284986,
            "range": "± 58431",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 225599,
            "range": "± 1675",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 254423,
            "range": "± 12912",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 300742,
            "range": "± 7021",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 668370,
            "range": "± 12475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1126173,
            "range": "± 34734",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 286593,
            "range": "± 21383",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 293461,
            "range": "± 12801",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 294805,
            "range": "± 8996",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 300831,
            "range": "± 6860",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 634259,
            "range": "± 10191",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 685633,
            "range": "± 11356",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 861048,
            "range": "± 23395",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 138440,
            "range": "± 1287",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6107273,
            "range": "± 109721",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1210358,
            "range": "± 10440",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 163938,
            "range": "± 9925",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 197024,
            "range": "± 1924",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 248101,
            "range": "± 3402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 653538,
            "range": "± 15769",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1153051,
            "range": "± 4391",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2675606,
            "range": "± 37704",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5169675,
            "range": "± 118720",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9695350,
            "range": "± 458872",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1442919,
            "range": "± 113886",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2691339,
            "range": "± 96888",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5215037,
            "range": "± 119878",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2643472,
            "range": "± 44568",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5091128,
            "range": "± 86429",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9793925,
            "range": "± 221687",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17577972,
            "range": "± 874914",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30514967,
            "range": "± 1207588",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 51841216,
            "range": "± 3172942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 95439086,
            "range": "± 5291604",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10146540,
            "range": "± 178304",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18418514,
            "range": "± 390843",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30689283,
            "range": "± 414204",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 49171431,
            "range": "± 679858",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17159659,
            "range": "± 276674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 28049485,
            "range": "± 653264",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 47120748,
            "range": "± 536006",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78809454,
            "range": "± 889292",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 190418,
            "range": "± 693",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 192322,
            "range": "± 1111",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 196599,
            "range": "± 17240",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 223862,
            "range": "± 10362",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 388894,
            "range": "± 10964",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 486205,
            "range": "± 9114",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 710610,
            "range": "± 16139",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1424631,
            "range": "± 41017",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 190479,
            "range": "± 1197",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6570924,
            "range": "± 229677",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1263305,
            "range": "± 4540",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 204801,
            "range": "± 966",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 234553,
            "range": "± 2901",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 280703,
            "range": "± 1716",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 654006,
            "range": "± 13785",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1122405,
            "range": "± 13739",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 287900,
            "range": "± 19526",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 294741,
            "range": "± 20826",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 293748,
            "range": "± 15062",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 300417,
            "range": "± 12327",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 635034,
            "range": "± 5522",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 686833,
            "range": "± 13351",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 857624,
            "range": "± 46920",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 138224,
            "range": "± 11658",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6042050,
            "range": "± 904565",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1210644,
            "range": "± 11591",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 163127,
            "range": "± 1082",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 196617,
            "range": "± 16856",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 246695,
            "range": "± 7100",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 653028,
            "range": "± 37304",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1152092,
            "range": "± 14157",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2680679,
            "range": "± 56514",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5182776,
            "range": "± 232946",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9900457,
            "range": "± 405167",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1447345,
            "range": "± 55271",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2688048,
            "range": "± 120043",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5189791,
            "range": "± 120310",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2650571,
            "range": "± 59191",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5092289,
            "range": "± 87556",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9781703,
            "range": "± 300935",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17610346,
            "range": "± 542688",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30589086,
            "range": "± 1300402",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 52498057,
            "range": "± 2628723",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 96186673,
            "range": "± 5305222",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10161436,
            "range": "± 186101",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18487879,
            "range": "± 397873",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30661646,
            "range": "± 455451",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48889853,
            "range": "± 693275",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17189587,
            "range": "± 126669",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 27879996,
            "range": "± 623306",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 47036357,
            "range": "± 564911",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 79343118,
            "range": "± 988238",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 237903,
            "range": "± 2443",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 12493,
            "range": "± 1055",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 11260,
            "range": "± 775",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexandru Vasile",
            "username": "lexnv",
            "email": "60601340+lexnv@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "d390823aa4f4c8792d874aa25bb34632903af81b",
          "message": "server: Expose the subscription ID (#900)\n\n* server: Expose the subscription ID\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Check subscription ID is exposed correctly\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* server: Dedicated method for exposing the sub ID\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix clippy\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-10-13T09:50:13Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/d390823aa4f4c8792d874aa25bb34632903af81b"
        },
        "date": 1665756994656,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 310,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 447,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 313,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 454,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 168494,
            "range": "± 1862",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 169180,
            "range": "± 9196",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 173484,
            "range": "± 1503",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 200479,
            "range": "± 11267",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 357460,
            "range": "± 9856",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 448057,
            "range": "± 11749",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 647270,
            "range": "± 19855",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1313766,
            "range": "± 49675",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 168416,
            "range": "± 5727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 11217561,
            "range": "± 628609",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1242716,
            "range": "± 6565",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 183787,
            "range": "± 4072",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 212384,
            "range": "± 6337",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 259066,
            "range": "± 4076",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 631433,
            "range": "± 72397",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1110913,
            "range": "± 13165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 271954,
            "range": "± 32278",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 269860,
            "range": "± 28035",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 292654,
            "range": "± 42563",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 279364,
            "range": "± 15556",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 630050,
            "range": "± 41424",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 591826,
            "range": "± 41035",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 763169,
            "range": "± 65741",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 124336,
            "range": "± 586",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6193807,
            "range": "± 76137",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1196659,
            "range": "± 14104",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 149782,
            "range": "± 926",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 182405,
            "range": "± 4952",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 233836,
            "range": "± 2619",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 641788,
            "range": "± 16275",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1140227,
            "range": "± 49610",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2661736,
            "range": "± 30632",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5148522,
            "range": "± 237067",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9822951,
            "range": "± 481771",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1447823,
            "range": "± 83497",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2672256,
            "range": "± 92009",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5167205,
            "range": "± 105101",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2634860,
            "range": "± 56929",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5083903,
            "range": "± 109744",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9773434,
            "range": "± 175733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17333057,
            "range": "± 935746",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 29345874,
            "range": "± 1231844",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 51201527,
            "range": "± 3148005",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 95090333,
            "range": "± 5700833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10048090,
            "range": "± 122123",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18334707,
            "range": "± 332606",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30463108,
            "range": "± 539463",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48806972,
            "range": "± 732477",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17056670,
            "range": "± 171992",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 27835525,
            "range": "± 480825",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 46950161,
            "range": "± 629811",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78467296,
            "range": "± 774940",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 189791,
            "range": "± 2318",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 190746,
            "range": "± 3059",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 196181,
            "range": "± 1230",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 222459,
            "range": "± 1911",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 381540,
            "range": "± 7448",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 480817,
            "range": "± 12448",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 701661,
            "range": "± 14061",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1401057,
            "range": "± 32206",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 190215,
            "range": "± 19140",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 7575951,
            "range": "± 661458",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1263450,
            "range": "± 8226",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 204390,
            "range": "± 1486",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 234345,
            "range": "± 1573",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 280054,
            "range": "± 15930",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 649166,
            "range": "± 27807",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1117989,
            "range": "± 138391",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 295675,
            "range": "± 18832",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 296993,
            "range": "± 15864",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 301068,
            "range": "± 12197",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 301115,
            "range": "± 7918",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 633870,
            "range": "± 9509",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 687565,
            "range": "± 12632",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 854964,
            "range": "± 19666",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 140425,
            "range": "± 1698",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6222236,
            "range": "± 339249",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1212700,
            "range": "± 13302",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 166526,
            "range": "± 3558",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 198525,
            "range": "± 680",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 250252,
            "range": "± 1416",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 658748,
            "range": "± 17170",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1157625,
            "range": "± 14602",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2657003,
            "range": "± 54351",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5147011,
            "range": "± 147855",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9915309,
            "range": "± 374427",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1442213,
            "range": "± 88364",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2672436,
            "range": "± 116480",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5143380,
            "range": "± 175494",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2630373,
            "range": "± 75569",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5058497,
            "range": "± 108347",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9752134,
            "range": "± 342605",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17564453,
            "range": "± 779776",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30148371,
            "range": "± 1089606",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 51383375,
            "range": "± 2993947",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 96029375,
            "range": "± 6154749",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10073366,
            "range": "± 164493",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18407917,
            "range": "± 318517",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30488601,
            "range": "± 435083",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48706579,
            "range": "± 690999",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17105784,
            "range": "± 181072",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 27863422,
            "range": "± 643128",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 46924627,
            "range": "± 496064",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78642343,
            "range": "± 904476",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 240842,
            "range": "± 3147",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 14681,
            "range": "± 512",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 13188,
            "range": "± 633",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexandru Vasile",
            "username": "lexnv",
            "email": "60601340+lexnv@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "d390823aa4f4c8792d874aa25bb34632903af81b",
          "message": "server: Expose the subscription ID (#900)\n\n* server: Expose the subscription ID\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Check subscription ID is exposed correctly\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* server: Dedicated method for exposing the sub ID\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix clippy\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-10-13T09:50:13Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/d390823aa4f4c8792d874aa25bb34632903af81b"
        },
        "date": 1665842691623,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 310,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 449,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 313,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 454,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 187485,
            "range": "± 4475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 188161,
            "range": "± 1052",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 194421,
            "range": "± 2081",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 221295,
            "range": "± 4427",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 386257,
            "range": "± 9082",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 481392,
            "range": "± 9675",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 700723,
            "range": "± 11611",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1395338,
            "range": "± 21576",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 188781,
            "range": "± 922",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10219971,
            "range": "± 829410",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1260531,
            "range": "± 15965",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 201992,
            "range": "± 3442",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 229373,
            "range": "± 18145",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 277198,
            "range": "± 1478",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 648994,
            "range": "± 5890",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1124426,
            "range": "± 37774",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 286762,
            "range": "± 20875",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 295668,
            "range": "± 12536",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 291806,
            "range": "± 14405",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 296270,
            "range": "± 7211",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 631157,
            "range": "± 13278",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 678851,
            "range": "± 151790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 850385,
            "range": "± 30867",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 136425,
            "range": "± 1018",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6209237,
            "range": "± 163032",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1207611,
            "range": "± 8326",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 161608,
            "range": "± 3874",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 194253,
            "range": "± 446",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 247297,
            "range": "± 6278",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 653581,
            "range": "± 4438",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1148753,
            "range": "± 19781",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2652308,
            "range": "± 44712",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5106067,
            "range": "± 114384",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9835154,
            "range": "± 383738",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1425090,
            "range": "± 32620",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2642811,
            "range": "± 86051",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5107109,
            "range": "± 170324",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2619869,
            "range": "± 52992",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5035841,
            "range": "± 83855",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9716364,
            "range": "± 185374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17523910,
            "range": "± 576318",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 29621514,
            "range": "± 1196369",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 50325622,
            "range": "± 2594795",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 94714344,
            "range": "± 5801025",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 9997042,
            "range": "± 126732",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18292367,
            "range": "± 254968",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30419761,
            "range": "± 465507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48907193,
            "range": "± 729585",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17069065,
            "range": "± 264670",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 27813422,
            "range": "± 654919",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 47039296,
            "range": "± 469477",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 79011296,
            "range": "± 799250",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 188873,
            "range": "± 1436",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 189480,
            "range": "± 1416",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 194933,
            "range": "± 2476",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 221837,
            "range": "± 2351",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 386452,
            "range": "± 4654",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 479776,
            "range": "± 9084",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 700616,
            "range": "± 15097",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1393197,
            "range": "± 18720",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 187415,
            "range": "± 1176",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6477265,
            "range": "± 86657",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1259799,
            "range": "± 5677",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 201874,
            "range": "± 1233",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 231988,
            "range": "± 1258",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 277759,
            "range": "± 2510",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 650267,
            "range": "± 5062",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1109922,
            "range": "± 18680",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 289058,
            "range": "± 17265",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 294040,
            "range": "± 18227",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 293160,
            "range": "± 12710",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 296572,
            "range": "± 11841",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 629114,
            "range": "± 34235",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 680145,
            "range": "± 7858",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 844967,
            "range": "± 46289",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 136474,
            "range": "± 1526",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6461610,
            "range": "± 272403",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1208108,
            "range": "± 9775",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 162000,
            "range": "± 3419",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 195400,
            "range": "± 2666",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 246003,
            "range": "± 8756",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 653765,
            "range": "± 6733",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1157440,
            "range": "± 9721",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2631156,
            "range": "± 97045",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5123833,
            "range": "± 117112",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9690830,
            "range": "± 361479",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1416234,
            "range": "± 87354",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2626816,
            "range": "± 57336",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5074228,
            "range": "± 95860",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2611062,
            "range": "± 59284",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5009011,
            "range": "± 112932",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9632623,
            "range": "± 315201",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17215424,
            "range": "± 753906",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 30014371,
            "range": "± 1037793",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 50452834,
            "range": "± 2475112",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 93484709,
            "range": "± 5303751",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 9926630,
            "range": "± 257607",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 17951578,
            "range": "± 359320",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30326271,
            "range": "± 512443",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48502715,
            "range": "± 616772",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17113424,
            "range": "± 148007",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 27731243,
            "range": "± 607626",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 46625103,
            "range": "± 484582",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78064925,
            "range": "± 815700",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 239247,
            "range": "± 3267",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 7514,
            "range": "± 1155",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 6913,
            "range": "± 456",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}