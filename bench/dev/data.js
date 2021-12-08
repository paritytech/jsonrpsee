window.BENCHMARK_DATA = {
  "lastUpdate": 1638923216633,
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
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "360a69a4a70129dc70f58f8de72e82d8e5c0553c",
          "message": "DRY error handling for methods (#515)\n\n* DRY error handling for methods\r\n\r\n* Fix clippy issues + unnecessary borrow",
          "timestamp": "2021-10-13T20:22:27Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/360a69a4a70129dc70f58f8de72e82d8e5c0553c"
        },
        "date": 1634170892793,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 148,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 166,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 130377,
            "range": "± 5232",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 110662,
            "range": "± 1530",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 101192,
            "range": "± 15789",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 166489,
            "range": "± 2493",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 278068,
            "range": "± 7372",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1602700,
            "range": "± 72845",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3124561,
            "range": "± 21755",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6135625,
            "range": "± 66297",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12099584,
            "range": "± 145985",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 23993586,
            "range": "± 158404",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 47941863,
            "range": "± 275108",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 65957,
            "range": "± 3089",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 83316,
            "range": "± 1771",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 91148,
            "range": "± 4246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 110506,
            "range": "± 15756",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 160115,
            "range": "± 5206",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 125862,
            "range": "± 2108",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 207577,
            "range": "± 4951",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 368087,
            "range": "± 6588",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 783602,
            "range": "± 37779",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1465917,
            "range": "± 62877",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2847313,
            "range": "± 202990",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 129613,
            "range": "± 5115",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 111229,
            "range": "± 7495",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 96023,
            "range": "± 4877",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 159323,
            "range": "± 4817",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 281406,
            "range": "± 7096",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1614309,
            "range": "± 119292",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3115998,
            "range": "± 108461",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6126352,
            "range": "± 161821",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12153924,
            "range": "± 428417",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 23961874,
            "range": "± 450693",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48279092,
            "range": "± 573299",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 63827,
            "range": "± 2121",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 83841,
            "range": "± 1589",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 91352,
            "range": "± 1705",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 111156,
            "range": "± 1287",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 160200,
            "range": "± 1372",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 124822,
            "range": "± 3976",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 208287,
            "range": "± 6311",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 373698,
            "range": "± 13857",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 775994,
            "range": "± 33258",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1494833,
            "range": "± 84821",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2915125,
            "range": "± 231587",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 81005,
            "range": "± 2403",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 13141,
            "range": "± 3292",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 922,
            "range": "± 92",
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
          "id": "e734afe28e91be4ee45da570304636420da45d0a",
          "message": "chore: update readme to new pending release (#516)",
          "timestamp": "2021-10-14T11:16:58Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/e734afe28e91be4ee45da570304636420da45d0a"
        },
        "date": 1634257300919,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 148,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 129259,
            "range": "± 4457",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 109101,
            "range": "± 1238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 95276,
            "range": "± 4790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 153455,
            "range": "± 1746",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 275292,
            "range": "± 2959",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1601457,
            "range": "± 19061",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3118545,
            "range": "± 37921",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6168468,
            "range": "± 56247",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12240439,
            "range": "± 119715",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 24267633,
            "range": "± 154385",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 48613732,
            "range": "± 363369",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 64693,
            "range": "± 2704",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 81478,
            "range": "± 1733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 91317,
            "range": "± 1419",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 111163,
            "range": "± 6092",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 162803,
            "range": "± 10589",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 126518,
            "range": "± 2453",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 209256,
            "range": "± 7248",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 379417,
            "range": "± 12063",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 776037,
            "range": "± 100966",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1485299,
            "range": "± 65942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2875570,
            "range": "± 189431",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 130574,
            "range": "± 3184",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 110474,
            "range": "± 13302",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 93707,
            "range": "± 2053",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 155713,
            "range": "± 1779",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 278037,
            "range": "± 2504",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1609671,
            "range": "± 11929",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3123808,
            "range": "± 24103",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6150946,
            "range": "± 51927",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12223388,
            "range": "± 78711",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 24164629,
            "range": "± 163359",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48308199,
            "range": "± 255399",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65740,
            "range": "± 2794",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 81629,
            "range": "± 2412",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 91037,
            "range": "± 1916",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 110975,
            "range": "± 1130",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 161849,
            "range": "± 2025",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 126883,
            "range": "± 12219",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 207821,
            "range": "± 3454",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 375951,
            "range": "± 29984",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 757424,
            "range": "± 32912",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1459173,
            "range": "± 63359",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2807444,
            "range": "± 179449",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 83338,
            "range": "± 4217",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18609,
            "range": "± 2238",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 979,
            "range": "± 92",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "4adf6665b9ef3e3e5f0eb06d080979be3bdc9f39",
          "message": "Bump actions/checkout from 2.3.4 to 2.3.5 (#532)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 2.3.4 to 2.3.5.\r\n- [Release notes](https://github.com/actions/checkout/releases)\r\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/actions/checkout/compare/v2.3.4...v2.3.5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/checkout\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2021-10-18T15:53:42Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/4adf6665b9ef3e3e5f0eb06d080979be3bdc9f39"
        },
        "date": 1634603035735,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 191,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 212,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 175495,
            "range": "± 28005",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 164126,
            "range": "± 11704",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 149602,
            "range": "± 22428",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 231727,
            "range": "± 12738",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 407570,
            "range": "± 55263",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1887292,
            "range": "± 59447",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3559604,
            "range": "± 222359",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6855268,
            "range": "± 186737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13647661,
            "range": "± 317238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 26963443,
            "range": "± 1068594",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 53731548,
            "range": "± 1229824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 101948,
            "range": "± 7731",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 117,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 121748,
            "range": "± 9980",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 136491,
            "range": "± 19957",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 161058,
            "range": "± 16812",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 189542,
            "range": "± 12291",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 172734,
            "range": "± 9742",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 277139,
            "range": "± 52755",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 484863,
            "range": "± 80019",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1013446,
            "range": "± 231469",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1957566,
            "range": "± 131391",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3645859,
            "range": "± 373212",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 177994,
            "range": "± 41015",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 166510,
            "range": "± 7926",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 149597,
            "range": "± 16715",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 246166,
            "range": "± 37399",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 412000,
            "range": "± 44053",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1908111,
            "range": "± 71502",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3603466,
            "range": "± 199225",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6961046,
            "range": "± 326467",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13673074,
            "range": "± 806595",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 27020522,
            "range": "± 966802",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 53713224,
            "range": "± 1045578",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 103827,
            "range": "± 24482",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 109,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 120847,
            "range": "± 11718",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 136880,
            "range": "± 13388",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 163464,
            "range": "± 11534",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 191806,
            "range": "± 7371",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 178431,
            "range": "± 20732",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 282434,
            "range": "± 32488",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 489957,
            "range": "± 70972",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1004955,
            "range": "± 49372",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1910025,
            "range": "± 124662",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3675222,
            "range": "± 394427",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 131382,
            "range": "± 9922",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18284,
            "range": "± 2873",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1969,
            "range": "± 445",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "4adf6665b9ef3e3e5f0eb06d080979be3bdc9f39",
          "message": "Bump actions/checkout from 2.3.4 to 2.3.5 (#532)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 2.3.4 to 2.3.5.\r\n- [Release notes](https://github.com/actions/checkout/releases)\r\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/actions/checkout/compare/v2.3.4...v2.3.5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/checkout\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2021-10-18T15:53:42Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/4adf6665b9ef3e3e5f0eb06d080979be3bdc9f39"
        },
        "date": 1634689642840,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 200,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 218,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 184105,
            "range": "± 16855",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 108,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 190162,
            "range": "± 31409",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 175740,
            "range": "± 19833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 293563,
            "range": "± 32568",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 471706,
            "range": "± 40553",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2074627,
            "range": "± 155665",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3911322,
            "range": "± 293963",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7450646,
            "range": "± 474411",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14850896,
            "range": "± 916374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 29026506,
            "range": "± 1092102",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 58780463,
            "range": "± 3302486",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 127571,
            "range": "± 95628",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 112,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 149119,
            "range": "± 17222",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 160692,
            "range": "± 25674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 187151,
            "range": "± 24982",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 225128,
            "range": "± 17970",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 185400,
            "range": "± 25130",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 326849,
            "range": "± 79527",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 593260,
            "range": "± 86338",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1077690,
            "range": "± 100349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2198515,
            "range": "± 469511",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4204669,
            "range": "± 664109",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 180858,
            "range": "± 30324",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 110,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 194897,
            "range": "± 17557",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 178444,
            "range": "± 25226",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 296672,
            "range": "± 36429",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 456135,
            "range": "± 55869",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2016483,
            "range": "± 113842",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3913305,
            "range": "± 272889",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7503406,
            "range": "± 457049",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14820655,
            "range": "± 831623",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29521163,
            "range": "± 1436157",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 58443721,
            "range": "± 3002883",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 130490,
            "range": "± 20388",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 115,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 140129,
            "range": "± 24605",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 159459,
            "range": "± 25219",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 182986,
            "range": "± 19700",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 218985,
            "range": "± 16777",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 190454,
            "range": "± 36375",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 329163,
            "range": "± 36050",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 532444,
            "range": "± 83750",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1137405,
            "range": "± 95079",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2189850,
            "range": "± 237722",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3974936,
            "range": "± 738675",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 150480,
            "range": "± 16218",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 20606,
            "range": "± 6119",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1972,
            "range": "± 917",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "4adf6665b9ef3e3e5f0eb06d080979be3bdc9f39",
          "message": "Bump actions/checkout from 2.3.4 to 2.3.5 (#532)\n\nBumps [actions/checkout](https://github.com/actions/checkout) from 2.3.4 to 2.3.5.\r\n- [Release notes](https://github.com/actions/checkout/releases)\r\n- [Changelog](https://github.com/actions/checkout/blob/main/CHANGELOG.md)\r\n- [Commits](https://github.com/actions/checkout/compare/v2.3.4...v2.3.5)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: actions/checkout\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-patch\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2021-10-18T15:53:42Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/4adf6665b9ef3e3e5f0eb06d080979be3bdc9f39"
        },
        "date": 1634775724828,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 148,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 169,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 132186,
            "range": "± 17459",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 112110,
            "range": "± 5784",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 95871,
            "range": "± 2590",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 155172,
            "range": "± 3198",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 279025,
            "range": "± 3666",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1596860,
            "range": "± 10238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3112249,
            "range": "± 15674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6133930,
            "range": "± 44246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12166547,
            "range": "± 165780",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 24036003,
            "range": "± 150471",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 47884017,
            "range": "± 280314",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 65778,
            "range": "± 2620",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 83670,
            "range": "± 2107",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 93453,
            "range": "± 1857",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 112040,
            "range": "± 1726",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 162288,
            "range": "± 2060",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 125534,
            "range": "± 2414",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 207896,
            "range": "± 5024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 380465,
            "range": "± 35713",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 774638,
            "range": "± 26200",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1501071,
            "range": "± 78340",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2875052,
            "range": "± 223333",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 126540,
            "range": "± 5696",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 110888,
            "range": "± 1804",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 95992,
            "range": "± 7293",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 156697,
            "range": "± 5246",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 277339,
            "range": "± 3932",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1602685,
            "range": "± 7496",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3095399,
            "range": "± 13687",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6108168,
            "range": "± 46207",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12074196,
            "range": "± 77421",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 24074772,
            "range": "± 511861",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 47886839,
            "range": "± 257874",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67695,
            "range": "± 2984",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 83855,
            "range": "± 2504",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 92622,
            "range": "± 1332",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 113766,
            "range": "± 1554",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 161505,
            "range": "± 1993",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 125075,
            "range": "± 2026",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 208877,
            "range": "± 4233",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 373084,
            "range": "± 8922",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 762527,
            "range": "± 23523",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1487329,
            "range": "± 72379",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2890048,
            "range": "± 230176",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 83421,
            "range": "± 1320",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18546,
            "range": "± 2310",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1051,
            "range": "± 144",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1634862257470,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 198,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 236,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 193124,
            "range": "± 20491",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 179212,
            "range": "± 13482",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 162917,
            "range": "± 10656",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 259628,
            "range": "± 30584",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 442007,
            "range": "± 26042",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1976234,
            "range": "± 177540",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3779044,
            "range": "± 155405",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7233433,
            "range": "± 390970",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14057709,
            "range": "± 447392",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 28287236,
            "range": "± 1131283",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56506563,
            "range": "± 2067373",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 117470,
            "range": "± 11366",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 127784,
            "range": "± 8007",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 149248,
            "range": "± 19851",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 178998,
            "range": "± 25359",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 208645,
            "range": "± 31634",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 190389,
            "range": "± 52851",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 299181,
            "range": "± 59780",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 522345,
            "range": "± 197727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1068605,
            "range": "± 114630",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2138902,
            "range": "± 296610",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3950811,
            "range": "± 490114",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 183614,
            "range": "± 13290",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 179026,
            "range": "± 16779",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 162691,
            "range": "± 8649",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 264011,
            "range": "± 35314",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 441761,
            "range": "± 35640",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2008221,
            "range": "± 97088",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3759122,
            "range": "± 114973",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7474214,
            "range": "± 989968",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14378044,
            "range": "± 800224",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28451604,
            "range": "± 1072990",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56722131,
            "range": "± 2380068",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 121938,
            "range": "± 9476",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 129252,
            "range": "± 8204",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 149236,
            "range": "± 18602",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 181044,
            "range": "± 20868",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 206876,
            "range": "± 17426",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 188190,
            "range": "± 17961",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 300243,
            "range": "± 57047",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 514323,
            "range": "± 30373",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1052340,
            "range": "± 71576",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2039626,
            "range": "± 164576",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3976012,
            "range": "± 456154",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 143740,
            "range": "± 9541",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18036,
            "range": "± 1988",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2043,
            "range": "± 281",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1634948505795,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 149,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 143,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 135366,
            "range": "± 11710",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 114514,
            "range": "± 1773",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 97896,
            "range": "± 33573",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 157388,
            "range": "± 26344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 285654,
            "range": "± 3479",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1608313,
            "range": "± 51172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3136788,
            "range": "± 89085",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6117680,
            "range": "± 817572",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12193868,
            "range": "± 224705",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 24056222,
            "range": "± 198713",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 47911216,
            "range": "± 547857",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 69620,
            "range": "± 3659",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 77,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 88780,
            "range": "± 1465",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 94360,
            "range": "± 2503",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 114584,
            "range": "± 4674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 163360,
            "range": "± 1826",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 127369,
            "range": "± 1790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 210845,
            "range": "± 14535",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 381353,
            "range": "± 11507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 768454,
            "range": "± 43030",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1526434,
            "range": "± 261317",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2906119,
            "range": "± 285652",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 133052,
            "range": "± 8563",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 114011,
            "range": "± 28921",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 105372,
            "range": "± 6854",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 162658,
            "range": "± 5345",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 285763,
            "range": "± 4921",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1618720,
            "range": "± 121031",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3160610,
            "range": "± 76132",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6168800,
            "range": "± 147609",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12219328,
            "range": "± 377069",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 24249629,
            "range": "± 966913",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48502310,
            "range": "± 2563985",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 69052,
            "range": "± 2913",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 87667,
            "range": "± 33630",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 97814,
            "range": "± 19277",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 116250,
            "range": "± 2684",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 165807,
            "range": "± 1931",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 123422,
            "range": "± 3422",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 211857,
            "range": "± 5294",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 598654,
            "range": "± 14030",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 780385,
            "range": "± 24114",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2238444,
            "range": "± 209634",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4419816,
            "range": "± 730626",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 126666,
            "range": "± 6126",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 8116,
            "range": "± 3086",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 832,
            "range": "± 69",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1635034988885,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 170,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 195,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 147452,
            "range": "± 5411",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 128302,
            "range": "± 3016",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 110837,
            "range": "± 2778",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 181776,
            "range": "± 5625",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 316287,
            "range": "± 10518",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1896831,
            "range": "± 138973",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3645424,
            "range": "± 39417",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7069945,
            "range": "± 130271",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14072471,
            "range": "± 198578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27975605,
            "range": "± 459638",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 55852833,
            "range": "± 820315",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 77634,
            "range": "± 3489",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 100,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 96712,
            "range": "± 6440",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 106793,
            "range": "± 3883",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 119494,
            "range": "± 3777",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 177027,
            "range": "± 4127",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 141660,
            "range": "± 3744",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 230190,
            "range": "± 15857",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 407793,
            "range": "± 15354",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 819811,
            "range": "± 32175",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1570161,
            "range": "± 79504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3015571,
            "range": "± 233019",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 139367,
            "range": "± 7478",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 127386,
            "range": "± 3662",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 112282,
            "range": "± 6940",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 183929,
            "range": "± 6516",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 314976,
            "range": "± 5896",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1863973,
            "range": "± 37187",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3685789,
            "range": "± 75079",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7083947,
            "range": "± 112514",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14212384,
            "range": "± 168177",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28075659,
            "range": "± 501404",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56035498,
            "range": "± 781173",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 78926,
            "range": "± 3740",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 97116,
            "range": "± 2584",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 105224,
            "range": "± 4840",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 118502,
            "range": "± 7803",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 180232,
            "range": "± 3625",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 143155,
            "range": "± 2431",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 229480,
            "range": "± 6123",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 407573,
            "range": "± 22353",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 834460,
            "range": "± 60263",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1606716,
            "range": "± 85229",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3081832,
            "range": "± 276850",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 94023,
            "range": "± 2471",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 20445,
            "range": "± 1203",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1071,
            "range": "± 281",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1635121307947,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 147,
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
            "name": "sync/http_round_trip",
            "value": 131067,
            "range": "± 5306",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 109313,
            "range": "± 5131",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 95331,
            "range": "± 2074",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 151712,
            "range": "± 5646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 273980,
            "range": "± 18360",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1600880,
            "range": "± 10063",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3118297,
            "range": "± 45374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6171962,
            "range": "± 44927",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12230326,
            "range": "± 40674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 24128376,
            "range": "± 367763",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 48203451,
            "range": "± 270652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 64698,
            "range": "± 2466",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 81890,
            "range": "± 2766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 91083,
            "range": "± 1263",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 110801,
            "range": "± 5014",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 159211,
            "range": "± 1131",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 124501,
            "range": "± 1971",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 206451,
            "range": "± 3230",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 376205,
            "range": "± 16187",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 762815,
            "range": "± 29526",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1448584,
            "range": "± 73374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2789323,
            "range": "± 214189",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 131056,
            "range": "± 2675",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 110223,
            "range": "± 1278",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 96838,
            "range": "± 3004",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 163551,
            "range": "± 1764",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 277124,
            "range": "± 2448",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1617902,
            "range": "± 29386",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3141706,
            "range": "± 16486",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6180004,
            "range": "± 36046",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12264916,
            "range": "± 64904",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 24194525,
            "range": "± 184915",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48317316,
            "range": "± 324579",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65091,
            "range": "± 1880",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 82366,
            "range": "± 1862",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 92093,
            "range": "± 1794",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 111267,
            "range": "± 1340",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 160727,
            "range": "± 2744",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 124778,
            "range": "± 1806",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 206951,
            "range": "± 4813",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 376191,
            "range": "± 11816",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 740810,
            "range": "± 22357",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1443785,
            "range": "± 64545",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2779599,
            "range": "± 182417",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 82018,
            "range": "± 1751",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 13137,
            "range": "± 2959",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 976,
            "range": "± 115",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1635207773822,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 188,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 217,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 160311,
            "range": "± 12084",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 161431,
            "range": "± 31942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 151656,
            "range": "± 12566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 242205,
            "range": "± 21178",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 399646,
            "range": "± 36987",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1792183,
            "range": "± 94945",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3739747,
            "range": "± 192079",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7023442,
            "range": "± 329438",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13265283,
            "range": "± 612590",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 26267353,
            "range": "± 1615960",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 51976495,
            "range": "± 2825561",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 102178,
            "range": "± 14722",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 99,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 117660,
            "range": "± 12918",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 127666,
            "range": "± 7666",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 159179,
            "range": "± 10163",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 199027,
            "range": "± 14951",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 160288,
            "range": "± 14447",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 260678,
            "range": "± 18434",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 483440,
            "range": "± 37270",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 974174,
            "range": "± 111908",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2005678,
            "range": "± 236043",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3612174,
            "range": "± 442388",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 164152,
            "range": "± 26375",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 106,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 171279,
            "range": "± 42936",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 141622,
            "range": "± 18136",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 227201,
            "range": "± 18894",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 397061,
            "range": "± 35695",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1977739,
            "range": "± 140448",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3458681,
            "range": "± 242491",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6649376,
            "range": "± 481318",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13356837,
            "range": "± 606804",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28152585,
            "range": "± 1671179",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56469418,
            "range": "± 2765480",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 98777,
            "range": "± 8525",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 92,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 122333,
            "range": "± 10381",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 134562,
            "range": "± 17664",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 163346,
            "range": "± 17784",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 192510,
            "range": "± 21790",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 177946,
            "range": "± 19387",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 292241,
            "range": "± 36566",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 509620,
            "range": "± 68627",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 944523,
            "range": "± 78349",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1746388,
            "range": "± 198254",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3630822,
            "range": "± 400380",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 130926,
            "range": "± 7617",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 24488,
            "range": "± 3829",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1511,
            "range": "± 943",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1635294174791,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 187,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 202,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 153258,
            "range": "± 22569",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 105,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 136243,
            "range": "± 17025",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 133886,
            "range": "± 16212",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 181691,
            "range": "± 21207",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 324370,
            "range": "± 20646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2161251,
            "range": "± 233054",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3838907,
            "range": "± 310628",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7393702,
            "range": "± 390591",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 15001174,
            "range": "± 974955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 28866708,
            "range": "± 1406751",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 59011942,
            "range": "± 3337176",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 78597,
            "range": "± 7922",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 105,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 98842,
            "range": "± 8934",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 111327,
            "range": "± 9875",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 126031,
            "range": "± 32124",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 191216,
            "range": "± 20635",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 147932,
            "range": "± 19382",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 230343,
            "range": "± 9863",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 433194,
            "range": "± 78458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 889545,
            "range": "± 175106",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1739693,
            "range": "± 241251",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3235003,
            "range": "± 353964",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 152570,
            "range": "± 15893",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 137564,
            "range": "± 11478",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 129900,
            "range": "± 12305",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 198373,
            "range": "± 14425",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 326342,
            "range": "± 20538",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1992566,
            "range": "± 149051",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3838624,
            "range": "± 197986",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7281637,
            "range": "± 432655",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14858614,
            "range": "± 689764",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29351023,
            "range": "± 2911914",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56458711,
            "range": "± 514275",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 82083,
            "range": "± 8749",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 105,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 100511,
            "range": "± 13462",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 110092,
            "range": "± 7192",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 121994,
            "range": "± 11349",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 183956,
            "range": "± 36658",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 147489,
            "range": "± 17794",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 242387,
            "range": "± 23903",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 433054,
            "range": "± 39945",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 848026,
            "range": "± 73134",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1620700,
            "range": "± 127285",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3031897,
            "range": "± 296748",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 95930,
            "range": "± 6665",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18923,
            "range": "± 2471",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1351,
            "range": "± 493",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1635380634414,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 180,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 208,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 183441,
            "range": "± 28976",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 97,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 174723,
            "range": "± 12436",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 158771,
            "range": "± 14265",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 251118,
            "range": "± 17463",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 438808,
            "range": "± 59377",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1919313,
            "range": "± 93760",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3639059,
            "range": "± 146123",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7066352,
            "range": "± 300924",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13733105,
            "range": "± 527618",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27446185,
            "range": "± 959039",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 54795233,
            "range": "± 2748442",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 115646,
            "range": "± 8823",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 100,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 124530,
            "range": "± 10250",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 143266,
            "range": "± 22436",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 177389,
            "range": "± 137434",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 203880,
            "range": "± 13113",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 185815,
            "range": "± 92910",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 294775,
            "range": "± 35550",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 496570,
            "range": "± 116048",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1018896,
            "range": "± 108381",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1892315,
            "range": "± 320749",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3535575,
            "range": "± 557831",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 174232,
            "range": "± 25858",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 100,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 172817,
            "range": "± 15486",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 154280,
            "range": "± 21011",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 246329,
            "range": "± 22171",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 430097,
            "range": "± 35269",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1911610,
            "range": "± 247135",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3565198,
            "range": "± 307235",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6899989,
            "range": "± 430969",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13574383,
            "range": "± 748602",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 26952325,
            "range": "± 1005009",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 53834462,
            "range": "± 2972843",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 112309,
            "range": "± 13259",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 98,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 127057,
            "range": "± 23773",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 145645,
            "range": "± 11772",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 174865,
            "range": "± 30524",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 198744,
            "range": "± 14248",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 182057,
            "range": "± 45512",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 311358,
            "range": "± 64864",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 509392,
            "range": "± 65462",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1010825,
            "range": "± 86700",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1965311,
            "range": "± 173635",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3618067,
            "range": "± 520736",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 137542,
            "range": "± 45003",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 20926,
            "range": "± 3744",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1593,
            "range": "± 417",
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
          "id": "926f8914b0233bf335824f409d30bc23bf4a205b",
          "message": "feat(ws client): support custom headers. (#535)\n\n* feat(ws client): support custom headers.\r\n\r\nClosing #531\r\n\r\n* remove empty line\r\n\r\n* address grumbles: more user-friendly API\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-client/src/client.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* address grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-21T11:46:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/926f8914b0233bf335824f409d30bc23bf4a205b"
        },
        "date": 1635466905682,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 149,
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
            "name": "sync/http_round_trip",
            "value": 130164,
            "range": "± 4050",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 108704,
            "range": "± 1489",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 94644,
            "range": "± 8889",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 152151,
            "range": "± 4549",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 274283,
            "range": "± 2686",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1587695,
            "range": "± 12610",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3082286,
            "range": "± 96662",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6064549,
            "range": "± 146154",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12043965,
            "range": "± 250905",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 23778575,
            "range": "± 161843",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 47466975,
            "range": "± 253781",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 64479,
            "range": "± 1913",
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
            "value": 82231,
            "range": "± 1329",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 91469,
            "range": "± 2321",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 111134,
            "range": "± 1216",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 161782,
            "range": "± 5602",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 123998,
            "range": "± 3159",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 206058,
            "range": "± 3786",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 365064,
            "range": "± 41768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 740332,
            "range": "± 134806",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1456242,
            "range": "± 84060",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2728287,
            "range": "± 222796",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 125655,
            "range": "± 5268",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 109138,
            "range": "± 4082",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 93755,
            "range": "± 3088",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 152679,
            "range": "± 2289",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 275429,
            "range": "± 8596",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1598065,
            "range": "± 14557",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3085871,
            "range": "± 49957",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6056849,
            "range": "± 33905",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11989014,
            "range": "± 43078",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 23686224,
            "range": "± 175806",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 47382973,
            "range": "± 408421",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 64469,
            "range": "± 1792",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 83142,
            "range": "± 5623",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 92534,
            "range": "± 1507",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 112027,
            "range": "± 1933",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 162059,
            "range": "± 3713",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 122529,
            "range": "± 2903",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 204949,
            "range": "± 3578",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 366810,
            "range": "± 6348",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 741096,
            "range": "± 20907",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1479694,
            "range": "± 63644",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2825673,
            "range": "± 219349",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 80809,
            "range": "± 3136",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 15348,
            "range": "± 2044",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 918,
            "range": "± 105",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "68154226650808d795d018989ea269c60c47c96d",
          "message": "clients: add support for `webpki and native certificate stores` (#533)\n\n* Update tokio-rustls requirement from 0.22 to 0.23\r\n\r\nUpdates the requirements on [tokio-rustls](https://github.com/tokio-rs/tls) to permit the latest version.\r\n- [Release notes](https://github.com/tokio-rs/tls/releases)\r\n- [Commits](https://github.com/tokio-rs/tls/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-rustls\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* push fixes but requires rustls-native-certs v0.6\r\n\r\n* update native certs to 0.6.0\r\n\r\n* fix clippy warnings\r\n\r\n* remove webpki roots support\r\n\r\n* Revert \"remove webpki roots support\"\r\n\r\nThis reverts commit 1144d567b343049ab7c967d320fc2fe162ba0f7c.\r\n\r\n* support both native cert store and webpki\r\n\r\n* sort deps in Cargo.toml\r\n\r\n* Update ws-client/src/transport.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-29T19:42:13Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/68154226650808d795d018989ea269c60c47c96d"
        },
        "date": 1635553409591,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 173,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 197,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 146603,
            "range": "± 4533",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 128942,
            "range": "± 2471",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 109649,
            "range": "± 11025",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 178137,
            "range": "± 5200",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 309420,
            "range": "± 4250",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1981558,
            "range": "± 26533",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3890740,
            "range": "± 73182",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7648604,
            "range": "± 82294",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14998075,
            "range": "± 151037",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 29554484,
            "range": "± 391522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 59682235,
            "range": "± 877530",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 74809,
            "range": "± 3634",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 96607,
            "range": "± 8033",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 105659,
            "range": "± 3537",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 114733,
            "range": "± 2308",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 171859,
            "range": "± 2688",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 135760,
            "range": "± 3406",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 224209,
            "range": "± 13165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 399077,
            "range": "± 16556",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 798248,
            "range": "± 26065",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1554848,
            "range": "± 70255",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3033652,
            "range": "± 392381",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 145727,
            "range": "± 17814",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 130563,
            "range": "± 2851",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 110207,
            "range": "± 3919",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 178732,
            "range": "± 6962",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 307481,
            "range": "± 4725",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1999093,
            "range": "± 76529",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3869039,
            "range": "± 44172",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7610153,
            "range": "± 66003",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 15129219,
            "range": "± 215282",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29654067,
            "range": "± 369767",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 59156613,
            "range": "± 702086",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 76294,
            "range": "± 1644",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 106,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 96338,
            "range": "± 3454",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 106730,
            "range": "± 2974",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 117116,
            "range": "± 4087",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 177858,
            "range": "± 8473",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 140709,
            "range": "± 3278",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 226523,
            "range": "± 3865",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 405341,
            "range": "± 14747",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 805865,
            "range": "± 34062",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1569944,
            "range": "± 72776",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3003392,
            "range": "± 210696",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 109270,
            "range": "± 15416",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 20300,
            "range": "± 2254",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1060,
            "range": "± 141",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "68154226650808d795d018989ea269c60c47c96d",
          "message": "clients: add support for `webpki and native certificate stores` (#533)\n\n* Update tokio-rustls requirement from 0.22 to 0.23\r\n\r\nUpdates the requirements on [tokio-rustls](https://github.com/tokio-rs/tls) to permit the latest version.\r\n- [Release notes](https://github.com/tokio-rs/tls/releases)\r\n- [Commits](https://github.com/tokio-rs/tls/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-rustls\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* push fixes but requires rustls-native-certs v0.6\r\n\r\n* update native certs to 0.6.0\r\n\r\n* fix clippy warnings\r\n\r\n* remove webpki roots support\r\n\r\n* Revert \"remove webpki roots support\"\r\n\r\nThis reverts commit 1144d567b343049ab7c967d320fc2fe162ba0f7c.\r\n\r\n* support both native cert store and webpki\r\n\r\n* sort deps in Cargo.toml\r\n\r\n* Update ws-client/src/transport.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-29T19:42:13Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/68154226650808d795d018989ea269c60c47c96d"
        },
        "date": 1635639754078,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 147,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 178,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 135036,
            "range": "± 6242",
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
            "value": 112669,
            "range": "± 1363",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 95553,
            "range": "± 2564",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 156810,
            "range": "± 4742",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 284289,
            "range": "± 65683",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1605231,
            "range": "± 25459",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3136342,
            "range": "± 22208",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6156585,
            "range": "± 22187",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12220375,
            "range": "± 71202",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 24145569,
            "range": "± 218812",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 48128459,
            "range": "± 303031",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 68260,
            "range": "± 4560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 86811,
            "range": "± 1371",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 94499,
            "range": "± 1887",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 116222,
            "range": "± 1906",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 167569,
            "range": "± 2335",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 130011,
            "range": "± 14073",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 214069,
            "range": "± 4145",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 383585,
            "range": "± 29123",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 790721,
            "range": "± 143407",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1546960,
            "range": "± 111494",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2992320,
            "range": "± 263052",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 136674,
            "range": "± 15073",
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
            "value": 116672,
            "range": "± 1604",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 97742,
            "range": "± 13490",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 169009,
            "range": "± 3654",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 286291,
            "range": "± 3381",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1619862,
            "range": "± 12552",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3143081,
            "range": "± 24214",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6194676,
            "range": "± 62863",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12205604,
            "range": "± 95675",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 24106349,
            "range": "± 198957",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48134007,
            "range": "± 301854",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67164,
            "range": "± 2149",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 89,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 85024,
            "range": "± 2008",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 94406,
            "range": "± 1461",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 113780,
            "range": "± 1069",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 163961,
            "range": "± 3265",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 128365,
            "range": "± 2533",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 211232,
            "range": "± 5251",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 380458,
            "range": "± 7261",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 770905,
            "range": "± 30137",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1513865,
            "range": "± 65720",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2924600,
            "range": "± 233386",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 80102,
            "range": "± 2008",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 12471,
            "range": "± 2129",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 964,
            "range": "± 135",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "68154226650808d795d018989ea269c60c47c96d",
          "message": "clients: add support for `webpki and native certificate stores` (#533)\n\n* Update tokio-rustls requirement from 0.22 to 0.23\r\n\r\nUpdates the requirements on [tokio-rustls](https://github.com/tokio-rs/tls) to permit the latest version.\r\n- [Release notes](https://github.com/tokio-rs/tls/releases)\r\n- [Commits](https://github.com/tokio-rs/tls/commits)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: tokio-rustls\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* push fixes but requires rustls-native-certs v0.6\r\n\r\n* update native certs to 0.6.0\r\n\r\n* fix clippy warnings\r\n\r\n* remove webpki roots support\r\n\r\n* Revert \"remove webpki roots support\"\r\n\r\nThis reverts commit 1144d567b343049ab7c967d320fc2fe162ba0f7c.\r\n\r\n* support both native cert store and webpki\r\n\r\n* sort deps in Cargo.toml\r\n\r\n* Update ws-client/src/transport.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-10-29T19:42:13Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/68154226650808d795d018989ea269c60c47c96d"
        },
        "date": 1635726223556,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 173,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 193,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 144956,
            "range": "± 4900",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 106,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 129387,
            "range": "± 6951",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 111318,
            "range": "± 2721",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 181806,
            "range": "± 6993",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 314248,
            "range": "± 5643",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1819437,
            "range": "± 36836",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3514170,
            "range": "± 63653",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6985949,
            "range": "± 119699",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13801840,
            "range": "± 356508",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27238797,
            "range": "± 405544",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 54779874,
            "range": "± 1823891",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 74803,
            "range": "± 2329",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 105,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 98167,
            "range": "± 2008",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 106786,
            "range": "± 23785",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 112624,
            "range": "± 3270",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 171126,
            "range": "± 3638",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 134757,
            "range": "± 4570",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 225595,
            "range": "± 5136",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 390818,
            "range": "± 42793",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 783953,
            "range": "± 44852",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1507387,
            "range": "± 74000",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2927009,
            "range": "± 223062",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 144237,
            "range": "± 6013",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 99,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 127525,
            "range": "± 3346",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 106482,
            "range": "± 3361",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 179944,
            "range": "± 9483",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 308279,
            "range": "± 7679",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1834309,
            "range": "± 29993",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3563315,
            "range": "± 64651",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7048937,
            "range": "± 73616",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13757960,
            "range": "± 160540",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 27706673,
            "range": "± 388367",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 55500002,
            "range": "± 684280",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 75095,
            "range": "± 2689",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 98,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 94044,
            "range": "± 1773",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 105102,
            "range": "± 3678",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 114336,
            "range": "± 3599",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 168896,
            "range": "± 8510",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 135095,
            "range": "± 4402",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 221475,
            "range": "± 5068",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 385957,
            "range": "± 30213",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 786641,
            "range": "± 27477",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1544345,
            "range": "± 71324",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2937119,
            "range": "± 245117",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 89082,
            "range": "± 3138",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18566,
            "range": "± 2663",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1000,
            "range": "± 127",
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
          "id": "092081a0a2b8904c6ebd2cd99e16c7bc13ffc3ae",
          "message": "fix(ws server): `batch` wait until all methods has been executed. (#542)\n\n* reproduce Kian's issue\r\n\r\n* fix ws server wait until batches has completed\r\n\r\n* fix nit\r\n\r\n* clippify\r\n\r\n* enable benches for ws batch requests\r\n\r\n* use stream instead of futures::join_all\r\n\r\n* clippify\r\n\r\n* address grumbles: better assert",
          "timestamp": "2021-11-01T11:20:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/092081a0a2b8904c6ebd2cd99e16c7bc13ffc3ae"
        },
        "date": 1635812733628,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 182,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 205,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 145867,
            "range": "± 14185",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 100,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 132730,
            "range": "± 9731",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 112100,
            "range": "± 5356",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 187647,
            "range": "± 14552",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 313729,
            "range": "± 6023",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1966874,
            "range": "± 160869",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3652749,
            "range": "± 179227",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7391556,
            "range": "± 663458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14256088,
            "range": "± 549528",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 29138484,
            "range": "± 2000156",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56805650,
            "range": "± 839941",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 78740,
            "range": "± 6725",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 98624,
            "range": "± 7139",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 106460,
            "range": "± 4195",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 116076,
            "range": "± 2722",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 174970,
            "range": "± 3348",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 144235,
            "range": "± 10018",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 240170,
            "range": "± 34767",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 407588,
            "range": "± 62531",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 860118,
            "range": "± 201096",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1626790,
            "range": "± 178385",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3071195,
            "range": "± 406296",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 149267,
            "range": "± 18020",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 104,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 133917,
            "range": "± 9750",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 120512,
            "range": "± 26066",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 194009,
            "range": "± 12486",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 316830,
            "range": "± 7546",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1931552,
            "range": "± 156910",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3683794,
            "range": "± 129336",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7226623,
            "range": "± 168209",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14255093,
            "range": "± 138462",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28049798,
            "range": "± 623071",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56002455,
            "range": "± 1055653",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 79212,
            "range": "± 6012",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 99,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 96103,
            "range": "± 3309",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 106613,
            "range": "± 2529",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 122176,
            "range": "± 10130",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 177066,
            "range": "± 5827",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 144488,
            "range": "± 50137",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 236274,
            "range": "± 34401",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 407195,
            "range": "± 62106",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 794959,
            "range": "± 30404",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1618846,
            "range": "± 182272",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2997684,
            "range": "± 245151",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 94838,
            "range": "± 7346",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 20174,
            "range": "± 2670",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1325,
            "range": "± 291",
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
          "id": "092081a0a2b8904c6ebd2cd99e16c7bc13ffc3ae",
          "message": "fix(ws server): `batch` wait until all methods has been executed. (#542)\n\n* reproduce Kian's issue\r\n\r\n* fix ws server wait until batches has completed\r\n\r\n* fix nit\r\n\r\n* clippify\r\n\r\n* enable benches for ws batch requests\r\n\r\n* use stream instead of futures::join_all\r\n\r\n* clippify\r\n\r\n* address grumbles: better assert",
          "timestamp": "2021-11-01T11:20:41Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/092081a0a2b8904c6ebd2cd99e16c7bc13ffc3ae"
        },
        "date": 1635899220490,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 149,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 131920,
            "range": "± 2902",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 110262,
            "range": "± 1433",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 102060,
            "range": "± 2217",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 159453,
            "range": "± 4067",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 277825,
            "range": "± 2818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1570270,
            "range": "± 25117",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3035186,
            "range": "± 28942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5985988,
            "range": "± 77170",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11868841,
            "range": "± 159860",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 23602166,
            "range": "± 240336",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 47198738,
            "range": "± 282818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 64821,
            "range": "± 2890",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 87041,
            "range": "± 19035",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 91433,
            "range": "± 1438",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 111643,
            "range": "± 1165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 159875,
            "range": "± 7990",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 127902,
            "range": "± 7560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 215081,
            "range": "± 3652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 376379,
            "range": "± 13739",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 799999,
            "range": "± 24243",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1572973,
            "range": "± 151062",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2963295,
            "range": "± 605507",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 130526,
            "range": "± 3939",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 109981,
            "range": "± 1930",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 101919,
            "range": "± 4662",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 161400,
            "range": "± 4568",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 277376,
            "range": "± 8960",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1570145,
            "range": "± 23793",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3017147,
            "range": "± 26410",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5907490,
            "range": "± 91884",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11853297,
            "range": "± 171309",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 23643331,
            "range": "± 210915",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 47289904,
            "range": "± 279866",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 64354,
            "range": "± 1457",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 82677,
            "range": "± 1768",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 91949,
            "range": "± 3379",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 111617,
            "range": "± 1038",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 159739,
            "range": "± 1617",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 128560,
            "range": "± 2063",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 214220,
            "range": "± 3632",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 371312,
            "range": "± 9410",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 777157,
            "range": "± 20454",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1531171,
            "range": "± 67177",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2935777,
            "range": "± 177196",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 83147,
            "range": "± 1984",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 15629,
            "range": "± 2908",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 895,
            "range": "± 208",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "¯\\_(ツ)_/¯",
            "username": "DefinitelyNotHilbert",
            "email": "92186471+DefinitelyNotHilbert@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "ff3337b107bf29bef6067164c20c6a0b0b5bdc75",
          "message": "Proc mac support map param (#544)\n\n* feat(proc_macro): add support for map arguments\r\n\r\n* feat(proc_macro): formatting\r\n\r\n* feat(proc_macro): fix issues with Into trait\r\n\r\n* feat(proc_macro): param_format for methods\r\n\r\n* feat(proc_macro): improve param_format checking\r\n\r\n- Addressed @niklasad1's suggestion to use an Option instead of just\r\ndefaulting to \"array\".\r\n\r\n* feat(proc_macro): apply suggestions, add test case\r\n\r\n- Use enum for param format.\r\n- Extract parsing logic into separate function.\r\n- Add ui test.\r\n\r\n* feat(proc_macro): run cargo fmt\r\n\r\n* feat(proc_macro): address suggestions\r\n\r\n* feat(proc_macro): document param_kind argument\r\n\r\n* feat(proc_macro):  consistent spacing\r\n\r\nApply @maciejhirsz formatting suggestion.\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>\r\n\r\n* feat(proc_macro): apply suggestions\r\n\r\n- make parameter encoding DRY\r\n- remove strings from param_kind\r\n- return result from parse_param_kind\r\n\r\n* feat(proc_macro): formatting\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>",
          "timestamp": "2021-11-03T14:26:17Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/ff3337b107bf29bef6067164c20c6a0b0b5bdc75"
        },
        "date": 1635985594878,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 184,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 219,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 181606,
            "range": "± 22619",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 95,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 169916,
            "range": "± 33469",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 160498,
            "range": "± 24594",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 244477,
            "range": "± 42898",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 429509,
            "range": "± 52867",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1904024,
            "range": "± 145383",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3586908,
            "range": "± 242913",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6964183,
            "range": "± 372107",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13615817,
            "range": "± 833629",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27176059,
            "range": "± 1475992",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 54061318,
            "range": "± 2471320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 104531,
            "range": "± 12132",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 94,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 119255,
            "range": "± 22126",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 137292,
            "range": "± 13414",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 159700,
            "range": "± 15808",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 207275,
            "range": "± 23952",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 184985,
            "range": "± 21673",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 296896,
            "range": "± 41797",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 522649,
            "range": "± 60880",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1066536,
            "range": "± 127768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2005695,
            "range": "± 279731",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3838792,
            "range": "± 501870",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 179859,
            "range": "± 13369",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 95,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 172774,
            "range": "± 17942",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 162512,
            "range": "± 106161",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 250715,
            "range": "± 47866",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 434974,
            "range": "± 48900",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1929223,
            "range": "± 130525",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3629489,
            "range": "± 218707",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6946809,
            "range": "± 441843",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13840940,
            "range": "± 702753",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 27391134,
            "range": "± 1711999",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 53765165,
            "range": "± 3659926",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 104797,
            "range": "± 22944",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 94,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 121100,
            "range": "± 7240",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 136391,
            "range": "± 14793",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 162696,
            "range": "± 15480",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 205112,
            "range": "± 18843",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 184078,
            "range": "± 40275",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 302387,
            "range": "± 64331",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 522098,
            "range": "± 69947",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1072447,
            "range": "± 108728",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2130123,
            "range": "± 221345",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3894367,
            "range": "± 538603",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 125493,
            "range": "± 65995",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 2905,
            "range": "± 7216",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1592,
            "range": "± 606",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "¯\\_(ツ)_/¯",
            "username": "DefinitelyNotHilbert",
            "email": "92186471+DefinitelyNotHilbert@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "ff3337b107bf29bef6067164c20c6a0b0b5bdc75",
          "message": "Proc mac support map param (#544)\n\n* feat(proc_macro): add support for map arguments\r\n\r\n* feat(proc_macro): formatting\r\n\r\n* feat(proc_macro): fix issues with Into trait\r\n\r\n* feat(proc_macro): param_format for methods\r\n\r\n* feat(proc_macro): improve param_format checking\r\n\r\n- Addressed @niklasad1's suggestion to use an Option instead of just\r\ndefaulting to \"array\".\r\n\r\n* feat(proc_macro): apply suggestions, add test case\r\n\r\n- Use enum for param format.\r\n- Extract parsing logic into separate function.\r\n- Add ui test.\r\n\r\n* feat(proc_macro): run cargo fmt\r\n\r\n* feat(proc_macro): address suggestions\r\n\r\n* feat(proc_macro): document param_kind argument\r\n\r\n* feat(proc_macro):  consistent spacing\r\n\r\nApply @maciejhirsz formatting suggestion.\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>\r\n\r\n* feat(proc_macro): apply suggestions\r\n\r\n- make parameter encoding DRY\r\n- remove strings from param_kind\r\n- return result from parse_param_kind\r\n\r\n* feat(proc_macro): formatting\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>",
          "timestamp": "2021-11-03T14:26:17Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/ff3337b107bf29bef6067164c20c6a0b0b5bdc75"
        },
        "date": 1636071894773,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 165,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 200,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 171264,
            "range": "± 19400",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 86,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 154807,
            "range": "± 10476",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 142373,
            "range": "± 7093",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 227204,
            "range": "± 17006",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 390377,
            "range": "± 16765",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1769807,
            "range": "± 79053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3377187,
            "range": "± 161489",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6451388,
            "range": "± 286928",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12916114,
            "range": "± 561096",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 25423820,
            "range": "± 997934",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 51315335,
            "range": "± 2510539",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 101837,
            "range": "± 13285",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 95,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 115331,
            "range": "± 5646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 132538,
            "range": "± 6818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 157068,
            "range": "± 7406",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 198197,
            "range": "± 11457",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 178945,
            "range": "± 18657",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 282349,
            "range": "± 25294",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 490372,
            "range": "± 24573",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1010463,
            "range": "± 102473",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1953050,
            "range": "± 196257",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3656525,
            "range": "± 369564",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 174313,
            "range": "± 11422",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 89,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 163038,
            "range": "± 17729",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 146993,
            "range": "± 10672",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 228236,
            "range": "± 14330",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 389692,
            "range": "± 18143",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1795556,
            "range": "± 108701",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3356530,
            "range": "± 250451",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6453307,
            "range": "± 275572",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12651035,
            "range": "± 409515",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 25508598,
            "range": "± 969130",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 51217113,
            "range": "± 2367427",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 100462,
            "range": "± 7074",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 85,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 114979,
            "range": "± 5962",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 132812,
            "range": "± 10666",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 155792,
            "range": "± 15972",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 193393,
            "range": "± 10147",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 179681,
            "range": "± 14352",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 288099,
            "range": "± 19017",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 489684,
            "range": "± 26985",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 969281,
            "range": "± 52892",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1886736,
            "range": "± 153318",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3522101,
            "range": "± 268186",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 132109,
            "range": "± 11435",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 21851,
            "range": "± 1678",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2042,
            "range": "± 445",
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
          "id": "32d29259acb644591592aaba0ded18649342d078",
          "message": "clients: request ID as RAII guard (#543)\n\n* request ID as RAII guard\r\n\r\n* clippify\r\n\r\n* fmt\r\n\r\n* address grumbles: naming\r\n\r\n`RequestIdGuard` -> `RequestIdManager`\r\n`RequestId` -> `RequestIdGuard`",
          "timestamp": "2021-11-05T16:15:22Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/32d29259acb644591592aaba0ded18649342d078"
        },
        "date": 1636158300235,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 171,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 186,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 143889,
            "range": "± 7910",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 97,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 127819,
            "range": "± 2726",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 120517,
            "range": "± 2833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 186609,
            "range": "± 8362",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 311352,
            "range": "± 17498",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1909326,
            "range": "± 132817",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3536297,
            "range": "± 133067",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7278991,
            "range": "± 443976",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14342682,
            "range": "± 293799",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 28308667,
            "range": "± 360176",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56315513,
            "range": "± 785502",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 76460,
            "range": "± 1741",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 98,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 98166,
            "range": "± 3522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 100655,
            "range": "± 3331",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 115150,
            "range": "± 3378",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 176754,
            "range": "± 10880",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 143397,
            "range": "± 3566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 230763,
            "range": "± 5218",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 392139,
            "range": "± 25782",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 829484,
            "range": "± 29645",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1615756,
            "range": "± 105788",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3012918,
            "range": "± 233442",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 147077,
            "range": "± 6991",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 99,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 126230,
            "range": "± 3429",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 119638,
            "range": "± 5145",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 193647,
            "range": "± 4054",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 308036,
            "range": "± 8361",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1858045,
            "range": "± 56301",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3609671,
            "range": "± 111541",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7047858,
            "range": "± 218422",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13820362,
            "range": "± 339309",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 27213222,
            "range": "± 589755",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 55424206,
            "range": "± 2423261",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 75654,
            "range": "± 2195",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 99,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 99464,
            "range": "± 2969",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 106651,
            "range": "± 18427",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 117326,
            "range": "± 3894",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 174966,
            "range": "± 3862",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 140661,
            "range": "± 5420",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 233808,
            "range": "± 10432",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 403114,
            "range": "± 13948",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 839635,
            "range": "± 27163",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1599744,
            "range": "± 77675",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3057410,
            "range": "± 228703",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 91433,
            "range": "± 4713",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 17583,
            "range": "± 1964",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 868,
            "range": "± 154",
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
          "id": "32d29259acb644591592aaba0ded18649342d078",
          "message": "clients: request ID as RAII guard (#543)\n\n* request ID as RAII guard\r\n\r\n* clippify\r\n\r\n* fmt\r\n\r\n* address grumbles: naming\r\n\r\n`RequestIdGuard` -> `RequestIdManager`\r\n`RequestId` -> `RequestIdGuard`",
          "timestamp": "2021-11-05T16:15:22Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/32d29259acb644591592aaba0ded18649342d078"
        },
        "date": 1636244815306,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 179,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 212,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 177232,
            "range": "± 13607",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 93,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 162249,
            "range": "± 43206",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 151635,
            "range": "± 9606",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 237288,
            "range": "± 19438",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 397414,
            "range": "± 18660",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1895234,
            "range": "± 70193",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3538382,
            "range": "± 144681",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6861137,
            "range": "± 295824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13488870,
            "range": "± 245616",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 26754043,
            "range": "± 875575",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 53653138,
            "range": "± 1012472",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 103460,
            "range": "± 17520",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 92,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 114490,
            "range": "± 11352",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 134918,
            "range": "± 12063",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 160114,
            "range": "± 9633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 199093,
            "range": "± 8845",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 178606,
            "range": "± 15089",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 295356,
            "range": "± 54654",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 508318,
            "range": "± 104733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1027107,
            "range": "± 76484",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2041546,
            "range": "± 154261",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3795515,
            "range": "± 582567",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 176117,
            "range": "± 17953",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 93,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 160454,
            "range": "± 11953",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 145747,
            "range": "± 13114",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 236900,
            "range": "± 25151",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 394678,
            "range": "± 24723",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1904831,
            "range": "± 140607",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3575168,
            "range": "± 197519",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6913312,
            "range": "± 342904",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13595435,
            "range": "± 363346",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 26717293,
            "range": "± 498452",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 53661687,
            "range": "± 1257429",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 101821,
            "range": "± 6331",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 93,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 115257,
            "range": "± 3173",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 135230,
            "range": "± 12832",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 160628,
            "range": "± 14013",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 200214,
            "range": "± 10769",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 177148,
            "range": "± 31419",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 285070,
            "range": "± 11766",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 507042,
            "range": "± 43029",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1040881,
            "range": "± 57011",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1989312,
            "range": "± 130070",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3639449,
            "range": "± 340203",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 133908,
            "range": "± 28561",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 22972,
            "range": "± 1680",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1327,
            "range": "± 270",
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
          "id": "32d29259acb644591592aaba0ded18649342d078",
          "message": "clients: request ID as RAII guard (#543)\n\n* request ID as RAII guard\r\n\r\n* clippify\r\n\r\n* fmt\r\n\r\n* address grumbles: naming\r\n\r\n`RequestIdGuard` -> `RequestIdManager`\r\n`RequestId` -> `RequestIdGuard`",
          "timestamp": "2021-11-05T16:15:22Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/32d29259acb644591592aaba0ded18649342d078"
        },
        "date": 1636331075215,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 150,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 167,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 132678,
            "range": "± 16216",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 113405,
            "range": "± 1308",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 198070,
            "range": "± 54025",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 159430,
            "range": "± 8120",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 281832,
            "range": "± 5566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1617215,
            "range": "± 60757",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3107844,
            "range": "± 73762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6094400,
            "range": "± 60762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12187977,
            "range": "± 821519",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 24204084,
            "range": "± 187249",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 48193248,
            "range": "± 585979",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 67637,
            "range": "± 2797",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 86,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 87158,
            "range": "± 1770",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 94581,
            "range": "± 1702",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 114695,
            "range": "± 2572",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 163323,
            "range": "± 1768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 134934,
            "range": "± 25749",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 219560,
            "range": "± 4044",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 394328,
            "range": "± 31836",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 813760,
            "range": "± 54510",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1603420,
            "range": "± 82222",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2974816,
            "range": "± 268460",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 133420,
            "range": "± 3190",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 85,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 112502,
            "range": "± 2397",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 105306,
            "range": "± 2192",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 163544,
            "range": "± 3430",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 278393,
            "range": "± 3919",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1600899,
            "range": "± 11932",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3076926,
            "range": "± 128057",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6017135,
            "range": "± 61644",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11927876,
            "range": "± 142381",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 23832984,
            "range": "± 268133",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 47626451,
            "range": "± 514498",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67133,
            "range": "± 2885",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 87326,
            "range": "± 1866",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 94023,
            "range": "± 1634",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 115431,
            "range": "± 11467",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 162591,
            "range": "± 4634",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 132027,
            "range": "± 2215",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 221045,
            "range": "± 3862",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 397448,
            "range": "± 11241",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 827395,
            "range": "± 53981",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1646666,
            "range": "± 323500",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3067498,
            "range": "± 304999",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 86333,
            "range": "± 3357",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18076,
            "range": "± 2373",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 828,
            "range": "± 140",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Chris Sosnin",
            "username": "slumber",
            "email": "48099298+slumber@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "afcf411d9bbf1fce95caacab8b5e66857880064b",
          "message": "Allow awaiting on server handles (#550)\n\n* Implement Future for server handles\r\n\r\n* Explicitly assert timeout errors in tests",
          "timestamp": "2021-11-08T15:57:06Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/afcf411d9bbf1fce95caacab8b5e66857880064b"
        },
        "date": 1636417570143,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 194,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 216,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 184547,
            "range": "± 11804",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 93,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 171887,
            "range": "± 11853",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 155810,
            "range": "± 12227",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 251843,
            "range": "± 44455",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 422149,
            "range": "± 27115",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1940627,
            "range": "± 134446",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3809204,
            "range": "± 153242",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7310849,
            "range": "± 225356",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14123516,
            "range": "± 560611",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27867133,
            "range": "± 1085440",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56159368,
            "range": "± 1485867",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 111461,
            "range": "± 7466",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 121674,
            "range": "± 8190",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 143283,
            "range": "± 19594",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 165542,
            "range": "± 11408",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 200251,
            "range": "± 10432",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 186249,
            "range": "± 14551",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 293161,
            "range": "± 61807",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 515389,
            "range": "± 32831",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1078152,
            "range": "± 56893",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2074089,
            "range": "± 243277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3864221,
            "range": "± 451240",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 182153,
            "range": "± 13207",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 88,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 169925,
            "range": "± 10736",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 159415,
            "range": "± 12798",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 250300,
            "range": "± 18056",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 430393,
            "range": "± 36371",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1951266,
            "range": "± 73844",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3768924,
            "range": "± 264308",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7321215,
            "range": "± 332001",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14415786,
            "range": "± 474091",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28736184,
            "range": "± 1053579",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 56533180,
            "range": "± 2395245",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111065,
            "range": "± 12474",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 92,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 126924,
            "range": "± 10835",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 143521,
            "range": "± 8577",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 163414,
            "range": "± 13577",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 208091,
            "range": "± 34974",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 187175,
            "range": "± 11003",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 302574,
            "range": "± 20567",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 518279,
            "range": "± 35467",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1039784,
            "range": "± 84783",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2040232,
            "range": "± 125316",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3805664,
            "range": "± 340665",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 135372,
            "range": "± 40025",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 24757,
            "range": "± 3792",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1693,
            "range": "± 807",
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
          "id": "6dac20da11305f59707280fa5acf8b46de014316",
          "message": "ws server: respect max limit for received messages (#537)\n\n* ws server: don't kill connection max limit exceeds\r\n\r\n* Update ws-server/src/server.rs\r\n\r\n* actually use max size in soketto\r\n\r\n* rewrite me\r\n\r\n* improve logs\r\n\r\n* use soketto fix\r\n\r\n* rewrite me\r\n\r\n* fix nit\r\n\r\n* revert unintentional change\r\n\r\n* use soketto 0.7.1\r\n\r\n* fix logger\r\n\r\n* Update ws-server/src/server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-server/src/server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-server/src/server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-server/src/server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update ws-server/src/server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* fix build\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-09T14:57:30Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/6dac20da11305f59707280fa5acf8b46de014316"
        },
        "date": 1636504012299,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 202,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 238,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 178762,
            "range": "± 16735",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 97,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 181045,
            "range": "± 12352",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 169002,
            "range": "± 17494",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 263418,
            "range": "± 27001",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 444916,
            "range": "± 41304",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2130901,
            "range": "± 110979",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 4032871,
            "range": "± 274846",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7481547,
            "range": "± 398547",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14883151,
            "range": "± 802760",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 29961876,
            "range": "± 1123083",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 58971529,
            "range": "± 2660537",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 118061,
            "range": "± 7920",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 97,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 130998,
            "range": "± 11189",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 146560,
            "range": "± 26790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 173357,
            "range": "± 29511",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 219764,
            "range": "± 31344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 197822,
            "range": "± 29318",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 318105,
            "range": "± 61507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 576675,
            "range": "± 47570",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1167214,
            "range": "± 160290",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2246883,
            "range": "± 513817",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4159477,
            "range": "± 408294",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 184322,
            "range": "± 16248",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 97,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 185757,
            "range": "± 16155",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 169702,
            "range": "± 26324",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 258260,
            "range": "± 19626",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 445427,
            "range": "± 24081",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2104293,
            "range": "± 113261",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3942484,
            "range": "± 180725",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7837974,
            "range": "± 515302",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 15083717,
            "range": "± 587031",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29976130,
            "range": "± 1103995",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 60733020,
            "range": "± 2494301",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 116695,
            "range": "± 12088",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 97,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 136451,
            "range": "± 14934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 151804,
            "range": "± 17192",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 177305,
            "range": "± 16645",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 226897,
            "range": "± 23538",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 201311,
            "range": "± 20866",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 323929,
            "range": "± 53511",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 544132,
            "range": "± 48954",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1113898,
            "range": "± 96084",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2164554,
            "range": "± 204194",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4156819,
            "range": "± 489589",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 141836,
            "range": "± 11958",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 26519,
            "range": "± 2451",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1693,
            "range": "± 699",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "f9b99ad6f29d9ed3e4e7cbd96db0ade3a50e135f",
          "message": "Re-export tracing for macros (#555)",
          "timestamp": "2021-11-10T13:55:28Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/f9b99ad6f29d9ed3e4e7cbd96db0ade3a50e135f"
        },
        "date": 1636590399321,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 202,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 234,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 189251,
            "range": "± 32684",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 96,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 178456,
            "range": "± 15831",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 173769,
            "range": "± 18159",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 258938,
            "range": "± 67251",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 432659,
            "range": "± 32754",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2124766,
            "range": "± 116765",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 4032132,
            "range": "± 205895",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7690911,
            "range": "± 307949",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 15188763,
            "range": "± 549439",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 29815761,
            "range": "± 1898552",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 59823669,
            "range": "± 2667480",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 112329,
            "range": "± 15059",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 96,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 128958,
            "range": "± 12502",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 142295,
            "range": "± 13877",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 167291,
            "range": "± 23461",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 223137,
            "range": "± 42732",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 198418,
            "range": "± 23077",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 318335,
            "range": "± 72394",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 555734,
            "range": "± 83598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1115173,
            "range": "± 113615",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2282141,
            "range": "± 2100762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4462782,
            "range": "± 603773",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 191091,
            "range": "± 19331",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 97,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 181475,
            "range": "± 21312",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 166699,
            "range": "± 23920",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 261119,
            "range": "± 20331",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 431151,
            "range": "± 36949",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2163838,
            "range": "± 177404",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 4023383,
            "range": "± 267221",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7908202,
            "range": "± 458333",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 15254920,
            "range": "± 897909",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29975356,
            "range": "± 1466757",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 60228617,
            "range": "± 3665142",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111353,
            "range": "± 14071",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 97,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 129624,
            "range": "± 12335",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 142513,
            "range": "± 18009",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 173059,
            "range": "± 16648",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 217263,
            "range": "± 25289",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 196874,
            "range": "± 28647",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 317606,
            "range": "± 68371",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 556184,
            "range": "± 97556",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1134043,
            "range": "± 116767",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2127986,
            "range": "± 170272",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4015726,
            "range": "± 438121",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 142797,
            "range": "± 20497",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 25482,
            "range": "± 2757",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1868,
            "range": "± 711",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7",
          "message": "Periodically wake `DriverSelect` so we can poll whether or not `stop` had been called. (#556)\n\n* Fix some clippy issues\r\n\r\n* Add an interval to periodically wake the SelectDriver Waker\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Naming grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-11T15:53:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7"
        },
        "date": 1636676781639,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 197,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 220,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 183653,
            "range": "± 15937",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 91,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 177198,
            "range": "± 28581",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 160325,
            "range": "± 14611",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 254339,
            "range": "± 18043",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 439948,
            "range": "± 32662",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1927070,
            "range": "± 94769",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3735045,
            "range": "± 128259",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7278758,
            "range": "± 359796",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13905927,
            "range": "± 793727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27720469,
            "range": "± 1156657",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56987975,
            "range": "± 2133732",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 113306,
            "range": "± 10923",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 90,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 124544,
            "range": "± 9327",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 139300,
            "range": "± 20224",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 163464,
            "range": "± 8311",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 206038,
            "range": "± 127034",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 190527,
            "range": "± 9885",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 315245,
            "range": "± 28157",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 581610,
            "range": "± 59103",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1110904,
            "range": "± 84056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2096386,
            "range": "± 156507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3985912,
            "range": "± 422793",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 183433,
            "range": "± 13369",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 89,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 170238,
            "range": "± 31794",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 160354,
            "range": "± 16450",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 270312,
            "range": "± 32478",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 425864,
            "range": "± 33479",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2008194,
            "range": "± 98480",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3820761,
            "range": "± 188317",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7279250,
            "range": "± 299681",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14307750,
            "range": "± 693434",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28364398,
            "range": "± 1227807",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 57355601,
            "range": "± 2187973",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111624,
            "range": "± 8280",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 96,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 123815,
            "range": "± 7423",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 137139,
            "range": "± 14277",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 162504,
            "range": "± 10137",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 208912,
            "range": "± 22961",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 188889,
            "range": "± 13612",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 311501,
            "range": "± 20581",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 540031,
            "range": "± 27855",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1086632,
            "range": "± 46185",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2139514,
            "range": "± 156911",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4158262,
            "range": "± 460091",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 136930,
            "range": "± 7847",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 25423,
            "range": "± 3065",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1103,
            "range": "± 363",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7",
          "message": "Periodically wake `DriverSelect` so we can poll whether or not `stop` had been called. (#556)\n\n* Fix some clippy issues\r\n\r\n* Add an interval to periodically wake the SelectDriver Waker\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Naming grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-11T15:53:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7"
        },
        "date": 1636763152065,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 190,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 217,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 173288,
            "range": "± 23110",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 94,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 170345,
            "range": "± 13861",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 155494,
            "range": "± 32349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 243251,
            "range": "± 21791",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 399925,
            "range": "± 17728",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1985220,
            "range": "± 164453",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3775018,
            "range": "± 202522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7261147,
            "range": "± 445903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14110660,
            "range": "± 552737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27774989,
            "range": "± 1045695",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56043636,
            "range": "± 2314876",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 107331,
            "range": "± 8183",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 95,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 127048,
            "range": "± 14585",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 137842,
            "range": "± 14193",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 163226,
            "range": "± 17187",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 205681,
            "range": "± 18349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 190334,
            "range": "± 15164",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 299290,
            "range": "± 33955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 532239,
            "range": "± 80710",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1065589,
            "range": "± 86538",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2087740,
            "range": "± 212442",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3981089,
            "range": "± 666362",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 164380,
            "range": "± 18417",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 90,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 168266,
            "range": "± 13390",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 153586,
            "range": "± 12408",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 239723,
            "range": "± 37162",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 406584,
            "range": "± 30523",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1990062,
            "range": "± 270315",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3747447,
            "range": "± 185029",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7163857,
            "range": "± 353197",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14197195,
            "range": "± 666350",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28278738,
            "range": "± 1393976",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 57040241,
            "range": "± 3118745",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 106929,
            "range": "± 76643",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 91,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 124814,
            "range": "± 10332",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 138926,
            "range": "± 15027",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 163694,
            "range": "± 17659",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 212825,
            "range": "± 49114",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 185494,
            "range": "± 39668",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 297721,
            "range": "± 14348",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 527688,
            "range": "± 62207",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1043039,
            "range": "± 70054",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2049029,
            "range": "± 224696",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3712369,
            "range": "± 373919",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 137520,
            "range": "± 9100",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 24762,
            "range": "± 1790",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1474,
            "range": "± 553",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7",
          "message": "Periodically wake `DriverSelect` so we can poll whether or not `stop` had been called. (#556)\n\n* Fix some clippy issues\r\n\r\n* Add an interval to periodically wake the SelectDriver Waker\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Naming grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-11T15:53:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7"
        },
        "date": 1636849566881,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 180,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 212,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 183923,
            "range": "± 15990",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 98,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 175917,
            "range": "± 9519",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 160725,
            "range": "± 10019",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 255971,
            "range": "± 14652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 450616,
            "range": "± 36385",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2009013,
            "range": "± 109984",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3775877,
            "range": "± 142973",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7286691,
            "range": "± 370174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14715997,
            "range": "± 671857",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 28624445,
            "range": "± 1320413",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 58642191,
            "range": "± 2812751",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 113566,
            "range": "± 9273",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 100,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 128900,
            "range": "± 11587",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 145515,
            "range": "± 12408",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 171493,
            "range": "± 18106",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 214823,
            "range": "± 20762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 203675,
            "range": "± 13609",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 333268,
            "range": "± 28104",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 576143,
            "range": "± 101228",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1163284,
            "range": "± 162082",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2266635,
            "range": "± 200378",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4026936,
            "range": "± 457679",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 185125,
            "range": "± 13267",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 177832,
            "range": "± 8624",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 163493,
            "range": "± 12017",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 251591,
            "range": "± 25900",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 436251,
            "range": "± 30005",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1987141,
            "range": "± 142887",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3707330,
            "range": "± 190925",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7329083,
            "range": "± 390782",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14338260,
            "range": "± 513643",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 28393005,
            "range": "± 1311445",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 55944504,
            "range": "± 1965250",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 110372,
            "range": "± 9509",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 97,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 127194,
            "range": "± 8611",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 144872,
            "range": "± 11322",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 169874,
            "range": "± 16163",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 210098,
            "range": "± 17339",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 196501,
            "range": "± 15022",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 326914,
            "range": "± 93326",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 565611,
            "range": "± 58030",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1128613,
            "range": "± 52609",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2175060,
            "range": "± 162663",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4092407,
            "range": "± 486390",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 140010,
            "range": "± 7351",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 24784,
            "range": "± 3106",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1665,
            "range": "± 1069",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7",
          "message": "Periodically wake `DriverSelect` so we can poll whether or not `stop` had been called. (#556)\n\n* Fix some clippy issues\r\n\r\n* Add an interval to periodically wake the SelectDriver Waker\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Naming grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-11T15:53:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7"
        },
        "date": 1636936030683,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 204,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 233,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 200370,
            "range": "± 11544",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 181672,
            "range": "± 19215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 170596,
            "range": "± 11190",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 263635,
            "range": "± 21849",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 452057,
            "range": "± 25130",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2143657,
            "range": "± 95915",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 4055660,
            "range": "± 175118",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7805634,
            "range": "± 283554",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 15339898,
            "range": "± 457602",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 30686773,
            "range": "± 908983",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 60690467,
            "range": "± 1623781",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 123994,
            "range": "± 18983",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 141779,
            "range": "± 8180",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 158773,
            "range": "± 8971",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 187069,
            "range": "± 22671",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 232444,
            "range": "± 13925",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 213552,
            "range": "± 17065",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 338092,
            "range": "± 26598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 612835,
            "range": "± 56153",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1249596,
            "range": "± 184194",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2388042,
            "range": "± 181614",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4369278,
            "range": "± 508227",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 202592,
            "range": "± 11481",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 191173,
            "range": "± 17725",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 174191,
            "range": "± 9488",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 260809,
            "range": "± 19221",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 449667,
            "range": "± 17948",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2155762,
            "range": "± 92862",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 4070009,
            "range": "± 181578",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7760866,
            "range": "± 300291",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 15498640,
            "range": "± 908076",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 30485908,
            "range": "± 1035212",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 61399211,
            "range": "± 1822791",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 127560,
            "range": "± 7774",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 143535,
            "range": "± 15618",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 158863,
            "range": "± 10079",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 184430,
            "range": "± 8895",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 230648,
            "range": "± 23636",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 206549,
            "range": "± 24925",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 342130,
            "range": "± 44919",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 592528,
            "range": "± 45473",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1209416,
            "range": "± 98486",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2332008,
            "range": "± 185140",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4369312,
            "range": "± 460398",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 148233,
            "range": "± 13570",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 27242,
            "range": "± 2538",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1964,
            "range": "± 1206",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7",
          "message": "Periodically wake `DriverSelect` so we can poll whether or not `stop` had been called. (#556)\n\n* Fix some clippy issues\r\n\r\n* Add an interval to periodically wake the SelectDriver Waker\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Naming grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-11T15:53:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7"
        },
        "date": 1637022395535,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 198,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 228,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 192412,
            "range": "± 98443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 98,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 179142,
            "range": "± 30970",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 171662,
            "range": "± 23416",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 264239,
            "range": "± 62625",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 445085,
            "range": "± 54546",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2039577,
            "range": "± 183239",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3860398,
            "range": "± 201414",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7609508,
            "range": "± 380249",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14763485,
            "range": "± 1034303",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 29811848,
            "range": "± 2064161",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 58966892,
            "range": "± 2890744",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 113256,
            "range": "± 23660",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 95,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 126606,
            "range": "± 10062",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 143873,
            "range": "± 30912",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 174266,
            "range": "± 16766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 210243,
            "range": "± 30539",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 200132,
            "range": "± 18150",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 319274,
            "range": "± 79668",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 543212,
            "range": "± 59442",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1107195,
            "range": "± 91617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2160985,
            "range": "± 194911",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4076774,
            "range": "± 479785",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 194725,
            "range": "± 15478",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 97,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 181697,
            "range": "± 24967",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 164969,
            "range": "± 17636",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 261061,
            "range": "± 23841",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 451810,
            "range": "± 40273",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2055229,
            "range": "± 217216",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3797572,
            "range": "± 223823",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7618696,
            "range": "± 513610",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14838242,
            "range": "± 885032",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29007955,
            "range": "± 1506192",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 58272458,
            "range": "± 2974273",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 112112,
            "range": "± 19614",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 98,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 127417,
            "range": "± 6587",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 144611,
            "range": "± 9409",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 174356,
            "range": "± 61734",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 216710,
            "range": "± 37002",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 194475,
            "range": "± 31892",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 306684,
            "range": "± 38946",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 539160,
            "range": "± 52636",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1100395,
            "range": "± 133648",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2170384,
            "range": "± 224026",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4030234,
            "range": "± 503850",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 146746,
            "range": "± 16643",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 26230,
            "range": "± 4243",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1718,
            "range": "± 1256",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Maciej Hirsz",
            "username": "maciejhirsz",
            "email": "1096222+maciejhirsz@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7",
          "message": "Periodically wake `DriverSelect` so we can poll whether or not `stop` had been called. (#556)\n\n* Fix some clippy issues\r\n\r\n* Add an interval to periodically wake the SelectDriver Waker\r\n\r\n* Apply suggestions from code review\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Naming grumbles\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-11T15:53:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/aacf7c0ecdb71da345e7c5cb0283f5cb5a040bd7"
        },
        "date": 1637108756488,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 181,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 204,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 184630,
            "range": "± 11368",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 92,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 178061,
            "range": "± 20144",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 154493,
            "range": "± 12902",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 245919,
            "range": "± 19633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 418444,
            "range": "± 61250",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2095128,
            "range": "± 223802",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3793270,
            "range": "± 232340",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7363401,
            "range": "± 332904",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14076687,
            "range": "± 710159",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27631186,
            "range": "± 1311612",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56982499,
            "range": "± 3135455",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 112337,
            "range": "± 7631",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 90,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 124997,
            "range": "± 8874",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 142776,
            "range": "± 11725",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 165043,
            "range": "± 13302",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 209418,
            "range": "± 13781",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 187670,
            "range": "± 22991",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 303331,
            "range": "± 20313",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 530657,
            "range": "± 62656",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1085531,
            "range": "± 70060",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2157597,
            "range": "± 151773",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4000300,
            "range": "± 392388",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 180003,
            "range": "± 18763",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 90,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 168322,
            "range": "± 8536",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 157381,
            "range": "± 8935",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 243097,
            "range": "± 20895",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 406906,
            "range": "± 32995",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1949989,
            "range": "± 121418",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3685652,
            "range": "± 178078",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7078891,
            "range": "± 348500",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13930364,
            "range": "± 519505",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 27678028,
            "range": "± 1245101",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 55393057,
            "range": "± 2614586",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111566,
            "range": "± 7618",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 91,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 126254,
            "range": "± 7133",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 143059,
            "range": "± 9003",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 172626,
            "range": "± 15679",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 226517,
            "range": "± 79118",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 192901,
            "range": "± 16745",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 303202,
            "range": "± 41509",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 541821,
            "range": "± 33993",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1064585,
            "range": "± 96102",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2065995,
            "range": "± 179160",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3900368,
            "range": "± 419100",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 138388,
            "range": "± 13154",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 23662,
            "range": "± 3611",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1710,
            "range": "± 660",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "David",
            "username": "dvdplm",
            "email": "dvdplm@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "6af6db24b9f92e7f9ce1232d4f667f7d36db583a",
          "message": "Implement SubscriptionClient for HttpClient (#563)\n\nCloses https://github.com/paritytech/jsonrpsee/issues/448\r\n\r\nThis PR adds an implementation for `SubscriptionClient` to the `HttpClient` struct, which makes it possible for http clients to use macro-generated RPC servers. If an http client tries to set up a subscription it will fail with a `HttpNotImplemented` error.",
          "timestamp": "2021-11-17T13:53:27Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/6af6db24b9f92e7f9ce1232d4f667f7d36db583a"
        },
        "date": 1637195251129,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 195,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 220,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 187886,
            "range": "± 15438",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 93,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 183743,
            "range": "± 27401",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 171085,
            "range": "± 30979",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 269832,
            "range": "± 203087",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 445069,
            "range": "± 45207",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1995232,
            "range": "± 190948",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3668715,
            "range": "± 280351",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7207602,
            "range": "± 478169",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13990917,
            "range": "± 475301",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27810268,
            "range": "± 1096429",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 55913815,
            "range": "± 2549271",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 122871,
            "range": "± 15822",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 92,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 132682,
            "range": "± 19323",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 161252,
            "range": "± 14887",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 182268,
            "range": "± 10147",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 221981,
            "range": "± 33258",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 204941,
            "range": "± 47347",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 330015,
            "range": "± 80786",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 581727,
            "range": "± 133620",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1144735,
            "range": "± 98839",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2194517,
            "range": "± 267125",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4412401,
            "range": "± 831003",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 189164,
            "range": "± 14804",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 92,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 184920,
            "range": "± 202773",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 164378,
            "range": "± 8243",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 261098,
            "range": "± 19322",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 449408,
            "range": "± 33868",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1970072,
            "range": "± 150372",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3679862,
            "range": "± 167255",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7121902,
            "range": "± 260735",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13955204,
            "range": "± 434363",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 27622134,
            "range": "± 1237400",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 55811419,
            "range": "± 1904312",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 118803,
            "range": "± 15909",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 92,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 129350,
            "range": "± 11189",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 158134,
            "range": "± 18214",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 177651,
            "range": "± 31487",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 210062,
            "range": "± 17877",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 201239,
            "range": "± 19488",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 325287,
            "range": "± 42773",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 546574,
            "range": "± 33359",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1120714,
            "range": "± 117616",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2178586,
            "range": "± 165392",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4458554,
            "range": "± 548830",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 154283,
            "range": "± 35104",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 25115,
            "range": "± 3957",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1455,
            "range": "± 362",
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
          "id": "0e46b5cea9cd632dc438a005c77bbaa5c2af562f",
          "message": "[rpc module]: improve `TestSubscription` to return `None` when closed (#566)\n\n* fix(TestSubscription): use None for closed.\r\n\r\n* add test for subscription close",
          "timestamp": "2021-11-18T11:03:57Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/0e46b5cea9cd632dc438a005c77bbaa5c2af562f"
        },
        "date": 1637281595035,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 191,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 218,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 188271,
            "range": "± 16792",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 190995,
            "range": "± 19901",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 168978,
            "range": "± 14778",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 262868,
            "range": "± 21614",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 441773,
            "range": "± 26128",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 2089520,
            "range": "± 153737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3878102,
            "range": "± 184792",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7467819,
            "range": "± 284020",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14787906,
            "range": "± 842389",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 28877216,
            "range": "± 1066965",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 58591985,
            "range": "± 3030260",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 119205,
            "range": "± 10182",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 104,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 137268,
            "range": "± 9329",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 160814,
            "range": "± 13301",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 179702,
            "range": "± 11822",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 222929,
            "range": "± 16070",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 201465,
            "range": "± 12366",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 327575,
            "range": "± 58695",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 563309,
            "range": "± 33019",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1128317,
            "range": "± 156572",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2172089,
            "range": "± 198151",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4193193,
            "range": "± 503915",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 189252,
            "range": "± 10840",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 104,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 181413,
            "range": "± 50932",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 173130,
            "range": "± 30428",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 262626,
            "range": "± 18349",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 452429,
            "range": "± 66441",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 2019621,
            "range": "± 132925",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3867754,
            "range": "± 212481",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7616332,
            "range": "± 352382",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14868848,
            "range": "± 547442",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29145874,
            "range": "± 1070076",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 57190019,
            "range": "± 2797455",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 122873,
            "range": "± 8918",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 140740,
            "range": "± 12094",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 152427,
            "range": "± 10463",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 185548,
            "range": "± 37361",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 232502,
            "range": "± 21764",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 206729,
            "range": "± 39787",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 329458,
            "range": "± 56476",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 568242,
            "range": "± 65115",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1157446,
            "range": "± 75012",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2156394,
            "range": "± 170268",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4088361,
            "range": "± 344284",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 156573,
            "range": "± 30390",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 21093,
            "range": "± 3987",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1779,
            "range": "± 1187",
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
          "id": "9c6fd4bfee44aec6ebb10dae0fb2779562ecf125",
          "message": "feat: make it possible to override `method name` in subscriptions (#568)\n\n* feat: override `method` subscription notif\r\n\r\n* Arrow syntax for overwrites (#569)\r\n\r\n* check that unique notifs are used\r\n\r\n* check that custom sub name is unique\r\n\r\n* cargo fmt\r\n\r\n* address grumbles\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\n* commit added tests\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update utils/src/server/rpc_module.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* grumbles\r\n\r\n* fix long lines\r\n\r\n* Update utils/src/server/rpc_module.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update utils/src/server/rpc_module.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* more grumbles\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-19T18:30:47Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9c6fd4bfee44aec6ebb10dae0fb2779562ecf125"
        },
        "date": 1637367817828,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 134,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 159,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 135257,
            "range": "± 12587",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 108989,
            "range": "± 4437",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 96118,
            "range": "± 3439",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 163693,
            "range": "± 3317",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 259760,
            "range": "± 12617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1692832,
            "range": "± 106717",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3304507,
            "range": "± 27286",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6416314,
            "range": "± 316240",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12878032,
            "range": "± 646541",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 23101203,
            "range": "± 678737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 45556645,
            "range": "± 358918",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 68759,
            "range": "± 3297",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 85182,
            "range": "± 3238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 94339,
            "range": "± 3535",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 112823,
            "range": "± 1299",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 164614,
            "range": "± 1277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 129345,
            "range": "± 5936",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 219782,
            "range": "± 5014",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 383303,
            "range": "± 16587",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 793115,
            "range": "± 28048",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1599423,
            "range": "± 74841",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2975907,
            "range": "± 177904",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 130509,
            "range": "± 11880",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 111671,
            "range": "± 1792",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 96936,
            "range": "± 1847",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 163984,
            "range": "± 4585",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 284807,
            "range": "± 5213",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1690689,
            "range": "± 79594",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3295327,
            "range": "± 160539",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6519235,
            "range": "± 112514",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11535964,
            "range": "± 182342",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 22723519,
            "range": "± 233240",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 45428849,
            "range": "± 326950",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 63729,
            "range": "± 2363",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 72,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 84509,
            "range": "± 1821",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 94544,
            "range": "± 2619",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 114906,
            "range": "± 1892",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 165358,
            "range": "± 1674",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 130565,
            "range": "± 3540",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 217687,
            "range": "± 3118",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 388627,
            "range": "± 14514",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 822630,
            "range": "± 26801",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1569031,
            "range": "± 57213",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2952099,
            "range": "± 243021",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 92908,
            "range": "± 2176",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 14155,
            "range": "± 2050",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1064,
            "range": "± 124",
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
          "id": "9c6fd4bfee44aec6ebb10dae0fb2779562ecf125",
          "message": "feat: make it possible to override `method name` in subscriptions (#568)\n\n* feat: override `method` subscription notif\r\n\r\n* Arrow syntax for overwrites (#569)\r\n\r\n* check that unique notifs are used\r\n\r\n* check that custom sub name is unique\r\n\r\n* cargo fmt\r\n\r\n* address grumbles\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\n* commit added tests\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update utils/src/server/rpc_module.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* grumbles\r\n\r\n* fix long lines\r\n\r\n* Update utils/src/server/rpc_module.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update utils/src/server/rpc_module.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/rpc_macro.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update proc-macros/src/render_server.rs\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* more grumbles\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>\r\nCo-authored-by: David <dvdplm@gmail.com>",
          "timestamp": "2021-11-19T18:30:47Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9c6fd4bfee44aec6ebb10dae0fb2779562ecf125"
        },
        "date": 1637454322327,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 157,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 181,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 162853,
            "range": "± 14864",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 78,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 157459,
            "range": "± 25249",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 141711,
            "range": "± 8210",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 222380,
            "range": "± 9034",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 383476,
            "range": "± 31860",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1826625,
            "range": "± 107156",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3415086,
            "range": "± 158441",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6592905,
            "range": "± 304135",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13187360,
            "range": "± 773379",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 25509441,
            "range": "± 1070232",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 52107786,
            "range": "± 2579614",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 105376,
            "range": "± 8220",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 77,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 114849,
            "range": "± 6498",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 139199,
            "range": "± 25677",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 151804,
            "range": "± 17643",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 184453,
            "range": "± 16160",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 166653,
            "range": "± 16031",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 275484,
            "range": "± 21151",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 537568,
            "range": "± 44672",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1038457,
            "range": "± 63350",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1761400,
            "range": "± 217270",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3396936,
            "range": "± 371896",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 163661,
            "range": "± 12604",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 77,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 149873,
            "range": "± 7777",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 141020,
            "range": "± 14939",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 223464,
            "range": "± 11122",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 379052,
            "range": "± 19019",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1802315,
            "range": "± 96952",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3492926,
            "range": "± 166180",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6488774,
            "range": "± 374230",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 12912311,
            "range": "± 727826",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 25037655,
            "range": "± 1114367",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48055596,
            "range": "± 1774439",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 104000,
            "range": "± 26619",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 77,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 115248,
            "range": "± 12730",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 129177,
            "range": "± 27018",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 152025,
            "range": "± 12919",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 183432,
            "range": "± 11701",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 170765,
            "range": "± 23087",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 271140,
            "range": "± 23115",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 449095,
            "range": "± 19603",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 920579,
            "range": "± 79112",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1788433,
            "range": "± 127998",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3415206,
            "range": "± 348042",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 136350,
            "range": "± 6171",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 20763,
            "range": "± 3385",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2056,
            "range": "± 635",
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
          "id": "9a3c1e981bcdbbb558b1457bbd78277a14dca2da",
          "message": "proc-macros: Support deprecated methods for rpc client (#570)\n\n* proc-macros: Fix documentation typo of `rpc_identifier`\r\n\r\n* proc-macros: Support deprecated methods for rpc client (#564)\r\n\r\nCalling a deprecated method of the RPC client should warn\r\nthe user at compile-time.\r\n\r\nExtract the `#[deprecated]` macro as is while parsing the\r\nRpcMethod, and pass through the macro to the RPC client\r\nrendering.\r\n\r\n* tests/ui: Check deprecated method for rpc client (#564)\r\n\r\nTo ensure that the test will fail during compilation,\r\nwarnings are denied.\r\n\r\nCheck that the deprecate macro will generate warnings\r\njust for the methods that are utilized.",
          "timestamp": "2021-11-21T14:20:50Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9a3c1e981bcdbbb558b1457bbd78277a14dca2da"
        },
        "date": 1637540662689,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 149,
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
            "name": "sync/http_round_trip",
            "value": 133819,
            "range": "± 12352",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 111351,
            "range": "± 1739",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 103728,
            "range": "± 12847",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 164805,
            "range": "± 4905",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 284178,
            "range": "± 2217",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1603383,
            "range": "± 21158",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3128342,
            "range": "± 26879",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6149978,
            "range": "± 48697",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 12147544,
            "range": "± 105478",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 24120850,
            "range": "± 249089",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 47987436,
            "range": "± 392381",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 69875,
            "range": "± 5497",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 81,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 85815,
            "range": "± 2074",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 95283,
            "range": "± 2030",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 116640,
            "range": "± 4343",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 166156,
            "range": "± 2985",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 132354,
            "range": "± 1699",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 222551,
            "range": "± 13084",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 397993,
            "range": "± 8610",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 834106,
            "range": "± 67443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1551237,
            "range": "± 89194",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2940183,
            "range": "± 217215",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 115687,
            "range": "± 13979",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 70,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 107426,
            "range": "± 1611",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 99572,
            "range": "± 2512",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 167222,
            "range": "± 1847",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 282586,
            "range": "± 3470",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1622746,
            "range": "± 11886",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3172366,
            "range": "± 34904",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6233930,
            "range": "± 257564",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11052683,
            "range": "± 298730",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 21798210,
            "range": "± 391564",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 48815521,
            "range": "± 776277",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65416,
            "range": "± 4409",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 71,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 85541,
            "range": "± 6258",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 95998,
            "range": "± 2138",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 116932,
            "range": "± 1490",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 167165,
            "range": "± 1218",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 132727,
            "range": "± 2275",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 222218,
            "range": "± 3643",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 398767,
            "range": "± 10149",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 851477,
            "range": "± 15489",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1637430,
            "range": "± 82201",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3101940,
            "range": "± 245061",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 95350,
            "range": "± 2074",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 16553,
            "range": "± 1253",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1105,
            "range": "± 260",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "dependabot[bot]",
            "username": "dependabot[bot]",
            "email": "49699333+dependabot[bot]@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "e19e5051145c89f86ea02d01f52800cce9d1a516",
          "message": "Update hyper-rustls requirement from 0.22 to 0.23 (#571)\n\n* Update hyper-rustls requirement from 0.22 to 0.23\r\n\r\nUpdates the requirements on [hyper-rustls](https://github.com/ctz/hyper-rustls) to permit the latest version.\r\n- [Release notes](https://github.com/ctz/hyper-rustls/releases)\r\n- [Commits](https://github.com/ctz/hyper-rustls/compare/v/0.22.0...v/0.23.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: hyper-rustls\r\n  dependency-type: direct:production\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\n* make it work\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2021-11-22T13:57:06Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/e19e5051145c89f86ea02d01f52800cce9d1a516"
        },
        "date": 1637627298512,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 201,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 227,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 197575,
            "range": "± 159330",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 97,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 188840,
            "range": "± 28427",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 171594,
            "range": "± 18933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 256703,
            "range": "± 30255",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 433209,
            "range": "± 43464",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1801108,
            "range": "± 248239",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3284182,
            "range": "± 6862385",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6039286,
            "range": "± 2667777",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11771286,
            "range": "± 705857",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 22930068,
            "range": "± 8713585",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 46541946,
            "range": "± 22823541",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 114998,
            "range": "± 19536",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 97,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 137165,
            "range": "± 15338",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 148676,
            "range": "± 22675",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 173610,
            "range": "± 36206",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 222142,
            "range": "± 78263",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 200177,
            "range": "± 185355",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 313863,
            "range": "± 65323",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 544785,
            "range": "± 57990",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1098318,
            "range": "± 76649",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2149432,
            "range": "± 204720",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 4062892,
            "range": "± 576164",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 194035,
            "range": "± 22099",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 98,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 193772,
            "range": "± 339120",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 177410,
            "range": "± 106487",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 256934,
            "range": "± 185773",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 441632,
            "range": "± 28713",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1748863,
            "range": "± 270363",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3116361,
            "range": "± 198823",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5871916,
            "range": "± 602357",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11712625,
            "range": "± 816854",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 22548158,
            "range": "± 1243845",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 49323179,
            "range": "± 78612002",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 120676,
            "range": "± 146446",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 95,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 134073,
            "range": "± 13642",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 144912,
            "range": "± 75573",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 168746,
            "range": "± 13095",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 217800,
            "range": "± 28919",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 196739,
            "range": "± 328742",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 307303,
            "range": "± 316157",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 548371,
            "range": "± 139544",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1061095,
            "range": "± 79258",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2053973,
            "range": "± 209994",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 4090322,
            "range": "± 2113723",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 161104,
            "range": "± 119924",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 19709,
            "range": "± 30643",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1989,
            "range": "± 799",
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
          "id": "085df4144e87be2a0ec547d12cbe390d90a8b038",
          "message": "fix: better log for failed unsubscription call (#575)",
          "timestamp": "2021-11-23T19:24:24Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/085df4144e87be2a0ec547d12cbe390d90a8b038"
        },
        "date": 1637713624593,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 176,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 202,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 163296,
            "range": "± 20198",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 84,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 167667,
            "range": "± 15293",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 157943,
            "range": "± 10902",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 236350,
            "range": "± 36586",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 420509,
            "range": "± 45933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1520508,
            "range": "± 71763",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2868665,
            "range": "± 313160",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5440761,
            "range": "± 249570",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 10631478,
            "range": "± 467366",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 21066876,
            "range": "± 799757",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 41699977,
            "range": "± 1671255",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 99554,
            "range": "± 19296",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 80,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 115721,
            "range": "± 12999",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 129171,
            "range": "± 17722",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 152803,
            "range": "± 12351",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 206929,
            "range": "± 95056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 185478,
            "range": "± 33692",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 297475,
            "range": "± 40243",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 542498,
            "range": "± 67774",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1097008,
            "range": "± 72496",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2060789,
            "range": "± 259514",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3736493,
            "range": "± 437662",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 175398,
            "range": "± 13531",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 83,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 166535,
            "range": "± 12249",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 171191,
            "range": "± 25669",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 242666,
            "range": "± 29733",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 405287,
            "range": "± 20556",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1539350,
            "range": "± 80755",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2858602,
            "range": "± 144547",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5429440,
            "range": "± 301847",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 10420112,
            "range": "± 574294",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 20832385,
            "range": "± 805006",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 42200206,
            "range": "± 1686585",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 106140,
            "range": "± 15255",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 84,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 122148,
            "range": "± 13211",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 139377,
            "range": "± 6447",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 172481,
            "range": "± 28363",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 200975,
            "range": "± 12505",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 186751,
            "range": "± 20768",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 301589,
            "range": "± 42147",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 520096,
            "range": "± 95176",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1063921,
            "range": "± 108542",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2050194,
            "range": "± 200025",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3663076,
            "range": "± 426487",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 140135,
            "range": "± 11387",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 16082,
            "range": "± 4445",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1636,
            "range": "± 353",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "David",
            "username": "dvdplm",
            "email": "dvdplm@gmail.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "42ffbcc608afce97af4e8b394fb9d31920888346",
          "message": "[chore] Release v0.5 (#574)\n\n* Bump version –> 0.5\r\nFix try-build tests\r\n\r\n* Changelog\r\n\r\n* Update CHANGELOG.md\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* Update CHANGELOG.md\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2021-11-24T09:54:16Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/42ffbcc608afce97af4e8b394fb9d31920888346"
        },
        "date": 1637799851669,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 149,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 148,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 133385,
            "range": "± 5123",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 79,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 112382,
            "range": "± 1535",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 105276,
            "range": "± 2169",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 163851,
            "range": "± 3758",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 283019,
            "range": "± 6010",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1367659,
            "range": "± 29102",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2600571,
            "range": "± 12439",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5066514,
            "range": "± 90977",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 10000284,
            "range": "± 96614",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 19771100,
            "range": "± 143719",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 39468817,
            "range": "± 200938",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 67031,
            "range": "± 2560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 84410,
            "range": "± 1806",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 94346,
            "range": "± 2349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 115687,
            "range": "± 1289",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 164365,
            "range": "± 1681",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 131296,
            "range": "± 7308",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 219753,
            "range": "± 6320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 387131,
            "range": "± 7458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 814259,
            "range": "± 15490",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1583305,
            "range": "± 53510",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2999059,
            "range": "± 258150",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 133950,
            "range": "± 2888",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 112319,
            "range": "± 1518",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 105354,
            "range": "± 1782",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 164208,
            "range": "± 4911",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 281967,
            "range": "± 4372",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1368793,
            "range": "± 7581",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2609705,
            "range": "± 11802",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5088684,
            "range": "± 23397",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 10009077,
            "range": "± 79080",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 19679699,
            "range": "± 181975",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 39472463,
            "range": "± 204060",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 66220,
            "range": "± 2539",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 83959,
            "range": "± 1588",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 94622,
            "range": "± 1565",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 114513,
            "range": "± 2319",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 164138,
            "range": "± 1767",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 131852,
            "range": "± 4285",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 220746,
            "range": "± 3630",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 388086,
            "range": "± 10145",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 821368,
            "range": "± 18376",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1569145,
            "range": "± 57967",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2967958,
            "range": "± 231013",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 91643,
            "range": "± 1986",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 12715,
            "range": "± 1394",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1220,
            "range": "± 193",
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
          "id": "d4e53f83c06bc2a477735f7cd9b6e18f311787dd",
          "message": "fix rpc error: support unquoted strings (#578)\n\n* fix rpc error: support unquoted strings\r\n\r\n* Update types/src/v2/error.rs\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>\r\n\r\nCo-authored-by: Maciej Hirsz <1096222+maciejhirsz@users.noreply.github.com>",
          "timestamp": "2021-11-25T19:15:57Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/d4e53f83c06bc2a477735f7cd9b6e18f311787dd"
        },
        "date": 1637886313413,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 171,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 133301,
            "range": "± 2644",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 110572,
            "range": "± 1679",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 104299,
            "range": "± 2296",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 165783,
            "range": "± 12511",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 279443,
            "range": "± 3372",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1351319,
            "range": "± 35639",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2597861,
            "range": "± 48454",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5013516,
            "range": "± 43016",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 9883980,
            "range": "± 110083",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 19797178,
            "range": "± 198638",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 39359737,
            "range": "± 277467",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 66137,
            "range": "± 2714",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 82148,
            "range": "± 2277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 93740,
            "range": "± 6299",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 114081,
            "range": "± 1320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 167664,
            "range": "± 2563",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 131130,
            "range": "± 3754",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 217677,
            "range": "± 4934",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 391514,
            "range": "± 24552",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 811795,
            "range": "± 20165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1574416,
            "range": "± 162021",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2961331,
            "range": "± 283808",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 132646,
            "range": "± 2799",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 167130,
            "range": "± 38294",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 103289,
            "range": "± 3223",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 170074,
            "range": "± 43732",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 426078,
            "range": "± 8112",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1353058,
            "range": "± 12016",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2564869,
            "range": "± 23797",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5008208,
            "range": "± 36935",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 9899763,
            "range": "± 535781",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 19378073,
            "range": "± 132160",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 39094318,
            "range": "± 362667",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65604,
            "range": "± 2312",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 79,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 82778,
            "range": "± 2447",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 94317,
            "range": "± 1874",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 114450,
            "range": "± 1562",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 170599,
            "range": "± 130966",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 133015,
            "range": "± 35751",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 232311,
            "range": "± 16372",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 406283,
            "range": "± 42844",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 897985,
            "range": "± 123209",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1741745,
            "range": "± 84294",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3493230,
            "range": "± 380312",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 101201,
            "range": "± 24750",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 16270,
            "range": "± 1633",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1698,
            "range": "± 442",
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
          "id": "8c8676999ea3ddc74ef907b1f27858405750c17f",
          "message": "chore: release v0.5.1 (#579)",
          "timestamp": "2021-11-26T08:41:25Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/8c8676999ea3ddc74ef907b1f27858405750c17f"
        },
        "date": 1637972660872,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 168,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 135134,
            "range": "± 8425",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 114166,
            "range": "± 1643",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 105056,
            "range": "± 4026",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 168727,
            "range": "± 3517",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 286853,
            "range": "± 7891",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1381774,
            "range": "± 29182",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2630665,
            "range": "± 13954",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5103994,
            "range": "± 27516",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 10073188,
            "range": "± 67105",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 19918668,
            "range": "± 172111",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 39666916,
            "range": "± 684147",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 69695,
            "range": "± 6700",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 87917,
            "range": "± 1944",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 101265,
            "range": "± 3123",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 120712,
            "range": "± 1876",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 170468,
            "range": "± 2427",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 133484,
            "range": "± 3490",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 216684,
            "range": "± 4640",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 400137,
            "range": "± 16266",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 808887,
            "range": "± 16818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1577615,
            "range": "± 63788",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3226992,
            "range": "± 309583",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 135626,
            "range": "± 4213",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 114219,
            "range": "± 1715",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 104968,
            "range": "± 2367",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 165536,
            "range": "± 3531",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 288978,
            "range": "± 8320",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1377629,
            "range": "± 9137",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2632475,
            "range": "± 13892",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5098699,
            "range": "± 31019",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 9998717,
            "range": "± 89129",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 19828900,
            "range": "± 291713",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 39484741,
            "range": "± 243451",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67407,
            "range": "± 2111",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 80,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 87552,
            "range": "± 2010",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 97698,
            "range": "± 2201",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 118456,
            "range": "± 2473",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 183477,
            "range": "± 14281",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 133452,
            "range": "± 6649",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 223536,
            "range": "± 3251",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 402715,
            "range": "± 32252",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 843680,
            "range": "± 17591",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1624026,
            "range": "± 50275",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3103390,
            "range": "± 307374",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 91558,
            "range": "± 1786",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 15822,
            "range": "± 1458",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1104,
            "range": "± 160",
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
          "id": "8c8676999ea3ddc74ef907b1f27858405750c17f",
          "message": "chore: release v0.5.1 (#579)",
          "timestamp": "2021-11-26T08:41:25Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/8c8676999ea3ddc74ef907b1f27858405750c17f"
        },
        "date": 1638059215580,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 176,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 199,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 154184,
            "range": "± 6502",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 94,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 131197,
            "range": "± 5172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 121803,
            "range": "± 3115",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 182515,
            "range": "± 7696",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 321594,
            "range": "± 12100",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1548863,
            "range": "± 46674",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2932562,
            "range": "± 83871",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5765408,
            "range": "± 276925",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11709492,
            "range": "± 396010",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 23376995,
            "range": "± 865332",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 46226741,
            "range": "± 2459653",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 73879,
            "range": "± 3315",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 93,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 87603,
            "range": "± 4422",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 108817,
            "range": "± 6814",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 126243,
            "range": "± 10986",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 182073,
            "range": "± 4679",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 144311,
            "range": "± 13707",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 237166,
            "range": "± 10646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 416428,
            "range": "± 39738",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 845424,
            "range": "± 36595",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1676243,
            "range": "± 125456",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3154950,
            "range": "± 224921",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 145753,
            "range": "± 8862",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 91,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 131286,
            "range": "± 15705",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 122463,
            "range": "± 5159",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 193187,
            "range": "± 3665",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 326183,
            "range": "± 16103",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1607356,
            "range": "± 53418",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3033272,
            "range": "± 82837",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5926815,
            "range": "± 256463",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11591512,
            "range": "± 174875",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 22977794,
            "range": "± 848712",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 46388191,
            "range": "± 672210",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 80687,
            "range": "± 6702",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 93,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 90263,
            "range": "± 7453",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 110764,
            "range": "± 3987",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 124206,
            "range": "± 3692",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 178052,
            "range": "± 4670",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 146203,
            "range": "± 3837",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 242374,
            "range": "± 8536",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 418085,
            "range": "± 16772",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 887441,
            "range": "± 40138",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1704010,
            "range": "± 98087",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3191180,
            "range": "± 254095",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 98034,
            "range": "± 4877",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4940,
            "range": "± 2305",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2850,
            "range": "± 667",
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
          "id": "8c8676999ea3ddc74ef907b1f27858405750c17f",
          "message": "chore: release v0.5.1 (#579)",
          "timestamp": "2021-11-26T08:41:25Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/8c8676999ea3ddc74ef907b1f27858405750c17f"
        },
        "date": 1638145451817,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 137,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 177,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 132063,
            "range": "± 12979",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 111928,
            "range": "± 2385",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 105042,
            "range": "± 2831",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 163550,
            "range": "± 4526",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 281336,
            "range": "± 4534",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1356530,
            "range": "± 17423",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2375840,
            "range": "± 69953",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 4605271,
            "range": "± 228034",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 9772388,
            "range": "± 483233",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 19626078,
            "range": "± 182909",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 35193180,
            "range": "± 284847",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 64585,
            "range": "± 1512",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 81173,
            "range": "± 3898",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 92578,
            "range": "± 2322",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 113489,
            "range": "± 4844",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 166070,
            "range": "± 2760",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 130987,
            "range": "± 5226",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 221050,
            "range": "± 3978",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 389286,
            "range": "± 13534",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 778836,
            "range": "± 23612",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1579285,
            "range": "± 57926",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3021929,
            "range": "± 224992",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 134630,
            "range": "± 2224",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 111486,
            "range": "± 1054",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 98224,
            "range": "± 4269",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 166383,
            "range": "± 3083",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 280602,
            "range": "± 3396",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1353531,
            "range": "± 39217",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2609399,
            "range": "± 41870",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5059399,
            "range": "± 55044",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 9947223,
            "range": "± 111891",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 19732561,
            "range": "± 334410",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 38547331,
            "range": "± 1865977",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65558,
            "range": "± 1989",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 84,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 83323,
            "range": "± 3700",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 93276,
            "range": "± 1071",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 114717,
            "range": "± 1281",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 164898,
            "range": "± 1777",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 129898,
            "range": "± 2770",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 220547,
            "range": "± 3515",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 387214,
            "range": "± 8783",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 807017,
            "range": "± 14083",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1587375,
            "range": "± 71295",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2921127,
            "range": "± 255058",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 88537,
            "range": "± 2770",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 11479,
            "range": "± 3597",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1156,
            "range": "± 135",
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
          "id": "15b2f23858b06b6162b6821a7bbf0086f68c5eba",
          "message": "fix(http client): impl Clone (#583)",
          "timestamp": "2021-11-29T21:30:34Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/15b2f23858b06b6162b6821a7bbf0086f68c5eba"
        },
        "date": 1638231992742,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 191,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 213,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 164104,
            "range": "± 6783",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 138488,
            "range": "± 2657",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 126270,
            "range": "± 4578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 195157,
            "range": "± 8906",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 335959,
            "range": "± 6254",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1668047,
            "range": "± 68970",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3121566,
            "range": "± 99946",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6039432,
            "range": "± 66474",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11763671,
            "range": "± 107824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 23323444,
            "range": "± 201785",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 46472493,
            "range": "± 338606",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 81777,
            "range": "± 5918",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 103415,
            "range": "± 2576",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 115582,
            "range": "± 4462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 127420,
            "range": "± 5399",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 187573,
            "range": "± 3440",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 152020,
            "range": "± 11564",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 256978,
            "range": "± 19779",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 449466,
            "range": "± 23869",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 943551,
            "range": "± 21548",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1858448,
            "range": "± 102888",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3483598,
            "range": "± 268427",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 162505,
            "range": "± 7546",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 138373,
            "range": "± 42849",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 125370,
            "range": "± 1985",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 192592,
            "range": "± 4745",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 421969,
            "range": "± 84720",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1635061,
            "range": "± 52424",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3070303,
            "range": "± 56785",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5914360,
            "range": "± 220300",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11732871,
            "range": "± 120690",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 23156426,
            "range": "± 401798",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 46104917,
            "range": "± 537824",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 82982,
            "range": "± 4056",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 100,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 102923,
            "range": "± 10252",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 115555,
            "range": "± 3526",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 131061,
            "range": "± 3167",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 187669,
            "range": "± 3573",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 150430,
            "range": "± 5469",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 254250,
            "range": "± 9894",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 445096,
            "range": "± 9163",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 942379,
            "range": "± 30356",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1876783,
            "range": "± 109135",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3508872,
            "range": "± 217132",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 112596,
            "range": "± 4670",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 8094,
            "range": "± 1965",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3497,
            "range": "± 742",
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
          "id": "3c3f3ac9b6c12e81a39e845b898b085b9580b84e",
          "message": "fix(types): use `Cow` for deserializing `str` (#584)\n\n* fix(types): use `Cow` for deserializing `str`\r\n\r\n* use ToString",
          "timestamp": "2021-11-30T13:21:10Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/3c3f3ac9b6c12e81a39e845b898b085b9580b84e"
        },
        "date": 1638318382840,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 183,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 209,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 152684,
            "range": "± 6665",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 100,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 132089,
            "range": "± 2903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 121066,
            "range": "± 2706",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 187747,
            "range": "± 4109",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 317555,
            "range": "± 5633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1576735,
            "range": "± 29064",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2966404,
            "range": "± 68524",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5767342,
            "range": "± 161590",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11340806,
            "range": "± 184583",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 22046575,
            "range": "± 412610",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 43266956,
            "range": "± 1057522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 73267,
            "range": "± 2063",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 95,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 87924,
            "range": "± 4078",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 101974,
            "range": "± 5124",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 118685,
            "range": "± 2931",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 173232,
            "range": "± 5408",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 142928,
            "range": "± 4015",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 234811,
            "range": "± 6732",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 418274,
            "range": "± 16056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 870913,
            "range": "± 69543",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1681567,
            "range": "± 81993",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3213732,
            "range": "± 279144",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 149763,
            "range": "± 6024",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 95,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 127734,
            "range": "± 3169",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 117087,
            "range": "± 3843",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 182324,
            "range": "± 6036",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 308566,
            "range": "± 7794",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1554814,
            "range": "± 40333",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2953308,
            "range": "± 71090",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5708868,
            "range": "± 127581",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11148460,
            "range": "± 256602",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 22424884,
            "range": "± 433832",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 44340382,
            "range": "± 779200",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 73027,
            "range": "± 2593",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 96,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 87691,
            "range": "± 4115",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 104479,
            "range": "± 4246",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 117804,
            "range": "± 5913",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 173426,
            "range": "± 5537",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 138752,
            "range": "± 3949",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 229208,
            "range": "± 5527",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 402718,
            "range": "± 14167",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 835494,
            "range": "± 26251",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1593705,
            "range": "± 75716",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3102776,
            "range": "± 276506",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 95857,
            "range": "± 2608",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 5305,
            "range": "± 1353",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2417,
            "range": "± 429",
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
          "id": "be6f64ae65baf5ad1a5a0de8487aaf3407d39c5f",
          "message": "chore: release v0.6.0 (#587)",
          "timestamp": "2021-12-01T11:41:26Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/be6f64ae65baf5ad1a5a0de8487aaf3407d39c5f"
        },
        "date": 1638404764930,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 189,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 211,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 150364,
            "range": "± 9401",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 135259,
            "range": "± 6627",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 125160,
            "range": "± 7370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 195622,
            "range": "± 9990",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 327927,
            "range": "± 10024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1651869,
            "range": "± 61714",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3126799,
            "range": "± 183922",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 6017711,
            "range": "± 215070",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11530770,
            "range": "± 193093",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 22588339,
            "range": "± 570841",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 45059655,
            "range": "± 1144379",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 76456,
            "range": "± 7781",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 100,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 92685,
            "range": "± 9464",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 107698,
            "range": "± 72874",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 123198,
            "range": "± 2093",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 181683,
            "range": "± 7008",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 147488,
            "range": "± 3724",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 244647,
            "range": "± 9800",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 433394,
            "range": "± 13649",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 918846,
            "range": "± 79035",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1814188,
            "range": "± 76591",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3425691,
            "range": "± 271097",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 156784,
            "range": "± 6422",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 133390,
            "range": "± 5246",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 125114,
            "range": "± 5257",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 191341,
            "range": "± 4762",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 324920,
            "range": "± 8521",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1645697,
            "range": "± 48550",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3055967,
            "range": "± 56929",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 6044715,
            "range": "± 192555",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11757060,
            "range": "± 169687",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 23077870,
            "range": "± 696479",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 47066116,
            "range": "± 1200712",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 78867,
            "range": "± 5083",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 102,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 95661,
            "range": "± 4775",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 106034,
            "range": "± 5006",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 122577,
            "range": "± 3072",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 182020,
            "range": "± 2559",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 148812,
            "range": "± 16132",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 246406,
            "range": "± 6355",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 439201,
            "range": "± 23631",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 888416,
            "range": "± 23585",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1738291,
            "range": "± 118629",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3380465,
            "range": "± 242123",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 101587,
            "range": "± 5549",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 6199,
            "range": "± 1563",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3437,
            "range": "± 572",
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
          "id": "66aa6c49175da7195d4ced15543d4a90a69cd015",
          "message": "Create gitlab pipeline  (#534)\n\n* add badge to readme\r\n\r\n* first version of pipeline\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* add pre-cache script\r\n\r\n* fmt and clippy stable\r\n\r\n* add check and test\r\n\r\n* remove output text file from bench\r\n\r\n* Update scripts/ci/pre_cache.sh\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* small fix\r\n\r\n* fix test and schedule\r\n\r\n* CI: verbose is a surplus\r\n\r\n* CI: separately check rustdoc linx\r\n\r\n* fix refs\r\n\r\n* add bench to gh-pages\r\n\r\n* fix refs\r\n\r\n* fix benchmarks\r\n\r\n* added vault to ci\r\n\r\n* fix vars\r\n\r\n* comment bench\r\n\r\n* fix benches name\r\n\r\n* added script to push benchmark results to VM\r\n\r\n* make script executable\r\n\r\n* change bench psuh executor\r\n\r\n* changed benchmark task to run on a dedicated node pool\r\n\r\n* change prometheus metric name for benchmarks\r\n\r\n* send 2 metrics with benchmark results\r\n\r\n* disable non-schedule jobs from schedule run\r\n\r\n* empty commit for benchmark test\r\n\r\n* change metric name\r\n\r\n* empty commit for benchmark test\r\n\r\n* empty commit for benchmark test\r\n\r\n* add cirunner label to vm metric\r\n\r\n* split vm metric to 2 metrics\r\n\r\n* change runner description to runner tag in ci scripts\r\n\r\n* add pass runner tags from benchmark to publish job\r\n\r\n* change runner tag to runner description\r\n\r\n* add debug message\r\n\r\n* empty commit for test\r\n\r\n* empty commit for test\r\n\r\n* Update .scripts/ci/push_bench_results.sh\r\n\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>\r\n\r\n* add defaults, remove dups, change ci image for publish-bench\r\n\r\n* remove pre_cache.sh\r\n\r\n* move interruptible to defaults\r\n\r\n* add issue to fixme comment\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\nCo-authored-by: David <dvdplm@gmail.com>\r\nCo-authored-by: Denis P <denis.pisarev@parity.io>\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>",
          "timestamp": "2021-12-02T15:33:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/66aa6c49175da7195d4ced15543d4a90a69cd015"
        },
        "date": 1638491194923,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 192,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 219,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 182686,
            "range": "± 19041",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 95,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 184332,
            "range": "± 21124",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 163343,
            "range": "± 13159",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 237085,
            "range": "± 15191",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 423935,
            "range": "± 31066",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1547510,
            "range": "± 286316",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2674665,
            "range": "± 152337",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5248172,
            "range": "± 279041",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 10516342,
            "range": "± 554163",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 19551839,
            "range": "± 1289218",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 39653110,
            "range": "± 2491788",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 178902,
            "range": "± 13608",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 209202,
            "range": "± 19593",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 219418,
            "range": "± 20754",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 346894,
            "range": "± 27621",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 474121,
            "range": "± 43500",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 116865,
            "range": "± 28802",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 87,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 107265,
            "range": "± 8820",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 127031,
            "range": "± 9238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 152575,
            "range": "± 8942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 195650,
            "range": "± 18181",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 166196,
            "range": "± 10936",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 272272,
            "range": "± 16638",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 469350,
            "range": "± 53241",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 937759,
            "range": "± 102403",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1869005,
            "range": "± 166653",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3388756,
            "range": "± 374913",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 129805,
            "range": "± 21438",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 136240,
            "range": "± 10862",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 159799,
            "range": "± 11392",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 274510,
            "range": "± 22780",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 412613,
            "range": "± 32288",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 165509,
            "range": "± 15627",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 83,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 156604,
            "range": "± 11222",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 154837,
            "range": "± 16232",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 249204,
            "range": "± 26400",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 421992,
            "range": "± 33695",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1424442,
            "range": "± 196274",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2682257,
            "range": "± 132259",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5099622,
            "range": "± 261040",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 9926922,
            "range": "± 512970",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 19432780,
            "range": "± 1100457",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 38427999,
            "range": "± 2163428",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 171754,
            "range": "± 18609",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 187771,
            "range": "± 11845",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 205655,
            "range": "± 16500",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 337494,
            "range": "± 32239",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 545412,
            "range": "± 61211",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 114652,
            "range": "± 13237",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 97,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 120924,
            "range": "± 8815",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 134832,
            "range": "± 9221",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 162394,
            "range": "± 12471",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 207901,
            "range": "± 17464",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 178026,
            "range": "± 19851",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 289126,
            "range": "± 23776",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 498727,
            "range": "± 35108",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1038733,
            "range": "± 76424",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1939632,
            "range": "± 160405",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3553634,
            "range": "± 382143",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 128812,
            "range": "± 9278",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 140956,
            "range": "± 9957",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 170519,
            "range": "± 10272",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 348338,
            "range": "± 40216",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 514039,
            "range": "± 68421",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 148637,
            "range": "± 10904",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 22254,
            "range": "± 4188",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2525,
            "range": "± 899",
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
          "id": "66aa6c49175da7195d4ced15543d4a90a69cd015",
          "message": "Create gitlab pipeline  (#534)\n\n* add badge to readme\r\n\r\n* first version of pipeline\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* add pre-cache script\r\n\r\n* fmt and clippy stable\r\n\r\n* add check and test\r\n\r\n* remove output text file from bench\r\n\r\n* Update scripts/ci/pre_cache.sh\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* small fix\r\n\r\n* fix test and schedule\r\n\r\n* CI: verbose is a surplus\r\n\r\n* CI: separately check rustdoc linx\r\n\r\n* fix refs\r\n\r\n* add bench to gh-pages\r\n\r\n* fix refs\r\n\r\n* fix benchmarks\r\n\r\n* added vault to ci\r\n\r\n* fix vars\r\n\r\n* comment bench\r\n\r\n* fix benches name\r\n\r\n* added script to push benchmark results to VM\r\n\r\n* make script executable\r\n\r\n* change bench psuh executor\r\n\r\n* changed benchmark task to run on a dedicated node pool\r\n\r\n* change prometheus metric name for benchmarks\r\n\r\n* send 2 metrics with benchmark results\r\n\r\n* disable non-schedule jobs from schedule run\r\n\r\n* empty commit for benchmark test\r\n\r\n* change metric name\r\n\r\n* empty commit for benchmark test\r\n\r\n* empty commit for benchmark test\r\n\r\n* add cirunner label to vm metric\r\n\r\n* split vm metric to 2 metrics\r\n\r\n* change runner description to runner tag in ci scripts\r\n\r\n* add pass runner tags from benchmark to publish job\r\n\r\n* change runner tag to runner description\r\n\r\n* add debug message\r\n\r\n* empty commit for test\r\n\r\n* empty commit for test\r\n\r\n* Update .scripts/ci/push_bench_results.sh\r\n\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>\r\n\r\n* add defaults, remove dups, change ci image for publish-bench\r\n\r\n* remove pre_cache.sh\r\n\r\n* move interruptible to defaults\r\n\r\n* add issue to fixme comment\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\nCo-authored-by: David <dvdplm@gmail.com>\r\nCo-authored-by: Denis P <denis.pisarev@parity.io>\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>",
          "timestamp": "2021-12-02T15:33:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/66aa6c49175da7195d4ced15543d4a90a69cd015"
        },
        "date": 1638577622153,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 175,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 204,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 173542,
            "range": "± 17315",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 90,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 163752,
            "range": "± 23615",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 157741,
            "range": "± 27516",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 240623,
            "range": "± 43311",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 419802,
            "range": "± 38439",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1531663,
            "range": "± 77891",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2834853,
            "range": "± 126252",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5437153,
            "range": "± 454269",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 10615999,
            "range": "± 339028",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 20825384,
            "range": "± 1033077",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 41042779,
            "range": "± 799692",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 182751,
            "range": "± 7143",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 189039,
            "range": "± 10974",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 207717,
            "range": "± 22191",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 342450,
            "range": "± 31404",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 497859,
            "range": "± 52858",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 111551,
            "range": "± 12136",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 96,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 123258,
            "range": "± 10519",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 134756,
            "range": "± 5633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 163377,
            "range": "± 34599",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 200878,
            "range": "± 47252",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 183059,
            "range": "± 26853",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 298125,
            "range": "± 41971",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 522565,
            "range": "± 49388",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1038691,
            "range": "± 52638",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2024144,
            "range": "± 154931",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3780617,
            "range": "± 341024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 126047,
            "range": "± 12828",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 154260,
            "range": "± 51035",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 173667,
            "range": "± 21133",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 310433,
            "range": "± 13394",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 488307,
            "range": "± 32448",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 183292,
            "range": "± 11455",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 96,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 171064,
            "range": "± 12643",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 168274,
            "range": "± 19748",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 250117,
            "range": "± 21190",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 423469,
            "range": "± 29713",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1580717,
            "range": "± 103752",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2870549,
            "range": "± 116383",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5525250,
            "range": "± 289769",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 10737739,
            "range": "± 636776",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 20952443,
            "range": "± 492842",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 41653042,
            "range": "± 977985",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 190162,
            "range": "± 43006",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 194760,
            "range": "± 15503",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 215924,
            "range": "± 10224",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 356229,
            "range": "± 24771",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 535818,
            "range": "± 57119",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 99940,
            "range": "± 10404",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 93,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 115965,
            "range": "± 17065",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 134155,
            "range": "± 8231",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 158284,
            "range": "± 30037",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 196765,
            "range": "± 14102",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 176412,
            "range": "± 14031",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 289177,
            "range": "± 44730",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 507391,
            "range": "± 92982",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1013686,
            "range": "± 56528",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1944837,
            "range": "± 138707",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3652346,
            "range": "± 360497",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 123332,
            "range": "± 13805",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 137047,
            "range": "± 11098",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 153887,
            "range": "± 12058",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 315956,
            "range": "± 17319",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 500394,
            "range": "± 35959",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 143014,
            "range": "± 13744",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 16911,
            "range": "± 1998",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1720,
            "range": "± 937",
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
          "id": "66aa6c49175da7195d4ced15543d4a90a69cd015",
          "message": "Create gitlab pipeline  (#534)\n\n* add badge to readme\r\n\r\n* first version of pipeline\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* add pre-cache script\r\n\r\n* fmt and clippy stable\r\n\r\n* add check and test\r\n\r\n* remove output text file from bench\r\n\r\n* Update scripts/ci/pre_cache.sh\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* small fix\r\n\r\n* fix test and schedule\r\n\r\n* CI: verbose is a surplus\r\n\r\n* CI: separately check rustdoc linx\r\n\r\n* fix refs\r\n\r\n* add bench to gh-pages\r\n\r\n* fix refs\r\n\r\n* fix benchmarks\r\n\r\n* added vault to ci\r\n\r\n* fix vars\r\n\r\n* comment bench\r\n\r\n* fix benches name\r\n\r\n* added script to push benchmark results to VM\r\n\r\n* make script executable\r\n\r\n* change bench psuh executor\r\n\r\n* changed benchmark task to run on a dedicated node pool\r\n\r\n* change prometheus metric name for benchmarks\r\n\r\n* send 2 metrics with benchmark results\r\n\r\n* disable non-schedule jobs from schedule run\r\n\r\n* empty commit for benchmark test\r\n\r\n* change metric name\r\n\r\n* empty commit for benchmark test\r\n\r\n* empty commit for benchmark test\r\n\r\n* add cirunner label to vm metric\r\n\r\n* split vm metric to 2 metrics\r\n\r\n* change runner description to runner tag in ci scripts\r\n\r\n* add pass runner tags from benchmark to publish job\r\n\r\n* change runner tag to runner description\r\n\r\n* add debug message\r\n\r\n* empty commit for test\r\n\r\n* empty commit for test\r\n\r\n* Update .scripts/ci/push_bench_results.sh\r\n\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>\r\n\r\n* add defaults, remove dups, change ci image for publish-bench\r\n\r\n* remove pre_cache.sh\r\n\r\n* move interruptible to defaults\r\n\r\n* add issue to fixme comment\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\nCo-authored-by: David <dvdplm@gmail.com>\r\nCo-authored-by: Denis P <denis.pisarev@parity.io>\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>",
          "timestamp": "2021-12-02T15:33:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/66aa6c49175da7195d4ced15543d4a90a69cd015"
        },
        "date": 1638663958699,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 176,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 135456,
            "range": "± 3578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 113807,
            "range": "± 4807",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 103376,
            "range": "± 4517",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 164994,
            "range": "± 4209",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 286255,
            "range": "± 4952",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1387481,
            "range": "± 12238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 2640355,
            "range": "± 18546",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5107075,
            "range": "± 51634",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 10063092,
            "range": "± 105957",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 19859191,
            "range": "± 222064",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 39457164,
            "range": "± 939294",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 142419,
            "range": "± 3960",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 151536,
            "range": "± 2742",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 166636,
            "range": "± 2646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 259350,
            "range": "± 7252",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 379254,
            "range": "± 43402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 66232,
            "range": "± 2062",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 86,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 85642,
            "range": "± 1869",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 95989,
            "range": "± 2634",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 116144,
            "range": "± 2183",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 167627,
            "range": "± 2834",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 131579,
            "range": "± 3098",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 222021,
            "range": "± 3312",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 392663,
            "range": "± 9624",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 831091,
            "range": "± 38341",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1602322,
            "range": "± 51238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3007074,
            "range": "± 238559",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 84371,
            "range": "± 3810",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 95515,
            "range": "± 14849",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 113781,
            "range": "± 3300",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 233276,
            "range": "± 59855",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 375114,
            "range": "± 22136",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 135540,
            "range": "± 4236",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 116336,
            "range": "± 5680",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 106955,
            "range": "± 6245",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 164199,
            "range": "± 3641",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 284905,
            "range": "± 4173",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1386780,
            "range": "± 44573",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 2631337,
            "range": "± 19552",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5088056,
            "range": "± 73943",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 10033651,
            "range": "± 103827",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 19935528,
            "range": "± 184880",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 39500820,
            "range": "± 1203068",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 142844,
            "range": "± 3551",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 154450,
            "range": "± 3102",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 170135,
            "range": "± 3450",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 283547,
            "range": "± 5492",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 418838,
            "range": "± 16894",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67466,
            "range": "± 2312",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 86792,
            "range": "± 1785",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 96903,
            "range": "± 2308",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 115630,
            "range": "± 1521",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 167380,
            "range": "± 3107",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 131853,
            "range": "± 4116",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 223365,
            "range": "± 9683",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 395849,
            "range": "± 16954",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 826216,
            "range": "± 16430",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1610883,
            "range": "± 72315",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3088921,
            "range": "± 209058",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 85938,
            "range": "± 2349",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 96229,
            "range": "± 2358",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 116021,
            "range": "± 3254",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 245509,
            "range": "± 3242",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 407807,
            "range": "± 18072",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 93639,
            "range": "± 3038",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 15995,
            "range": "± 1129",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1064,
            "range": "± 281",
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
          "id": "66aa6c49175da7195d4ced15543d4a90a69cd015",
          "message": "Create gitlab pipeline  (#534)\n\n* add badge to readme\r\n\r\n* first version of pipeline\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* add pre-cache script\r\n\r\n* fmt and clippy stable\r\n\r\n* add check and test\r\n\r\n* remove output text file from bench\r\n\r\n* Update scripts/ci/pre_cache.sh\r\n\r\nCo-authored-by: David <dvdplm@gmail.com>\r\n\r\n* Update .gitlab-ci.yml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* small fix\r\n\r\n* fix test and schedule\r\n\r\n* CI: verbose is a surplus\r\n\r\n* CI: separately check rustdoc linx\r\n\r\n* fix refs\r\n\r\n* add bench to gh-pages\r\n\r\n* fix refs\r\n\r\n* fix benchmarks\r\n\r\n* added vault to ci\r\n\r\n* fix vars\r\n\r\n* comment bench\r\n\r\n* fix benches name\r\n\r\n* added script to push benchmark results to VM\r\n\r\n* make script executable\r\n\r\n* change bench psuh executor\r\n\r\n* changed benchmark task to run on a dedicated node pool\r\n\r\n* change prometheus metric name for benchmarks\r\n\r\n* send 2 metrics with benchmark results\r\n\r\n* disable non-schedule jobs from schedule run\r\n\r\n* empty commit for benchmark test\r\n\r\n* change metric name\r\n\r\n* empty commit for benchmark test\r\n\r\n* empty commit for benchmark test\r\n\r\n* add cirunner label to vm metric\r\n\r\n* split vm metric to 2 metrics\r\n\r\n* change runner description to runner tag in ci scripts\r\n\r\n* add pass runner tags from benchmark to publish job\r\n\r\n* change runner tag to runner description\r\n\r\n* add debug message\r\n\r\n* empty commit for test\r\n\r\n* empty commit for test\r\n\r\n* Update .scripts/ci/push_bench_results.sh\r\n\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>\r\n\r\n* add defaults, remove dups, change ci image for publish-bench\r\n\r\n* remove pre_cache.sh\r\n\r\n* move interruptible to defaults\r\n\r\n* add issue to fixme comment\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\nCo-authored-by: David <dvdplm@gmail.com>\r\nCo-authored-by: Denis P <denis.pisarev@parity.io>\r\nCo-authored-by: Denis Pisarev <17856421+TriplEight@users.noreply.github.com>",
          "timestamp": "2021-12-02T15:33:52Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/66aa6c49175da7195d4ced15543d4a90a69cd015"
        },
        "date": 1638750367972,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 189,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 212,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 154766,
            "range": "± 8370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 103,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 135449,
            "range": "± 2056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 123909,
            "range": "± 3491",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 192263,
            "range": "± 6900",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 323184,
            "range": "± 6053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1614908,
            "range": "± 32865",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3059231,
            "range": "± 36864",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 5934869,
            "range": "± 118556",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 11654027,
            "range": "± 55875",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 22868114,
            "range": "± 300975",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 45697043,
            "range": "± 339459",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 165209,
            "range": "± 6026",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 175395,
            "range": "± 4259",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 194486,
            "range": "± 2253",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 306439,
            "range": "± 6276",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 440530,
            "range": "± 11436",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 74667,
            "range": "± 1893",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 90833,
            "range": "± 2805",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 107427,
            "range": "± 2502",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 121844,
            "range": "± 2421",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 180463,
            "range": "± 2922",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 145951,
            "range": "± 3172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 241909,
            "range": "± 4484",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 420468,
            "range": "± 15121",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 897450,
            "range": "± 40108",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1707608,
            "range": "± 76361",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3254135,
            "range": "± 250730",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 96891,
            "range": "± 9736",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 109037,
            "range": "± 1688",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 129737,
            "range": "± 2788",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 278142,
            "range": "± 39552",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 440746,
            "range": "± 31610",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 154769,
            "range": "± 6175",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 103,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 134912,
            "range": "± 3689",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 125929,
            "range": "± 2189",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 191546,
            "range": "± 3459",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 324153,
            "range": "± 3379",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1607585,
            "range": "± 14350",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3054284,
            "range": "± 13297",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 5918785,
            "range": "± 58283",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 11667207,
            "range": "± 48849",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 22933888,
            "range": "± 236209",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 45599845,
            "range": "± 811070",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 168677,
            "range": "± 7082",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 182411,
            "range": "± 2938",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 201027,
            "range": "± 3138",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 332778,
            "range": "± 4900",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 496006,
            "range": "± 16052",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 76987,
            "range": "± 5610",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 97219,
            "range": "± 3372",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 110325,
            "range": "± 1977",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 122930,
            "range": "± 3081",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 181693,
            "range": "± 2477",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 146792,
            "range": "± 3818",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 243522,
            "range": "± 7172",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 426015,
            "range": "± 7505",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 881827,
            "range": "± 17309",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1682845,
            "range": "± 65861",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3170430,
            "range": "± 223510",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 99217,
            "range": "± 6471",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 112548,
            "range": "± 2807",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 130845,
            "range": "± 2854",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 290287,
            "range": "± 4374",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 480997,
            "range": "± 9712",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 101983,
            "range": "± 4131",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 7255,
            "range": "± 1270",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3228,
            "range": "± 501",
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
          "id": "3f1c7fcf4b19fc89bf1786cd89f6435b15e96948",
          "message": "clients: feature gate `tls` (#545)\n\n* clients: introduce tls feature flag\r\n\r\n* Update tests/tests/integration_tests.rs\r\n\r\n* fix: don't rebuild tls connector of every connect\r\n\r\n* fix tests + remove url dep\r\n\r\n* fix tests again",
          "timestamp": "2021-12-06T14:26:15Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/3f1c7fcf4b19fc89bf1786cd89f6435b15e96948"
        },
        "date": 1638836716419,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 160,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 176,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 124432,
            "range": "± 10927",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 120889,
            "range": "± 5002",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 111367,
            "range": "± 6316",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 170078,
            "range": "± 9058",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 282187,
            "range": "± 13056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 210991,
            "range": "± 9576",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 391233,
            "range": "± 17392",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 702918,
            "range": "± 32220",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 1337950,
            "range": "± 62874",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 2579969,
            "range": "± 95136",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 5120372,
            "range": "± 268354",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 150382,
            "range": "± 6557",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 162547,
            "range": "± 6740",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 176665,
            "range": "± 6743",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 280142,
            "range": "± 15832",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 407960,
            "range": "± 20130",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 68994,
            "range": "± 7902",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 86,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 79929,
            "range": "± 4862",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 99131,
            "range": "± 4769",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 111862,
            "range": "± 5528",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 165099,
            "range": "± 7079",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 134291,
            "range": "± 7560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 222199,
            "range": "± 10852",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 389144,
            "range": "± 29175",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 800364,
            "range": "± 38238",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1523797,
            "range": "± 92224",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2942126,
            "range": "± 240673",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 90638,
            "range": "± 11215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 98586,
            "range": "± 4096",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 113635,
            "range": "± 5449",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 250174,
            "range": "± 13224",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 388558,
            "range": "± 17490",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 138940,
            "range": "± 6569",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 86,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 118531,
            "range": "± 5243",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 109908,
            "range": "± 5521",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 171204,
            "range": "± 8182",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 292558,
            "range": "± 11750",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 220708,
            "range": "± 9352",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 398391,
            "range": "± 16954",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 705589,
            "range": "± 26167",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 1342035,
            "range": "± 71128",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 2639188,
            "range": "± 188883",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 5129043,
            "range": "± 262766",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 149876,
            "range": "± 7010",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 162244,
            "range": "± 7738",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 182698,
            "range": "± 7110",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 311927,
            "range": "± 14760",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 446503,
            "range": "± 19100",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 68398,
            "range": "± 3230",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 88,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 81852,
            "range": "± 5176",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 98268,
            "range": "± 5614",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 109758,
            "range": "± 5397",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 166014,
            "range": "± 7483",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 136866,
            "range": "± 5026",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 218269,
            "range": "± 8758",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 379052,
            "range": "± 19898",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 780894,
            "range": "± 35517",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1486557,
            "range": "± 70555",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2806412,
            "range": "± 239008",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 93269,
            "range": "± 4442",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 102746,
            "range": "± 4939",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 117575,
            "range": "± 5748",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 257028,
            "range": "± 14120",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 429045,
            "range": "± 16753",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 89886,
            "range": "± 4790",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 5606,
            "range": "± 2559",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2294,
            "range": "± 404",
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
          "id": "7bb628af3a40caea3f6a9886bbfdfbdae4d7f865",
          "message": "clients: use `FxHashMap` instead `FnvHashMap` (#592)\n\n* deps: use `FxHashMap` instead `fnv`\r\n\r\n* fmt",
          "timestamp": "2021-12-07T15:49:26Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/7bb628af3a40caea3f6a9886bbfdfbdae4d7f865"
        },
        "date": 1638923215353,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 168,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 201,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 181321,
            "range": "± 48040",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 163437,
            "range": "± 19320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 153922,
            "range": "± 11987",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 237128,
            "range": "± 12020",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 415287,
            "range": "± 72617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 291106,
            "range": "± 22914",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 524098,
            "range": "± 24923",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 955152,
            "range": "± 56128",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 1829973,
            "range": "± 97515",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 3564376,
            "range": "± 320804",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 7220816,
            "range": "± 369458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 181667,
            "range": "± 32531",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 190136,
            "range": "± 34102",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 210185,
            "range": "± 22660",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 329002,
            "range": "± 78060",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 464014,
            "range": "± 19988",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 102926,
            "range": "± 27629",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 89,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 117133,
            "range": "± 12587",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 138635,
            "range": "± 13565",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 161135,
            "range": "± 17492",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 189166,
            "range": "± 44281",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 173253,
            "range": "± 21022",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 284467,
            "range": "± 18844",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 486636,
            "range": "± 48643",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 993835,
            "range": "± 55828",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1950117,
            "range": "± 196819",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3563349,
            "range": "± 380508",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 124754,
            "range": "± 5349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 136612,
            "range": "± 38521",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 157499,
            "range": "± 22401",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 291984,
            "range": "± 27165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 453508,
            "range": "± 41274",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 174318,
            "range": "± 21539",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 87,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 167133,
            "range": "± 24152",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 154805,
            "range": "± 18087",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 237577,
            "range": "± 14100",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 402688,
            "range": "± 32408",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 286560,
            "range": "± 24868",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 539307,
            "range": "± 25057",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 1017768,
            "range": "± 82937",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 1851701,
            "range": "± 75461",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 3644546,
            "range": "± 312606",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 7389022,
            "range": "± 339857",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 178968,
            "range": "± 6991",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 191797,
            "range": "± 11679",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 206899,
            "range": "± 14402",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 336922,
            "range": "± 40058",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 503771,
            "range": "± 66689",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 101732,
            "range": "± 7314",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 88,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 114516,
            "range": "± 12136",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 136070,
            "range": "± 8211",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 159557,
            "range": "± 9662",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 186894,
            "range": "± 8056",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 179686,
            "range": "± 13901",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 287104,
            "range": "± 17823",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 479613,
            "range": "± 23466",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 968750,
            "range": "± 54983",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1869722,
            "range": "± 158122",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3444057,
            "range": "± 394370",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 127269,
            "range": "± 6343",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 140071,
            "range": "± 14809",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 157174,
            "range": "± 16641",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 308876,
            "range": "± 43829",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 485329,
            "range": "± 46050",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 139952,
            "range": "± 10248",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 20008,
            "range": "± 1174",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1646,
            "range": "± 413",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}