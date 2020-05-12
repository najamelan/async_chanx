// Benchmark different channels:
//
// # Types of usage:
// - spsc
// - mpsc
// - mpmc
//
// # Metrics:
// - without contention
// - under contention
// - memory usages
// - latency
// - throughput
//

mod common;

use
{
	common :: { * } ,
};

const MESSAGES: usize = 10_000;


fn spsc( c: &mut Criterion )
{
	// let _ = flexi_logger::Logger::with_str( "warn, executor_benchmarks=trace" ).start();


	let mut group = c.benchmark_group( "Bounded" );

	for buffer_size in [ 1, 10, 50, 100 ].iter()
	{
		// match buffer_size
		// {
		// 	10  => { group.sample_size( 100 ); }
		// 	100 => { group.sample_size( 50  ); }
		// 	200 => { group.sample_size( 30  ); }
		// 	_   => { unreachable!();           }
		// }

		group.sample_size( 10 );

		group.bench_function
		(
			format!( "Spsc Tokio sync, buffer size: {} msgs", &buffer_size ),

			|b| b.iter_batched
			(
				move || // setup
				{
					let (tx, rx) = mpsc::channel( *buffer_size +1 );
					let mut tx = TokioSender::new( tx );

					let (start_tx, start_rx) = oneshot::channel();

					let handle = std::thread::spawn( move ||
					{
						block_on( async move
						{
							start_rx.await.expect( "oneshot receive" );

							for i in 0..MESSAGES
							{
								tx.send( Msg::new(i) ).await.expect( "send msg" );
							}
						});
					});

					(start_tx, rx, handle)
				},


				|(start_tx, mut rx, handle)| // routine
				{
					start_tx.send(()).expect( "oneshot send" );

					block_on( async move
					{
						let mut count = 0;

						while let Some(_) = rx.next().await
						{
							count += 1;
						}

						assert_eq!( count, MESSAGES );
					});

					handle.join().expect( "join thread" );
				},

				BatchSize::SmallInput
			)
		);

		group.bench_function
		(
			format!( "Spsc futures mpsc, buffer size: {} msgs", &buffer_size ),

			|b| b.iter_batched
			(
				move || // setup
				{
					let (mut tx, rx) = futures::channel::mpsc::channel( *buffer_size );

					let (start_tx, start_rx) = oneshot::channel();

					let handle = std::thread::spawn( move ||
					{
						block_on( async move
						{
							start_rx.await.expect( "oneshot receive" );

							for i in 0..MESSAGES
							{
								tx.send( Msg::new(i) ).await.expect( "send msg" );
							}
						});
					});

					(start_tx, rx, handle)
				},


				|(start_tx, mut rx, handle)| // routine
				{
					start_tx.send(()).expect( "oneshot send" );

					block_on( async move
					{
						let mut count = 0;

						while let Some(_) = rx.next().await
						{
							count += 1;
						}

						assert_eq!( count, MESSAGES );
					});

					handle.join().expect( "join thread" );
				},

				BatchSize::SmallInput
			)
		);

		group.bench_function
		(
			format!( "Spsc influmenza mpsc, buffer size: {} msgs", &buffer_size ),

			|b| b.iter_batched
			(
				move || // setup
				{
					let (mut tx, rx) = influmenza::channel( Some( *buffer_size ) );

					let (start_tx, start_rx) = oneshot::channel();

					let handle = std::thread::spawn( move ||
					{
						block_on( async move
						{
							start_rx.await.expect( "oneshot receive" );

							for i in 0..MESSAGES
							{
								tx.send( Msg::new(i) ).await.expect( "send msg" );
							}
						});
					});

					(start_tx, rx, handle)
				},


				|(start_tx, mut rx, handle)| // routine
				{
					start_tx.send(()).expect( "oneshot send" );

					block_on( async move
					{
						let mut count = 0;

						while let Some(_) = rx.next().await
						{
							count += 1;
						}

						assert_eq!( count, MESSAGES );
					});

					handle.join().expect( "join thread" );
				},

				BatchSize::SmallInput
			)
		);
	}
}


criterion_group!( benches, spsc );
criterion_main! ( benches );
