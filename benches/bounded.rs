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
const THREADS : usize = 4;


fn seq( c: &mut Criterion )
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

		group.bench_function( format!( "Seq Tokio sync, buffer size: {} msgs", &buffer_size ), |b|
		{
			let mut pool = LocalPool::new();
			let exec = pool.spawner();

			b.iter_batched
			(

				|| // setup
				{
					let (tx, rx) = mpsc::channel( *buffer_size +1 );
					let tx = TokioSender::new( tx );

					seq_run( tx, rx, MESSAGES, &exec );
				},

				// measure
				//
				|_| pool.run(),

				BatchSize::SmallInput
			)
		});


		group.bench_function( format!( "Seq futures mpsc, buffer size: {} msgs", &buffer_size ), |b|
		{
			let mut pool = LocalPool::new();
			let exec = pool.spawner();

			b.iter_batched
			(

				|| // setup
				{
					let (tx, rx) = futures::channel::mpsc::channel( *buffer_size );

					seq_run( tx, rx, MESSAGES, &exec );
				},

				// measure
				//
				|_| pool.run(),

				BatchSize::SmallInput
			)
		});


		group.bench_function( format!( "Seq influmenza mpsc, buffer size: {} msgs", &buffer_size ), |b|
		{
			let mut pool = LocalPool::new();
			let exec = pool.spawner();

			b.iter_batched
			(
				|| // setup
				{
					let (tx, rx) = influmenza::channel( Some( *buffer_size ) );

					seq_run( tx, rx, MESSAGES, &exec );
				},

				// measure
				//
				|_| pool.run(),

				BatchSize::SmallInput
			)
		});

	}
}


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
						for _ in 0..MESSAGES
						{
							rx.next().await;
						}

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
						for _ in 0..MESSAGES
						{
							rx.next().await;
						}

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
						for _ in 0..MESSAGES
						{
							rx.next().await;
						}

					});

					handle.join().expect( "join thread" );
				},

				BatchSize::SmallInput
			)
		);
	}
}



fn seq_run<S>( mut tx: S, mut rx: impl Stream + std::marker::Unpin + 'static, msgs: usize, exec: impl LocalSpawnExt )

	where S: Sink<Msg> + std::marker::Unpin + 'static,
	      S::Error: fmt::Debug,
{
	exec.spawn_local( async move
	{
		for i in 0..msgs
		{
			tx.send( Msg::new(i) ).await.expect( "send msg" );
		}

	}).expect( "spawn writer" );

	exec.spawn_local( async move
	{
		for _ in 0..msgs
		{
			rx.next().await;
		}

	}).expect( "spawn reader" );
}



fn mpsc( c: &mut Criterion )
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

		group.bench_function
		(
			format!( "Mpsc tokio sync mpsc, buffer size: {} msgs", &buffer_size ),

			|b| b.iter_batched
			(
				move || // setup
				{
					let (tx, rx) = mpsc::channel( *buffer_size +THREADS );
					let tx = TokioSender::new( tx );

					let mut handles   = Vec::new();
					let mut start_txs = Vec::new();

					for _ in 0..THREADS
					{
						let (start_tx, start_rx) = oneshot::channel();
						let mut tx = tx.clone();

						let handle = std::thread::spawn( move ||
						{
							block_on( async move
							{
								start_rx.await.expect( "oneshot receive" );

								for i in 0..MESSAGES/THREADS
								{
									tx.send( Msg::new(i) ).await.expect( "send msg" );
								}
							});
						});

						handles  .push( handle   );
						start_txs.push( start_tx );
					}

					(start_txs, rx, handles)
				},


				|(start_txs, mut rx, handles)| // routine
				{
					for s in start_txs.into_iter()
					{
						s.send(()).expect( "oneshot send" );
					}

					block_on( async move
					{
						for _ in 0..MESSAGES { rx.next().await; }
					});

					for h in handles.into_iter()
					{
						h.join().expect( "join thread" );
					}
				},

				BatchSize::SmallInput
			)
		);

		group.bench_function
		(
			format!( "Mpsc futures mpsc, buffer size: {} msgs", &buffer_size ),

			|b| b.iter_batched
			(
				move || // setup
				{
					let (tx, rx) = futures::channel::mpsc::channel( *buffer_size );

					let mut handles   = Vec::new();
					let mut start_txs = Vec::new();

					for _ in 0..THREADS
					{
						let (start_tx, start_rx) = oneshot::channel();
						let mut tx = tx.clone();

						let handle = std::thread::spawn( move ||
						{
							block_on( async move
							{
								start_rx.await.expect( "oneshot receive" );

								for i in 0..MESSAGES/THREADS
								{
									tx.send( Msg::new(i) ).await.expect( "send msg" );
								}
							});
						});

						handles  .push( handle   );
						start_txs.push( start_tx );
					}

					(start_txs, rx, handles)
				},


				|(start_txs, mut rx, handles)| // routine
				{
					for s in start_txs.into_iter()
					{
						s.send(()).expect( "oneshot send" );
					}

					block_on( async move
					{
						for _ in 0..MESSAGES { rx.next().await; }
					});

					for h in handles.into_iter()
					{
						h.join().expect( "join thread" );
					}
				},

				BatchSize::SmallInput
			)
		);

		// group.bench_function
		// (
		// 	format!( "Mpsc influmenza mpsc, buffer size: {} msgs", &buffer_size ),

		// 	|b| b.iter_batched
		// 	(
		// 		move || // setup
		// 		{
		// 			let (tx, rx) = influmenza::channel( Some( *buffer_size ) );

		// 			let mut handles   = Vec::new();
		// 			let mut start_txs = Vec::new();

		// 			for _ in 0..THREADS
		// 			{
		// 				let (start_tx, start_rx) = oneshot::channel();
		// 				let mut tx = tx.clone();

		// 				let handle = std::thread::spawn( move ||
		// 				{
		// 					block_on( async move
		// 					{
		// 						start_rx.await.expect( "oneshot receive" );

		// 						for i in 0..MESSAGES/THREADS
		// 						{
		// 							tx.send( Msg::new(i) ).await.expect( "send msg" );
		// 						}
		// 					});
		// 				});

		// 				handles  .push( handle   );
		// 				start_txs.push( start_tx );
		// 			}

		// 			(start_txs, rx, handles)
		// 		},


		// 		|(start_txs, mut rx, handles)| // routine
		// 		{
		// 			for s in start_txs.into_iter()
		// 			{
		// 				s.send(()).expect( "oneshot send" );
		// 			}

		// 			block_on( async move
		// 			{
		// 				for _ in 0..MESSAGES { rx.next().await; }
		// 			});

		// 			for h in handles.into_iter()
		// 			{
		// 				h.join().expect( "join thread" );
		// 			}
		// 		},

		// 		BatchSize::SmallInput
		// 	)
		// );
	}
}



criterion_group!( benches, /*seq, spsc,*/ mpsc );
criterion_main! ( benches );
