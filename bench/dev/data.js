window.BENCHMARK_DATA = {
  "lastUpdate": 1655426666532,
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
            "name": "sync/ws_round_trip",
            "value": 111479,
            "range": "± 10775",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 181991,
            "range": "± 13708",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 114682,
            "range": "± 11241",
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
            "name": "sync/ws_round_trip",
            "value": 65957,
            "range": "± 3089",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 129613,
            "range": "± 5115",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 63827,
            "range": "± 2121",
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
            "name": "sync/ws_round_trip",
            "value": 64693,
            "range": "± 2704",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 130574,
            "range": "± 3184",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65740,
            "range": "± 2794",
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
            "name": "sync/ws_round_trip",
            "value": 101948,
            "range": "± 7731",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 177994,
            "range": "± 41015",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 103827,
            "range": "± 24482",
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
            "name": "sync/ws_round_trip",
            "value": 127571,
            "range": "± 95628",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 180858,
            "range": "± 30324",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 130490,
            "range": "± 20388",
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
            "name": "sync/ws_round_trip",
            "value": 65778,
            "range": "± 2620",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 126540,
            "range": "± 5696",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67695,
            "range": "± 2984",
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
            "name": "sync/ws_round_trip",
            "value": 117470,
            "range": "± 11366",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 183614,
            "range": "± 13290",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 121938,
            "range": "± 9476",
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
            "name": "sync/ws_round_trip",
            "value": 69620,
            "range": "± 3659",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 133052,
            "range": "± 8563",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 69052,
            "range": "± 2913",
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
            "name": "sync/ws_round_trip",
            "value": 77634,
            "range": "± 3489",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 139367,
            "range": "± 7478",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 78926,
            "range": "± 3740",
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
            "name": "sync/ws_round_trip",
            "value": 64698,
            "range": "± 2466",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 131056,
            "range": "± 2675",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65091,
            "range": "± 1880",
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
            "name": "sync/ws_round_trip",
            "value": 102178,
            "range": "± 14722",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 164152,
            "range": "± 26375",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 98777,
            "range": "± 8525",
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
            "name": "sync/ws_round_trip",
            "value": 78597,
            "range": "± 7922",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 152570,
            "range": "± 15893",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 82083,
            "range": "± 8749",
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
            "name": "sync/ws_round_trip",
            "value": 115646,
            "range": "± 8823",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 174232,
            "range": "± 25858",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 112309,
            "range": "± 13259",
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
            "name": "sync/ws_round_trip",
            "value": 64479,
            "range": "± 1913",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 125655,
            "range": "± 5268",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 64469,
            "range": "± 1792",
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
            "name": "sync/ws_round_trip",
            "value": 74809,
            "range": "± 3634",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 145727,
            "range": "± 17814",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 76294,
            "range": "± 1644",
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
            "name": "sync/ws_round_trip",
            "value": 68260,
            "range": "± 4560",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 136674,
            "range": "± 15073",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67164,
            "range": "± 2149",
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
            "name": "sync/ws_round_trip",
            "value": 74803,
            "range": "± 2329",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 144237,
            "range": "± 6013",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 75095,
            "range": "± 2689",
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
            "name": "sync/ws_round_trip",
            "value": 78740,
            "range": "± 6725",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 149267,
            "range": "± 18020",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 79212,
            "range": "± 6012",
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
            "name": "sync/ws_round_trip",
            "value": 64821,
            "range": "± 2890",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 130526,
            "range": "± 3939",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 64354,
            "range": "± 1457",
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
            "name": "sync/ws_round_trip",
            "value": 104531,
            "range": "± 12132",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 179859,
            "range": "± 13369",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 104797,
            "range": "± 22944",
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
            "name": "sync/ws_round_trip",
            "value": 101837,
            "range": "± 13285",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 174313,
            "range": "± 11422",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 100462,
            "range": "± 7074",
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
            "name": "sync/ws_round_trip",
            "value": 76460,
            "range": "± 1741",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 147077,
            "range": "± 6991",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 75654,
            "range": "± 2195",
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
            "name": "sync/ws_round_trip",
            "value": 103460,
            "range": "± 17520",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 176117,
            "range": "± 17953",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 101821,
            "range": "± 6331",
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
            "name": "sync/ws_round_trip",
            "value": 67637,
            "range": "± 2797",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 133420,
            "range": "± 3190",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67133,
            "range": "± 2885",
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
            "name": "sync/ws_round_trip",
            "value": 111461,
            "range": "± 7466",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 182153,
            "range": "± 13207",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111065,
            "range": "± 12474",
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
            "name": "sync/ws_round_trip",
            "value": 118061,
            "range": "± 7920",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 184322,
            "range": "± 16248",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 116695,
            "range": "± 12088",
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
            "name": "sync/ws_round_trip",
            "value": 112329,
            "range": "± 15059",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 191091,
            "range": "± 19331",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111353,
            "range": "± 14071",
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
            "name": "sync/ws_round_trip",
            "value": 113306,
            "range": "± 10923",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 183433,
            "range": "± 13369",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111624,
            "range": "± 8280",
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
            "name": "sync/ws_round_trip",
            "value": 107331,
            "range": "± 8183",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 164380,
            "range": "± 18417",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 106929,
            "range": "± 76643",
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
            "name": "sync/ws_round_trip",
            "value": 113566,
            "range": "± 9273",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 185125,
            "range": "± 13267",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 110372,
            "range": "± 9509",
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
            "name": "sync/ws_round_trip",
            "value": 123994,
            "range": "± 18983",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 202592,
            "range": "± 11481",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 127560,
            "range": "± 7774",
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
            "name": "sync/ws_round_trip",
            "value": 113256,
            "range": "± 23660",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 194725,
            "range": "± 15478",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 112112,
            "range": "± 19614",
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
            "name": "sync/ws_round_trip",
            "value": 112337,
            "range": "± 7631",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 180003,
            "range": "± 18763",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 111566,
            "range": "± 7618",
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
            "name": "sync/ws_round_trip",
            "value": 122871,
            "range": "± 15822",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 189164,
            "range": "± 14804",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 118803,
            "range": "± 15909",
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
            "name": "sync/ws_round_trip",
            "value": 119205,
            "range": "± 10182",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 189252,
            "range": "± 10840",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 122873,
            "range": "± 8918",
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
            "name": "sync/ws_round_trip",
            "value": 68759,
            "range": "± 3297",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 130509,
            "range": "± 11880",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 63729,
            "range": "± 2363",
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
            "name": "sync/ws_round_trip",
            "value": 105376,
            "range": "± 8220",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 163661,
            "range": "± 12604",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 104000,
            "range": "± 26619",
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
            "name": "sync/ws_round_trip",
            "value": 69875,
            "range": "± 5497",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 115687,
            "range": "± 13979",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65416,
            "range": "± 4409",
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
            "name": "sync/ws_round_trip",
            "value": 114998,
            "range": "± 19536",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 194035,
            "range": "± 22099",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 120676,
            "range": "± 146446",
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
            "name": "sync/ws_round_trip",
            "value": 99554,
            "range": "± 19296",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 175398,
            "range": "± 13531",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 106140,
            "range": "± 15255",
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
            "name": "sync/ws_round_trip",
            "value": 67031,
            "range": "± 2560",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 133950,
            "range": "± 2888",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 66220,
            "range": "± 2539",
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
            "name": "sync/ws_round_trip",
            "value": 66137,
            "range": "± 2714",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 132646,
            "range": "± 2799",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65604,
            "range": "± 2312",
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
            "name": "sync/ws_round_trip",
            "value": 69695,
            "range": "± 6700",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 135626,
            "range": "± 4213",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 67407,
            "range": "± 2111",
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
            "name": "sync/ws_round_trip",
            "value": 73879,
            "range": "± 3315",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 145753,
            "range": "± 8862",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 80687,
            "range": "± 6702",
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
            "name": "sync/ws_round_trip",
            "value": 64585,
            "range": "± 1512",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 134630,
            "range": "± 2224",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65558,
            "range": "± 1989",
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
            "name": "sync/ws_round_trip",
            "value": 81777,
            "range": "± 5918",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 162505,
            "range": "± 7546",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 82982,
            "range": "± 4056",
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
            "name": "sync/ws_round_trip",
            "value": 73267,
            "range": "± 2063",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 149763,
            "range": "± 6024",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 73027,
            "range": "± 2593",
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
            "name": "sync/ws_round_trip",
            "value": 76456,
            "range": "± 7781",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 156784,
            "range": "± 6422",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 78867,
            "range": "± 5083",
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
          "id": "1ba1a33bdd188c2ff22acdb9cb92a006f54498c0",
          "message": "fix: duplicate env logger deps (#595)",
          "timestamp": "2021-12-08T17:25:51Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/1ba1a33bdd188c2ff22acdb9cb92a006f54498c0"
        },
        "date": 1639009561696,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 166,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 175,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 149740,
            "range": "± 48160",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 188467,
            "range": "± 17382",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 199268,
            "range": "± 14167",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 215712,
            "range": "± 19001",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 338498,
            "range": "± 25910",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 492691,
            "range": "± 50234",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 116263,
            "range": "± 13123",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 139977,
            "range": "± 9512",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 149292,
            "range": "± 9190",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 170446,
            "range": "± 17083",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 304736,
            "range": "± 23710",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 451895,
            "range": "± 44569",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 155994,
            "range": "± 17179",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 193645,
            "range": "± 10306",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 202508,
            "range": "± 14377",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 222994,
            "range": "± 12298",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 359497,
            "range": "± 25259",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 544304,
            "range": "± 62718",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 113816,
            "range": "± 10846",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 140035,
            "range": "± 13697",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 150796,
            "range": "± 15949",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 169116,
            "range": "± 19710",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 325085,
            "range": "± 20934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 527265,
            "range": "± 49652",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 149110,
            "range": "± 12588",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 17774,
            "range": "± 3240",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1327,
            "range": "± 1032",
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
          "id": "b41acaba88774eedc4cd5ad59895e7dc6c437a59",
          "message": "Remove usage of the `palaver` crate in an example (#597)\n\n* Remove usage of the `palaver` crate in an example\r\n\r\n* fmt",
          "timestamp": "2021-12-09T15:19:44Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/b41acaba88774eedc4cd5ad59895e7dc6c437a59"
        },
        "date": 1639095850207,
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
            "value": 175,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip",
            "value": 122695,
            "range": "± 8215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/2",
            "value": 137253,
            "range": "± 1869",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/5",
            "value": 145059,
            "range": "± 3941",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/10",
            "value": 156061,
            "range": "± 3221",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/50",
            "value": 238922,
            "range": "± 6391",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/100",
            "value": 352951,
            "range": "± 4544",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip",
            "value": 61606,
            "range": "± 2098",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/2",
            "value": 83375,
            "range": "± 3662",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/5",
            "value": 95447,
            "range": "± 3407",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/10",
            "value": 113197,
            "range": "± 4675",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/50",
            "value": 229911,
            "range": "± 3908",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/100",
            "value": 359252,
            "range": "± 6405",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip",
            "value": 120519,
            "range": "± 6465",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/2",
            "value": 144407,
            "range": "± 2290",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/5",
            "value": 153553,
            "range": "± 3068",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/10",
            "value": 170462,
            "range": "± 3851",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/50",
            "value": 279128,
            "range": "± 5567",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/100",
            "value": 412837,
            "range": "± 6635",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip",
            "value": 65360,
            "range": "± 1309",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/2",
            "value": 85809,
            "range": "± 2626",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/5",
            "value": 98162,
            "range": "± 3471",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/10",
            "value": 114444,
            "range": "± 3766",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/50",
            "value": 243224,
            "range": "± 2614",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/100",
            "value": 388568,
            "range": "± 3309",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 90263,
            "range": "± 2044",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 11773,
            "range": "± 2009",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 1163,
            "range": "± 116",
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
          "id": "00aeb4ed42cdb923c83f93ddd645983ac364599f",
          "message": "ci: bring back daily benchmarks (#777)\n\n* ci: bring back daily benchmarks\r\n\r\n* remove ugly spaces",
          "timestamp": "2022-05-20T16:30:34Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/00aeb4ed42cdb923c83f93ddd645983ac364599f"
        },
        "date": 1653094186849,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 191,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 207,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 158838,
            "range": "± 37828",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3215436,
            "range": "± 196160",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1373575,
            "range": "± 41565",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 326002,
            "range": "± 18252",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 583238,
            "range": "± 34795",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1062886,
            "range": "± 69212",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2046614,
            "range": "± 118739",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 4049442,
            "range": "± 245011",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 8145913,
            "range": "± 438039",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 16354609,
            "range": "± 806349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 33465868,
            "range": "± 1770954",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 66858737,
            "range": "± 4154070",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 135785165,
            "range": "± 7152798",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 156905,
            "range": "± 24972",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 167273,
            "range": "± 14760",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 182580,
            "range": "± 14442",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 322068,
            "range": "± 16778",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 482950,
            "range": "± 20489",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9757760,
            "range": "± 677452",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 23480311,
            "range": "± 1409828",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 48883278,
            "range": "± 2224537",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 244342319,
            "range": "± 5955547",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 485912659,
            "range": "± 11454339",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2520132,
            "range": "± 26214",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5894996,
            "range": "± 59098",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11523912,
            "range": "± 75891",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 56601942,
            "range": "± 633720",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 112878658,
            "range": "± 741931",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 120758,
            "range": "± 8198",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2609610,
            "range": "± 79154",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1335795,
            "range": "± 22812",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 451604,
            "range": "± 85477",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 845932,
            "range": "± 132215",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1671448,
            "range": "± 174245",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3346135,
            "range": "± 489127",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 6818650,
            "range": "± 795582",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 13648484,
            "range": "± 1414906",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 26766524,
            "range": "± 2911619",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 56929900,
            "range": "± 3499921",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 117590414,
            "range": "± 6195890",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 239688480,
            "range": "± 8918028",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 620844,
            "range": "± 101934",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1322059,
            "range": "± 255474",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2516877,
            "range": "± 269799",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5589777,
            "range": "± 472790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 11340802,
            "range": "± 849119",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 23394946,
            "range": "± 1740426",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 47580397,
            "range": "± 2739549",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 96174166,
            "range": "± 4938174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 195867365,
            "range": "± 8419614",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 412497765,
            "range": "± 18660994",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 126352,
            "range": "± 9296",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 150966,
            "range": "± 14702",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 165242,
            "range": "± 12897",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 319416,
            "range": "± 215696",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 482596,
            "range": "± 40698",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5380640,
            "range": "± 322498",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 19127165,
            "range": "± 1529367",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 39894187,
            "range": "± 2086766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 244828069,
            "range": "± 7269949",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 484731444,
            "range": "± 11647259",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2490395,
            "range": "± 39392",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5862944,
            "range": "± 47824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11465439,
            "range": "± 159086",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 56373197,
            "range": "± 357174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 112836894,
            "range": "± 591256",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 149482,
            "range": "± 8450",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 3207518,
            "range": "± 149703",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1375398,
            "range": "± 35407",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 316685,
            "range": "± 23188",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 602563,
            "range": "± 27212",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1072964,
            "range": "± 62161",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 2072961,
            "range": "± 143527",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3885665,
            "range": "± 236002",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 7963615,
            "range": "± 486710",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 15488190,
            "range": "± 933521",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 32291878,
            "range": "± 2399838",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 64995387,
            "range": "± 3446014",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 128950824,
            "range": "± 7193187",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 156441,
            "range": "± 14433",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 173970,
            "range": "± 16905",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 181960,
            "range": "± 12703",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 334287,
            "range": "± 22063",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 502686,
            "range": "± 25465",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 6410540,
            "range": "± 312550",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 15752399,
            "range": "± 769370",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 43487049,
            "range": "± 2386074",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 224959817,
            "range": "± 5779007",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 491782054,
            "range": "± 9593040",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2518514,
            "range": "± 38130",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5885949,
            "range": "± 39483",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11517779,
            "range": "± 106490",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 56549445,
            "range": "± 546184",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 112636742,
            "range": "± 775163",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 112030,
            "range": "± 11167",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2730720,
            "range": "± 118721",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1348716,
            "range": "± 39703",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 441851,
            "range": "± 61568",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 836653,
            "range": "± 74557",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1644154,
            "range": "± 206768",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 3330060,
            "range": "± 490438",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 6991632,
            "range": "± 922967",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 14219535,
            "range": "± 1633773",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 28210325,
            "range": "± 2406998",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 54304599,
            "range": "± 3450164",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 110838288,
            "range": "± 5851387",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 232509587,
            "range": "± 9304464",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 592957,
            "range": "± 98819",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1323872,
            "range": "± 114593",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2571310,
            "range": "± 232020",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5268195,
            "range": "± 473915",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 10995523,
            "range": "± 1128434",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 22887530,
            "range": "± 1907911",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 44722266,
            "range": "± 3094033",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 92266168,
            "range": "± 5670059",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 183409584,
            "range": "± 11533896",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 394142092,
            "range": "± 17092145",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 136888,
            "range": "± 10635",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 141687,
            "range": "± 11307",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 166173,
            "range": "± 18868",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 315399,
            "range": "± 24609",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 494880,
            "range": "± 49375",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5260875,
            "range": "± 167501",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 14378357,
            "range": "± 1162812",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 39061311,
            "range": "± 2165868",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 246133787,
            "range": "± 6688110",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 486586877,
            "range": "± 10959388",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2486814,
            "range": "± 45186",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5869202,
            "range": "± 47923",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11451262,
            "range": "± 54336",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 56536666,
            "range": "± 320134",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 112439359,
            "range": "± 454688",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 154686,
            "range": "± 10958",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 8668,
            "range": "± 3423",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4981,
            "range": "± 1147",
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
          "id": "00aeb4ed42cdb923c83f93ddd645983ac364599f",
          "message": "ci: bring back daily benchmarks (#777)\n\n* ci: bring back daily benchmarks\r\n\r\n* remove ugly spaces",
          "timestamp": "2022-05-20T16:30:34Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/00aeb4ed42cdb923c83f93ddd645983ac364599f"
        },
        "date": 1653180497247,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 160,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 216,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 162500,
            "range": "± 15498",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2895363,
            "range": "± 189004",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1369425,
            "range": "± 30986",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 284063,
            "range": "± 33403",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 515109,
            "range": "± 42377",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 931759,
            "range": "± 48610",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1836889,
            "range": "± 105191",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 3510621,
            "range": "± 179118",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 6884678,
            "range": "± 361072",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 14111524,
            "range": "± 854059",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 28235062,
            "range": "± 1586893",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 56656064,
            "range": "± 2912550",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 119729106,
            "range": "± 4618340",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 148613,
            "range": "± 11265",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 158009,
            "range": "± 28877",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 169212,
            "range": "± 14125",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 295111,
            "range": "± 22307",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 441036,
            "range": "± 62233",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 8101009,
            "range": "± 603332",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 19246070,
            "range": "± 1666299",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 42158707,
            "range": "± 2899546",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 219619488,
            "range": "± 8856513",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 451016605,
            "range": "± 21393387",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2507523,
            "range": "± 72995",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5912710,
            "range": "± 145807",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11523784,
            "range": "± 114957",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 56620707,
            "range": "± 256550",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 112918634,
            "range": "± 977944",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 104453,
            "range": "± 9482",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2294645,
            "range": "± 216020",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1344680,
            "range": "± 20659",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 437714,
            "range": "± 65828",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 748859,
            "range": "± 67235",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1379870,
            "range": "± 212381",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3030619,
            "range": "± 407851",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 6315195,
            "range": "± 848250",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 13746259,
            "range": "± 1754152",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 26444912,
            "range": "± 2232553",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 53585336,
            "range": "± 3983346",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 104187815,
            "range": "± 5575213",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 212230628,
            "range": "± 10597287",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 601489,
            "range": "± 61109",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1264440,
            "range": "± 130745",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2424074,
            "range": "± 247273",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4977923,
            "range": "± 504791",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 10745204,
            "range": "± 1224728",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 20487399,
            "range": "± 2153781",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 44898245,
            "range": "± 3655617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 86270807,
            "range": "± 4001986",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 170032416,
            "range": "± 7706839",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 358356433,
            "range": "± 16565302",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 118875,
            "range": "± 14158",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 127795,
            "range": "± 11441",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 151860,
            "range": "± 17864",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 281395,
            "range": "± 92130",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 476016,
            "range": "± 72142",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4873133,
            "range": "± 318439",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 19299085,
            "range": "± 1957508",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 36334496,
            "range": "± 2884090",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 258880829,
            "range": "± 8105791",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 493458070,
            "range": "± 15183324",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2504124,
            "range": "± 45375",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5867603,
            "range": "± 87773",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11482941,
            "range": "± 160654",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 56603357,
            "range": "± 565600",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 113051802,
            "range": "± 751088",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 145586,
            "range": "± 18137",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2995161,
            "range": "± 150732",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1360926,
            "range": "± 24905",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 273600,
            "range": "± 116006",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 514377,
            "range": "± 45352",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 934495,
            "range": "± 40134",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1772830,
            "range": "± 97967",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3383370,
            "range": "± 188247",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 6792899,
            "range": "± 379548",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 14431491,
            "range": "± 861354",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 29661141,
            "range": "± 1522131",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 60225726,
            "range": "± 2580938",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 110096046,
            "range": "± 5105850",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 163863,
            "range": "± 19013",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 168612,
            "range": "± 18056",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 172687,
            "range": "± 10858",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 293833,
            "range": "± 21086",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 436236,
            "range": "± 38067",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5472067,
            "range": "± 297256",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 13903200,
            "range": "± 983635",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 36417528,
            "range": "± 2108548",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 212123224,
            "range": "± 10217272",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 448554662,
            "range": "± 17505593",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2525377,
            "range": "± 41849",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5895591,
            "range": "± 96375",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11518498,
            "range": "± 93687",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 56564217,
            "range": "± 638046",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 112860902,
            "range": "± 845872",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 107480,
            "range": "± 14630",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2353029,
            "range": "± 125809",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1345780,
            "range": "± 41496",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 412023,
            "range": "± 51922",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 783647,
            "range": "± 84071",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1434485,
            "range": "± 172859",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 3126121,
            "range": "± 448795",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5969605,
            "range": "± 782726",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 12660113,
            "range": "± 1240553",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 24201829,
            "range": "± 1795756",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 49238409,
            "range": "± 3468978",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 98944933,
            "range": "± 6864908",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 205500282,
            "range": "± 17592557",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 564116,
            "range": "± 74053",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1146176,
            "range": "± 99158",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2309571,
            "range": "± 200570",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4941560,
            "range": "± 437404",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 9779686,
            "range": "± 827956",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 21205359,
            "range": "± 2242184",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 40882863,
            "range": "± 3241641",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 80537221,
            "range": "± 5572306",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 169300901,
            "range": "± 9454467",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 365547087,
            "range": "± 18652735",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 115622,
            "range": "± 11169",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 129788,
            "range": "± 10448",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 150651,
            "range": "± 16557",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 279602,
            "range": "± 24258",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 518839,
            "range": "± 54717",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5069412,
            "range": "± 301759",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 12489796,
            "range": "± 975718",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 36321585,
            "range": "± 2808800",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 220332562,
            "range": "± 9345934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 486779991,
            "range": "± 37260462",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2510156,
            "range": "± 48012",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5895381,
            "range": "± 154159",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11499579,
            "range": "± 96864",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 56587797,
            "range": "± 584499",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 112646107,
            "range": "± 776541",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 182934,
            "range": "± 31233",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 13285,
            "range": "± 4541",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5931,
            "range": "± 1505",
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
          "id": "00aeb4ed42cdb923c83f93ddd645983ac364599f",
          "message": "ci: bring back daily benchmarks (#777)\n\n* ci: bring back daily benchmarks\r\n\r\n* remove ugly spaces",
          "timestamp": "2022-05-20T16:30:34Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/00aeb4ed42cdb923c83f93ddd645983ac364599f"
        },
        "date": 1653266916557,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 179,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 209,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 120853,
            "range": "± 7771",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3794999,
            "range": "± 200884",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1287389,
            "range": "± 73991",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 236861,
            "range": "± 2962",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 440737,
            "range": "± 6985",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 793150,
            "range": "± 8577",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1509769,
            "range": "± 42507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2958010,
            "range": "± 58380",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5866379,
            "range": "± 162661",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 11749197,
            "range": "± 345364",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 24148110,
            "range": "± 702343",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 48349293,
            "range": "± 2142783",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 96965469,
            "range": "± 2478029",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 115649,
            "range": "± 1899",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 119428,
            "range": "± 1477",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 150916,
            "range": "± 9074",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 272609,
            "range": "± 8573",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 453950,
            "range": "± 33875",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 10365757,
            "range": "± 595388",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 26467559,
            "range": "± 1388869",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 53651217,
            "range": "± 1910433",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 245195629,
            "range": "± 5593007",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 487440673,
            "range": "± 10044395",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2377947,
            "range": "± 32052",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5636799,
            "range": "± 43504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11115334,
            "range": "± 65466",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 54414650,
            "range": "± 181720",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 108304338,
            "range": "± 300592",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 79238,
            "range": "± 1641",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2577141,
            "range": "± 89309",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1279911,
            "range": "± 38545",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 396410,
            "range": "± 44338",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 719493,
            "range": "± 52050",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1374923,
            "range": "± 138150",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2851981,
            "range": "± 384407",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5536619,
            "range": "± 571809",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 11660913,
            "range": "± 1009776",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 23930434,
            "range": "± 1486819",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 48004793,
            "range": "± 2240790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 97070451,
            "range": "± 2948359",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 201073521,
            "range": "± 3476494",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 528937,
            "range": "± 44114",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1132645,
            "range": "± 116007",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2192358,
            "range": "± 160286",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4664445,
            "range": "± 339225",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 9119858,
            "range": "± 591731",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 19178031,
            "range": "± 1130503",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 39728284,
            "range": "± 2339592",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 79516383,
            "range": "± 1954827",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 156285529,
            "range": "± 3684776",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 324519711,
            "range": "± 6604277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 90725,
            "range": "± 1492",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 109803,
            "range": "± 11085",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 127219,
            "range": "± 1661",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 261259,
            "range": "± 2136",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 427035,
            "range": "± 29881",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5878236,
            "range": "± 235269",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 20487856,
            "range": "± 1938830",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 45640821,
            "range": "± 1700153",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 246602019,
            "range": "± 6292637",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 478583038,
            "range": "± 11590810",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2376591,
            "range": "± 21490",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5627143,
            "range": "± 52664",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11083885,
            "range": "± 65110",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 54366974,
            "range": "± 218675",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 108086636,
            "range": "± 308788",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 117167,
            "range": "± 3049",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 3136000,
            "range": "± 92463",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1284606,
            "range": "± 42163",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 231946,
            "range": "± 4732",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 430031,
            "range": "± 4740",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 785134,
            "range": "± 10974",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1492782,
            "range": "± 65143",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2903707,
            "range": "± 60950",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 5725823,
            "range": "± 146211",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 11474225,
            "range": "± 432154",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 23069744,
            "range": "± 903355",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 46020165,
            "range": "± 1024374",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 95286914,
            "range": "± 3447751",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 115159,
            "range": "± 2437",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 118533,
            "range": "± 4988",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 132222,
            "range": "± 1714",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 263205,
            "range": "± 9145",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 420225,
            "range": "± 13137",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 6777758,
            "range": "± 393479",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 16783204,
            "range": "± 1032886",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 41470964,
            "range": "± 1540490",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 222575902,
            "range": "± 5116455",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 484754386,
            "range": "± 13618805",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2375280,
            "range": "± 33928",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5630627,
            "range": "± 41638",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11089083,
            "range": "± 77894",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 54379901,
            "range": "± 161240",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 108397251,
            "range": "± 417923",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 78487,
            "range": "± 1452",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2668688,
            "range": "± 91152",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1270764,
            "range": "± 28754",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 380322,
            "range": "± 36546",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 713282,
            "range": "± 53869",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1327085,
            "range": "± 126713",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2720393,
            "range": "± 287886",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5584287,
            "range": "± 635443",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 11969482,
            "range": "± 1102749",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 24207884,
            "range": "± 1252221",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 45713654,
            "range": "± 1756878",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 93904101,
            "range": "± 2474157",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 192328712,
            "range": "± 4818721",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 529534,
            "range": "± 51593",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1108859,
            "range": "± 87444",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2196671,
            "range": "± 186800",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4521147,
            "range": "± 334964",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 9133842,
            "range": "± 674344",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 19077529,
            "range": "± 1018912",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 38984917,
            "range": "± 2179093",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 77290268,
            "range": "± 1827406",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 154175338,
            "range": "± 3404160",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 326661209,
            "range": "± 8867776",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 96368,
            "range": "± 5518",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 107356,
            "range": "± 3174",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 125880,
            "range": "± 1773",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 263492,
            "range": "± 6762",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 423599,
            "range": "± 8837",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5561332,
            "range": "± 252059",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 14961184,
            "range": "± 814410",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 43190884,
            "range": "± 1953234",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 243609062,
            "range": "± 7232511",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 471341539,
            "range": "± 9957043",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2362107,
            "range": "± 33690",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5653873,
            "range": "± 43315",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11100521,
            "range": "± 57604",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 54335693,
            "range": "± 237532",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 108320338,
            "range": "± 395680",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 103886,
            "range": "± 2809",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 5312,
            "range": "± 1200",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4788,
            "range": "± 658",
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
          "id": "f2025ce16cc2de1a59941cc6dc59b811f4218147",
          "message": "chore(deps): bump Swatinem/rust-cache from 1.3.0 to 1.4.0 (#778)\n\nBumps [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) from 1.3.0 to 1.4.0.\r\n- [Release notes](https://github.com/Swatinem/rust-cache/releases)\r\n- [Changelog](https://github.com/Swatinem/rust-cache/blob/v1/CHANGELOG.md)\r\n- [Commits](https://github.com/Swatinem/rust-cache/compare/v1.3.0...v1.4.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: Swatinem/rust-cache\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-23T13:04:29Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/f2025ce16cc2de1a59941cc6dc59b811f4218147"
        },
        "date": 1653353069590,
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
            "value": 180,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 96469,
            "range": "± 3810",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2581035,
            "range": "± 75826",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1168613,
            "range": "± 36424",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 201466,
            "range": "± 2581",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 373303,
            "range": "± 14411",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 675465,
            "range": "± 7942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1292001,
            "range": "± 36848",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2536712,
            "range": "± 63859",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5028834,
            "range": "± 228657",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 10035967,
            "range": "± 321199",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 20082905,
            "range": "± 578988",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 40895763,
            "range": "± 1200600",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 83173308,
            "range": "± 2289191",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 95857,
            "range": "± 1493",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 100167,
            "range": "± 1630",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 112268,
            "range": "± 1162",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 227351,
            "range": "± 8586",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 365080,
            "range": "± 9160",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9852036,
            "range": "± 338402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 23531290,
            "range": "± 772328",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 45647040,
            "range": "± 972062",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 204895209,
            "range": "± 3157460",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 409978262,
            "range": "± 3008741",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2328173,
            "range": "± 60685",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5537731,
            "range": "± 80138",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10869116,
            "range": "± 59370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53559189,
            "range": "± 80143",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106802751,
            "range": "± 331051",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 67125,
            "range": "± 1132",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2016746,
            "range": "± 8471",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1172262,
            "range": "± 91871",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 345525,
            "range": "± 32203",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 612819,
            "range": "± 69167",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1156295,
            "range": "± 119495",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2557023,
            "range": "± 298004",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 4919314,
            "range": "± 460929",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10006995,
            "range": "± 756762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 20669113,
            "range": "± 1190663",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 41260228,
            "range": "± 2072699",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 83836824,
            "range": "± 2284447",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 172795939,
            "range": "± 3275676",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 454884,
            "range": "± 32544",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 973881,
            "range": "± 55265",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1928377,
            "range": "± 140267",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4084818,
            "range": "± 349607",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8236295,
            "range": "± 409925",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 16902966,
            "range": "± 732879",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 34417539,
            "range": "± 1278740",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 68253069,
            "range": "± 1839701",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 136889182,
            "range": "± 2809113",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 278799979,
            "range": "± 5683926",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 75663,
            "range": "± 1578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 89272,
            "range": "± 2835",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 106095,
            "range": "± 1030",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 219527,
            "range": "± 1961",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 353601,
            "range": "± 2788",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4322088,
            "range": "± 47437",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 15406858,
            "range": "± 501751",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 30722915,
            "range": "± 1210774",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 200916303,
            "range": "± 6557830",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 396346849,
            "range": "± 7673568",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2343293,
            "range": "± 43627",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5540204,
            "range": "± 38678",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10877809,
            "range": "± 96758",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53545716,
            "range": "± 227468",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106806608,
            "range": "± 360154",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 96028,
            "range": "± 2809",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2583722,
            "range": "± 31011",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1167626,
            "range": "± 57550",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 200111,
            "range": "± 3605",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 368777,
            "range": "± 5575",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 665884,
            "range": "± 9542",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1268228,
            "range": "± 17065",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2472079,
            "range": "± 82817",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4929135,
            "range": "± 81665",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9725990,
            "range": "± 307530",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19603600,
            "range": "± 554070",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 39583705,
            "range": "± 903245",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 80425763,
            "range": "± 2078868",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 95636,
            "range": "± 1357",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 99978,
            "range": "± 3764",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 111755,
            "range": "± 1301",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 226535,
            "range": "± 8769",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 368173,
            "range": "± 10328",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5213711,
            "range": "± 129529",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 13732524,
            "range": "± 685645",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 35307349,
            "range": "± 855117",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 188313647,
            "range": "± 3772716",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 414305966,
            "range": "± 5478647",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2317409,
            "range": "± 46630",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5530795,
            "range": "± 40528",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10874238,
            "range": "± 102630",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53524927,
            "range": "± 393514",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106752905,
            "range": "± 227207",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 67615,
            "range": "± 977",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2106710,
            "range": "± 17628",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1153961,
            "range": "± 57921",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 344819,
            "range": "± 76101",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 611246,
            "range": "± 51294",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1141683,
            "range": "± 136527",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2313071,
            "range": "± 285857",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 4703325,
            "range": "± 439930",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 9739724,
            "range": "± 692668",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 20858292,
            "range": "± 1169779",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 40652722,
            "range": "± 2544014",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 82242349,
            "range": "± 2015680",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 168168897,
            "range": "± 3555169",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 448533,
            "range": "± 37383",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 964471,
            "range": "± 70999",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1899177,
            "range": "± 147601",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4093690,
            "range": "± 380272",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8084093,
            "range": "± 538662",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16396909,
            "range": "± 642211",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33993135,
            "range": "± 1382032",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 67245757,
            "range": "± 2497930",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 135204125,
            "range": "± 2449468",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 279599374,
            "range": "± 12626163",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 76562,
            "range": "± 1771",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 91021,
            "range": "± 1975",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 105654,
            "range": "± 2086",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 218400,
            "range": "± 2137",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 351163,
            "range": "± 2713",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4592544,
            "range": "± 88649",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 12377908,
            "range": "± 272436",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 35303894,
            "range": "± 1134835",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 208099459,
            "range": "± 3641121",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 396212227,
            "range": "± 6843891",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2346583,
            "range": "± 46554",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5538569,
            "range": "± 32029",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10879439,
            "range": "± 69247",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53564681,
            "range": "± 262920",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106759151,
            "range": "± 386168",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 86546,
            "range": "± 3774",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4182,
            "range": "± 824",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4115,
            "range": "± 615",
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
          "id": "f2025ce16cc2de1a59941cc6dc59b811f4218147",
          "message": "chore(deps): bump Swatinem/rust-cache from 1.3.0 to 1.4.0 (#778)\n\nBumps [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) from 1.3.0 to 1.4.0.\r\n- [Release notes](https://github.com/Swatinem/rust-cache/releases)\r\n- [Changelog](https://github.com/Swatinem/rust-cache/blob/v1/CHANGELOG.md)\r\n- [Commits](https://github.com/Swatinem/rust-cache/compare/v1.3.0...v1.4.0)\r\n\r\n---\r\nupdated-dependencies:\r\n- dependency-name: Swatinem/rust-cache\r\n  dependency-type: direct:production\r\n  update-type: version-update:semver-minor\r\n...\r\n\r\nSigned-off-by: dependabot[bot] <support@github.com>\r\n\r\nCo-authored-by: dependabot[bot] <49699333+dependabot[bot]@users.noreply.github.com>",
          "timestamp": "2022-05-23T13:04:29Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/f2025ce16cc2de1a59941cc6dc59b811f4218147"
        },
        "date": 1653439485421,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 141,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 160,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 98116,
            "range": "± 4806",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2797052,
            "range": "± 234054",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1178826,
            "range": "± 22139",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 203306,
            "range": "± 4180",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 378654,
            "range": "± 4537",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 687809,
            "range": "± 9005",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1325226,
            "range": "± 41607",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2511353,
            "range": "± 92736",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4857094,
            "range": "± 133014",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 10212535,
            "range": "± 291650",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 20840616,
            "range": "± 519908",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 40144461,
            "range": "± 1140210",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 83016528,
            "range": "± 2785176",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 98642,
            "range": "± 2422",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 104368,
            "range": "± 1433",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 116476,
            "range": "± 5370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 238666,
            "range": "± 11241",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 370701,
            "range": "± 13865",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 7964302,
            "range": "± 308555",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 24104208,
            "range": "± 996673",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 45951774,
            "range": "± 1589572",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 217410670,
            "range": "± 6702253",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 419628181,
            "range": "± 14429480",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2325538,
            "range": "± 12680",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5548570,
            "range": "± 17369",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10944779,
            "range": "± 61450",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53627588,
            "range": "± 463868",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106993191,
            "range": "± 211737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 70242,
            "range": "± 5105",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2240259,
            "range": "± 55299",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1181095,
            "range": "± 30595",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 320951,
            "range": "± 35703",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 637813,
            "range": "± 42505",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1202225,
            "range": "± 146829",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2503779,
            "range": "± 267286",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 4934609,
            "range": "± 481693",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10432658,
            "range": "± 1041438",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 22017288,
            "range": "± 1193988",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 43539370,
            "range": "± 1657527",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 88818313,
            "range": "± 2130506",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 181315867,
            "range": "± 4044973",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 469420,
            "range": "± 87811",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 997860,
            "range": "± 78111",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1970170,
            "range": "± 144056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4253838,
            "range": "± 359250",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8341929,
            "range": "± 490374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17461757,
            "range": "± 865053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 35560924,
            "range": "± 1695564",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 71440201,
            "range": "± 4141267",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 144706124,
            "range": "± 3632724",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 309584186,
            "range": "± 18225740",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 83396,
            "range": "± 2530",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 95304,
            "range": "± 4024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 110640,
            "range": "± 1631",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 225796,
            "range": "± 7654",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 362269,
            "range": "± 11784",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5390089,
            "range": "± 285630",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 20651046,
            "range": "± 701370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 38170567,
            "range": "± 1573067",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 219257993,
            "range": "± 7917956",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 417441415,
            "range": "± 29031381",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2339931,
            "range": "± 23362",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5570042,
            "range": "± 24316",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10958557,
            "range": "± 35064",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53688457,
            "range": "± 435842",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 107005310,
            "range": "± 426281",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 97151,
            "range": "± 2974",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2762182,
            "range": "± 115611",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1183749,
            "range": "± 29403",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 194840,
            "range": "± 4879",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 376803,
            "range": "± 6879",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 681660,
            "range": "± 16426",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1291697,
            "range": "± 42349",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2572903,
            "range": "± 93455",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 5023119,
            "range": "± 92157",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 10076050,
            "range": "± 256176",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19391664,
            "range": "± 608253",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 41730849,
            "range": "± 1165921",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 84081102,
            "range": "± 2037569",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 97446,
            "range": "± 8663",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 102484,
            "range": "± 4682",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 116109,
            "range": "± 5001",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 233491,
            "range": "± 11273",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 373223,
            "range": "± 22481",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5932881,
            "range": "± 175176",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 15592922,
            "range": "± 422424",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 38016448,
            "range": "± 1591368",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 191441228,
            "range": "± 6657626",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 408536051,
            "range": "± 12949952",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2325659,
            "range": "± 31088",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5561421,
            "range": "± 23274",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10935499,
            "range": "± 42741",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53679416,
            "range": "± 148357",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106974153,
            "range": "± 544524",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 71297,
            "range": "± 1749",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2399371,
            "range": "± 73993",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1205225,
            "range": "± 46832",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 344476,
            "range": "± 40522",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 629622,
            "range": "± 70606",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1180160,
            "range": "± 107397",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2481438,
            "range": "± 295128",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 4976575,
            "range": "± 416255",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10572073,
            "range": "± 924335",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 21703108,
            "range": "± 1197403",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 42478592,
            "range": "± 1686451",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 87290840,
            "range": "± 2211741",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 175836794,
            "range": "± 4064721",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 460317,
            "range": "± 26734",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 991012,
            "range": "± 57981",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1948082,
            "range": "± 141064",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4207738,
            "range": "± 298501",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8191407,
            "range": "± 522603",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 17304956,
            "range": "± 837355",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 35025016,
            "range": "± 1316952",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 69227774,
            "range": "± 1904360",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 140715519,
            "range": "± 12542911",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 289924755,
            "range": "± 5397937",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 80943,
            "range": "± 2535",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 94566,
            "range": "± 2895",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 109669,
            "range": "± 5252",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 220011,
            "range": "± 7044",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 356512,
            "range": "± 20390",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5142078,
            "range": "± 201093",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 14175700,
            "range": "± 474376",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 39837879,
            "range": "± 1007308",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 210877760,
            "range": "± 7549391",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 412053241,
            "range": "± 10990832",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2342979,
            "range": "± 11149",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5552912,
            "range": "± 26292",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10925739,
            "range": "± 31116",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53687763,
            "range": "± 143523",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106982416,
            "range": "± 388302",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 93797,
            "range": "± 2511",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4648,
            "range": "± 810",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4444,
            "range": "± 596",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexander Polakov",
            "username": "polachok",
            "email": "polachok@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "47d36b9b1b886422717a341086978ca10cdf7cad",
          "message": "fix: skip leading whitespace (#783)",
          "timestamp": "2022-05-25T21:20:32Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/47d36b9b1b886422717a341086978ca10cdf7cad"
        },
        "date": 1653525908696,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 146,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 168,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 96386,
            "range": "± 2977",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3152107,
            "range": "± 95774",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1167035,
            "range": "± 156653",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 204565,
            "range": "± 5253",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 376573,
            "range": "± 6249",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 681340,
            "range": "± 8345",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1300965,
            "range": "± 37804",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2555340,
            "range": "± 92858",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5073921,
            "range": "± 178758",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 10157713,
            "range": "± 244872",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 20333613,
            "range": "± 566651",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 41563320,
            "range": "± 1332151",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 85019976,
            "range": "± 2398528",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 96872,
            "range": "± 11698",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 101143,
            "range": "± 1105",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 113038,
            "range": "± 1583",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 230309,
            "range": "± 8982",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 368660,
            "range": "± 11447",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9056743,
            "range": "± 386246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 23266981,
            "range": "± 475624",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 45345647,
            "range": "± 1628101",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 209329553,
            "range": "± 3115619",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 410582241,
            "range": "± 2308927",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2414449,
            "range": "± 60815",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5544244,
            "range": "± 27631",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10892531,
            "range": "± 55174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53569797,
            "range": "± 96109",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106827290,
            "range": "± 193778",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 68791,
            "range": "± 1540",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2096371,
            "range": "± 20055",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1175461,
            "range": "± 50671",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 346619,
            "range": "± 72582",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 626129,
            "range": "± 81323",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1192592,
            "range": "± 150125",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2486500,
            "range": "± 260332",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5051761,
            "range": "± 525883",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10351359,
            "range": "± 955689",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 23335232,
            "range": "± 1869256",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 46139610,
            "range": "± 2206043",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 94132592,
            "range": "± 3340602",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 194859367,
            "range": "± 4658246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 487055,
            "range": "± 40980",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1033645,
            "range": "± 92625",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2154211,
            "range": "± 189510",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4540286,
            "range": "± 450037",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8972371,
            "range": "± 666639",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 18812077,
            "range": "± 1100458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 38280763,
            "range": "± 2261603",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 76533225,
            "range": "± 2098980",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 154633417,
            "range": "± 4435087",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 317473588,
            "range": "± 13773011",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 85300,
            "range": "± 6247",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 97446,
            "range": "± 3918",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 111931,
            "range": "± 4719",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 228900,
            "range": "± 6495",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 365642,
            "range": "± 19269",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5014965,
            "range": "± 273896",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 17928434,
            "range": "± 1285539",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 33184910,
            "range": "± 1299152",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 204441470,
            "range": "± 10293652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 375829414,
            "range": "± 10412567",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2353918,
            "range": "± 30276",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5576142,
            "range": "± 38205",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10984539,
            "range": "± 43314",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53607874,
            "range": "± 67011",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106841327,
            "range": "± 464869",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 98822,
            "range": "± 3773",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2623377,
            "range": "± 69849",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1197838,
            "range": "± 24416",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 206039,
            "range": "± 8955",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 380280,
            "range": "± 10936",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 693307,
            "range": "± 33258",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1319853,
            "range": "± 33451",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2634749,
            "range": "± 117205",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 5254558,
            "range": "± 166863",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 10731763,
            "range": "± 391912",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 21708796,
            "range": "± 818384",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 42435161,
            "range": "± 1263406",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 88512414,
            "range": "± 1898488",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 101681,
            "range": "± 1497",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 109096,
            "range": "± 2652",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 119103,
            "range": "± 3383",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 241063,
            "range": "± 8234",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 379062,
            "range": "± 10761",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5828114,
            "range": "± 115460",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 14847203,
            "range": "± 213922",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 36982895,
            "range": "± 873134",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 192551691,
            "range": "± 2916969",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 411371258,
            "range": "± 3421021",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2322798,
            "range": "± 20204",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5550751,
            "range": "± 15471",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10892780,
            "range": "± 31161",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53548745,
            "range": "± 56368",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106759440,
            "range": "± 78304",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 77178,
            "range": "± 6026",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2167034,
            "range": "± 39972",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1193729,
            "range": "± 46889",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 345084,
            "range": "± 30166",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 657590,
            "range": "± 61260",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1255208,
            "range": "± 117888",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2523653,
            "range": "± 303978",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5284363,
            "range": "± 499261",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 11247119,
            "range": "± 760313",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 23439008,
            "range": "± 1168732",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 45325147,
            "range": "± 1931774",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 93441328,
            "range": "± 3040998",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 189276982,
            "range": "± 4640101",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 481300,
            "range": "± 37677",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1032089,
            "range": "± 56411",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2094490,
            "range": "± 199620",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4420859,
            "range": "± 349560",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8884419,
            "range": "± 568610",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 18605648,
            "range": "± 1384392",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 38220709,
            "range": "± 2255051",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 74916801,
            "range": "± 8908748",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 144664536,
            "range": "± 5803022",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 302882285,
            "range": "± 9105821",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 82386,
            "range": "± 4192",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 96876,
            "range": "± 4481",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 114085,
            "range": "± 6409",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 229771,
            "range": "± 6965",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 367721,
            "range": "± 7760",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4964132,
            "range": "± 308352",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 13276377,
            "range": "± 471415",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 39340829,
            "range": "± 1635539",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 209337793,
            "range": "± 4759059",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 399607224,
            "range": "± 6856695",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2356495,
            "range": "± 28719",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5574307,
            "range": "± 35354",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10960881,
            "range": "± 42199",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53622155,
            "range": "± 62007",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106847809,
            "range": "± 98879",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 95091,
            "range": "± 5518",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4768,
            "range": "± 1451",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4242,
            "range": "± 2107",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Alexander Polakov",
            "username": "polachok",
            "email": "polachok@users.noreply.github.com"
          },
          "committer": {
            "name": "GitHub",
            "username": "web-flow",
            "email": "noreply@github.com"
          },
          "id": "47d36b9b1b886422717a341086978ca10cdf7cad",
          "message": "fix: skip leading whitespace (#783)",
          "timestamp": "2022-05-25T21:20:32Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/47d36b9b1b886422717a341086978ca10cdf7cad"
        },
        "date": 1653612318291,
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
            "value": 170,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 104171,
            "range": "± 9985",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3225687,
            "range": "± 105981",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1189054,
            "range": "± 14188",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 203535,
            "range": "± 3139",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 380920,
            "range": "± 7695",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 687505,
            "range": "± 10322",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1312139,
            "range": "± 40220",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2600683,
            "range": "± 121798",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5236166,
            "range": "± 132126",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 10377353,
            "range": "± 400919",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 21230063,
            "range": "± 656642",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 43638169,
            "range": "± 1535061",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 88424206,
            "range": "± 17276191",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 99670,
            "range": "± 1810",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 103778,
            "range": "± 1074",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 115755,
            "range": "± 1613",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 226293,
            "range": "± 9165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 377758,
            "range": "± 12191",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 8544225,
            "range": "± 236107",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 21560961,
            "range": "± 899316",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 45920066,
            "range": "± 1576696",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 210826510,
            "range": "± 2283013",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 421535366,
            "range": "± 4234241",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2324534,
            "range": "± 9164",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5550119,
            "range": "± 16363",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10950701,
            "range": "± 38571",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53621386,
            "range": "± 49833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106854464,
            "range": "± 114094",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 70170,
            "range": "± 1329",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2109565,
            "range": "± 17533",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1198314,
            "range": "± 41229",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 344029,
            "range": "± 54469",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 622118,
            "range": "± 51812",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1177220,
            "range": "± 134169",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2530226,
            "range": "± 309955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5022093,
            "range": "± 434693",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10147419,
            "range": "± 841032",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 21772527,
            "range": "± 1188370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 42922987,
            "range": "± 2130685",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 87589071,
            "range": "± 1655792",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 182130982,
            "range": "± 5290731",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 472126,
            "range": "± 86529",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 992960,
            "range": "± 187249",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1968020,
            "range": "± 120715",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4193782,
            "range": "± 373729",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8457062,
            "range": "± 631072",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17273509,
            "range": "± 880156",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 35422978,
            "range": "± 3463354",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 70638151,
            "range": "± 1879818",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 144914288,
            "range": "± 3641606",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 293628227,
            "range": "± 7247159",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 85184,
            "range": "± 3357",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 97902,
            "range": "± 3515",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 112105,
            "range": "± 1796",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 224086,
            "range": "± 6023",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 358168,
            "range": "± 7157",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5212342,
            "range": "± 176113",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 17734202,
            "range": "± 1133616",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 35108872,
            "range": "± 1111217",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 208683132,
            "range": "± 8543679",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 414612366,
            "range": "± 7041010",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2351691,
            "range": "± 8402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5574484,
            "range": "± 26045",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11042040,
            "range": "± 22392",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53643733,
            "range": "± 411129",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106907680,
            "range": "± 107215",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 99872,
            "range": "± 3191",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2471347,
            "range": "± 41296",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1184082,
            "range": "± 12295",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 199581,
            "range": "± 2699",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 372798,
            "range": "± 7390",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 673251,
            "range": "± 26701",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1291969,
            "range": "± 36088",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2539084,
            "range": "± 135621",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4937198,
            "range": "± 138242",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 10018417,
            "range": "± 296692",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19968211,
            "range": "± 568750",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 41369023,
            "range": "± 1198355",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 83654101,
            "range": "± 2235973",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 100281,
            "range": "± 2078",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 102458,
            "range": "± 1536",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 114912,
            "range": "± 1650",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 229681,
            "range": "± 10568",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 375833,
            "range": "± 12158",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5891140,
            "range": "± 105539",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 15317412,
            "range": "± 272528",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 37232150,
            "range": "± 832082",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 196951562,
            "range": "± 2458640",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 429409045,
            "range": "± 7734633",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2327480,
            "range": "± 9955",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5588012,
            "range": "± 20459",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10979325,
            "range": "± 34257",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53630962,
            "range": "± 106817",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106849621,
            "range": "± 78149",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 71270,
            "range": "± 1844",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2222845,
            "range": "± 15366",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1213768,
            "range": "± 34144",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 344111,
            "range": "± 29248",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 624917,
            "range": "± 71561",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1172736,
            "range": "± 114219",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2506832,
            "range": "± 269411",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 4858402,
            "range": "± 484640",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10067482,
            "range": "± 931415",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 21183390,
            "range": "± 1352741",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 41834359,
            "range": "± 1507749",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 84681703,
            "range": "± 2166004",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 172385488,
            "range": "± 3607413",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 462992,
            "range": "± 51982",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 980506,
            "range": "± 54363",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1934649,
            "range": "± 136486",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4162805,
            "range": "± 406134",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8222175,
            "range": "± 512882",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16964477,
            "range": "± 1099539",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 34672610,
            "range": "± 1124902",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 68935861,
            "range": "± 10298303",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 139844427,
            "range": "± 2526767",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 288801314,
            "range": "± 5310452",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 80144,
            "range": "± 3269",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 93796,
            "range": "± 2270",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 110099,
            "range": "± 2259",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 222932,
            "range": "± 3785",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 355488,
            "range": "± 4278",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4727873,
            "range": "± 187788",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 13964206,
            "range": "± 400518",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 37663941,
            "range": "± 781770",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 213837003,
            "range": "± 3930356",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 414977962,
            "range": "± 6686230",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2346664,
            "range": "± 13141",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5566917,
            "range": "± 19397",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10930571,
            "range": "± 50200",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53598320,
            "range": "± 43620",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106865724,
            "range": "± 220086",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 92589,
            "range": "± 2786",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4843,
            "range": "± 898",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4431,
            "range": "± 1029",
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
          "id": "9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c",
          "message": "Implement `ping-pong` for WebSocket server (#782)\n\n* ws-server: Implement `ping-ping`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Set builder's ping_interval\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Handle just `ping` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Simplify `select`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Use `futures_util::select` instead of `select!` macro\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Avoid pinning the delay\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Log when a `Pong` frame is received\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Use tokio for submitting pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-05-27T14:30:59Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c"
        },
        "date": 1653698610088,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 128,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 151,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 94908,
            "range": "± 4263",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2886445,
            "range": "± 162795",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1215992,
            "range": "± 64462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 202326,
            "range": "± 6273",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 382586,
            "range": "± 27442",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 657190,
            "range": "± 8050",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1257421,
            "range": "± 45668",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2460863,
            "range": "± 104595",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4875718,
            "range": "± 250196",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9725662,
            "range": "± 291095",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 19483771,
            "range": "± 572835",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 39709530,
            "range": "± 952551",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 80654168,
            "range": "± 2574841",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 98190,
            "range": "± 1610",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 102723,
            "range": "± 1535",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 114954,
            "range": "± 1503",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 234620,
            "range": "± 10700",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 341799,
            "range": "± 14176",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 8880612,
            "range": "± 215741",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 21099512,
            "range": "± 700903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 41674472,
            "range": "± 1556268",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 191523437,
            "range": "± 6247915",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 378449600,
            "range": "± 7350659",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2397494,
            "range": "± 56216",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5605014,
            "range": "± 77896",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10974152,
            "range": "± 85572",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53960056,
            "range": "± 261322",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 107399095,
            "range": "± 380839",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 70099,
            "range": "± 1282",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2050118,
            "range": "± 53646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1211264,
            "range": "± 79203",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 325662,
            "range": "± 46243",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 605888,
            "range": "± 82773",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1217024,
            "range": "± 151531",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2514320,
            "range": "± 304209",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5022596,
            "range": "± 591333",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10531524,
            "range": "± 927889",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 20673329,
            "range": "± 1190177",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 41927363,
            "range": "± 1690589",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 86688638,
            "range": "± 2968460",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 177483752,
            "range": "± 6176645",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 468177,
            "range": "± 74944",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1001413,
            "range": "± 111159",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1928649,
            "range": "± 176581",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4295527,
            "range": "± 317086",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8091458,
            "range": "± 638537",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 16750444,
            "range": "± 1015799",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 35224637,
            "range": "± 1256815",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 69927167,
            "range": "± 2130390",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 142765780,
            "range": "± 2735694",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 303274320,
            "range": "± 6672041",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 79956,
            "range": "± 2048",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 95227,
            "range": "± 3211",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 110611,
            "range": "± 1117",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 224982,
            "range": "± 8413",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 340883,
            "range": "± 2531",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4339408,
            "range": "± 69232",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 16781887,
            "range": "± 499172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 33570300,
            "range": "± 915396",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 194427976,
            "range": "± 2042432",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 370744794,
            "range": "± 8903608",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2351389,
            "range": "± 34922",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5619556,
            "range": "± 52056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10989497,
            "range": "± 74829",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53783497,
            "range": "± 189489",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 107221143,
            "range": "± 416493",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 93911,
            "range": "± 2441",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2419665,
            "range": "± 65475",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1233799,
            "range": "± 62610",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 183402,
            "range": "± 2611",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 355407,
            "range": "± 10626",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 647291,
            "range": "± 5831",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1292859,
            "range": "± 34793",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2410325,
            "range": "± 103312",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4729149,
            "range": "± 144081",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9442341,
            "range": "± 198021",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19286541,
            "range": "± 829020",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 38906981,
            "range": "± 1152393",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 77597658,
            "range": "± 1860827",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 97030,
            "range": "± 1385",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 102178,
            "range": "± 1028",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 114427,
            "range": "± 1043",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 236500,
            "range": "± 9135",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 369567,
            "range": "± 18263",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 4717447,
            "range": "± 83403",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 12675709,
            "range": "± 421149",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 33015817,
            "range": "± 745631",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 174812494,
            "range": "± 4407476",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 378058340,
            "range": "± 4270794",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2388427,
            "range": "± 52785",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5545696,
            "range": "± 77468",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10926122,
            "range": "± 71006",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53912584,
            "range": "± 519962",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 107423610,
            "range": "± 946012",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 71064,
            "range": "± 937",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2019944,
            "range": "± 47590",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1236932,
            "range": "± 65044",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 311601,
            "range": "± 72432",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 570350,
            "range": "± 46143",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1137605,
            "range": "± 117388",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2442193,
            "range": "± 254147",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 4988239,
            "range": "± 474686",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 9676040,
            "range": "± 928673",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 21047585,
            "range": "± 1388333",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 41607212,
            "range": "± 1788689",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 84381753,
            "range": "± 2100343",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 172006154,
            "range": "± 2492793",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 461258,
            "range": "± 36884",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 991054,
            "range": "± 50940",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1849269,
            "range": "± 133602",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4106974,
            "range": "± 308205",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8245130,
            "range": "± 429928",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16703976,
            "range": "± 689374",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 34697427,
            "range": "± 1405533",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 68026179,
            "range": "± 2006572",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 138138491,
            "range": "± 2865449",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 286569092,
            "range": "± 5000979",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 79557,
            "range": "± 1868",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 95220,
            "range": "± 1234",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 109958,
            "range": "± 1324",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 212387,
            "range": "± 3355",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 336934,
            "range": "± 8606",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4359307,
            "range": "± 99541",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 11413504,
            "range": "± 214650",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 33739607,
            "range": "± 889683",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 192164302,
            "range": "± 2441793",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 350384722,
            "range": "± 13638927",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2417235,
            "range": "± 47347",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5605288,
            "range": "± 67088",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10963233,
            "range": "± 74761",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53899353,
            "range": "± 252328",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 107351036,
            "range": "± 526608",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 89448,
            "range": "± 2778",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4328,
            "range": "± 1331",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3526,
            "range": "± 689",
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
          "id": "9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c",
          "message": "Implement `ping-pong` for WebSocket server (#782)\n\n* ws-server: Implement `ping-ping`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Set builder's ping_interval\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Handle just `ping` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Simplify `select`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Use `futures_util::select` instead of `select!` macro\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Avoid pinning the delay\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Log when a `Pong` frame is received\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Use tokio for submitting pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-05-27T14:30:59Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c"
        },
        "date": 1653785464112,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 189,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 224,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 149292,
            "range": "± 17892",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3341971,
            "range": "± 198212",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1365798,
            "range": "± 103989",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 316836,
            "range": "± 16778",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 590484,
            "range": "± 49872",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1047289,
            "range": "± 76784",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1994956,
            "range": "± 115993",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 3890813,
            "range": "± 227292",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 7965072,
            "range": "± 380021",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 15842239,
            "range": "± 1068938",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30863630,
            "range": "± 1073619",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 64650676,
            "range": "± 2137666",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 128295840,
            "range": "± 3679244",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 150480,
            "range": "± 8108",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 163218,
            "range": "± 41075",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 176641,
            "range": "± 9996",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 310317,
            "range": "± 20635",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 492973,
            "range": "± 27290",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 8969658,
            "range": "± 488785",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 22784314,
            "range": "± 1640984",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 50676154,
            "range": "± 2482695",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 240692641,
            "range": "± 6780214",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 487598159,
            "range": "± 10889327",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2510550,
            "range": "± 40570",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5882242,
            "range": "± 73810",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11457991,
            "range": "± 178301",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 56472472,
            "range": "± 408098",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 112783422,
            "range": "± 785033",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 111908,
            "range": "± 8725",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2628716,
            "range": "± 101943",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1341842,
            "range": "± 37420",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 451009,
            "range": "± 59333",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 851946,
            "range": "± 99992",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1577150,
            "range": "± 164278",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3315908,
            "range": "± 533956",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 6458221,
            "range": "± 728954",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 14098464,
            "range": "± 1385975",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 28306580,
            "range": "± 2188888",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 56376259,
            "range": "± 3237859",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 114514978,
            "range": "± 3708586",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 240377523,
            "range": "± 8005515",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 636027,
            "range": "± 70895",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1422463,
            "range": "± 112558",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2627583,
            "range": "± 252066",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5684935,
            "range": "± 492850",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 11062903,
            "range": "± 1031639",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 23562825,
            "range": "± 1641979",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 47074398,
            "range": "± 3149240",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 94492990,
            "range": "± 3712458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 201633126,
            "range": "± 8342337",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 405830093,
            "range": "± 37172424",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 127455,
            "range": "± 9803",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 143082,
            "range": "± 9953",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 160595,
            "range": "± 12075",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 312548,
            "range": "± 16704",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 543874,
            "range": "± 75103",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5247569,
            "range": "± 199996",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 18609589,
            "range": "± 1398374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 41954465,
            "range": "± 1965350",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 221690075,
            "range": "± 7544983",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 479362255,
            "range": "± 24411050",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2482371,
            "range": "± 36876",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5851045,
            "range": "± 103890",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11468114,
            "range": "± 74866",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 56456280,
            "range": "± 425344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 112650169,
            "range": "± 797741",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 149626,
            "range": "± 11268",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 3176555,
            "range": "± 119609",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1363889,
            "range": "± 25521",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 309618,
            "range": "± 18806",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 580549,
            "range": "± 35033",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1006459,
            "range": "± 68506",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1949732,
            "range": "± 73737",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3794841,
            "range": "± 129270",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 7700212,
            "range": "± 388304",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 16016915,
            "range": "± 1141299",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 31890826,
            "range": "± 1398605",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 63077455,
            "range": "± 3856625",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 125939264,
            "range": "± 4204923",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 153022,
            "range": "± 19588",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 159076,
            "range": "± 18711",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 175671,
            "range": "± 12145",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 322486,
            "range": "± 24406",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 510253,
            "range": "± 114026",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 7123792,
            "range": "± 416798",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 17334746,
            "range": "± 695302",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 42447025,
            "range": "± 1196598",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 229148630,
            "range": "± 6681135",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 503197596,
            "range": "± 13253540",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2489837,
            "range": "± 61691",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5859719,
            "range": "± 103591",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11547580,
            "range": "± 190396",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 56199042,
            "range": "± 984741",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 112796473,
            "range": "± 1094007",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 113661,
            "range": "± 10809",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2714415,
            "range": "± 108893",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1326567,
            "range": "± 34414",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 469269,
            "range": "± 49867",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 862241,
            "range": "± 73463",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1564619,
            "range": "± 194960",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 3319893,
            "range": "± 512637",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 6502269,
            "range": "± 726828",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 13900918,
            "range": "± 1258986",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 27728729,
            "range": "± 2854788",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 55588049,
            "range": "± 3404594",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 115606695,
            "range": "± 4005101",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 230967871,
            "range": "± 7693710",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 616646,
            "range": "± 83969",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1308310,
            "range": "± 113952",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2618870,
            "range": "± 270960",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5310651,
            "range": "± 449014",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 10950085,
            "range": "± 1048830",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 23183544,
            "range": "± 1385017",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 45430090,
            "range": "± 2440663",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 91906488,
            "range": "± 4299009",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 186975071,
            "range": "± 9093726",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 406342302,
            "range": "± 14158240",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 132995,
            "range": "± 11818",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 144449,
            "range": "± 13385",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 164248,
            "range": "± 12739",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 309440,
            "range": "± 22297",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 493699,
            "range": "± 41296",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5939388,
            "range": "± 298396",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 15431544,
            "range": "± 681067",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 43166300,
            "range": "± 2019386",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 249324041,
            "range": "± 8018938",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 505961071,
            "range": "± 11695936",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2490467,
            "range": "± 41667",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5846579,
            "range": "± 69128",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11470207,
            "range": "± 121278",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 56462367,
            "range": "± 321560",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 112699836,
            "range": "± 700356",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 163455,
            "range": "± 16458",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 6905,
            "range": "± 1990",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5332,
            "range": "± 807",
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
          "id": "9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c",
          "message": "Implement `ping-pong` for WebSocket server (#782)\n\n* ws-server: Implement `ping-ping`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Set builder's ping_interval\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Handle just `ping` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Simplify `select`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Use `futures_util::select` instead of `select!` macro\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Avoid pinning the delay\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Log when a `Pong` frame is received\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Use tokio for submitting pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-05-27T14:30:59Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c"
        },
        "date": 1653871715165,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 193,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 210,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 124742,
            "range": "± 13223",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3563449,
            "range": "± 283652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1285967,
            "range": "± 34221",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 237487,
            "range": "± 11988",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 444571,
            "range": "± 23862",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 790434,
            "range": "± 26024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1511256,
            "range": "± 64032",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2878913,
            "range": "± 90861",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5852389,
            "range": "± 202777",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 11698659,
            "range": "± 266661",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 23250312,
            "range": "± 603369",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 47702238,
            "range": "± 1181028",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 96121473,
            "range": "± 2577251",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 116089,
            "range": "± 16206",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 120693,
            "range": "± 3501",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 132216,
            "range": "± 4948",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 254576,
            "range": "± 21442",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 412217,
            "range": "± 37025",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9727483,
            "range": "± 723180",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 25473576,
            "range": "± 1206415",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 52065265,
            "range": "± 3816747",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 245827883,
            "range": "± 8189915",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 472709902,
            "range": "± 13673961",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2424899,
            "range": "± 79701",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5652270,
            "range": "± 88023",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11137653,
            "range": "± 126062",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 54309839,
            "range": "± 461566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 108003528,
            "range": "± 393298",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 75874,
            "range": "± 4688",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2644467,
            "range": "± 115433",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1292882,
            "range": "± 44002",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 395275,
            "range": "± 48465",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 706720,
            "range": "± 61572",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1299341,
            "range": "± 162110",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2672879,
            "range": "± 370838",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5373581,
            "range": "± 607482",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 11150386,
            "range": "± 1158843",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 22172307,
            "range": "± 1580105",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 46856873,
            "range": "± 2621856",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 96291750,
            "range": "± 3870248",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 193742989,
            "range": "± 6116291",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 520534,
            "range": "± 42817",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1118082,
            "range": "± 50693",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2160891,
            "range": "± 170833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4682687,
            "range": "± 427426",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 9223405,
            "range": "± 664140",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 18596152,
            "range": "± 1192245",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 37489813,
            "range": "± 2415250",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 75470536,
            "range": "± 3463370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 155798789,
            "range": "± 4683168",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 325021004,
            "range": "± 10238691",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 94003,
            "range": "± 19990",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 104640,
            "range": "± 3327",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 124541,
            "range": "± 5206",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 257269,
            "range": "± 17782",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 407394,
            "range": "± 24845",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5592799,
            "range": "± 338886",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 20182047,
            "range": "± 1448613",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 43079909,
            "range": "± 1481508",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 237578098,
            "range": "± 6779330",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 470336813,
            "range": "± 12118154",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2416687,
            "range": "± 39002",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5621623,
            "range": "± 78077",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11020974,
            "range": "± 73176",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 54028167,
            "range": "± 141243",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 107963374,
            "range": "± 346633",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 118132,
            "range": "± 6301",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 3197881,
            "range": "± 160390",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1296481,
            "range": "± 62657",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 230433,
            "range": "± 6665",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 425074,
            "range": "± 10412",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 764695,
            "range": "± 68289",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1444244,
            "range": "± 59950",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2761801,
            "range": "± 100808",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 5460044,
            "range": "± 253677",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 10900738,
            "range": "± 414058",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 21560609,
            "range": "± 760379",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 44496287,
            "range": "± 1657359",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 87767312,
            "range": "± 2710013",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 115314,
            "range": "± 4588",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 118265,
            "range": "± 6496",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 127333,
            "range": "± 23708",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 254515,
            "range": "± 16259",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 415103,
            "range": "± 56818",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 6296463,
            "range": "± 356240",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 16576972,
            "range": "± 954424",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 41891163,
            "range": "± 1899568",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 217308037,
            "range": "± 7467711",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 483881136,
            "range": "± 15208594",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2385637,
            "range": "± 44078",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5685477,
            "range": "± 73566",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11022185,
            "range": "± 99615",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 54193123,
            "range": "± 487352",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 108330504,
            "range": "± 456957",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 77254,
            "range": "± 6022",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2788219,
            "range": "± 114498",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1281645,
            "range": "± 146296",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 371534,
            "range": "± 83483",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 679546,
            "range": "± 68763",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1249719,
            "range": "± 148048",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2705713,
            "range": "± 323181",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5408546,
            "range": "± 704915",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10916952,
            "range": "± 987413",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 23017459,
            "range": "± 1464462",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 45788053,
            "range": "± 7702191",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 90507617,
            "range": "± 3973725",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 189643383,
            "range": "± 6383420",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 514532,
            "range": "± 43131",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1108482,
            "range": "± 84603",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2088480,
            "range": "± 180966",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4538568,
            "range": "± 322691",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8906429,
            "range": "± 599272",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 18175700,
            "range": "± 1379119",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 37527922,
            "range": "± 2415820",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 74414059,
            "range": "± 2785511",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 150912130,
            "range": "± 5615640",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 312356896,
            "range": "± 11581276",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 87144,
            "range": "± 4691",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 104328,
            "range": "± 5487",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 126506,
            "range": "± 4676",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 257079,
            "range": "± 22302",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 413886,
            "range": "± 24348",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5477261,
            "range": "± 226782",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 15585663,
            "range": "± 741552",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 44436443,
            "range": "± 1614988",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 245188175,
            "range": "± 7418970",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 466335861,
            "range": "± 12316740",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2383009,
            "range": "± 46802",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5622964,
            "range": "± 59783",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11066992,
            "range": "± 95747",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 54093380,
            "range": "± 189045",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 108227459,
            "range": "± 567559",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 98345,
            "range": "± 5003",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 5045,
            "range": "± 875",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4502,
            "range": "± 596",
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
          "id": "9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c",
          "message": "Implement `ping-pong` for WebSocket server (#782)\n\n* ws-server: Implement `ping-ping`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Set builder's ping_interval\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Handle just `ping` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Simplify `select`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Use `futures_util::select` instead of `select!` macro\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Avoid pinning the delay\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Log when a `Pong` frame is received\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Use tokio for submitting pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-05-27T14:30:59Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c"
        },
        "date": 1653957863027,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 141,
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
            "name": "sync/http_round_trip/fast_call",
            "value": 97229,
            "range": "± 4148",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2483099,
            "range": "± 174032",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1242134,
            "range": "± 34086",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 192228,
            "range": "± 1912",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 355917,
            "range": "± 6341",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 641313,
            "range": "± 5376",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1226865,
            "range": "± 27419",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2405477,
            "range": "± 103572",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4759825,
            "range": "± 124615",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9477310,
            "range": "± 263640",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 19134854,
            "range": "± 762457",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 38517084,
            "range": "± 1415596",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 77564813,
            "range": "± 1872022",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 96388,
            "range": "± 2341",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 95381,
            "range": "± 2962",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 107774,
            "range": "± 2801",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 214581,
            "range": "± 5421",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 343309,
            "range": "± 4024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 7541225,
            "range": "± 429101",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 18720841,
            "range": "± 1015233",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 38206345,
            "range": "± 816255",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 187686252,
            "range": "± 1755655",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 371303919,
            "range": "± 3038344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2316738,
            "range": "± 42575",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5531480,
            "range": "± 105732",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10918943,
            "range": "± 105324",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53750017,
            "range": "± 388512",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 107420904,
            "range": "± 599729",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 64483,
            "range": "± 986",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2142494,
            "range": "± 11402",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1187279,
            "range": "± 31813",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 364801,
            "range": "± 59687",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 676907,
            "range": "± 100890",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1363005,
            "range": "± 178186",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2722079,
            "range": "± 381631",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5369964,
            "range": "± 545487",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10764719,
            "range": "± 968854",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 22353999,
            "range": "± 1192745",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 43513018,
            "range": "± 2260935",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 87927842,
            "range": "± 3365943",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 179002936,
            "range": "± 3438517",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 499874,
            "range": "± 98566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1020399,
            "range": "± 83522",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2101436,
            "range": "± 252344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4323960,
            "range": "± 421434",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8528223,
            "range": "± 590173",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17572199,
            "range": "± 909169",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 35756325,
            "range": "± 1208106",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 71056193,
            "range": "± 2362043",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 141723807,
            "range": "± 3043498",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 294230300,
            "range": "± 5784832",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 73059,
            "range": "± 1371",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 82230,
            "range": "± 2292",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 102633,
            "range": "± 1300",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 223856,
            "range": "± 2024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 361147,
            "range": "± 7665",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4631535,
            "range": "± 41540",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 14547290,
            "range": "± 301356",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 33135047,
            "range": "± 628424",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 161474137,
            "range": "± 5757633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 341539602,
            "range": "± 16052475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2260157,
            "range": "± 30582",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5512062,
            "range": "± 59647",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11003804,
            "range": "± 117366",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53940122,
            "range": "± 373096",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106670044,
            "range": "± 542295",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 96894,
            "range": "± 2044",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2587301,
            "range": "± 18282",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1240278,
            "range": "± 40386",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 192712,
            "range": "± 2643",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 357473,
            "range": "± 13480",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 639838,
            "range": "± 6588",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1221292,
            "range": "± 38697",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2387987,
            "range": "± 69981",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4728679,
            "range": "± 207544",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9399812,
            "range": "± 284899",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 18838503,
            "range": "± 563934",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 37792208,
            "range": "± 1036415",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 76026112,
            "range": "± 1963925",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 96822,
            "range": "± 2466",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 97109,
            "range": "± 3027",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 111050,
            "range": "± 3973",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 213935,
            "range": "± 3571",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 345677,
            "range": "± 4844",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5049332,
            "range": "± 47095",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 11997560,
            "range": "± 177561",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 30421473,
            "range": "± 577968",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 168063123,
            "range": "± 1219799",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 366641321,
            "range": "± 2874648",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2234912,
            "range": "± 14213",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5503281,
            "range": "± 59211",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11058612,
            "range": "± 109567",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53954713,
            "range": "± 393408",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 107495628,
            "range": "± 485624",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 63010,
            "range": "± 2607",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2169959,
            "range": "± 25164",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1183342,
            "range": "± 28524",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 363605,
            "range": "± 58561",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 663085,
            "range": "± 71400",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1310518,
            "range": "± 167626",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2742211,
            "range": "± 383457",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5597992,
            "range": "± 663852",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10775933,
            "range": "± 897113",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 22209934,
            "range": "± 1256908",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 43283924,
            "range": "± 1988714",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 86593331,
            "range": "± 2359702",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 176139317,
            "range": "± 3578541",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 503457,
            "range": "± 72781",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1021195,
            "range": "± 74217",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2050705,
            "range": "± 236901",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4302360,
            "range": "± 394946",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8482029,
            "range": "± 614251",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 17412513,
            "range": "± 919618",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 35822657,
            "range": "± 1398620",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 70829510,
            "range": "± 3204917",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 141415409,
            "range": "± 3101474",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 298140481,
            "range": "± 13727385",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 72721,
            "range": "± 1056",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 81393,
            "range": "± 1225",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 103048,
            "range": "± 1594",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 222339,
            "range": "± 2529",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 359047,
            "range": "± 1561",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4616880,
            "range": "± 25848",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 11658973,
            "range": "± 202791",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 32108202,
            "range": "± 1501311",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 187489243,
            "range": "± 2479928",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 359982077,
            "range": "± 6504964",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2279219,
            "range": "± 39616",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5522103,
            "range": "± 57637",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11024244,
            "range": "± 117035",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53936171,
            "range": "± 326517",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 107675751,
            "range": "± 514593",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 81466,
            "range": "± 2853",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4381,
            "range": "± 2088",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3503,
            "range": "± 817",
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
          "id": "9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c",
          "message": "Implement `ping-pong` for WebSocket server (#782)\n\n* ws-server: Implement `ping-ping`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Set builder's ping_interval\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Handle just `ping` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Simplify `select`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Use `futures_util::select` instead of `select!` macro\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Avoid pinning the delay\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Log when a `Pong` frame is received\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws-server: Use tokio for submitting pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-05-27T14:30:59Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/9fe25b1cf24a3bd6eba222a96ea1d9ab6523a80c"
        },
        "date": 1654044674236,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 194,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 222,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 160147,
            "range": "± 38911",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3470249,
            "range": "± 273194",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1381336,
            "range": "± 90734",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 325610,
            "range": "± 29789",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 585726,
            "range": "± 32924",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1061927,
            "range": "± 124269",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2002187,
            "range": "± 117612",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 4022894,
            "range": "± 248859",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 7891945,
            "range": "± 459636",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 16228316,
            "range": "± 1019878",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 32320276,
            "range": "± 1300956",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 67239590,
            "range": "± 3986820",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 134046649,
            "range": "± 4385106",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 162009,
            "range": "± 15880",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 176334,
            "range": "± 20983",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 192722,
            "range": "± 76451",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 321490,
            "range": "± 33178",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 512441,
            "range": "± 61248",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 11793390,
            "range": "± 578184",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 27804567,
            "range": "± 1750716",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 54288827,
            "range": "± 2802314",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 246511983,
            "range": "± 7648670",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 492234207,
            "range": "± 17503462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2524945,
            "range": "± 67078",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5898240,
            "range": "± 77835",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11508897,
            "range": "± 78123",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 56489531,
            "range": "± 606239",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 112571815,
            "range": "± 648404",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 116085,
            "range": "± 13766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2738429,
            "range": "± 182226",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1347033,
            "range": "± 47658",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 467266,
            "range": "± 83320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 880448,
            "range": "± 104543",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1608117,
            "range": "± 258041",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3453002,
            "range": "± 560593",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 6909302,
            "range": "± 780973",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 14284312,
            "range": "± 1669966",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 31141939,
            "range": "± 3447955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 59428391,
            "range": "± 4272727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 115784791,
            "range": "± 4657424",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 242961304,
            "range": "± 7533559",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 641108,
            "range": "± 117936",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1309101,
            "range": "± 128907",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2620957,
            "range": "± 292439",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5527974,
            "range": "± 644601",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 11119649,
            "range": "± 1102842",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 23704266,
            "range": "± 1627396",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 46622728,
            "range": "± 2593147",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 93885360,
            "range": "± 4497682",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 191267474,
            "range": "± 7520851",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 396024803,
            "range": "± 13234267",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 127124,
            "range": "± 17396",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 142449,
            "range": "± 12866",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 161884,
            "range": "± 11251",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 313770,
            "range": "± 49882",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 510620,
            "range": "± 51480",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5605244,
            "range": "± 473915",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 19954501,
            "range": "± 1928032",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 43295441,
            "range": "± 2616183",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 243707754,
            "range": "± 7744578",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 483001076,
            "range": "± 14359311",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2489719,
            "range": "± 89336",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5857358,
            "range": "± 69249",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11474368,
            "range": "± 74559",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 56450685,
            "range": "± 1111312",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 112594320,
            "range": "± 626664",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 154083,
            "range": "± 31115",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 3034505,
            "range": "± 144103",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1375677,
            "range": "± 57155",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 338445,
            "range": "± 43424",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 622744,
            "range": "± 76290",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1059847,
            "range": "± 46099",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1970490,
            "range": "± 94268",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3919838,
            "range": "± 286460",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 7757347,
            "range": "± 396649",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 15738009,
            "range": "± 666094",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 31422168,
            "range": "± 1234013",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 65445949,
            "range": "± 3892849",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 128101335,
            "range": "± 4368074",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 161966,
            "range": "± 13647",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 170463,
            "range": "± 13706",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 192141,
            "range": "± 34367",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 324958,
            "range": "± 27021",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 503070,
            "range": "± 111178",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 7040947,
            "range": "± 390887",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 17182019,
            "range": "± 963376",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 43605037,
            "range": "± 1817218",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 227404166,
            "range": "± 7397740",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 492236197,
            "range": "± 13543033",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2510872,
            "range": "± 38786",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5910430,
            "range": "± 102709",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11543141,
            "range": "± 144496",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 56578490,
            "range": "± 329662",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 113031338,
            "range": "± 1039530",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 111831,
            "range": "± 26446",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2558302,
            "range": "± 86746",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1345905,
            "range": "± 36684",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 465636,
            "range": "± 58978",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 846958,
            "range": "± 92906",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1560584,
            "range": "± 203888",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 3619628,
            "range": "± 780048",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 6581052,
            "range": "± 960071",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 14373439,
            "range": "± 1156361",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 28207268,
            "range": "± 2108584",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 54102852,
            "range": "± 3427533",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 110628373,
            "range": "± 3880977",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 228083577,
            "range": "± 8161839",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 614115,
            "range": "± 125587",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1300345,
            "range": "± 106152",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2662443,
            "range": "± 283967",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5479761,
            "range": "± 539233",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 10792267,
            "range": "± 1275152",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 23243460,
            "range": "± 1841493",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 45519564,
            "range": "± 2354590",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 90528204,
            "range": "± 3857257",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 181899457,
            "range": "± 6813998",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 397432657,
            "range": "± 14559592",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 127645,
            "range": "± 13489",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 139495,
            "range": "± 14110",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 164573,
            "range": "± 13820",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 320359,
            "range": "± 44273",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 564828,
            "range": "± 79800",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 6007285,
            "range": "± 513273",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 15958128,
            "range": "± 974303",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 41058009,
            "range": "± 2119446",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 241886813,
            "range": "± 7613918",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 476338191,
            "range": "± 14450247",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2503158,
            "range": "± 29073",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5866569,
            "range": "± 52444",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11492617,
            "range": "± 78820",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 56429893,
            "range": "± 446231",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 112602046,
            "range": "± 1059055",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 161905,
            "range": "± 26467",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 7735,
            "range": "± 2851",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5355,
            "range": "± 1092",
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
          "id": "a0813cbdb99f894fd2f118763c275463bd20a40a",
          "message": "Implement `ping-pong` for WebSocket clients (#772)\n\n* ws: Implement ping for `TransportSenderT` trait\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws/client: Receive pong frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* core/client: Use `select!` macro for the background task\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Propagate ping interval to background task\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* async_client: Submit ping requests\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* async_client: Handle pong replies\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Handle frontend messages to dedicated fn\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Handle backend messages in dedicated fn\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Add terminated fuse for opt-out pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Set opt-out behavior for client pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Move imports\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Handle handle_frontend_messages errors\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Add custom error related to byteslice conversions\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Modify `send_ping` to send empty slices\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix `cargo hack check` and use `select_biased`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Handle sending pings with lowest priority\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* core: Add proper number of params to `background_task`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix wasm client\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Handle raw bytes and string received messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix Cargo.toml feature\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Panic when empty slice does not fit into `ByteSlice125`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* wasm: Add operation not supported for pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Rename `ReceivedMessage` from Data to Text\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Rename test variable\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Add documentation\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Use `future::select` for  cancel safety\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Remove `pong` handling logic\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Update ping documentation\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/Cargo.toml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* Update core/Cargo.toml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* logs: Keep debug log for submitting `Ping` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Print debug logs when receiving `Pong` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-01T13:52:01Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/a0813cbdb99f894fd2f118763c275463bd20a40a"
        },
        "date": 1654130679638,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 141,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 165,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 96006,
            "range": "± 7069",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2561066,
            "range": "± 86182",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1173309,
            "range": "± 30364",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 190332,
            "range": "± 10244",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 353536,
            "range": "± 11145",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 636433,
            "range": "± 6122",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1212630,
            "range": "± 31169",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2391508,
            "range": "± 129303",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4722868,
            "range": "± 80698",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9437890,
            "range": "± 302942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 18829448,
            "range": "± 678588",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 38057439,
            "range": "± 1039042",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 76616872,
            "range": "± 1867176",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 94824,
            "range": "± 2302",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 96599,
            "range": "± 2954",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 107964,
            "range": "± 2714",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 210941,
            "range": "± 2717",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 339234,
            "range": "± 7336",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 6894445,
            "range": "± 79697",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 19304459,
            "range": "± 1178139",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 37362874,
            "range": "± 1276477",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 186909195,
            "range": "± 1650257",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 369532296,
            "range": "± 2203164",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2239454,
            "range": "± 7174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5530715,
            "range": "± 59318",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10899383,
            "range": "± 122547",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53496580,
            "range": "± 289840",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106718754,
            "range": "± 116083",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 62501,
            "range": "± 1046",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2331996,
            "range": "± 22253",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1138797,
            "range": "± 18475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 369626,
            "range": "± 66696",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 664578,
            "range": "± 106797",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1321650,
            "range": "± 171213",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2684173,
            "range": "± 301809",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5451321,
            "range": "± 552943",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10687862,
            "range": "± 1196046",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 22057909,
            "range": "± 1372065",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 43914822,
            "range": "± 2460103",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 87770915,
            "range": "± 2865574",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 179803300,
            "range": "± 3371689",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 504595,
            "range": "± 53220",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1036039,
            "range": "± 144964",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2075706,
            "range": "± 200980",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4372145,
            "range": "± 384321",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8566356,
            "range": "± 759070",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17654293,
            "range": "± 812790",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 36829939,
            "range": "± 2376555",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 72193815,
            "range": "± 2083897",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 144368099,
            "range": "± 2972336",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 298897295,
            "range": "± 5833556",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 74043,
            "range": "± 1224",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 84185,
            "range": "± 2307",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 102735,
            "range": "± 1768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 227264,
            "range": "± 1787",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 368217,
            "range": "± 9355",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5067300,
            "range": "± 49174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 12975726,
            "range": "± 228585",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 35495665,
            "range": "± 1211853",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 199405710,
            "range": "± 2744384",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 381705716,
            "range": "± 5133899",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2212272,
            "range": "± 7588",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5425934,
            "range": "± 12825",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10832075,
            "range": "± 29217",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53479181,
            "range": "± 66467",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106685670,
            "range": "± 86170",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 95473,
            "range": "± 2647",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2670592,
            "range": "± 60150",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1169760,
            "range": "± 6666",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 191370,
            "range": "± 2353",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 354292,
            "range": "± 4601",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 635010,
            "range": "± 10750",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1208417,
            "range": "± 23192",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2369577,
            "range": "± 52632",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4672282,
            "range": "± 144205",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9292784,
            "range": "± 254329",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 18834914,
            "range": "± 700138",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 37584346,
            "range": "± 1044219",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 75520292,
            "range": "± 1948559",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 96498,
            "range": "± 2376",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 95231,
            "range": "± 3536",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 107959,
            "range": "± 2564",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 210893,
            "range": "± 3850",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 338778,
            "range": "± 4181",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5233959,
            "range": "± 87637",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 12910588,
            "range": "± 205690",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 32556512,
            "range": "± 387278",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 169451527,
            "range": "± 2102260",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 367406265,
            "range": "± 2687134",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2234456,
            "range": "± 6108",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5437311,
            "range": "± 14700",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10843697,
            "range": "± 29926",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53477880,
            "range": "± 57876",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106671316,
            "range": "± 108998",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 63432,
            "range": "± 3326",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2397100,
            "range": "± 15797",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1138632,
            "range": "± 7199",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 359035,
            "range": "± 97132",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 667740,
            "range": "± 124918",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1323046,
            "range": "± 144347",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2666141,
            "range": "± 289023",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5336439,
            "range": "± 644856",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10427516,
            "range": "± 1021577",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 22198193,
            "range": "± 1593757",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 43596175,
            "range": "± 1734981",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 87014946,
            "range": "± 2381469",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 176482667,
            "range": "± 3660334",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 495125,
            "range": "± 82629",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1033352,
            "range": "± 100053",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2059741,
            "range": "± 215271",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4376588,
            "range": "± 395014",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8549442,
            "range": "± 614347",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 17369363,
            "range": "± 804447",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 36169626,
            "range": "± 1423531",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 72094799,
            "range": "± 2016119",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 142959170,
            "range": "± 2802380",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 297911345,
            "range": "± 18403513",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 74121,
            "range": "± 991",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 82597,
            "range": "± 756",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 102929,
            "range": "± 1679",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 225945,
            "range": "± 1296",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 369200,
            "range": "± 3448",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5190489,
            "range": "± 40160",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 13011985,
            "range": "± 170183",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 35214702,
            "range": "± 1017807",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 197715218,
            "range": "± 4562022",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 387300759,
            "range": "± 5666293",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2223650,
            "range": "± 13339",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5437781,
            "range": "± 13617",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10849731,
            "range": "± 28305",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53494882,
            "range": "± 80194",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106670470,
            "range": "± 80259",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 80343,
            "range": "± 2314",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 3893,
            "range": "± 1103",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3416,
            "range": "± 530",
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
          "id": "a0813cbdb99f894fd2f118763c275463bd20a40a",
          "message": "Implement `ping-pong` for WebSocket clients (#772)\n\n* ws: Implement ping for `TransportSenderT` trait\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* ws/client: Receive pong frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* core/client: Use `select!` macro for the background task\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Propagate ping interval to background task\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* async_client: Submit ping requests\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* async_client: Handle pong replies\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Handle frontend messages to dedicated fn\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Handle backend messages in dedicated fn\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Add terminated fuse for opt-out pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Set opt-out behavior for client pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Move imports\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Handle handle_frontend_messages errors\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Add custom error related to byteslice conversions\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Modify `send_ping` to send empty slices\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix `cargo hack check` and use `select_biased`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Handle sending pings with lowest priority\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* core: Add proper number of params to `background_task`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix wasm client\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Handle raw bytes and string received messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Fix Cargo.toml feature\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Panic when empty slice does not fit into `ByteSlice125`\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* wasm: Add operation not supported for pings\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Rename `ReceivedMessage` from Data to Text\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Rename test variable\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Add documentation\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Use `future::select` for  cancel safety\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Remove `pong` handling logic\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* client: Update ping documentation\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\n\r\n* Update core/Cargo.toml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* Update core/Cargo.toml\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>\r\n\r\n* logs: Keep debug log for submitting `Ping` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Print debug logs when receiving `Pong` frames\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Update core/src/client/async_client/mod.rs\r\n\r\nCo-authored-by: Tarik Gul <47201679+TarikGul@users.noreply.github.com>\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-01T13:52:01Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/a0813cbdb99f894fd2f118763c275463bd20a40a"
        },
        "date": 1654217439020,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 188,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 212,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 155316,
            "range": "± 14331",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3586721,
            "range": "± 251389",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1308914,
            "range": "± 25310",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 304908,
            "range": "± 24916",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 565289,
            "range": "± 23126",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1018655,
            "range": "± 49072",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1982818,
            "range": "± 124024",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 3820799,
            "range": "± 155982",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 7753063,
            "range": "± 355157",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 15903284,
            "range": "± 751274",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 31877380,
            "range": "± 1622879",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 63262241,
            "range": "± 2886552",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 129874449,
            "range": "± 8260725",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 151478,
            "range": "± 8595",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 163691,
            "range": "± 14031",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 183145,
            "range": "± 16624",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 325906,
            "range": "± 18980",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 489905,
            "range": "± 19688",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 10072882,
            "range": "± 732670",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 24142107,
            "range": "± 1633301",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 47870757,
            "range": "± 2800441",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 226000638,
            "range": "± 8722148",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 454872212,
            "range": "± 11158851",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2421865,
            "range": "± 36128",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5745023,
            "range": "± 59175",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11275804,
            "range": "± 93226",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 55432855,
            "range": "± 468200",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 110331520,
            "range": "± 1002483",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 110665,
            "range": "± 14120",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2788346,
            "range": "± 143370",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1274013,
            "range": "± 36590",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 429115,
            "range": "± 40210",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 825085,
            "range": "± 84322",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1524975,
            "range": "± 148707",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3534268,
            "range": "± 431251",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 6454068,
            "range": "± 636643",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 13387624,
            "range": "± 1388606",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 26599206,
            "range": "± 1841141",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 54410824,
            "range": "± 3034033",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 107620235,
            "range": "± 5749766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 225664981,
            "range": "± 8653281",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 566206,
            "range": "± 66861",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1241435,
            "range": "± 102507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2464599,
            "range": "± 202753",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5338506,
            "range": "± 509187",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 10462426,
            "range": "± 1205099",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 22038229,
            "range": "± 1546374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 43589647,
            "range": "± 2394400",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 89046234,
            "range": "± 3493539",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 182215877,
            "range": "± 6845670",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 387515840,
            "range": "± 10119450",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 137019,
            "range": "± 13472",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 147620,
            "range": "± 10138",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 166197,
            "range": "± 17598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 309232,
            "range": "± 21925",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 542803,
            "range": "± 47987",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 6380588,
            "range": "± 457053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 24412916,
            "range": "± 1158724",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 43802376,
            "range": "± 1977830",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 241545372,
            "range": "± 6382823",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 464938499,
            "range": "± 8184943",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2408817,
            "range": "± 42381",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5713336,
            "range": "± 90656",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11288080,
            "range": "± 115839",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 55611009,
            "range": "± 687269",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 110362074,
            "range": "± 994999",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 149554,
            "range": "± 14631",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2987950,
            "range": "± 170428",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1307640,
            "range": "± 35175",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 296664,
            "range": "± 12809",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 554074,
            "range": "± 36329",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1000182,
            "range": "± 58338",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1905890,
            "range": "± 84247",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3691886,
            "range": "± 153448",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 7304578,
            "range": "± 369869",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 14955984,
            "range": "± 571915",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 29778092,
            "range": "± 1229554",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 61470705,
            "range": "± 3920656",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 120249779,
            "range": "± 4074320",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 156328,
            "range": "± 15984",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 166956,
            "range": "± 14025",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 189560,
            "range": "± 23683",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 326908,
            "range": "± 15546",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 472481,
            "range": "± 37567",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 6144208,
            "range": "± 451997",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 16198145,
            "range": "± 731849",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 40287970,
            "range": "± 1598406",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 206441874,
            "range": "± 11737629",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 434544685,
            "range": "± 11768775",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2457175,
            "range": "± 48984",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5767554,
            "range": "± 64255",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11329509,
            "range": "± 140816",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 55603653,
            "range": "± 604258",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 111671575,
            "range": "± 973352",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 107206,
            "range": "± 14222",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2799545,
            "range": "± 126643",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1285165,
            "range": "± 31369",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 430592,
            "range": "± 42005",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 805095,
            "range": "± 79189",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1490842,
            "range": "± 168315",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 3304216,
            "range": "± 383016",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 6547758,
            "range": "± 801070",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 13384625,
            "range": "± 1231374",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 26631208,
            "range": "± 1893762",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 52591208,
            "range": "± 2776684",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 105454862,
            "range": "± 5957981",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 218911306,
            "range": "± 8624785",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 570396,
            "range": "± 231200",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1247345,
            "range": "± 115616",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2459395,
            "range": "± 205195",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5179311,
            "range": "± 481410",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 10444385,
            "range": "± 1060139",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 21522552,
            "range": "± 1444942",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 45589930,
            "range": "± 2612945",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 90066649,
            "range": "± 3984732",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 185712077,
            "range": "± 8099008",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 376512224,
            "range": "± 13928267",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 124428,
            "range": "± 9857",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 139212,
            "range": "± 11844",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 159115,
            "range": "± 19037",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 308121,
            "range": "± 23728",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 512907,
            "range": "± 75522",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 6084133,
            "range": "± 336605",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 16263446,
            "range": "± 658467",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 43809481,
            "range": "± 1764740",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 244627930,
            "range": "± 7736005",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 474143173,
            "range": "± 10057813",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2428485,
            "range": "± 31704",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5727513,
            "range": "± 57193",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11274110,
            "range": "± 88325",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 55535039,
            "range": "± 518637",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 110023667,
            "range": "± 1319622",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 154882,
            "range": "± 9994",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 6201,
            "range": "± 1364",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5169,
            "range": "± 763",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654303455092,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 141,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 96105,
            "range": "± 3571",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2468081,
            "range": "± 118323",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1264758,
            "range": "± 31976",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 187509,
            "range": "± 3070",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 348140,
            "range": "± 5509",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 618648,
            "range": "± 5097",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1176422,
            "range": "± 25498",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2315307,
            "range": "± 85309",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4571114,
            "range": "± 135038",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9096469,
            "range": "± 287660",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 18354687,
            "range": "± 679156",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 36886173,
            "range": "± 955738",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 74909378,
            "range": "± 2684941",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 95949,
            "range": "± 2508",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 99396,
            "range": "± 4932",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 110885,
            "range": "± 4945",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 211306,
            "range": "± 4107",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 336296,
            "range": "± 4719",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 7822330,
            "range": "± 56166",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 20074917,
            "range": "± 940819",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 38489878,
            "range": "± 518437",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 191347372,
            "range": "± 1141777",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 379755014,
            "range": "± 1736808",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2357462,
            "range": "± 34379",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5592481,
            "range": "± 58737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11145266,
            "range": "± 105180",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 54259532,
            "range": "± 326324",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 108129691,
            "range": "± 423257",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 61928,
            "range": "± 931",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2323644,
            "range": "± 4932",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1206714,
            "range": "± 22922",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 348106,
            "range": "± 74784",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 626370,
            "range": "± 159774",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1216685,
            "range": "± 125188",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2642414,
            "range": "± 304043",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5210272,
            "range": "± 622186",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10262193,
            "range": "± 920657",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 20631878,
            "range": "± 1430882",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 40299531,
            "range": "± 1786554",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 81616425,
            "range": "± 3763727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 167344954,
            "range": "± 3867742",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 463216,
            "range": "± 39937",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 989316,
            "range": "± 81228",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1906289,
            "range": "± 119645",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4119629,
            "range": "± 426040",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8144164,
            "range": "± 524001",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 16504756,
            "range": "± 903082",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 33365817,
            "range": "± 1381800",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 65963020,
            "range": "± 2482233",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 131708364,
            "range": "± 2826637",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 277383089,
            "range": "± 6759794",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 70208,
            "range": "± 1219",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 79872,
            "range": "± 1692",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 99198,
            "range": "± 792",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 221607,
            "range": "± 2095",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 368728,
            "range": "± 1069",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4976436,
            "range": "± 10998",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 17759258,
            "range": "± 330300",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 33984853,
            "range": "± 474280",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 199970905,
            "range": "± 1905646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 388135597,
            "range": "± 6709945",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2332970,
            "range": "± 41948",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5588295,
            "range": "± 53806",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11077160,
            "range": "± 95524",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 54293164,
            "range": "± 350006",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 108443735,
            "range": "± 528939",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 94490,
            "range": "± 2366",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2386703,
            "range": "± 14234",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1272905,
            "range": "± 47636",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 187388,
            "range": "± 1720",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 348663,
            "range": "± 29109",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 618938,
            "range": "± 6418",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1178079,
            "range": "± 39369",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2311651,
            "range": "± 53725",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4575596,
            "range": "± 124816",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9120686,
            "range": "± 217580",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 18335032,
            "range": "± 612941",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 36846199,
            "range": "± 968555",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 74291588,
            "range": "± 2120525",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 95662,
            "range": "± 3065",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 98956,
            "range": "± 4177",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 109323,
            "range": "± 4417",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 211498,
            "range": "± 2972",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 337051,
            "range": "± 8667",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 4815566,
            "range": "± 100256",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 12087196,
            "range": "± 161875",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 31836205,
            "range": "± 414838",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 168358120,
            "range": "± 1370124",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 373441107,
            "range": "± 3002151",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2355815,
            "range": "± 33962",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5600749,
            "range": "± 55062",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11154567,
            "range": "± 109073",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 54277525,
            "range": "± 415636",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 108162783,
            "range": "± 551413",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 62032,
            "range": "± 800",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2324631,
            "range": "± 12501",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1206420,
            "range": "± 28340",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 341597,
            "range": "± 87937",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 619526,
            "range": "± 165080",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1223337,
            "range": "± 128435",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2601773,
            "range": "± 327014",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5147380,
            "range": "± 623455",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10259476,
            "range": "± 1063837",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 20192233,
            "range": "± 1365766",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 40614569,
            "range": "± 2136386",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 81713329,
            "range": "± 2319098",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 167081974,
            "range": "± 4024265",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 463832,
            "range": "± 30680",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 991824,
            "range": "± 88383",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1951841,
            "range": "± 174060",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4154489,
            "range": "± 357184",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8327357,
            "range": "± 654616",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16609770,
            "range": "± 1101411",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33270406,
            "range": "± 1327736",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 65959121,
            "range": "± 2252771",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 132185347,
            "range": "± 3146717",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 280266708,
            "range": "± 11030691",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 70296,
            "range": "± 1060",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 79757,
            "range": "± 1161",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 100384,
            "range": "± 3022",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 222013,
            "range": "± 1472",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 363502,
            "range": "± 1883",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4816747,
            "range": "± 12373",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 12013488,
            "range": "± 175957",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 31447054,
            "range": "± 1501522",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 190612755,
            "range": "± 7058046",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 382542612,
            "range": "± 5360467",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2312725,
            "range": "± 43933",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5569848,
            "range": "± 46517",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11094012,
            "range": "± 103570",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 54298666,
            "range": "± 463403",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 108181069,
            "range": "± 367328",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 81290,
            "range": "± 1405",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 3555,
            "range": "± 481",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3312,
            "range": "± 462",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654389912214,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 145,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 164,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 93686,
            "range": "± 5934",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2502508,
            "range": "± 96347",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1171858,
            "range": "± 8588",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 190466,
            "range": "± 6520",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 354692,
            "range": "± 7458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 649795,
            "range": "± 20874",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1237027,
            "range": "± 39693",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2464694,
            "range": "± 61495",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4800594,
            "range": "± 113833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9470560,
            "range": "± 312891",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 19056972,
            "range": "± 650019",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 38887124,
            "range": "± 1245185",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 80355365,
            "range": "± 2430344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 89127,
            "range": "± 1146",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 95793,
            "range": "± 1485",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 106604,
            "range": "± 3056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 204744,
            "range": "± 5342",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 330905,
            "range": "± 12255",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 8883000,
            "range": "± 428055",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 22171909,
            "range": "± 1317983",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 44175235,
            "range": "± 1852810",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 202069254,
            "range": "± 7259475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 401832976,
            "range": "± 13000552",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2317843,
            "range": "± 7734",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5539435,
            "range": "± 35163",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10939538,
            "range": "± 97598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53697828,
            "range": "± 286982",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106829998,
            "range": "± 347284",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 66320,
            "range": "± 1227",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2180514,
            "range": "± 51894",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1153163,
            "range": "± 78722",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 337036,
            "range": "± 46534",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 633837,
            "range": "± 79498",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1190650,
            "range": "± 134652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2574967,
            "range": "± 357477",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5165343,
            "range": "± 670184",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10244952,
            "range": "± 1036062",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 20893341,
            "range": "± 1494149",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 41363377,
            "range": "± 1702699",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 85545385,
            "range": "± 2184012",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 176862186,
            "range": "± 3857820",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 461568,
            "range": "± 75179",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1015588,
            "range": "± 111919",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1957696,
            "range": "± 127216",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4237386,
            "range": "± 393483",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8245305,
            "range": "± 613992",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 16591858,
            "range": "± 840020",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 34840478,
            "range": "± 1368887",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 69823059,
            "range": "± 2185599",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 138868510,
            "range": "± 3247900",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 297820985,
            "range": "± 8890633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 75668,
            "range": "± 1457",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 89865,
            "range": "± 2016",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 106678,
            "range": "± 5538",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 221581,
            "range": "± 4404",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 358351,
            "range": "± 2754",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5037813,
            "range": "± 77859",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 20280484,
            "range": "± 643748",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 40517867,
            "range": "± 793249",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 225920616,
            "range": "± 2815443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 440789247,
            "range": "± 5804412",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2344368,
            "range": "± 8602",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5540341,
            "range": "± 12938",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10919892,
            "range": "± 60536",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53546169,
            "range": "± 146591",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106806775,
            "range": "± 663314",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 97454,
            "range": "± 2774",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2504811,
            "range": "± 27015",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1170120,
            "range": "± 29320",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 200068,
            "range": "± 4487",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 372662,
            "range": "± 5817",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 667724,
            "range": "± 5638",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1269478,
            "range": "± 30688",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2470030,
            "range": "± 92958",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4875272,
            "range": "± 103324",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9671671,
            "range": "± 311465",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19503858,
            "range": "± 618327",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 39557602,
            "range": "± 1056546",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 80661571,
            "range": "± 2200559",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 95932,
            "range": "± 1094",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 99296,
            "range": "± 907",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 111012,
            "range": "± 3376",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 235830,
            "range": "± 8790",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 376717,
            "range": "± 14645",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5356924,
            "range": "± 141792",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 13898727,
            "range": "± 466418",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 37417449,
            "range": "± 1286645",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 195551289,
            "range": "± 4457879",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 420961127,
            "range": "± 4605756",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2489659,
            "range": "± 67195",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5654999,
            "range": "± 80795",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10856881,
            "range": "± 37754",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53582094,
            "range": "± 146690",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106816973,
            "range": "± 57627",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 68519,
            "range": "± 735",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2324010,
            "range": "± 60652",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1289356,
            "range": "± 83074",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 339327,
            "range": "± 33621",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 629784,
            "range": "± 37209",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1192137,
            "range": "± 136574",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2496310,
            "range": "± 300251",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5189642,
            "range": "± 580967",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10400770,
            "range": "± 1026301",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 20933841,
            "range": "± 1005149",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 41364627,
            "range": "± 2353314",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 84744071,
            "range": "± 1922917",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 172224722,
            "range": "± 3794067",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 466506,
            "range": "± 43256",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1003559,
            "range": "± 74814",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1944845,
            "range": "± 147963",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4075890,
            "range": "± 377399",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8174013,
            "range": "± 567090",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16441421,
            "range": "± 761821",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33821413,
            "range": "± 1409006",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 67186222,
            "range": "± 2227949",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 135040449,
            "range": "± 2939870",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 283927607,
            "range": "± 5194535",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 77863,
            "range": "± 2830",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 93819,
            "range": "± 1352",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 108991,
            "range": "± 1855",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 226374,
            "range": "± 3885",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 361192,
            "range": "± 1935",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4901333,
            "range": "± 64337",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 13832695,
            "range": "± 282073",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 41295974,
            "range": "± 799506",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 224236460,
            "range": "± 3195007",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 436500139,
            "range": "± 5198376",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2342957,
            "range": "± 6276",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5536725,
            "range": "± 47465",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10886239,
            "range": "± 30420",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53540587,
            "range": "± 53815",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106823235,
            "range": "± 145122",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 86120,
            "range": "± 2323",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4307,
            "range": "± 1661",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4305,
            "range": "± 451",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654476687094,
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
            "value": 218,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 156918,
            "range": "± 17575",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3570602,
            "range": "± 271373",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1326018,
            "range": "± 53757",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 310299,
            "range": "± 17752",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 570573,
            "range": "± 30300",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1048088,
            "range": "± 92563",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 2023271,
            "range": "± 138260",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 3946006,
            "range": "± 176443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 7861203,
            "range": "± 347052",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 16016209,
            "range": "± 718154",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 32601325,
            "range": "± 1444665",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 66686862,
            "range": "± 2515960",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 136234721,
            "range": "± 4365766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 165575,
            "range": "± 27369",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 181405,
            "range": "± 50081",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 190629,
            "range": "± 29773",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 336614,
            "range": "± 28151",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 491925,
            "range": "± 32589",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 11363659,
            "range": "± 695624",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 26777079,
            "range": "± 1441919",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 52090290,
            "range": "± 2701418",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 237046863,
            "range": "± 7148541",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 466227908,
            "range": "± 10976670",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2453741,
            "range": "± 26205",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5786759,
            "range": "± 52636",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11399167,
            "range": "± 197352",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 55741070,
            "range": "± 504378",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 111023879,
            "range": "± 1353544",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 114619,
            "range": "± 10280",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2998254,
            "range": "± 177038",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1305589,
            "range": "± 96702",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 439433,
            "range": "± 82258",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 854901,
            "range": "± 124615",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1558895,
            "range": "± 210977",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3782689,
            "range": "± 605885",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 6987285,
            "range": "± 940437",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 14022596,
            "range": "± 1456849",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 27715039,
            "range": "± 2777629",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 55943484,
            "range": "± 3224757",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 113935093,
            "range": "± 4269634",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 234723966,
            "range": "± 8008524",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 590075,
            "range": "± 137926",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1249424,
            "range": "± 95515",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2483567,
            "range": "± 305938",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5182888,
            "range": "± 503418",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 10791749,
            "range": "± 1157262",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 22944203,
            "range": "± 1512469",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 43183977,
            "range": "± 3116208",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 86738967,
            "range": "± 3827167",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 177700856,
            "range": "± 6812726",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 377536835,
            "range": "± 13833140",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 127391,
            "range": "± 8375",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 143406,
            "range": "± 23168",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 162883,
            "range": "± 9563",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 310621,
            "range": "± 20853",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 497928,
            "range": "± 210359",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 6577087,
            "range": "± 403286",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 23030035,
            "range": "± 1559744",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 43666985,
            "range": "± 1545426",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 246502494,
            "range": "± 6738967",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 476231448,
            "range": "± 10719125",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2417550,
            "range": "± 33844",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5758864,
            "range": "± 44507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11319519,
            "range": "± 64622",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 55629208,
            "range": "± 473203",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 110812969,
            "range": "± 1084445",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 154330,
            "range": "± 9982",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 3061308,
            "range": "± 182869",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1326223,
            "range": "± 79893",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 309993,
            "range": "± 17805",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 554661,
            "range": "± 36596",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1026920,
            "range": "± 65566",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1990937,
            "range": "± 108835",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3923597,
            "range": "± 198610",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 7564020,
            "range": "± 306883",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 15205511,
            "range": "± 524240",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 31545986,
            "range": "± 1318718",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 64529182,
            "range": "± 2358728",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 128148337,
            "range": "± 4299688",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 160077,
            "range": "± 12493",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 173926,
            "range": "± 12027",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 189510,
            "range": "± 16504",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 324665,
            "range": "± 74210",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 496659,
            "range": "± 28703",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 6969523,
            "range": "± 487658",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 16547850,
            "range": "± 751803",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 41189130,
            "range": "± 1466497",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 210000266,
            "range": "± 5601033",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 455503712,
            "range": "± 9470147",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2454449,
            "range": "± 57566",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5795041,
            "range": "± 106043",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11344406,
            "range": "± 118658",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 55689771,
            "range": "± 363178",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 110794249,
            "range": "± 636665",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 112380,
            "range": "± 13436",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2870037,
            "range": "± 153934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1305806,
            "range": "± 61075",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 425040,
            "range": "± 42955",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 825372,
            "range": "± 112041",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1520409,
            "range": "± 167991",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 3638182,
            "range": "± 577761",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 6860330,
            "range": "± 872179",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 14184282,
            "range": "± 1475506",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 26998100,
            "range": "± 2741112",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 53633715,
            "range": "± 2246537",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 107836946,
            "range": "± 3944784",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 222436490,
            "range": "± 7179540",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 589480,
            "range": "± 71577",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1251275,
            "range": "± 88451",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2397000,
            "range": "± 209090",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5310723,
            "range": "± 521206",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 10610996,
            "range": "± 1126075",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 22193435,
            "range": "± 2331270",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 44279415,
            "range": "± 2754392",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 87372498,
            "range": "± 3923911",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 176124862,
            "range": "± 5949631",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 383085333,
            "range": "± 11054545",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 133417,
            "range": "± 15692",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 144178,
            "range": "± 9402",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 166390,
            "range": "± 22072",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 315629,
            "range": "± 26208",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 511648,
            "range": "± 47258",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 6417426,
            "range": "± 391086",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 16526615,
            "range": "± 906321",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 42273777,
            "range": "± 1908908",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 239689386,
            "range": "± 6860263",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 473750649,
            "range": "± 11052240",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2424055,
            "range": "± 19053",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5768098,
            "range": "± 77958",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11346328,
            "range": "± 140117",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 55605979,
            "range": "± 450536",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 110817657,
            "range": "± 993732",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 155793,
            "range": "± 18997",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 7459,
            "range": "± 2161",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5139,
            "range": "± 2197",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654562687361,
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
            "value": 169,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 95955,
            "range": "± 3974",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3052630,
            "range": "± 134089",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1385481,
            "range": "± 50224",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 202451,
            "range": "± 31465",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 374753,
            "range": "± 13344",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 677076,
            "range": "± 6004",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1288714,
            "range": "± 21270",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2527602,
            "range": "± 71460",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4981059,
            "range": "± 201419",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9927297,
            "range": "± 277861",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 19904713,
            "range": "± 606709",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 40439651,
            "range": "± 1104910",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 81866121,
            "range": "± 2825246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 95497,
            "range": "± 1658",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 99884,
            "range": "± 1196",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 112659,
            "range": "± 1326",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 237768,
            "range": "± 7844",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 375887,
            "range": "± 11989",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 7096041,
            "range": "± 80412",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 18898931,
            "range": "± 444120",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 41971949,
            "range": "± 824727",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 207094817,
            "range": "± 5198506",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 415203324,
            "range": "± 1759849",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2521198,
            "range": "± 48681",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5772693,
            "range": "± 75930",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11131271,
            "range": "± 139122",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 54239773,
            "range": "± 301979",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 108013506,
            "range": "± 388155",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 66615,
            "range": "± 872",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2407676,
            "range": "± 35630",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1397363,
            "range": "± 100206",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 339551,
            "range": "± 36389",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 635119,
            "range": "± 50425",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1187552,
            "range": "± 138280",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2508354,
            "range": "± 273728",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5082793,
            "range": "± 451266",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10096419,
            "range": "± 990280",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 20499236,
            "range": "± 1276217",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 40957082,
            "range": "± 1622799",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 84430122,
            "range": "± 2213315",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 174464390,
            "range": "± 4154157",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 462951,
            "range": "± 35120",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1004916,
            "range": "± 127382",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1967983,
            "range": "± 160059",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4192113,
            "range": "± 330639",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8273272,
            "range": "± 611207",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 16506382,
            "range": "± 803652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 33738684,
            "range": "± 2709633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 66642595,
            "range": "± 1842662",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 136761665,
            "range": "± 3019943",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 300490923,
            "range": "± 7114324",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 75904,
            "range": "± 2543",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 89899,
            "range": "± 2136",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 102039,
            "range": "± 2515",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 223622,
            "range": "± 1787",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 361466,
            "range": "± 2632",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4937999,
            "range": "± 86458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 17262862,
            "range": "± 1197211",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 38471034,
            "range": "± 901893",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 224681732,
            "range": "± 5443128",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 435011721,
            "range": "± 5939086",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2521169,
            "range": "± 51933",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5760009,
            "range": "± 75453",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11147988,
            "range": "± 106100",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 54262369,
            "range": "± 227454",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 108162526,
            "range": "± 661635",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 93496,
            "range": "± 2123",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2549228,
            "range": "± 33024",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1397583,
            "range": "± 53754",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 200597,
            "range": "± 3032",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 374695,
            "range": "± 21969",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 672998,
            "range": "± 7015",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1281127,
            "range": "± 25113",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2512050,
            "range": "± 54521",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4923197,
            "range": "± 236070",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9807557,
            "range": "± 215395",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19704114,
            "range": "± 509854",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 39808752,
            "range": "± 714027",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 80435266,
            "range": "± 2772095",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 95289,
            "range": "± 1363",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 99691,
            "range": "± 1464",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 111776,
            "range": "± 863",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 238262,
            "range": "± 8751",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 381025,
            "range": "± 10781",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 4995340,
            "range": "± 555769",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 12996071,
            "range": "± 344390",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 34578048,
            "range": "± 1724603",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 183442331,
            "range": "± 3939979",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 410719685,
            "range": "± 8563108",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2508970,
            "range": "± 49700",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5747432,
            "range": "± 91296",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11155816,
            "range": "± 87468",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 54339699,
            "range": "± 257436",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 107916573,
            "range": "± 574130",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 67306,
            "range": "± 725",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2335588,
            "range": "± 63318",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1392225,
            "range": "± 112465",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 338563,
            "range": "± 29348",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 630353,
            "range": "± 77911",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1193290,
            "range": "± 106048",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2458749,
            "range": "± 293020",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5007484,
            "range": "± 647228",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10008377,
            "range": "± 912208",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 20444564,
            "range": "± 1188782",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 40798359,
            "range": "± 1866957",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 82580602,
            "range": "± 2361418",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 163686570,
            "range": "± 3038621",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 474607,
            "range": "± 42669",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1005297,
            "range": "± 85544",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1948651,
            "range": "± 138058",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4105324,
            "range": "± 338766",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8187825,
            "range": "± 683936",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16495402,
            "range": "± 889387",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33586736,
            "range": "± 1537793",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 66484759,
            "range": "± 2062967",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 134786249,
            "range": "± 2502608",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 291976433,
            "range": "± 6596974",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 75914,
            "range": "± 2249",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 90652,
            "range": "± 1754",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 107429,
            "range": "± 755",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 223271,
            "range": "± 3240",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 361821,
            "range": "± 2256",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4992475,
            "range": "± 83102",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 13308039,
            "range": "± 346458",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 35614915,
            "range": "± 795519",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 218479060,
            "range": "± 5140973",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 433815494,
            "range": "± 6157321",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2522335,
            "range": "± 57052",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5708458,
            "range": "± 66773",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11135048,
            "range": "± 110420",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 54231872,
            "range": "± 231798",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 107711389,
            "range": "± 661757",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 85050,
            "range": "± 6239",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4214,
            "range": "± 368",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4036,
            "range": "± 314",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654649383352,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 174,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 205,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 141945,
            "range": "± 25299",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2856421,
            "range": "± 113287",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1309245,
            "range": "± 24865",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 297968,
            "range": "± 10952",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 547380,
            "range": "± 25689",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1001798,
            "range": "± 110699",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1917773,
            "range": "± 133210",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 3769162,
            "range": "± 233887",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 7535951,
            "range": "± 368270",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 15185203,
            "range": "± 606776",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 30889881,
            "range": "± 1965762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 61980463,
            "range": "± 2360461",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 125804926,
            "range": "± 3448004",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 147280,
            "range": "± 23435",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 159137,
            "range": "± 33613",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 172893,
            "range": "± 9462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 311296,
            "range": "± 14221",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 467248,
            "range": "± 296106",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 8585201,
            "range": "± 470461",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 24447238,
            "range": "± 1013599",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 47515584,
            "range": "± 1925375",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 222307195,
            "range": "± 6466313",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 442053590,
            "range": "± 10353443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2435274,
            "range": "± 25482",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5748556,
            "range": "± 31484",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11288452,
            "range": "± 73539",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 55568261,
            "range": "± 412288",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 110818121,
            "range": "± 707458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 103999,
            "range": "± 11169",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2664316,
            "range": "± 84165",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1289884,
            "range": "± 33571",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 432342,
            "range": "± 73764",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 822771,
            "range": "± 97953",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1516580,
            "range": "± 200548",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3018279,
            "range": "± 404647",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 6125692,
            "range": "± 836517",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 13215502,
            "range": "± 1284483",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 26286856,
            "range": "± 2099768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 52907514,
            "range": "± 2512093",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 105755070,
            "range": "± 5084538",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 214919359,
            "range": "± 9998276",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 584314,
            "range": "± 89543",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1262387,
            "range": "± 134684",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2286473,
            "range": "± 157857",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4697136,
            "range": "± 506901",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 9063614,
            "range": "± 728656",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 19901154,
            "range": "± 1338186",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 39456109,
            "range": "± 2352934",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 80275514,
            "range": "± 4338474",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 167835821,
            "range": "± 6384944",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 347129039,
            "range": "± 12943392",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 113948,
            "range": "± 10164",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 126060,
            "range": "± 51006",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 144734,
            "range": "± 8189",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 284245,
            "range": "± 19056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 460504,
            "range": "± 38804",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5073363,
            "range": "± 164633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 17595554,
            "range": "± 1132202",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 39071209,
            "range": "± 1825168",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 221703127,
            "range": "± 7219313",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 429842637,
            "range": "± 10183423",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2446199,
            "range": "± 68590",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5978384,
            "range": "± 151458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11411235,
            "range": "± 210496",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 56731058,
            "range": "± 858121",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 113166677,
            "range": "± 1645029",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 131271,
            "range": "± 46390",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2788862,
            "range": "± 106363",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1341314,
            "range": "± 50475",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 281012,
            "range": "± 27776",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 501260,
            "range": "± 23106",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 978141,
            "range": "± 71539",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1759407,
            "range": "± 78171",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3425687,
            "range": "± 291965",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 6732676,
            "range": "± 285247",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 13974603,
            "range": "± 534687",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 28653607,
            "range": "± 1289962",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 55276864,
            "range": "± 2453327",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 112330578,
            "range": "± 5401063",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 139382,
            "range": "± 12194",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 147769,
            "range": "± 23427",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 169681,
            "range": "± 16526",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 315838,
            "range": "± 21824",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 472800,
            "range": "± 25927",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5638712,
            "range": "± 367551",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 14844252,
            "range": "± 914070",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 36413107,
            "range": "± 2755016",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 195799109,
            "range": "± 7671244",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 418809179,
            "range": "± 13427632",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2557300,
            "range": "± 89778",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 6018049,
            "range": "± 153001",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11641952,
            "range": "± 221492",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 56208904,
            "range": "± 875840",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 112040058,
            "range": "± 1729836",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 96010,
            "range": "± 5621",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2550297,
            "range": "± 127076",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1328921,
            "range": "± 52738",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 399185,
            "range": "± 38950",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 744124,
            "range": "± 81536",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1413080,
            "range": "± 184912",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2844100,
            "range": "± 425411",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5958571,
            "range": "± 733181",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 12348020,
            "range": "± 1192595",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 24124831,
            "range": "± 4258004",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 48816817,
            "range": "± 2799446",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 96388619,
            "range": "± 4015592",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 203149405,
            "range": "± 7394574",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 553007,
            "range": "± 72281",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1176180,
            "range": "± 95567",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2212645,
            "range": "± 146783",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4721389,
            "range": "± 404857",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 9166210,
            "range": "± 922569",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 19616240,
            "range": "± 1775905",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 38905214,
            "range": "± 2813470",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 80901950,
            "range": "± 4554138",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 163833658,
            "range": "± 7803217",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 352173960,
            "range": "± 15087254",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 119382,
            "range": "± 21315",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 139119,
            "range": "± 13741",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 147204,
            "range": "± 6741",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 285510,
            "range": "± 18829",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 476336,
            "range": "± 74837",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5088596,
            "range": "± 191153",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 14189853,
            "range": "± 785824",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 36753030,
            "range": "± 2019299",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 219089220,
            "range": "± 6289237",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 429553266,
            "range": "± 8330426",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2518754,
            "range": "± 86856",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5786994,
            "range": "± 93716",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11561340,
            "range": "± 225907",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 56366836,
            "range": "± 828497",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 111775414,
            "range": "± 1679514",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 138617,
            "range": "± 9302",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 5558,
            "range": "± 1041",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4761,
            "range": "± 700",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654735841670,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 200,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 228,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 162663,
            "range": "± 17196",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3764204,
            "range": "± 268187",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1283489,
            "range": "± 21261",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 328092,
            "range": "± 17202",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 590474,
            "range": "± 30681",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 1035375,
            "range": "± 49624",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1997942,
            "range": "± 103132",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 3913027,
            "range": "± 164548",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 7959761,
            "range": "± 472763",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 15780336,
            "range": "± 671054",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 31891349,
            "range": "± 1202271",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 64342483,
            "range": "± 3640800",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 129388827,
            "range": "± 4339960",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 169736,
            "range": "± 12301",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 179257,
            "range": "± 17617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 195967,
            "range": "± 18689",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 355651,
            "range": "± 49247",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 511307,
            "range": "± 33002",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9371199,
            "range": "± 412824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 21985139,
            "range": "± 1162745",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 46876931,
            "range": "± 1597054",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 246416581,
            "range": "± 7220424",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 492012768,
            "range": "± 11153231",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2385023,
            "range": "± 32762",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5627389,
            "range": "± 39635",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11001886,
            "range": "± 69361",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 54204280,
            "range": "± 244007",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 108019411,
            "range": "± 615131",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 120240,
            "range": "± 7950",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 3002247,
            "range": "± 93453",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1238417,
            "range": "± 17131",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 474221,
            "range": "± 56607",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 892911,
            "range": "± 95970",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1677577,
            "range": "± 197848",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 3866484,
            "range": "± 602930",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 7022624,
            "range": "± 720603",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 14208075,
            "range": "± 1324763",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 27623845,
            "range": "± 1971975",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 55724993,
            "range": "± 2478778",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 113789155,
            "range": "± 5289186",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 243702471,
            "range": "± 7062833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 638771,
            "range": "± 62106",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1341657,
            "range": "± 110321",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2553626,
            "range": "± 215247",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 5664140,
            "range": "± 586920",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 11545098,
            "range": "± 1147272",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 23370516,
            "range": "± 1799057",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 46169976,
            "range": "± 3201607",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 90928143,
            "range": "± 5105479",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 184139752,
            "range": "± 23095320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 389743251,
            "range": "± 12757756",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 130022,
            "range": "± 7493",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 140805,
            "range": "± 11003",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 161695,
            "range": "± 7646",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 321487,
            "range": "± 10545",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 518132,
            "range": "± 45407",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5902158,
            "range": "± 226240",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 20945221,
            "range": "± 875552",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 39573810,
            "range": "± 2050581",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 256914307,
            "range": "± 7650353",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 507406853,
            "range": "± 13520421",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2337776,
            "range": "± 32449",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5583557,
            "range": "± 44601",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10993707,
            "range": "± 85314",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 54118135,
            "range": "± 251367",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 108068519,
            "range": "± 380554",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 159078,
            "range": "± 11494",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 3256307,
            "range": "± 192571",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1284802,
            "range": "± 24650",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 310311,
            "range": "± 17303",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 582050,
            "range": "± 124145",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 1091897,
            "range": "± 82046",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1962888,
            "range": "± 75432",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 3894127,
            "range": "± 173472",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 7616846,
            "range": "± 302027",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 15313971,
            "range": "± 547297",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 31580848,
            "range": "± 1812238",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 62096240,
            "range": "± 2423971",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 123952726,
            "range": "± 3639484",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 156135,
            "range": "± 5980",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 164495,
            "range": "± 20162",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 192177,
            "range": "± 17688",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 333902,
            "range": "± 17980",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 494553,
            "range": "± 15486",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 6255669,
            "range": "± 236622",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 15457159,
            "range": "± 872508",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 38368595,
            "range": "± 1655134",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 220159505,
            "range": "± 5819962",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 478157231,
            "range": "± 9143242",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2379732,
            "range": "± 40387",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5643045,
            "range": "± 60704",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11051548,
            "range": "± 70403",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 54117531,
            "range": "± 283709",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 108065414,
            "range": "± 402749",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 113849,
            "range": "± 12455",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2939391,
            "range": "± 76540",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1235084,
            "range": "± 37524",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 463613,
            "range": "± 44387",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 873272,
            "range": "± 55533",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1683158,
            "range": "± 206201",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 3810598,
            "range": "± 546658",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 7265876,
            "range": "± 752737",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 14017899,
            "range": "± 1466272",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 26943890,
            "range": "± 2316854",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 54417288,
            "range": "± 2849001",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 111547394,
            "range": "± 5006791",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 232282404,
            "range": "± 6775483",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 618181,
            "range": "± 64446",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1337825,
            "range": "± 112793",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2671681,
            "range": "± 212366",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 5651456,
            "range": "± 598070",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 11029261,
            "range": "± 1145658",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 23301192,
            "range": "± 1684598",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 44591053,
            "range": "± 2448178",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 89652080,
            "range": "± 3801063",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 178779659,
            "range": "± 5014934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 394290242,
            "range": "± 8934013",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 133119,
            "range": "± 15069",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 146240,
            "range": "± 11421",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 170189,
            "range": "± 29293",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 325790,
            "range": "± 26737",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 516361,
            "range": "± 29484",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 6095609,
            "range": "± 179624",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 15073531,
            "range": "± 612449",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 40024928,
            "range": "± 1668133",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 248129638,
            "range": "± 9320730",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 500148762,
            "range": "± 14078479",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2342334,
            "range": "± 25431",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5582389,
            "range": "± 44908",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11021019,
            "range": "± 69190",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 54115048,
            "range": "± 227317",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 107981628,
            "range": "± 378006",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 161455,
            "range": "± 10272",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 7420,
            "range": "± 2141",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 5157,
            "range": "± 771",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654822127469,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 175,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 199,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 129626,
            "range": "± 16896",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3332693,
            "range": "± 221156",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1280690,
            "range": "± 50904",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 239997,
            "range": "± 19073",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 444776,
            "range": "± 6330",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 791480,
            "range": "± 5944",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1496433,
            "range": "± 46563",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2931485,
            "range": "± 84234",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5789334,
            "range": "± 188955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 11667416,
            "range": "± 398013",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 23260091,
            "range": "± 941132",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 47568425,
            "range": "± 1139913",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 96786784,
            "range": "± 2230900",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 120909,
            "range": "± 2493",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 121803,
            "range": "± 6231",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 136772,
            "range": "± 9084",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 265953,
            "range": "± 9150",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 443172,
            "range": "± 22439",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9799176,
            "range": "± 459462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 25626318,
            "range": "± 1791868",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 53217595,
            "range": "± 3101359",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 247392440,
            "range": "± 7603218",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 488683969,
            "range": "± 10511210",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2370078,
            "range": "± 45721",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5694982,
            "range": "± 74958",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11058402,
            "range": "± 67007",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 54310193,
            "range": "± 257415",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 108236306,
            "range": "± 611475",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 78535,
            "range": "± 1172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2900614,
            "range": "± 98256",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1263158,
            "range": "± 36169",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 395697,
            "range": "± 42204",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 728527,
            "range": "± 53193",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1379174,
            "range": "± 182056",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2913505,
            "range": "± 349207",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5774819,
            "range": "± 737564",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 11763514,
            "range": "± 890078",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 23470127,
            "range": "± 1220347",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 47020407,
            "range": "± 3397453",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 94885640,
            "range": "± 4255272",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 191399660,
            "range": "± 5288575",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 541284,
            "range": "± 39301",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1143559,
            "range": "± 79930",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2178681,
            "range": "± 149153",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4658907,
            "range": "± 420887",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 9150208,
            "range": "± 903151",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 19481604,
            "range": "± 1423073",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 39198263,
            "range": "± 2893870",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 77129441,
            "range": "± 2648573",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 155088457,
            "range": "± 3682031",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 336938397,
            "range": "± 7975789",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 93776,
            "range": "± 10532",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 109761,
            "range": "± 9307",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 129498,
            "range": "± 4277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 274621,
            "range": "± 24347",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 439757,
            "range": "± 21568",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 6606559,
            "range": "± 294358",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 22393189,
            "range": "± 1787229",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 48055384,
            "range": "± 2071637",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 260420906,
            "range": "± 8377908",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 506185336,
            "range": "± 10081569",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2381951,
            "range": "± 56175",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5615849,
            "range": "± 47955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 11068022,
            "range": "± 73904",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 54224801,
            "range": "± 276926",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 108200517,
            "range": "± 475009",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 123943,
            "range": "± 5417",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2962878,
            "range": "± 91586",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1295173,
            "range": "± 42613",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 237430,
            "range": "± 3451",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 444778,
            "range": "± 21990",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 788494,
            "range": "± 8330",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1485161,
            "range": "± 42740",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2924129,
            "range": "± 64994",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 5710843,
            "range": "± 254247",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 11368228,
            "range": "± 370228",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 22963345,
            "range": "± 584187",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 46272008,
            "range": "± 850936",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 93607301,
            "range": "± 2769544",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 122663,
            "range": "± 6132",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 122989,
            "range": "± 3687",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 139073,
            "range": "± 4948",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 278125,
            "range": "± 71716",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 448435,
            "range": "± 22137",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 6435611,
            "range": "± 256784",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 16841720,
            "range": "± 899443",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 42489503,
            "range": "± 1641309",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 223420512,
            "range": "± 6394274",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 484121279,
            "range": "± 11895562",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2367525,
            "range": "± 23160",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5622996,
            "range": "± 130484",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11052322,
            "range": "± 93286",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 54238078,
            "range": "± 275491",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 108726408,
            "range": "± 579673",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 78845,
            "range": "± 1287",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 3028290,
            "range": "± 144415",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1266110,
            "range": "± 39192",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 392598,
            "range": "± 64891",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 730458,
            "range": "± 60079",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1344094,
            "range": "± 162536",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2915835,
            "range": "± 336432",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5865861,
            "range": "± 713159",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 11989237,
            "range": "± 1083497",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 23941341,
            "range": "± 1403739",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 47018555,
            "range": "± 2410276",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 94042513,
            "range": "± 2956723",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 192658115,
            "range": "± 5945100",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 536707,
            "range": "± 43791",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1169955,
            "range": "± 95141",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2199814,
            "range": "± 194743",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4744482,
            "range": "± 466731",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 9088644,
            "range": "± 658427",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 19227295,
            "range": "± 1253043",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 39249713,
            "range": "± 2142795",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 76146557,
            "range": "± 2069500",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 153371634,
            "range": "± 3845692",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 334319547,
            "range": "± 9333718",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 91507,
            "range": "± 2346",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 108213,
            "range": "± 7317",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 128347,
            "range": "± 4880",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 280900,
            "range": "± 16267",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 453179,
            "range": "± 50469",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 6228974,
            "range": "± 297055",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 17299173,
            "range": "± 968732",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 47079711,
            "range": "± 2344500",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 259904288,
            "range": "± 8414078",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 506792346,
            "range": "± 15327968",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2378086,
            "range": "± 19621",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5639705,
            "range": "± 46430",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11001733,
            "range": "± 87924",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 54572540,
            "range": "± 406728",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 108057727,
            "range": "± 475809",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 102382,
            "range": "± 6254",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4978,
            "range": "± 894",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4487,
            "range": "± 622",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654908286045,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 157,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 174,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 99423,
            "range": "± 3685",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2569870,
            "range": "± 87728",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1191385,
            "range": "± 95952",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 201754,
            "range": "± 3384",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 542055,
            "range": "± 12946",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 682820,
            "range": "± 21471",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1292448,
            "range": "± 34172",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2538886,
            "range": "± 78085",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5150383,
            "range": "± 184064",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 10000451,
            "range": "± 319926",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 20626063,
            "range": "± 665753",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 41601356,
            "range": "± 1216687",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 84413033,
            "range": "± 2799460",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 101837,
            "range": "± 4242",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 108727,
            "range": "± 6255",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 124385,
            "range": "± 4740",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 259052,
            "range": "± 17328",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 409413,
            "range": "± 15336",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9089204,
            "range": "± 719899",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 25615413,
            "range": "± 954709",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 48969967,
            "range": "± 1403220",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 222672276,
            "range": "± 6275637",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 433307562,
            "range": "± 10297003",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2435091,
            "range": "± 48044",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5660724,
            "range": "± 170395",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 11097658,
            "range": "± 60980",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53722365,
            "range": "± 194596",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 107047074,
            "range": "± 586837",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 68279,
            "range": "± 3296",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2480035,
            "range": "± 122768",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1196977,
            "range": "± 38282",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 343514,
            "range": "± 42363",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 644563,
            "range": "± 58011",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1243098,
            "range": "± 151403",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2694009,
            "range": "± 309338",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5253209,
            "range": "± 585174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10402414,
            "range": "± 925592",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 21352268,
            "range": "± 1492476",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 43190620,
            "range": "± 2004941",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 87836486,
            "range": "± 2533601",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 195141655,
            "range": "± 10399794",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 470223,
            "range": "± 41913",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1019554,
            "range": "± 769231",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2023791,
            "range": "± 192905",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4150302,
            "range": "± 347796",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8259589,
            "range": "± 689824",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17230040,
            "range": "± 1211308",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 34347476,
            "range": "± 2295158",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 69059849,
            "range": "± 2446007",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 141388077,
            "range": "± 4375253",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 308697107,
            "range": "± 7195577",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 77843,
            "range": "± 1833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 92294,
            "range": "± 1827",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 104698,
            "range": "± 2976",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 227343,
            "range": "± 17466",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 365970,
            "range": "± 8375",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5049451,
            "range": "± 142302",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 14175053,
            "range": "± 1149076",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 30526737,
            "range": "± 2309602",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 226054901,
            "range": "± 4915197",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 447330503,
            "range": "± 57588947",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2340858,
            "range": "± 45607",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5540875,
            "range": "± 29545",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10865387,
            "range": "± 78669",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53507868,
            "range": "± 65519",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106787225,
            "range": "± 109842",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 95506,
            "range": "± 2844",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2714080,
            "range": "± 51637",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1165058,
            "range": "± 11240",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 199839,
            "range": "± 2873",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 375094,
            "range": "± 6042",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 673221,
            "range": "± 5606",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1280425,
            "range": "± 38837",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2500325,
            "range": "± 73158",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4969676,
            "range": "± 113141",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9854050,
            "range": "± 231415",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19887354,
            "range": "± 544551",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 40610745,
            "range": "± 919348",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 81992653,
            "range": "± 1878099",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 96031,
            "range": "± 1427",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 100431,
            "range": "± 3545",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 113022,
            "range": "± 1791",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 237470,
            "range": "± 8151",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 384883,
            "range": "± 11865",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5378096,
            "range": "± 140666",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 14554547,
            "range": "± 558925",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 37220919,
            "range": "± 814056",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 201958341,
            "range": "± 2090922",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 433845868,
            "range": "± 2904925",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2318523,
            "range": "± 21191",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5530029,
            "range": "± 26700",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10853264,
            "range": "± 19429",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53525363,
            "range": "± 77451",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106799672,
            "range": "± 110819",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 67113,
            "range": "± 1344",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2503169,
            "range": "± 44481",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1157596,
            "range": "± 33172",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 354278,
            "range": "± 39102",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 642972,
            "range": "± 52382",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1207272,
            "range": "± 129567",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2611136,
            "range": "± 365410",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5985664,
            "range": "± 1255485",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 9993394,
            "range": "± 1025950",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 21241239,
            "range": "± 1279640",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 41873898,
            "range": "± 2057109",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 85238714,
            "range": "± 2538887",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 175367075,
            "range": "± 3416665",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 480536,
            "range": "± 65529",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1008910,
            "range": "± 95781",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1963395,
            "range": "± 149538",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4097262,
            "range": "± 420876",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8294347,
            "range": "± 708079",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16853440,
            "range": "± 765875",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 34246623,
            "range": "± 1326794",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 68002890,
            "range": "± 3436675",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 139104066,
            "range": "± 2827378",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 298262893,
            "range": "± 7342364",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 76978,
            "range": "± 1266",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 91055,
            "range": "± 1468",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 107575,
            "range": "± 1694",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 225559,
            "range": "± 1807",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 365756,
            "range": "± 3694",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5271765,
            "range": "± 65548",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 13687683,
            "range": "± 313196",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 43256982,
            "range": "± 1831379",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 231535361,
            "range": "± 9576684",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 448703102,
            "range": "± 5281786",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2429360,
            "range": "± 64271",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5642752,
            "range": "± 84139",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10931157,
            "range": "± 99047",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53544863,
            "range": "± 263401",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106851317,
            "range": "± 281768",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 89000,
            "range": "± 3166",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4654,
            "range": "± 1082",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4239,
            "range": "± 450",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1654994743837,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 163,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 184,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 99362,
            "range": "± 3097",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3144180,
            "range": "± 145081",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1171995,
            "range": "± 19722",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 203881,
            "range": "± 8620",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 379363,
            "range": "± 13783",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 682276,
            "range": "± 10188",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1293649,
            "range": "± 27696",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2540160,
            "range": "± 85726",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5020184,
            "range": "± 209694",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 10108171,
            "range": "± 1025369",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 20567454,
            "range": "± 656468",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 41511585,
            "range": "± 1035435",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 83864709,
            "range": "± 2192287",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 97868,
            "range": "± 4277",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 100904,
            "range": "± 1724",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 112909,
            "range": "± 1330",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 239709,
            "range": "± 10549",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 380732,
            "range": "± 10514",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 7820259,
            "range": "± 222691",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 22578018,
            "range": "± 499943",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 44115475,
            "range": "± 739246",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 212923500,
            "range": "± 1606974",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 424736264,
            "range": "± 2913096",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2322502,
            "range": "± 25384",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5535781,
            "range": "± 63349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10871175,
            "range": "± 44253",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53505847,
            "range": "± 99430",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106826890,
            "range": "± 194641",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 67468,
            "range": "± 1064",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2583271,
            "range": "± 18653",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1163098,
            "range": "± 42119",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 350244,
            "range": "± 51222",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 664598,
            "range": "± 80162",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1232461,
            "range": "± 122989",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2653748,
            "range": "± 393820",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5420421,
            "range": "± 536361",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 11472753,
            "range": "± 1107730",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 23281694,
            "range": "± 1461022",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 46453788,
            "range": "± 1937766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 93200329,
            "range": "± 4204454",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 192447051,
            "range": "± 5556566",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 486177,
            "range": "± 44051",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1013948,
            "range": "± 71227",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2049252,
            "range": "± 208050",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4193199,
            "range": "± 334045",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8712154,
            "range": "± 1092179",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 18306296,
            "range": "± 1103636",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 36618742,
            "range": "± 2444771",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 71656263,
            "range": "± 3320787",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 142845375,
            "range": "± 5503707",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 304110015,
            "range": "± 11716291",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 77458,
            "range": "± 2961",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 95117,
            "range": "± 4531",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 117504,
            "range": "± 10511",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 230317,
            "range": "± 8500",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 390177,
            "range": "± 29553",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5507459,
            "range": "± 124088",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 20310918,
            "range": "± 1217163",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 41169265,
            "range": "± 4870843",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 227862378,
            "range": "± 3893975",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 446523591,
            "range": "± 6231358",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2340389,
            "range": "± 26258",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5544517,
            "range": "± 38759",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10873860,
            "range": "± 78504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53489212,
            "range": "± 176539",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106970644,
            "range": "± 264775",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 96877,
            "range": "± 2031",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2687101,
            "range": "± 102901",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1172938,
            "range": "± 20761",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 201402,
            "range": "± 3556",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 377718,
            "range": "± 5009",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 677353,
            "range": "± 115792",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1274011,
            "range": "± 28370",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2487743,
            "range": "± 106770",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4891304,
            "range": "± 145056",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9750093,
            "range": "± 258134",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19475631,
            "range": "± 571079",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 39222971,
            "range": "± 977485",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 79785643,
            "range": "± 1864460",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 96639,
            "range": "± 1347",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 100292,
            "range": "± 3595",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 113180,
            "range": "± 1387",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 240000,
            "range": "± 8694",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 384437,
            "range": "± 15299",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5445605,
            "range": "± 130517",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 14134316,
            "range": "± 447257",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 37199579,
            "range": "± 820526",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 203974519,
            "range": "± 5554785",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 447200269,
            "range": "± 10385126",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2340779,
            "range": "± 19816",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5549667,
            "range": "± 59854",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 11061410,
            "range": "± 83659",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53986064,
            "range": "± 571685",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 107460994,
            "range": "± 413643",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 73614,
            "range": "± 3464",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2673279,
            "range": "± 101191",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1213676,
            "range": "± 38626",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 346393,
            "range": "± 32108",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 667725,
            "range": "± 102059",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1279785,
            "range": "± 135653",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2581625,
            "range": "± 326676",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5136477,
            "range": "± 626409",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10540192,
            "range": "± 1060124",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 22420411,
            "range": "± 1436089",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 44522268,
            "range": "± 2279457",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 88339375,
            "range": "± 3988253",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 172747943,
            "range": "± 9420527",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 463065,
            "range": "± 35280",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1013283,
            "range": "± 95410",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1958014,
            "range": "± 163989",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4174518,
            "range": "± 348220",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8278234,
            "range": "± 632857",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16661449,
            "range": "± 837065",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33416624,
            "range": "± 1437039",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 66301943,
            "range": "± 1973242",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 135422131,
            "range": "± 3431861",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 282786790,
            "range": "± 5492890",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 76216,
            "range": "± 1758",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 92140,
            "range": "± 1714",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 107476,
            "range": "± 1542",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 224592,
            "range": "± 2807",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 361911,
            "range": "± 12399",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5330850,
            "range": "± 113988",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 14258108,
            "range": "± 309769",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 39613292,
            "range": "± 786773",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 230561449,
            "range": "± 3272921",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 445412124,
            "range": "± 15648336",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2337179,
            "range": "± 43608",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5539653,
            "range": "± 22027",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10867418,
            "range": "± 43894",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53537273,
            "range": "± 98936",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106899754,
            "range": "± 411073",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 86133,
            "range": "± 3666",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4561,
            "range": "± 487",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4322,
            "range": "± 515",
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
          "id": "64215300ddb31265d6d6f12748263ab7623d4bc7",
          "message": "ws-server: Submit ping regardless of WS messages (#788)\n\n* ws-server: Submit ping regardless of WS messages\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* use tokio_stream::IntervalStream for less boxing\r\n\r\nCo-authored-by: Niklas Adolfsson <niklasadolfsson1@gmail.com>",
          "timestamp": "2022-06-03T08:16:45Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/64215300ddb31265d6d6f12748263ab7623d4bc7"
        },
        "date": 1655081090755,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 150,
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
            "name": "sync/http_round_trip/fast_call",
            "value": 94986,
            "range": "± 1665",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2409591,
            "range": "± 85859",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1249251,
            "range": "± 71692",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 202018,
            "range": "± 5140",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 377951,
            "range": "± 4650",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 676882,
            "range": "± 12774",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1290688,
            "range": "± 55752",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2535042,
            "range": "± 70565",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5033710,
            "range": "± 70376",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9980969,
            "range": "± 202211",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 20050273,
            "range": "± 627330",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 41048536,
            "range": "± 1571598",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 82676676,
            "range": "± 2065737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 96342,
            "range": "± 1736",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 99622,
            "range": "± 1201",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 111916,
            "range": "± 1751",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 237714,
            "range": "± 10045",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 381316,
            "range": "± 13182",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 7880912,
            "range": "± 150797",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 21394882,
            "range": "± 787379",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 43453478,
            "range": "± 962163",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 207064148,
            "range": "± 2015655",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 414980569,
            "range": "± 2446299",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2442014,
            "range": "± 61690",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5634057,
            "range": "± 96490",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10950593,
            "range": "± 89989",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53838805,
            "range": "± 246742",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106782921,
            "range": "± 375078",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 67428,
            "range": "± 2428",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2291380,
            "range": "± 35305",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1310069,
            "range": "± 77835",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 340593,
            "range": "± 166828",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 644195,
            "range": "± 38080",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1205521,
            "range": "± 108945",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2530754,
            "range": "± 321891",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5114203,
            "range": "± 571286",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10119887,
            "range": "± 887984",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 21285774,
            "range": "± 1216637",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 42104495,
            "range": "± 1819971",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 86850412,
            "range": "± 2439231",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 178329100,
            "range": "± 3634770",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 464636,
            "range": "± 51652",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1014447,
            "range": "± 56788",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1984670,
            "range": "± 191104",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4252142,
            "range": "± 364493",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8200807,
            "range": "± 688129",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 16831201,
            "range": "± 991332",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 34199689,
            "range": "± 2532495",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 68737434,
            "range": "± 2434865",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 137769473,
            "range": "± 3372729",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 283471705,
            "range": "± 6275800",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 76276,
            "range": "± 1679",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 92079,
            "range": "± 1922",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 106721,
            "range": "± 3737",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 224457,
            "range": "± 2129",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 364753,
            "range": "± 2545",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4881052,
            "range": "± 39288",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 16922585,
            "range": "± 1082642",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 40218727,
            "range": "± 746626",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 219748055,
            "range": "± 3400895",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 430964103,
            "range": "± 5606204",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2345031,
            "range": "± 41450",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5627338,
            "range": "± 73462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10970816,
            "range": "± 79320",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53639078,
            "range": "± 202520",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 107416914,
            "range": "± 304731",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 96146,
            "range": "± 2785",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2586861,
            "range": "± 23541",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1235478,
            "range": "± 57258",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 199037,
            "range": "± 2275",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 371211,
            "range": "± 5954",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 665158,
            "range": "± 6173",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1264228,
            "range": "± 35021",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2473531,
            "range": "± 71753",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4873057,
            "range": "± 141592",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 9712285,
            "range": "± 200804",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 19467987,
            "range": "± 527762",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 39372873,
            "range": "± 1354717",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 79703362,
            "range": "± 1918141",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 96311,
            "range": "± 1576",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 99854,
            "range": "± 1134",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 112011,
            "range": "± 1857",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 238482,
            "range": "± 11002",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 382988,
            "range": "± 9997",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 4954075,
            "range": "± 92012",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 13504547,
            "range": "± 351140",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 35697684,
            "range": "± 873072",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 188192838,
            "range": "± 2056668",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 412175694,
            "range": "± 1882628",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2359201,
            "range": "± 52253",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5556495,
            "range": "± 52070",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10886160,
            "range": "± 64869",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53762830,
            "range": "± 237079",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 107359147,
            "range": "± 388645",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 67385,
            "range": "± 750",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2449788,
            "range": "± 10194",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1287295,
            "range": "± 80538",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 347928,
            "range": "± 42256",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 641263,
            "range": "± 58161",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1215163,
            "range": "± 116066",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2506340,
            "range": "± 274512",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 4983417,
            "range": "± 593723",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10237587,
            "range": "± 975570",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 20600838,
            "range": "± 1292060",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 41199385,
            "range": "± 1975820",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 83428371,
            "range": "± 2931960",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 170365518,
            "range": "± 3253604",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 467063,
            "range": "± 42646",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1004564,
            "range": "± 64796",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1943904,
            "range": "± 149447",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4116884,
            "range": "± 333756",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8156406,
            "range": "± 622953",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16565731,
            "range": "± 1111100",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33624006,
            "range": "± 1215172",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 67205261,
            "range": "± 2309909",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 136694293,
            "range": "± 9848833",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 283867558,
            "range": "± 5092003",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 76986,
            "range": "± 1754",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 92939,
            "range": "± 1036",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 107213,
            "range": "± 2511",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 224148,
            "range": "± 3735",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 364601,
            "range": "± 2219",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4898306,
            "range": "± 42775",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 13522693,
            "range": "± 268430",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 37644910,
            "range": "± 682366",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 217387798,
            "range": "± 2770346",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 430042166,
            "range": "± 5776701",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2410495,
            "range": "± 48515",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5633622,
            "range": "± 68323",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11000925,
            "range": "± 94333",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53935272,
            "range": "± 240535",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 107250889,
            "range": "± 420143",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 92938,
            "range": "± 2287",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4383,
            "range": "± 692",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4048,
            "range": "± 342",
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
          "id": "5a344c0c39bb2bf15c42350cc9b199789a179fa1",
          "message": "fix(servers): more descriptive errors when calls fail (#790)\n\n* fix(servers): more descriptive errors calls fail\r\n\r\nClosing #775\r\n\r\n* fix tests\r\n\r\n* rename constants\r\n\r\n* address grumbles",
          "timestamp": "2022-06-13T19:28:35Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/5a344c0c39bb2bf15c42350cc9b199789a179fa1"
        },
        "date": 1655167646848,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 183,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 197,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 112641,
            "range": "± 4126",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 3206612,
            "range": "± 96724",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1273853,
            "range": "± 37709",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 231211,
            "range": "± 16879",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 415295,
            "range": "± 10005",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 745766,
            "range": "± 15019",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1537054,
            "range": "± 71374",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2987407,
            "range": "± 42479",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 5605697,
            "range": "± 355528",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 11976156,
            "range": "± 534582",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 24048791,
            "range": "± 628125",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 48190817,
            "range": "± 1486243",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 90555304,
            "range": "± 4565888",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 109311,
            "range": "± 3843",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 113089,
            "range": "± 3445",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 135671,
            "range": "± 11871",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 281581,
            "range": "± 28766",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 450063,
            "range": "± 49223",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 10016731,
            "range": "± 506826",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 25964763,
            "range": "± 1722619",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 49658711,
            "range": "± 4043470",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 231474054,
            "range": "± 6638188",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 464681662,
            "range": "± 11073491",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2361282,
            "range": "± 12616",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5612094,
            "range": "± 26698",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10981724,
            "range": "± 52747",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53831265,
            "range": "± 91659",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 107390786,
            "range": "± 189495",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 74267,
            "range": "± 1931",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2870232,
            "range": "± 78289",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1276503,
            "range": "± 30113",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 387344,
            "range": "± 36297",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 714827,
            "range": "± 84733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1332596,
            "range": "± 136160",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2816491,
            "range": "± 396153",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5554700,
            "range": "± 639925",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 11983593,
            "range": "± 1058643",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 23310437,
            "range": "± 1444709",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 45744412,
            "range": "± 2381635",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 90941802,
            "range": "± 3145625",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 193649196,
            "range": "± 7554511",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 508524,
            "range": "± 43942",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1089420,
            "range": "± 108944",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 2152038,
            "range": "± 359739",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4431394,
            "range": "± 401185",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8511685,
            "range": "± 597316",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17539377,
            "range": "± 1168305",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 35387055,
            "range": "± 2237053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 70911199,
            "range": "± 3389349",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 141618154,
            "range": "± 4768919",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 306240223,
            "range": "± 11569097",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 87479,
            "range": "± 4912",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 97455,
            "range": "± 3163",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 119093,
            "range": "± 4636",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 252568,
            "range": "± 12179",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 401853,
            "range": "± 15664",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5721870,
            "range": "± 247504",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 19779974,
            "range": "± 1273875",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 36940902,
            "range": "± 1382903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 244580163,
            "range": "± 9173633",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 478881691,
            "range": "± 10386688",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2367311,
            "range": "± 12611",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5607858,
            "range": "± 17310",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10964848,
            "range": "± 20108",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53813022,
            "range": "± 161841",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 107418130,
            "range": "± 745881",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 111747,
            "range": "± 11316",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2867215,
            "range": "± 128739",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1269857,
            "range": "± 12003",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 218667,
            "range": "± 10943",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 402367,
            "range": "± 14164",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 779068,
            "range": "± 18148",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1475116,
            "range": "± 53584",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2911428,
            "range": "± 68811",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 5732633,
            "range": "± 226708",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 11376783,
            "range": "± 427486",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 21114429,
            "range": "± 803860",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 42502202,
            "range": "± 1260004",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 83922099,
            "range": "± 3054457",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 105212,
            "range": "± 8779",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 108972,
            "range": "± 3012",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 122624,
            "range": "± 3919",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 250310,
            "range": "± 13680",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 406963,
            "range": "± 20531",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5977256,
            "range": "± 239951",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 15047948,
            "range": "± 776381",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 38444570,
            "range": "± 1768197",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 208448621,
            "range": "± 6629134",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 459725562,
            "range": "± 13214291",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2356960,
            "range": "± 25156",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5610368,
            "range": "± 22104",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10973119,
            "range": "± 39908",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53790583,
            "range": "± 385527",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 107331467,
            "range": "± 114615",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 72408,
            "range": "± 2286",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2605298,
            "range": "± 61837",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1269028,
            "range": "± 31287",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 378919,
            "range": "± 44740",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 695922,
            "range": "± 77424",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1294562,
            "range": "± 110517",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2745231,
            "range": "± 350286",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5404021,
            "range": "± 648104",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 11282363,
            "range": "± 1117487",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 21964476,
            "range": "± 1564868",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 44115964,
            "range": "± 2468904",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 87257757,
            "range": "± 3996059",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 179966269,
            "range": "± 16538703",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 503773,
            "range": "± 107778",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 1082722,
            "range": "± 60014",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 2049861,
            "range": "± 137809",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4415994,
            "range": "± 417085",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8686737,
            "range": "± 686550",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 17730228,
            "range": "± 1199317",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 35402156,
            "range": "± 2113535",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 69831444,
            "range": "± 3044448",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 147501928,
            "range": "± 7388091",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 342876730,
            "range": "± 11888560",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 84902,
            "range": "± 3274",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 98332,
            "range": "± 3472",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 119748,
            "range": "± 4315",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 250291,
            "range": "± 8235",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 396494,
            "range": "± 10836",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5687491,
            "range": "± 151533",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 15224648,
            "range": "± 602207",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 44654831,
            "range": "± 1217744",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 246334590,
            "range": "± 5470283",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 487459080,
            "range": "± 15941660",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2379934,
            "range": "± 14128",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5622113,
            "range": "± 29805",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 11019009,
            "range": "± 44629",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 54143673,
            "range": "± 188333",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 107375732,
            "range": "± 161225",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 98213,
            "range": "± 5046",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 4575,
            "range": "± 1065",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 4033,
            "range": "± 463",
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
          "id": "01577daf6affb8471617166654b2760a1978cf36",
          "message": "Add resource limiting for `Subscriptions` (#786)\n\n* ws-server: Fix copyright for tests\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* rpc_module: Return a resource builder when subscribing\r\n\r\nRegistering a subscription returns the subscription'\r\ncallback wrapped into a `MethodResourcesBuilder` for resource\r\nlimiting purposes.\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Fix `register_subscription` tests\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* server: Drop `ResourceGuard` with `SubscriptionSink` for resource limit\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Check resource limits for subscription\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* proc-macros: Render resource limits for subscription macro\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Extend subscription limiting test via macro generation\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* core: Check if the `unsubscribe` method was already inserted\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Fix unsupported fields for subscriptions\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* server: Verify subscription methods before registering them\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Update test comment for subscription limiting\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Modify tests comments\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-06-14T16:01:24Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/01577daf6affb8471617166654b2760a1978cf36"
        },
        "date": 1655340268707,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 144,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "jsonrpsee_types_v2_vec",
            "value": 162,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/fast_call",
            "value": 95575,
            "range": "± 3443",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2486455,
            "range": "± 114740",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1174577,
            "range": "± 10517",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 186372,
            "range": "± 2377",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 346107,
            "range": "± 19951",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 616054,
            "range": "± 5877",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1172276,
            "range": "± 41678",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2320465,
            "range": "± 81798",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4590385,
            "range": "± 256200",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9162406,
            "range": "± 323073",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 18666712,
            "range": "± 772672",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 37547026,
            "range": "± 1071954",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 76901646,
            "range": "± 2125341",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 94276,
            "range": "± 1530",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 96543,
            "range": "± 3733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 109802,
            "range": "± 3319",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 212095,
            "range": "± 3273",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 337798,
            "range": "± 4626",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 8232379,
            "range": "± 218094",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 21029539,
            "range": "± 1266108",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 42620650,
            "range": "± 769379",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 193802549,
            "range": "± 1975085",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 383090166,
            "range": "± 1976650",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2252587,
            "range": "± 17089",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5440826,
            "range": "± 28935",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10844563,
            "range": "± 34092",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53462429,
            "range": "± 78307",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106632405,
            "range": "± 343956",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 62409,
            "range": "± 1036",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2316349,
            "range": "± 6903",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1148083,
            "range": "± 7871",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 343120,
            "range": "± 52694",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 635486,
            "range": "± 153158",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1233528,
            "range": "± 126283",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2595379,
            "range": "± 302239",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5336317,
            "range": "± 631451",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10618784,
            "range": "± 1135085",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 21164685,
            "range": "± 1638162",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 41504174,
            "range": "± 2282291",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 83571289,
            "range": "± 2283954",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 170681113,
            "range": "± 3574174",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 466445,
            "range": "± 51080",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 997323,
            "range": "± 58326",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1923889,
            "range": "± 149900",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4194551,
            "range": "± 349733",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8333627,
            "range": "± 659511",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17082858,
            "range": "± 1000974",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 33712420,
            "range": "± 1621590",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 67269620,
            "range": "± 2356415",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 135472876,
            "range": "± 3127317",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 286734712,
            "range": "± 7388053",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 71240,
            "range": "± 1201",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 81442,
            "range": "± 2833",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 100946,
            "range": "± 2082",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 221542,
            "range": "± 2494",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 362606,
            "range": "± 5196",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 5022610,
            "range": "± 56187",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 17853046,
            "range": "± 514047",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 35353063,
            "range": "± 503783",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 203609104,
            "range": "± 3356458",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 393539085,
            "range": "± 5569264",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2228520,
            "range": "± 11507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5445093,
            "range": "± 18462",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10864913,
            "range": "± 38656",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53433428,
            "range": "± 82529",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106593092,
            "range": "± 358890",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 94768,
            "range": "± 2014",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2472108,
            "range": "± 16387",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1172280,
            "range": "± 10414",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 186826,
            "range": "± 2896",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 346076,
            "range": "± 4529",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 616081,
            "range": "± 17847",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1168277,
            "range": "± 24273",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2299220,
            "range": "± 54476",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4551865,
            "range": "± 142332",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 8984649,
            "range": "± 442220",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 18126419,
            "range": "± 857874",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 36973966,
            "range": "± 1012569",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 73314836,
            "range": "± 2650478",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 93976,
            "range": "± 1654",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 95742,
            "range": "± 4363",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 107411,
            "range": "± 3355",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 212170,
            "range": "± 3131",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 339432,
            "range": "± 5181",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5072523,
            "range": "± 40211",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 12508519,
            "range": "± 378970",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 32681493,
            "range": "± 317299",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 168971424,
            "range": "± 1549433",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 375335792,
            "range": "± 3551003",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2232479,
            "range": "± 6214",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5433282,
            "range": "± 11863",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10829735,
            "range": "± 81322",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53382081,
            "range": "± 61322",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106582357,
            "range": "± 72316",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 62834,
            "range": "± 1458",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2450097,
            "range": "± 46115",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1142655,
            "range": "± 28110",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 348611,
            "range": "± 58449",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 628022,
            "range": "± 77084",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1227727,
            "range": "± 114760",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2549296,
            "range": "± 209969",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5254242,
            "range": "± 568640",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10229353,
            "range": "± 983537",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 20607387,
            "range": "± 1404959",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 41113241,
            "range": "± 2177227",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 81976850,
            "range": "± 2220809",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 165903279,
            "range": "± 3085748",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 454673,
            "range": "± 29798",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 988212,
            "range": "± 78735",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1929851,
            "range": "± 171006",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4106871,
            "range": "± 359528",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8216782,
            "range": "± 575041",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16806045,
            "range": "± 944291",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33662779,
            "range": "± 1390111",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 66377899,
            "range": "± 2716620",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 132272238,
            "range": "± 2565521",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 273852930,
            "range": "± 19623793",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 71005,
            "range": "± 1092",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 80337,
            "range": "± 1258",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 101737,
            "range": "± 1337",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 222103,
            "range": "± 1652",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 361379,
            "range": "± 7676",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 4961953,
            "range": "± 24799",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 12937132,
            "range": "± 237739",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 36609123,
            "range": "± 1013074",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 202092640,
            "range": "± 3182557",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 390190641,
            "range": "± 6205680",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2226937,
            "range": "± 10746",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5435596,
            "range": "± 12737",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10852403,
            "range": "± 32643",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53430325,
            "range": "± 300367",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106634446,
            "range": "± 244145",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 82477,
            "range": "± 1059",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 3752,
            "range": "± 453",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3543,
            "range": "± 555",
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
          "id": "01577daf6affb8471617166654b2760a1978cf36",
          "message": "Add resource limiting for `Subscriptions` (#786)\n\n* ws-server: Fix copyright for tests\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* rpc_module: Return a resource builder when subscribing\r\n\r\nRegistering a subscription returns the subscription'\r\ncallback wrapped into a `MethodResourcesBuilder` for resource\r\nlimiting purposes.\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Fix `register_subscription` tests\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* server: Drop `ResourceGuard` with `SubscriptionSink` for resource limit\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Check resource limits for subscription\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* proc-macros: Render resource limits for subscription macro\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Extend subscription limiting test via macro generation\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* core: Check if the `unsubscribe` method was already inserted\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* tests: Fix unsupported fields for subscriptions\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* server: Verify subscription methods before registering them\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Update test comment for subscription limiting\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>\r\n\r\n* Modify tests comments\r\n\r\nSigned-off-by: Alexandru Vasile <alexandru.vasile@parity.io>",
          "timestamp": "2022-06-14T16:01:24Z",
          "url": "https://github.com/paritytech/jsonrpsee/commit/01577daf6affb8471617166654b2760a1978cf36"
        },
        "date": 1655426665306,
        "tool": "cargo",
        "benches": [
          {
            "name": "jsonrpsee_types_v2_array_ref",
            "value": 150,
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
            "name": "sync/http_round_trip/fast_call",
            "value": 101654,
            "range": "± 5840",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/memory_intense",
            "value": 2545260,
            "range": "± 319953",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_round_trip/slow_call",
            "value": 1196156,
            "range": "± 14540",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/2",
            "value": 189962,
            "range": "± 1813",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/4",
            "value": 351106,
            "range": "± 14266",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/8",
            "value": 625432,
            "range": "± 8452",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/16",
            "value": 1196565,
            "range": "± 32228",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/32",
            "value": 2353677,
            "range": "± 82193",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/64",
            "value": 4681474,
            "range": "± 195746",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/128",
            "value": 9333764,
            "range": "± 286686",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/256",
            "value": 18704565,
            "range": "± 511457",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/512",
            "value": 38325753,
            "range": "± 1536125",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_concurrent_conn_calls/fast_call/1024",
            "value": 77750689,
            "range": "± 1912385",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/2",
            "value": 97645,
            "range": "± 2006",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/5",
            "value": 98549,
            "range": "± 1884",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/10",
            "value": 111171,
            "range": "± 2236",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/50",
            "value": 215601,
            "range": "± 3070",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/fast_call/100",
            "value": 345960,
            "range": "± 5408",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/2",
            "value": 9047568,
            "range": "± 596801",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/5",
            "value": 21590490,
            "range": "± 1180925",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/10",
            "value": 42704607,
            "range": "± 881852",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/50",
            "value": 192753267,
            "range": "± 2139850",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/memory_intense/100",
            "value": 384906101,
            "range": "± 3309341",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/2",
            "value": 2260670,
            "range": "± 7507",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/5",
            "value": 5468902,
            "range": "± 10927",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/10",
            "value": 10872731,
            "range": "± 30678",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/50",
            "value": 53505311,
            "range": "± 74700",
            "unit": "ns/iter"
          },
          {
            "name": "sync/http_batch_requests/slow_call/100",
            "value": 106692730,
            "range": "± 158027",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/fast_call",
            "value": 64291,
            "range": "± 955",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/memory_intense",
            "value": 2325175,
            "range": "± 23872",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_round_trip/slow_call",
            "value": 1155711,
            "range": "± 29324",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/2",
            "value": 345853,
            "range": "± 43608",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/4",
            "value": 638496,
            "range": "± 57419",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/8",
            "value": 1259162,
            "range": "± 138617",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/16",
            "value": 2628785,
            "range": "± 335560",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/32",
            "value": 5168518,
            "range": "± 673744",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/64",
            "value": 10828046,
            "range": "± 1060339",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/128",
            "value": 21235760,
            "range": "± 1624837",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/256",
            "value": 42005891,
            "range": "± 2090068",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/512",
            "value": 84250240,
            "range": "± 2933321",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_calls/1024",
            "value": 171205356,
            "range": "± 4255417",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/2",
            "value": 464065,
            "range": "± 52669",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/4",
            "value": 1009487,
            "range": "± 145224",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/8",
            "value": 1950411,
            "range": "± 180484",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/16",
            "value": 4134048,
            "range": "± 400757",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/32",
            "value": 8450692,
            "range": "± 696218",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/64",
            "value": 17081248,
            "range": "± 1161173",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/128",
            "value": 34516017,
            "range": "± 1539537",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/256",
            "value": 67437016,
            "range": "± 2839296",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/512",
            "value": 136133061,
            "range": "± 2900483",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_concurrent_conn_subs/1024",
            "value": 281734004,
            "range": "± 7858008",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/2",
            "value": 72287,
            "range": "± 1122",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/5",
            "value": 82767,
            "range": "± 2325",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/10",
            "value": 103845,
            "range": "± 3041",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/50",
            "value": 231048,
            "range": "± 3827",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/fast_call/100",
            "value": 370424,
            "range": "± 5547",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/2",
            "value": 4991036,
            "range": "± 113345",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/5",
            "value": 13295403,
            "range": "± 308940",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/10",
            "value": 27323564,
            "range": "± 496243",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/50",
            "value": 200311882,
            "range": "± 2875306",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/memory_intense/100",
            "value": 389875809,
            "range": "± 5100770",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/2",
            "value": 2232095,
            "range": "± 16272",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/5",
            "value": 5445738,
            "range": "± 12631",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/10",
            "value": 10889971,
            "range": "± 42136",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/50",
            "value": 53486498,
            "range": "± 102101",
            "unit": "ns/iter"
          },
          {
            "name": "sync/ws_batch_requests/slow_call/100",
            "value": 106643397,
            "range": "± 112411",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/fast_call",
            "value": 95475,
            "range": "± 4458",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/memory_intense",
            "value": 2465894,
            "range": "± 61199",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_round_trip/slow_call",
            "value": 1169616,
            "range": "± 7856",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/2",
            "value": 187808,
            "range": "± 2317",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/4",
            "value": 346995,
            "range": "± 3939",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/8",
            "value": 620126,
            "range": "± 6316",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/16",
            "value": 1170927,
            "range": "± 28033",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/32",
            "value": 2291940,
            "range": "± 65818",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/64",
            "value": 4522704,
            "range": "± 174398",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/128",
            "value": 8989082,
            "range": "± 408115",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/256",
            "value": 18616572,
            "range": "± 894970",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/512",
            "value": 36769190,
            "range": "± 973681",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_concurrent_conn_calls/fast_call/1024",
            "value": 73480949,
            "range": "± 2227150",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/2",
            "value": 95394,
            "range": "± 2361",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/5",
            "value": 101139,
            "range": "± 22005",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/10",
            "value": 110708,
            "range": "± 4129",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/50",
            "value": 215402,
            "range": "± 3671",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/fast_call/100",
            "value": 343949,
            "range": "± 7708",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/2",
            "value": 5029278,
            "range": "± 38385",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/5",
            "value": 12612902,
            "range": "± 208078",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/10",
            "value": 32901056,
            "range": "± 364285",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/50",
            "value": 171383092,
            "range": "± 2086403",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/memory_intense/100",
            "value": 374935237,
            "range": "± 2020586",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/2",
            "value": 2250072,
            "range": "± 7588",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/5",
            "value": 5453797,
            "range": "± 10197",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/10",
            "value": 10876137,
            "range": "± 37611",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/50",
            "value": 53491712,
            "range": "± 65138",
            "unit": "ns/iter"
          },
          {
            "name": "async/http_batch_requests/slow_call/100",
            "value": 106667645,
            "range": "± 71962",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/fast_call",
            "value": 63457,
            "range": "± 1451",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/memory_intense",
            "value": 2386628,
            "range": "± 6755",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_round_trip/slow_call",
            "value": 1147674,
            "range": "± 6650",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/2",
            "value": 345003,
            "range": "± 35855",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/4",
            "value": 623848,
            "range": "± 62762",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/8",
            "value": 1273460,
            "range": "± 136489",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/16",
            "value": 2664487,
            "range": "± 341198",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/32",
            "value": 5217035,
            "range": "± 586380",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/64",
            "value": 10433874,
            "range": "± 1168601",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/128",
            "value": 20540067,
            "range": "± 1605776",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/256",
            "value": 40337754,
            "range": "± 1953203",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/512",
            "value": 82917867,
            "range": "± 3323879",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_calls/1024",
            "value": 166561630,
            "range": "± 4590127",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/2",
            "value": 462968,
            "range": "± 50928",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/4",
            "value": 997645,
            "range": "± 47940",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/8",
            "value": 1901161,
            "range": "± 153246",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/16",
            "value": 4169419,
            "range": "± 401325",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/32",
            "value": 8199844,
            "range": "± 598915",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/64",
            "value": 16490803,
            "range": "± 876861",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/128",
            "value": 33106820,
            "range": "± 1242976",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/256",
            "value": 65166342,
            "range": "± 2257431",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/512",
            "value": 132164552,
            "range": "± 3284633",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_concurrent_conn_subs/1024",
            "value": 273530849,
            "range": "± 16489598",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/2",
            "value": 71676,
            "range": "± 1343",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/5",
            "value": 81769,
            "range": "± 2078",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/10",
            "value": 102849,
            "range": "± 1544",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/50",
            "value": 223690,
            "range": "± 4083",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/fast_call/100",
            "value": 367352,
            "range": "± 3237",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/2",
            "value": 5045975,
            "range": "± 52180",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/5",
            "value": 12882102,
            "range": "± 183120",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/10",
            "value": 34598891,
            "range": "± 637951",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/50",
            "value": 201740723,
            "range": "± 4158192",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/memory_intense/100",
            "value": 389064373,
            "range": "± 6376654",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/2",
            "value": 2214684,
            "range": "± 30026",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/5",
            "value": 5474753,
            "range": "± 37517",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/10",
            "value": 10863586,
            "range": "± 45934",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/50",
            "value": 53476985,
            "range": "± 110234",
            "unit": "ns/iter"
          },
          {
            "name": "async/ws_batch_requests/slow_call/100",
            "value": 106633943,
            "range": "± 430745",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe",
            "value": 82299,
            "range": "± 1608",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/subscribe_response",
            "value": 3617,
            "range": "± 760",
            "unit": "ns/iter"
          },
          {
            "name": "subscriptions/unsub",
            "value": 3360,
            "range": "± 549",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}