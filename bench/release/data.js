window.BENCHMARK_DATA = {
  "lastUpdate": 1666365167516,
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
          "id": "4b0f410684b6b44ee73e46c66d5280aaf87f6cf9",
          "message": "v0.14 bench ci5-1",
          "timestamp": "2022-10-21T13:16:35Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/912/commits/4b0f410684b6b44ee73e46c66d5280aaf87f6cf9"
        },
        "date": 1666364923851,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 308,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 443,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 309,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 449,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 212829,
            "range": "± 6981",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 213687,
            "range": "± 1235",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 218750,
            "range": "± 2027",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 246071,
            "range": "± 6416",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 413300,
            "range": "± 7081",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 521432,
            "range": "± 14892",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 775984,
            "range": "± 13921",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1498692,
            "range": "± 53456",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 213083,
            "range": "± 2185",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 11197476,
            "range": "± 513918",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1284920,
            "range": "± 4992",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 226677,
            "range": "± 1695",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 255524,
            "range": "± 1910",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 301340,
            "range": "± 3002",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 670660,
            "range": "± 18447",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1122018,
            "range": "± 21296",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 310951,
            "range": "± 39598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 305324,
            "range": "± 39457",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 311779,
            "range": "± 29291",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 323506,
            "range": "± 26514",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 796097,
            "range": "± 9476",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 1034328,
            "range": "± 11395",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 1684752,
            "range": "± 19947",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 153960,
            "range": "± 1625",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6668786,
            "range": "± 111280",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1223365,
            "range": "± 1241",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 179376,
            "range": "± 696",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 212496,
            "range": "± 2947",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 264287,
            "range": "± 1043",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 677889,
            "range": "± 6196",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1178134,
            "range": "± 9059",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2967374,
            "range": "± 24548",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5712615,
            "range": "± 130386",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 10731484,
            "range": "± 465226",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 2864009,
            "range": "± 52490",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 5500331,
            "range": "± 80264",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 10477299,
            "range": "± 271721",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5061285,
            "range": "± 104037",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 9025304,
            "range": "± 337294",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 12330415,
            "range": "± 695334",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 18807335,
            "range": "± 743824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 32281635,
            "range": "± 1328655",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 55195188,
            "range": "± 2652747",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 104650922,
            "range": "± 5747091",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 17808795,
            "range": "± 267504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 31366044,
            "range": "± 546617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 53886468,
            "range": "± 497544",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 94624672,
            "range": "± 828154",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 22665361,
            "range": "± 415056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 47698893,
            "range": "± 393805",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 87616235,
            "range": "± 639001",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 157780218,
            "range": "± 865380",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 213302,
            "range": "± 874",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 200163,
            "range": "± 936",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 218551,
            "range": "± 648",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 244768,
            "range": "± 1026",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 410076,
            "range": "± 7226",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 523256,
            "range": "± 7122",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 773876,
            "range": "± 8487",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1469436,
            "range": "± 66323",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 211958,
            "range": "± 4950",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6634159,
            "range": "± 171454",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1272397,
            "range": "± 4221",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 214519,
            "range": "± 3943",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 243100,
            "range": "± 1256",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 290755,
            "range": "± 1868",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 655491,
            "range": "± 5179",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1112366,
            "range": "± 11704",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 293817,
            "range": "± 38775",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 307924,
            "range": "± 33470",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 305098,
            "range": "± 29197",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 327076,
            "range": "± 23002",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 798529,
            "range": "± 11762",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 1034819,
            "range": "± 17387",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 1679505,
            "range": "± 28505",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 147382,
            "range": "± 20370",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6640850,
            "range": "± 117914",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1224140,
            "range": "± 12103",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 178830,
            "range": "± 35640",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 212867,
            "range": "± 3381",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 265976,
            "range": "± 1657",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 680351,
            "range": "± 3041",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1189609,
            "range": "± 5881",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2968971,
            "range": "± 40342",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5733012,
            "range": "± 209371",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 10753992,
            "range": "± 497165",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 2867754,
            "range": "± 43724",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 5482539,
            "range": "± 72744",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 10360834,
            "range": "± 269957",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5051562,
            "range": "± 121088",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8936700,
            "range": "± 348239",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 12130800,
            "range": "± 618197",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 18417529,
            "range": "± 868865",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 32345381,
            "range": "± 1111887",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 55826637,
            "range": "± 2842029",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 103096915,
            "range": "± 5862537",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 17847109,
            "range": "± 239842",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 31478021,
            "range": "± 609563",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 53798019,
            "range": "± 609153",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 94564681,
            "range": "± 888436",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 22635682,
            "range": "± 352986",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 47727369,
            "range": "± 375958",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 87671989,
            "range": "± 565487",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 157693331,
            "range": "± 997357",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 265575,
            "range": "± 2081",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 6488,
            "range": "± 3817",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5416,
            "range": "± 216",
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
          "id": "a14c4acc52b56f73b49f773dd5dc507472ca6350",
          "message": "v0.15",
          "timestamp": "2022-10-21T13:16:35Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/913/commits/a14c4acc52b56f73b49f773dd5dc507472ca6350"
        },
        "date": 1666365166502,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_array_params_baseline",
            "value": 308,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_array_params",
            "value": 441,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params_baseline",
            "value": 310,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_object_params",
            "value": 448,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/0kb",
            "value": 209774,
            "range": "± 1665",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/1kb",
            "value": 210300,
            "range": "± 2116",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/5kb",
            "value": 216049,
            "range": "± 1172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/25kb",
            "value": 241341,
            "range": "± 15277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_custom_headers_round_trip/100kb",
            "value": 410234,
            "range": "± 9344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 517054,
            "range": "± 8278",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 771245,
            "range": "± 15339",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1478036,
            "range": "± 65598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 199310,
            "range": "± 16926",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 10366153,
            "range": "± 532889",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1280562,
            "range": "± 6845",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 223513,
            "range": "± 8069",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 251804,
            "range": "± 1612",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 286526,
            "range": "± 26065",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 651749,
            "range": "± 8641",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 1104883,
            "range": "± 12885",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/0kb",
            "value": 285137,
            "range": "± 40618",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/1kb",
            "value": 311011,
            "range": "± 39620",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/2kb",
            "value": 306304,
            "range": "± 33308",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_custom_headers_handshake/4kb",
            "value": 330168,
            "range": "± 24468",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/2",
            "value": 797132,
            "range": "± 19671",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/4",
            "value": 1030750,
            "range": "± 23497",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/8",
            "value": 1684738,
            "range": "± 26952",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 150867,
            "range": "± 3192",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 6643109,
            "range": "± 1601067",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1221038,
            "range": "± 3883",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 170070,
            "range": "± 18195",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 202674,
            "range": "± 2256",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 254420,
            "range": "± 1484",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 664133,
            "range": "± 72207",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 1169360,
            "range": "± 5582",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2941469,
            "range": "± 30472",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 5677482,
            "range": "± 130410",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 10591327,
            "range": "± 452426",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/16",
            "value": 2859302,
            "range": "± 34138",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/32",
            "value": 5507143,
            "range": "± 90277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/64",
            "value": 10480008,
            "range": "± 257278",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5087369,
            "range": "± 101598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 9043850,
            "range": "± 368824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 12154490,
            "range": "± 620126",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 18818503,
            "range": "± 650058",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 32120995,
            "range": "± 1400247",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 55508833,
            "range": "± 2993780",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 103305422,
            "range": "± 5363924",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/128",
            "value": 18019676,
            "range": "± 294892",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/256",
            "value": 31479103,
            "range": "± 701959",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/512",
            "value": 53810379,
            "range": "± 576864",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/fast_call/1024",
            "value": 94752535,
            "range": "± 891271",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 22661407,
            "range": "± 339050",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 47777681,
            "range": "± 456349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 87754626,
            "range": "± 601213",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 158149435,
            "range": "± 812373",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/0kb",
            "value": 210443,
            "range": "± 902",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/1kb",
            "value": 210986,
            "range": "± 1079",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/5kb",
            "value": 215158,
            "range": "± 555",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/25kb",
            "value": 243721,
            "range": "± 1639",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_custom_headers_round_trip/100kb",
            "value": 407731,
            "range": "± 24355",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 517767,
            "range": "± 14685",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 770730,
            "range": "± 10179",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1489287,
            "range": "± 55421",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 208559,
            "range": "± 1303",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 6630215,
            "range": "± 69773",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1281914,
            "range": "± 4882",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 222840,
            "range": "± 3108",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 252031,
            "range": "± 5654",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 299135,
            "range": "± 4514",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 666994,
            "range": "± 7203",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 1120602,
            "range": "± 12393",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/0kb",
            "value": 297053,
            "range": "± 38236",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/1kb",
            "value": 309738,
            "range": "± 39766",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/2kb",
            "value": 318963,
            "range": "± 27502",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_custom_headers_handshake/4kb",
            "value": 326803,
            "range": "± 24588",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/2",
            "value": 794252,
            "range": "± 10089",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/4",
            "value": 1034094,
            "range": "± 9938",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/8",
            "value": 1662713,
            "range": "± 52900",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 150847,
            "range": "± 11962",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 6717018,
            "range": "± 94882",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1221511,
            "range": "± 1791",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 175844,
            "range": "± 2810",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 209503,
            "range": "± 5236",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 260665,
            "range": "± 1240",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 674869,
            "range": "± 13548",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 1183921,
            "range": "± 149491",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2926383,
            "range": "± 69184",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 5564534,
            "range": "± 215696",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 10373193,
            "range": "± 617076",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/16",
            "value": 2804859,
            "range": "± 64019",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/32",
            "value": 5465001,
            "range": "± 136517",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/64",
            "value": 10317081,
            "range": "± 296970",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5027714,
            "range": "± 141190",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8828747,
            "range": "± 317927",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 12488188,
            "range": "± 606050",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 18468028,
            "range": "± 743636",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 32227810,
            "range": "± 1264727",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 55632899,
            "range": "± 2648676",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 103353070,
            "range": "± 5090467",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/128",
            "value": 17927370,
            "range": "± 306362",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/256",
            "value": 31505255,
            "range": "± 663657",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/512",
            "value": 53853750,
            "range": "± 576916",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/fast_call/1024",
            "value": 94546607,
            "range": "± 915176",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 22574264,
            "range": "± 344582",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 47782383,
            "range": "± 418267",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 87656911,
            "range": "± 610154",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 157958158,
            "range": "± 976919",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 266823,
            "range": "± 2477",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 6139,
            "range": "± 3878",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5393,
            "range": "± 1083",
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
          "message": "fix(benches): run benches against fixed client version (#890)\n\n* fix(jsonrpsee): add `types` to server feature\r\n\r\n* fix(benches): benches against fixed client version\r\n\r\n* address grumbles",
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
      }
    ]
  }
}