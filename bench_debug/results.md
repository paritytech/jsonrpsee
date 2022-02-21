Here I collected some results of benchmarks. 
The way how I tested it: I run 6 benchmarks in a row and put in the result column difference between maximum and minimum value of each benchmark.

1 - Benchmarks were run on ci5 hosts directly from binary without docker

2 - Result of 6 last pipelene runs from Victoria Metrics

3 - Benchmarks were run on a GCP machine (8 CPU, 8 GB RAM)

|1 benchname|difference|2 benchname|difference|3 benchname|difference|
|---|---|---|---|---|---|
|jsonrpsee_types_v2_array_ref|**0%**|jsonrpsee_types_v2_array_ref|**2%**|jsonrpsee_types_v2_array_ref|**1%**|
|jsonrpsee_types_v2_vec|**1%**|jsonrpsee_types_v2_vec|**4%**|jsonrpsee_types_v2_vec|**37%**|
|sync/http_round_trip|**15%**|sync/http_round_trip|**4%**|sync/http_round_trip|**4%**|
|sync/http_concurrent_round_trip/14|**5%**|sync/http_concurrent_round_trip/14|**4%**|sync/http_concurrent_round_trip/2|**3%**|
|sync/http_concurrent_round_trip/28|**2%**|sync/http_concurrent_round_trip/28|**3%**|sync/http_concurrent_round_trip/4|**2%**|
|sync/http_concurrent_round_trip/56|**2%**|sync/http_concurrent_round_trip/56|**2%**|sync/http_concurrent_round_trip/8|**2%**|
|sync/http_concurrent_round_trip/112|**3%**|sync/http_concurrent_round_trip/112|**4%**|sync/http_concurrent_round_trip/16|**2%**|
|sync/http_concurrent_round_trip/224|**4%**|sync/http_concurrent_round_trip/224|**5%**|sync/http_concurrent_round_trip/32|**1%**|
|sync/http_concurrent_connections/2|**4%**|sync/http_concurrent_connections/2|**2%**|sync/http_concurrent_connections/2|**1%**|
|sync/http_concurrent_connections/4|**7%**|sync/http_concurrent_connections/4|**4%**|sync/http_concurrent_connections/4|**2%**|
|sync/http_concurrent_connections/8|**8%**|sync/http_concurrent_connections/8|**4%**|sync/http_concurrent_connections/8|**1%**|
|sync/http_concurrent_connections/16|**6%**|sync/http_concurrent_connections/16|**5%**|sync/http_concurrent_connections/16|**2%**|
|sync/http_concurrent_connections/32|**4%**|sync/http_concurrent_connections/32|**3%**|sync/http_concurrent_connections/32|**3%**|
|sync/http_concurrent_connections/64|**1%**|sync/http_concurrent_connections/64|**2%**|sync/http_concurrent_connections/64|**2%**|
|sync/http_batch_requests/2|**5%**|sync/http_batch_requests/2|**8%**|sync/http_batch_requests/2|**8%**|
|sync/http_batch_requests/5|**6%**|sync/http_batch_requests/5|**3%**|sync/http_batch_requests/5|**7%**|
|sync/http_batch_requests/10|**1%**|sync/http_batch_requests/10|**5%**|sync/http_batch_requests/10|**5%**|
|sync/http_batch_requests/50|**8%**|sync/http_batch_requests/50|**6%**|sync/http_batch_requests/50|**4%**|
|sync/http_batch_requests/100|**0%**|sync/http_batch_requests/100|**2%**|sync/http_batch_requests/100|**2%**|
|sync/ws_round_trip|**10%**|sync/ws_round_trip|**14%**|sync/ws_round_trip|**6%**|
|sync/ws_concurrent_round_trip/14|**9%**|sync/ws_concurrent_round_trip/14|**8%**|sync/ws_concurrent_round_trip/2|**2%**|
|sync/ws_concurrent_round_trip/28|**1%**|sync/ws_concurrent_round_trip/28|**6%**|sync/ws_concurrent_round_trip/4|**2%**|
|sync/ws_concurrent_round_trip/56|**3%**|sync/ws_concurrent_round_trip/56|**6%**|sync/ws_concurrent_round_trip/8|**3%**|
|sync/ws_concurrent_round_trip/112|**2%**|sync/ws_concurrent_round_trip/112|**3%**|sync/ws_concurrent_round_trip/16|**4%**|
|sync/ws_concurrent_round_trip/224|**2%**|sync/ws_concurrent_round_trip/224|**1%**|sync/ws_concurrent_round_trip/32|**52%**|
|sync/ws_concurrent_connections/2|**9%**|sync/ws_concurrent_connections/2|**14%**|sync/ws_concurrent_connections/2|**10%**|
|sync/ws_concurrent_connections/4|**11%**|sync/ws_concurrent_connections/4|**11%**|sync/ws_concurrent_connections/4|**1%**|
|sync/ws_concurrent_connections/8|**4%**|sync/ws_concurrent_connections/8|**9%**|sync/ws_concurrent_connections/8|**1%**|
|sync/ws_concurrent_connections/16|**10%**|sync/ws_concurrent_connections/16|**12%**|sync/ws_concurrent_connections/16|**2%**|
|sync/ws_concurrent_connections/32|**4%**|sync/ws_concurrent_connections/32|**8%**|sync/ws_concurrent_connections/32|**2%**|
|sync/ws_concurrent_connections/64|**1%**|sync/ws_concurrent_connections/64|**6%**|sync/ws_concurrent_connections/64|**8%**|
|sync/ws_batch_requests/2|**7%**|sync/ws_batch_requests/2|**6%**|sync/ws_batch_requests/2|**4%**|
|sync/ws_batch_requests/5|**4%**|sync/ws_batch_requests/5|**8%**|sync/ws_batch_requests/5|**5%**|
|sync/ws_batch_requests/10|**5%**|sync/ws_batch_requests/10|**5%**|sync/ws_batch_requests/10|**3%**|
|sync/ws_batch_requests/50|**5%**|sync/ws_batch_requests/50|**5%**|sync/ws_batch_requests/50|**6%**|
|sync/ws_batch_requests/100|**6%**|sync/ws_batch_requests/100|**5%**|sync/ws_batch_requests/100|**2%**|
|async/http_round_trip|**10%**|async/http_round_trip|**12%**|async/http_round_trip|**2%**|
|async/http_concurrent_round_trip/14|**3%**|async/http_concurrent_round_trip/14|**2%**|async/http_concurrent_round_trip/2|**3%**|
|async/http_concurrent_round_trip/28|**1%**|async/http_concurrent_round_trip/28|**1%**|async/http_concurrent_round_trip/4|**2%**|
|async/http_concurrent_round_trip/56|**4%**|async/http_concurrent_round_trip/56|**2%**|async/http_concurrent_round_trip/8|**4%**|
|async/http_concurrent_round_trip/112|**4%**|async/http_concurrent_round_trip/112|**3%**|async/http_concurrent_round_trip/16|**3%**|
|async/http_concurrent_round_trip/224|**4%**|async/http_concurrent_round_trip/224|**5%**|async/http_concurrent_round_trip/32|**3%**|
|async/http_concurrent_connections/2|**3%**|async/http_concurrent_connections/2|**2%**|async/http_concurrent_connections/2|**1%**|
|async/http_concurrent_connections/4|**7%**|async/http_concurrent_connections/4|**3%**|async/http_concurrent_connections/4|**2%**|
|async/http_concurrent_connections/8|**6%**|async/http_concurrent_connections/8|**4%**|async/http_concurrent_connections/8|**2%**|
|async/http_concurrent_connections/16|**4%**|async/http_concurrent_connections/16|**2%**|async/http_concurrent_connections/16|**1%**|
|async/http_concurrent_connections/32|**4%**|async/http_concurrent_connections/32|**3%**|async/http_concurrent_connections/32|**3%**|
|async/http_concurrent_connections/64|**1%**|async/http_concurrent_connections/64|**2%**|async/http_concurrent_connections/64|**2%**|
|async/http_batch_requests/2|**11%**|async/http_batch_requests/2|**7%**|async/http_batch_requests/2|**4%**|
|async/http_batch_requests/5|**8%**|async/http_batch_requests/5|**7%**|async/http_batch_requests/5|**5%**|
|async/http_batch_requests/10|**5%**|async/http_batch_requests/10|**7%**|async/http_batch_requests/10|**5%**|
|async/http_batch_requests/50|**3%**|async/http_batch_requests/50|**8%**|async/http_batch_requests/50|**5%**|
|async/http_batch_requests/100|**3%**|async/http_batch_requests/100|**2%**|async/http_batch_requests/100|**2%**|
|async/ws_round_trip|**11%**|async/ws_round_trip|**11%**|async/ws_round_trip|**6%**|
|async/ws_concurrent_round_trip/14|**2%**|async/ws_concurrent_round_trip/14|**5%**|async/ws_concurrent_round_trip/2|**2%**|
|async/ws_concurrent_round_trip/28|**4%**|async/ws_concurrent_round_trip/28|**5%**|async/ws_concurrent_round_trip/4|**4%**|
|async/ws_concurrent_round_trip/56|**3%**|async/ws_concurrent_round_trip/56|**3%**|async/ws_concurrent_round_trip/8|**3%**|
|async/ws_concurrent_round_trip/112|**2%**|async/ws_concurrent_round_trip/112|**2%**|async/ws_concurrent_round_trip/16|**3%**|
|async/ws_concurrent_round_trip/224|**3%**|async/ws_concurrent_round_trip/224|**3%**|async/ws_concurrent_round_trip/32|**2%**|
|async/ws_concurrent_connections/2|**6%**|async/ws_concurrent_connections/2|**17%**|async/ws_concurrent_connections/2|**4%**|
|async/ws_concurrent_connections/4|**8%**|async/ws_concurrent_connections/4|**6%**|async/ws_concurrent_connections/4|**1%**|
|async/ws_concurrent_connections/8|**10%**|async/ws_concurrent_connections/8|**7%**|async/ws_concurrent_connections/8|**1%**|
|async/ws_concurrent_connections/16|**6%**|async/ws_concurrent_connections/16|**12%**|async/ws_concurrent_connections/16|**3%**|
|async/ws_concurrent_connections/32|**7%**|async/ws_concurrent_connections/32|**9%**|async/ws_concurrent_connections/32|**4%**|
|async/ws_concurrent_connections/64|**5%**|async/ws_concurrent_connections/64|**8%**|async/ws_concurrent_connections/64|**6%**|
|async/ws_batch_requests/2|**6%**|async/ws_batch_requests/2|**2%**|async/ws_batch_requests/2|**5%**|
|async/ws_batch_requests/5|**7%**|async/ws_batch_requests/5|**4%**|async/ws_batch_requests/5|**3%**|
|async/ws_batch_requests/10|**10%**|async/ws_batch_requests/10|**5%**|async/ws_batch_requests/10|**6%**|
|async/ws_batch_requests/50|**3%**|async/ws_batch_requests/50|**3%**|async/ws_batch_requests/50|**3%**|
|async/ws_batch_requests/100|**2%**|async/ws_batch_requests/100|**4%**|async/ws_batch_requests/100|**2%**|
|subscriptions/subscribe|**8%**|subscriptions/subscribe|**23%**|subscriptions/subscribe|**7%**|
|subscriptions/subscribe_response|**24%**|subscriptions/subscribe_response|**23%**|subscriptions/subscribe_response|**3%**|
|subscriptions/unsub|**52%**|subscriptions/unsub|**26%**|subscriptions/unsub|**6%**|
