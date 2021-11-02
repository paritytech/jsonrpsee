window.BENCHMARK_DATA = {
  "lastUpdate": 1635812734415,
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
      }
    ]
  }
}