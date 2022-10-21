window.BENCHMARK_DATA = {
  "lastUpdate": 1666364625501,
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
          "id": "0c7e0f8c6eb1a51fba19bf8ffb34602f8dfb571f",
          "message": "[DO NOT MERGE]: v0.14 bench results ",
          "timestamp": "2022-10-21T13:16:35Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/912/commits/0c7e0f8c6eb1a51fba19bf8ffb34602f8dfb571f"
        },
        "date": 1666360988144,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 151,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 66847,
            "range": "± 566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 65556,
            "range": "± 430",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 67426,
            "range": "± 560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 79968,
            "range": "± 822",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 132200,
            "range": "± 2155",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 167859,
            "range": "± 2016",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 231132,
            "range": "± 149689",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 324835,
            "range": "± 548672",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 66174,
            "range": "± 745",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3747790,
            "range": "± 122100",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1158973,
            "range": "± 7978",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 73810,
            "range": "± 475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 85771,
            "range": "± 515",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 96465,
            "range": "± 656",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 181951,
            "range": "± 1933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 284618,
            "range": "± 3611",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 81623,
            "range": "± 1104",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 83484,
            "range": "± 936",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 95180,
            "range": "± 6199",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 87346,
            "range": "± 5916",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 213704,
            "range": "± 14955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 275337,
            "range": "± 8428",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 377019,
            "range": "± 8663",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 51036,
            "range": "± 570",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2208626,
            "range": "± 61862",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1161534,
            "range": "± 6226",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 58484,
            "range": "± 402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 66768,
            "range": "± 530",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 80051,
            "range": "± 950",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 181272,
            "range": "± 2403",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 326550,
            "range": "± 1706",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 529076,
            "range": "± 813676",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 950621,
            "range": "± 1504039",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 1556999,
            "range": "± 3356091",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 568166,
            "range": "± 12506",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 1043455,
            "range": "± 23282",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 1949119,
            "range": "± 45968",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 1128273,
            "range": "± 21629",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 2104769,
            "range": "± 39376",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 3839367,
            "range": "± 74413",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 7045379,
            "range": "± 4135522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 14447770,
            "range": "± 7273419",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 29162654,
            "range": "± 14562049",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 76175941,
            "range": "± 24157876",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 4853734,
            "range": "± 118464",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 8709783,
            "range": "± 229012",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 14762945,
            "range": "± 411538",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 27111740,
            "range": "± 627818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 7434481,
            "range": "± 112966",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 14165050,
            "range": "± 271933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 25436195,
            "range": "± 366406",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 46546854,
            "range": "± 843016",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 69233,
            "range": "± 3414",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 68733,
            "range": "± 2149",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 68968,
            "range": "± 6618",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 94762,
            "range": "± 2557",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 185998,
            "range": "± 15222",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 202640,
            "range": "± 4519",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 230152,
            "range": "± 468249",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 312750,
            "range": "± 571313",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 81437,
            "range": "± 2075",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2758687,
            "range": "± 122380",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1185266,
            "range": "± 8173",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 74417,
            "range": "± 2008",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 101581,
            "range": "± 1862",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 113093,
            "range": "± 2428",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 206091,
            "range": "± 3848",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 329648,
            "range": "± 8757",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 99462,
            "range": "± 5156",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 102215,
            "range": "± 5566",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 107260,
            "range": "± 3608",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 114901,
            "range": "± 1343",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 235446,
            "range": "± 6498",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 281238,
            "range": "± 7440",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 376113,
            "range": "± 9321",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 55868,
            "range": "± 6049",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2229794,
            "range": "± 23449",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1160394,
            "range": "± 2740",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 71178,
            "range": "± 5586",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 75649,
            "range": "± 7405",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 80427,
            "range": "± 5614",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 180837,
            "range": "± 11553",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 325498,
            "range": "± 1279",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 527589,
            "range": "± 786954",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 941111,
            "range": "± 1842354",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 1549661,
            "range": "± 3345721",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 634103,
            "range": "± 24833",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 1334677,
            "range": "± 42620",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 2523559,
            "range": "± 104471",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 1113940,
            "range": "± 27919",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 2097914,
            "range": "± 33103",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 3893447,
            "range": "± 76652",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 7099516,
            "range": "± 4158766",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 13790313,
            "range": "± 9342512",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 34576746,
            "range": "± 12431630",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 67469806,
            "range": "± 24688914",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 3686232,
            "range": "± 87887",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 6752660,
            "range": "± 71862",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 13146157,
            "range": "± 184821",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 26306917,
            "range": "± 613015",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 7469574,
            "range": "± 150994",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 14372386,
            "range": "± 315802",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 25173226,
            "range": "± 432067",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 45740779,
            "range": "± 626021",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 76233,
            "range": "± 2406",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4971,
            "range": "± 198",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4930,
            "range": "± 107",
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
          "id": "0791aca66f57393b539895228870fc3cb3537176",
          "message": "v0.15",
          "timestamp": "2022-10-21T13:16:35Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/913/commits/0791aca66f57393b539895228870fc3cb3537176"
        },
        "date": 1666361369530,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 140,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 167,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 66030,
            "range": "± 814",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 65503,
            "range": "± 571",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 66986,
            "range": "± 523",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 78914,
            "range": "± 733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 129691,
            "range": "± 2150",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 166808,
            "range": "± 2014",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 192086,
            "range": "± 224375",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 240704,
            "range": "± 802493",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 65997,
            "range": "± 840",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3680550,
            "range": "± 141566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1158337,
            "range": "± 6408",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 73172,
            "range": "± 480",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 84030,
            "range": "± 655",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 95132,
            "range": "± 694",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 174630,
            "range": "± 1302",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 274267,
            "range": "± 2877",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 82090,
            "range": "± 1058",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 84345,
            "range": "± 1571",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 85721,
            "range": "± 989",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 88845,
            "range": "± 658",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 188081,
            "range": "± 2780",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 222934,
            "range": "± 5476",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 292199,
            "range": "± 7502",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 52224,
            "range": "± 371",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2086301,
            "range": "± 19297",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1142861,
            "range": "± 5453",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 58743,
            "range": "± 396",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 67084,
            "range": "± 427",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 80628,
            "range": "± 1283",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 176183,
            "range": "± 1161",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 316863,
            "range": "± 3002",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 370615,
            "range": "± 746790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 649038,
            "range": "± 1433999",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 1154223,
            "range": "± 3292387",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 443708,
            "range": "± 12869",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 754532,
            "range": "± 19906",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 1347174,
            "range": "± 22512",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 770790,
            "range": "± 16589",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 1379088,
            "range": "± 19298",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 2506102,
            "range": "± 32024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 6250214,
            "range": "± 3481884",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 12170115,
            "range": "± 7508218",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 25200686,
            "range": "± 11469937",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 56460260,
            "range": "± 20078752",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 2521397,
            "range": "± 35883",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 4804752,
            "range": "± 45903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 9125159,
            "range": "± 97576",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 17768748,
            "range": "± 233727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 4715137,
            "range": "± 52629",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 9243320,
            "range": "± 82641",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 17310450,
            "range": "± 162333",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 33779862,
            "range": "± 314295",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 66091,
            "range": "± 1052",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 65570,
            "range": "± 1091",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 67012,
            "range": "± 1038",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 80635,
            "range": "± 1690",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 125697,
            "range": "± 3002",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 161942,
            "range": "± 1869",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 185814,
            "range": "± 314273",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 230257,
            "range": "± 591271",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 65848,
            "range": "± 932",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2500742,
            "range": "± 155034",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1165748,
            "range": "± 8583",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 69549,
            "range": "± 991",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 81067,
            "range": "± 1176",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 92903,
            "range": "± 1658",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 170694,
            "range": "± 2309",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 270680,
            "range": "± 3609",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 82805,
            "range": "± 934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 84593,
            "range": "± 1021",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 85106,
            "range": "± 1176",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 88853,
            "range": "± 1203",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 186074,
            "range": "± 2842",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 218895,
            "range": "± 3565",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 286794,
            "range": "± 7002",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 48233,
            "range": "± 653",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2127457,
            "range": "± 40545",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1141262,
            "range": "± 8949",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 58962,
            "range": "± 1202",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 67493,
            "range": "± 388",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 80029,
            "range": "± 427",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 177491,
            "range": "± 1409",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 318308,
            "range": "± 3225",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 370242,
            "range": "± 795843",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 638265,
            "range": "± 1505790",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 1046912,
            "range": "± 3028578",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 439448,
            "range": "± 10913",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 748289,
            "range": "± 15070",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 1325769,
            "range": "± 19468",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 770377,
            "range": "± 18974",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 1363129,
            "range": "± 18905",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 2491942,
            "range": "± 29147",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 6253203,
            "range": "± 3546653",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 12240953,
            "range": "± 7171119",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 27319320,
            "range": "± 11044605",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 54783792,
            "range": "± 19445869",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 2545499,
            "range": "± 33392",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 4833291,
            "range": "± 49689",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 9130574,
            "range": "± 97320",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 17843028,
            "range": "± 265934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 4703001,
            "range": "± 53631",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 9165794,
            "range": "± 97596",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 17191770,
            "range": "± 150526",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 33474859,
            "range": "± 275543",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 82003,
            "range": "± 537",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4823,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4820,
            "range": "± 104",
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
          "id": "8431ff7f2e212db72fccc2ae6bed05c673c49d9f",
          "message": "master/v0.16",
          "timestamp": "2022-10-16T19:21:15Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/8431ff7f2e212db72fccc2ae6bed05c673c49d9f"
        },
        "date": 1665984540394,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 288,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 422,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 291,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 433,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 187854,
            "range": "± 1127",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 188410,
            "range": "± 785",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 193375,
            "range": "± 2270",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 221168,
            "range": "± 1310",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 389745,
            "range": "± 4076",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 482522,
            "range": "± 10557",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 691536,
            "range": "± 11205",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1399772,
            "range": "± 23024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 186452,
            "range": "± 3352",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10412396,
            "range": "± 434625",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1259124,
            "range": "± 5909",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 201401,
            "range": "± 4860",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 229565,
            "range": "± 2800",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 276622,
            "range": "± 5928",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 639645,
            "range": "± 23574",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1085159,
            "range": "± 73674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 286642,
            "range": "± 23332",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 294349,
            "range": "± 15785",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 296323,
            "range": "± 17209",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 298452,
            "range": "± 11376",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 628832,
            "range": "± 7727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 681831,
            "range": "± 10939",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 848877,
            "range": "± 36267",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 136964,
            "range": "± 578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6716034,
            "range": "± 119215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1208070,
            "range": "± 8674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 162947,
            "range": "± 3547",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 196391,
            "range": "± 8320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 247621,
            "range": "± 3089",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 656273,
            "range": "± 2933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1158216,
            "range": "± 12022",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2647693,
            "range": "± 29653",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5123130,
            "range": "± 112098",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 9764481,
            "range": "± 395568",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 1433065,
            "range": "± 117083",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 2656032,
            "range": "± 60795",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 5117467,
            "range": "± 83142",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 2605964,
            "range": "± 73252",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 5019325,
            "range": "± 67495",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 9705800,
            "range": "± 137935",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 17521812,
            "range": "± 479974",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 29981110,
            "range": "± 1152303",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 49971909,
            "range": "± 2755094",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 93346202,
            "range": "± 5255011",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 10058147,
            "range": "± 109147",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 18318627,
            "range": "± 262432",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 30400225,
            "range": "± 496115",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48792778,
            "range": "± 595716",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 17064778,
            "range": "± 176867",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 27764748,
            "range": "± 601113",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 46617939,
            "range": "± 538937",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 78013242,
            "range": "± 827082",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 187655,
            "range": "± 1642",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 189351,
            "range": "± 1634",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 194038,
            "range": "± 10944",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 220233,
            "range": "± 889",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 391934,
            "range": "± 5315",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 483262,
            "range": "± 8648",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 691261,
            "range": "± 7740",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1400403,
            "range": "± 26417",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 186304,
            "range": "± 1137",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6636005,
            "range": "± 103965",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1260677,
            "range": "± 12065",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 200963,
            "range": "± 1443",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 230146,
            "range": "± 2340",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 277856,
            "range": "± 3029",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 644244,
            "range": "± 5383",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1102463,
            "range": "± 16457",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 287636,
            "range": "± 26506",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 294236,
            "range": "± 19422",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 292512,
            "range": "± 17210",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 295165,
            "range": "± 9214",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 630932,
            "range": "± 7767",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 685861,
            "range": "± 11983",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 855236,
            "range": "± 15658",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 136764,
            "range": "± 2124",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6699944,
            "range": "± 105075",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1207948,
            "range": "± 3766",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 162597,
            "range": "± 5419",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 195688,
            "range": "± 2600",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 245684,
            "range": "± 962",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 653656,
            "range": "± 3523",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1155274,
            "range": "± 22009",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2649495,
            "range": "± 24308",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5144930,
            "range": "± 90375",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 9841304,
            "range": "± 323352",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 1428607,
            "range": "± 73954",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 2654395,
            "range": "± 84724",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 5124597,
            "range": "± 140409",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 2604846,
            "range": "± 57580",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 5022131,
            "range": "± 79680",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 9658345,
            "range": "± 173906",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 17491029,
            "range": "± 523714",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 29932337,
            "range": "± 1126907",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 51033303,
            "range": "± 2634229",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 94758336,
            "range": "± 4650470",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 10031212,
            "range": "± 147362",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 18253186,
            "range": "± 300557",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 30454590,
            "range": "± 502904",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 48709897,
            "range": "± 627934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 17028119,
            "range": "± 138627",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 27704818,
            "range": "± 554648",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 46476528,
            "range": "± 511410",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 78023331,
            "range": "± 881837",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 235190,
            "range": "± 1102",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 8654,
            "range": "± 750",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 9511,
            "range": "± 653",
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
          "id": "0c7e0f8c6eb1a51fba19bf8ffb34602f8dfb571f",
          "message": "v0.14 bench ci5-1",
          "timestamp": "2022-10-21T13:16:35Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/912/commits/0c7e0f8c6eb1a51fba19bf8ffb34602f8dfb571f"
        },
        "date": 1666364624784,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 151,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 66847,
            "range": "± 566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 65556,
            "range": "± 430",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 67426,
            "range": "± 560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 79968,
            "range": "± 822",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 132200,
            "range": "± 2155",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 167859,
            "range": "± 2016",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 231132,
            "range": "± 149689",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 324835,
            "range": "± 548672",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 66174,
            "range": "± 745",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3747790,
            "range": "± 122100",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1158973,
            "range": "± 7978",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 73810,
            "range": "± 475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 85771,
            "range": "± 515",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 96465,
            "range": "± 656",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 181951,
            "range": "± 1933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 284618,
            "range": "± 3611",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 81623,
            "range": "± 1104",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 83484,
            "range": "± 936",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 95180,
            "range": "± 6199",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 87346,
            "range": "± 5916",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 213704,
            "range": "± 14955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 275337,
            "range": "± 8428",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 377019,
            "range": "± 8663",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 51036,
            "range": "± 570",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2208626,
            "range": "± 61862",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1161534,
            "range": "± 6226",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 58484,
            "range": "± 402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 66768,
            "range": "± 530",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 80051,
            "range": "± 950",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 181272,
            "range": "± 2403",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 326550,
            "range": "± 1706",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 529076,
            "range": "± 813676",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 950621,
            "range": "± 1504039",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 1556999,
            "range": "± 3356091",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 568166,
            "range": "± 12506",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 1043455,
            "range": "± 23282",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 1949119,
            "range": "± 45968",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 1128273,
            "range": "± 21629",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 2104769,
            "range": "± 39376",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 3839367,
            "range": "± 74413",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 7045379,
            "range": "± 4135522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 14447770,
            "range": "± 7273419",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 29162654,
            "range": "± 14562049",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 76175941,
            "range": "± 24157876",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 4853734,
            "range": "± 118464",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 8709783,
            "range": "± 229012",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 14762945,
            "range": "± 411538",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 27111740,
            "range": "± 627818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 7434481,
            "range": "± 112966",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 14165050,
            "range": "± 271933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 25436195,
            "range": "± 366406",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 46546854,
            "range": "± 843016",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 69233,
            "range": "± 3414",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 68733,
            "range": "± 2149",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 68968,
            "range": "± 6618",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 94762,
            "range": "± 2557",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 185998,
            "range": "± 15222",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 202640,
            "range": "± 4519",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 230152,
            "range": "± 468249",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 312750,
            "range": "± 571313",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 81437,
            "range": "± 2075",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2758687,
            "range": "± 122380",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1185266,
            "range": "± 8173",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 74417,
            "range": "± 2008",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 101581,
            "range": "± 1862",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 113093,
            "range": "± 2428",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 206091,
            "range": "± 3848",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 329648,
            "range": "± 8757",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 99462,
            "range": "± 5156",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 102215,
            "range": "± 5566",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 107260,
            "range": "± 3608",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 114901,
            "range": "± 1343",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 235446,
            "range": "± 6498",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 281238,
            "range": "± 7440",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 376113,
            "range": "± 9321",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 55868,
            "range": "± 6049",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2229794,
            "range": "± 23449",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1160394,
            "range": "± 2740",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 71178,
            "range": "± 5586",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 75649,
            "range": "± 7405",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 80427,
            "range": "± 5614",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 180837,
            "range": "± 11553",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 325498,
            "range": "± 1279",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 527589,
            "range": "± 786954",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 941111,
            "range": "± 1842354",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 1549661,
            "range": "± 3345721",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 634103,
            "range": "± 24833",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 1334677,
            "range": "± 42620",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 2523559,
            "range": "± 104471",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 1113940,
            "range": "± 27919",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 2097914,
            "range": "± 33103",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 3893447,
            "range": "± 76652",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 7099516,
            "range": "± 4158766",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 13790313,
            "range": "± 9342512",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 34576746,
            "range": "± 12431630",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 67469806,
            "range": "± 24688914",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 3686232,
            "range": "± 87887",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 6752660,
            "range": "± 71862",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 13146157,
            "range": "± 184821",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 26306917,
            "range": "± 613015",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 7469574,
            "range": "± 150994",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 14372386,
            "range": "± 315802",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 25173226,
            "range": "± 432067",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 45740779,
            "range": "± 626021",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 76233,
            "range": "± 2406",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4971,
            "range": "± 198",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4930,
            "range": "± 107",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}