// NOTE: THIS BENCHMARK IS A DRAFT
// It mostly measures latency and not actually the number of requests per second that our server
// is capable of answering.

#[macro_use]
extern crate criterion;

jsonrpsee::rpc_api! {
    Foo { fn identity(s: String) -> String; }
}

pub fn criterion_benchmark(c: &mut criterion::Criterion) {
    c.bench_function("client-server-request", |b| {
        // TODO: use port 0 instead
        let port = rand::random::<u16>().saturating_add(1024);

        async_std::task::spawn(async move {
            let listen_addr = format!("127.0.0.1:{}", port).parse().unwrap();
            let mut server1 = jsonrpsee::http_server(&listen_addr).await.unwrap();

            while let Ok(request) = Foo::next_request(&mut server1).await {
                match request {
                    Foo::Identity { respond, s } => {
                        respond.ok(s).await;
                    }
                }
            }
        });

        let mut client = jsonrpsee::http_client(&format!("http://127.0.0.1:{}", port));
        let hello = "hello".to_string();
        b.iter(|| {
            async_std::task::block_on(async {
                for _ in 0..100 {
                    let _ = Foo::identity(&mut client, hello.clone()).await.unwrap();
                }
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
