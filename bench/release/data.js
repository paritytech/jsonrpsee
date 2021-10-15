window.BENCHMARK_DATA = {
  "lastUpdate": 1634329096360,
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
          "id": "2426ea6bffa8dca1c3240c1291e3cf2ecbc12b75",
          "message": "v0.2",
          "timestamp": "2021-10-15T13:13:38Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/2426ea6bffa8dca1c3240c1291e3cf2ecbc12b75"
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
          "id": "e6a52180305527fa804e07b1333a1a29173fefa1",
          "message": "v0.3",
          "timestamp": "2021-10-15T13:13:38Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/e6a52180305527fa804e07b1333a1a29173fefa1"
        },
        "date": 1634324698818,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 167,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 192,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 138850,
            "range": "± 21915",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 96,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 137307,
            "range": "± 15971",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 117384,
            "range": "± 12380",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 198727,
            "range": "± 32929",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 306273,
            "range": "± 29504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1902974,
            "range": "± 78783",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3701859,
            "range": "± 246627",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7347544,
            "range": "± 384680",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 14599875,
            "range": "± 714777",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 28364222,
            "range": "± 1511791",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 56814500,
            "range": "± 2237190",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 82884,
            "range": "± 8068",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/0",
            "value": 103,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/1",
            "value": 96431,
            "range": "± 10866",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 113288,
            "range": "± 12141",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 124194,
            "range": "± 24320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 179175,
            "range": "± 12178",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 131847,
            "range": "± 13034",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 198523,
            "range": "± 17154",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 346807,
            "range": "± 60456",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 723434,
            "range": "± 117462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 1425970,
            "range": "± 167810",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 2847370,
            "range": "± 298220",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 130756,
            "range": "± 14112",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 99,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 135844,
            "range": "± 18384",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 119103,
            "range": "± 14564",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 179736,
            "range": "± 13167",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 306955,
            "range": "± 19209",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1942132,
            "range": "± 123891",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3730460,
            "range": "± 266430",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7271624,
            "range": "± 301847",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 14667838,
            "range": "± 731740",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 29943023,
            "range": "± 1119114",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 58353757,
            "range": "± 2503638",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 87562,
            "range": "± 9004",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 101,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 94334,
            "range": "± 9749",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 103483,
            "range": "± 8089",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 116961,
            "range": "± 5909",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 176543,
            "range": "± 18317",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 129429,
            "range": "± 39285",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 206703,
            "range": "± 24249",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 366530,
            "range": "± 43924",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 739444,
            "range": "± 66570",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 1475324,
            "range": "± 134245",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 2892565,
            "range": "± 273608",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 96927,
            "range": "± 9728",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 19174,
            "range": "± 3147",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1276,
            "range": "± 425",
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
          "id": "20eb409249b943624c0f5909aa7ebbc45a47fb0c",
          "message": "testme",
          "timestamp": "2021-10-15T13:13:38Z",
          "url": "https://github.com/paritytech/jsonrpsee/pull/529/commits/20eb409249b943624c0f5909aa7ebbc45a47fb0c"
        },
        "date": 1634329095677,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 189,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 211,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 183867,
            "range": "± 9848",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/0",
            "value": 111,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/1",
            "value": 171202,
            "range": "± 16913",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/2",
            "value": 165423,
            "range": "± 15178",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/4",
            "value": 245379,
            "range": "± 16315",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_round_trip/8",
            "value": 428237,
            "range": "± 30093",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/2",
            "value": 1934188,
            "range": "± 70129",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/4",
            "value": 3647936,
            "range": "± 73744",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/8",
            "value": 7063622,
            "range": "± 293813",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/16",
            "value": 13891222,
            "range": "± 373406",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/32",
            "value": 27742493,
            "range": "± 1265777",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_connections/64",
            "value": 54945614,
            "range": "± 1522466",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 115413,
            "range": "± 6413",
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
            "value": 124795,
            "range": "± 9118",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/2",
            "value": 154413,
            "range": "± 13818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/4",
            "value": 170971,
            "range": "± 16583",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_round_trip/8",
            "value": 200554,
            "range": "± 19354",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/2",
            "value": 181367,
            "range": "± 33274",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/4",
            "value": 288243,
            "range": "± 30171",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/8",
            "value": 501466,
            "range": "± 57443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/16",
            "value": 1036416,
            "range": "± 50047",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/32",
            "value": 2014580,
            "range": "± 162362",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_connections/64",
            "value": 3704487,
            "range": "± 490596",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 176485,
            "range": "± 24853",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/0",
            "value": 105,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/1",
            "value": 176536,
            "range": "± 20426",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/2",
            "value": 156107,
            "range": "± 17263",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/4",
            "value": 247194,
            "range": "± 22375",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_round_trip/8",
            "value": 428595,
            "range": "± 19551",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/2",
            "value": 1946884,
            "range": "± 94601",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/4",
            "value": 3689363,
            "range": "± 149328",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/8",
            "value": 7100559,
            "range": "± 220295",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/16",
            "value": 13964077,
            "range": "± 382512",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/32",
            "value": 27728795,
            "range": "± 684447",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_connections/64",
            "value": 55280441,
            "range": "± 1079079",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 118996,
            "range": "± 6693",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/0",
            "value": 100,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/1",
            "value": 125358,
            "range": "± 7753",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/2",
            "value": 149497,
            "range": "± 9809",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/4",
            "value": 172921,
            "range": "± 16339",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_round_trip/8",
            "value": 200295,
            "range": "± 22826",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/2",
            "value": 182785,
            "range": "± 11640",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/4",
            "value": 291511,
            "range": "± 93965",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/8",
            "value": 502886,
            "range": "± 46649",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/16",
            "value": 1030864,
            "range": "± 52649",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/32",
            "value": 2043361,
            "range": "± 181796",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_connections/64",
            "value": 3602738,
            "range": "± 475226",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 136146,
            "range": "± 6185",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 18265,
            "range": "± 2607",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 2083,
            "range": "± 341",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}