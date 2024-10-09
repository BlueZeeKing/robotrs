searchState.loadedDescShard("futures_concurrency", 0, "Performant, portable, structured concurrency operations …\nHelper functions and types for fixed-length arrays.\nConcurrent execution of streams\nAsynchronous basic functionality.\nThe futures concurrency prelude.\nComposable asynchronous iteration.\nParallel iterator types for vectors (<code>Vec&lt;T&gt;</code>)\nA collection of errors.\nA stream that chains multiple streams one after another.\nA future which waits for two similarly-typed futures to …\nA stream that merges multiple streams into a single stream.\nA future which waits for the first future to complete.\nA future which waits for the first successful future to …\nA future which waits for all futures to complete …\nA stream that ‘zips up’ multiple streams into a single …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe consumer is done making progress, and the <code>finish</code> …\nConcurrently operate over items in a stream\nDescribes a type which can receive data.\nThe state of the consumer, used to communicate back to the …\nThe consumer is ready to keep making progress.\nThe consumer currently holds no values and should not be …\nA concurrent iterator that yields the current count and …\nConversion from a <code>ConcurrentStream</code>\nA concurrent for each implementation from a <code>Stream</code>\nWhat’s the type of the future containing our items?\nConversion into a <code>ConcurrentStream</code>\nWhich kind of iterator are we turning this into?\nThe type of the elements being iterated over.\nWhich item will we be yielding?\nA concurrent iterator that limits the amount of …\nConvert items from one type into another\nWhat is the type of the item we’re returning when …\nA concurrent iterator that only iterates over the first <code>n</code> …\nTransforms an iterator into a collection.\nHow much concurrency should we apply?\nInternal method used to define the behavior of this …\nCreates a stream which gives the current iteration count …\nWe have no more data left to send to the <code>Consumer</code>; wait …\nIterate over each item concurrently\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a value from a concurrent iterator.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nConvert <code>self</code> into a concurrent iterator.\nObtain a simple pass-through adapter.\nConvert items from one type into another\nMake progress on the consumer while doing something else.\nSend an item down to the next step in the processing queue.\nHow many items could we potentially end up returning?\nCreates a stream that yields the first <code>n</code> elements, or …\nIterate over each item concurrently, short-circuit on …\nThe resulting error type.\nThe resulting error type.\nThe <code>Future</code> implementation returned by this method.\nWhich kind of future are we turning this into?\nWhich kind of future are we turning this into?\nWhich kind of future are we turning this into?\nAn extension trait for the <code>Future</code> trait.\nA growable group of futures which act as a single unit.\nWait for all futures to complete.\nThe resulting output type.\nThe resulting output type.\nThe resulting output type.\nThe resulting output type.\nWait for the first future to complete.\nWait for the first successful future to complete.\nWait for all futures to complete successfully, or abort …\nSuspends a future until the specified deadline.\nReturns the argument unchanged.\nReturns the argument unchanged.\nA growable group of futures which act as a single unit.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nWait for both futures to complete.\nWaits for multiple futures to complete.\nWait for the first future to complete.\nWait for the first future to complete.\nWaits for the first successful future to complete.\nWaits for multiple futures to complete, either returning …\nDelay resolving the future until the given deadline.\nDelay resolving the future until the given deadline.\nA growable group of futures which act as a single unit.\nA key used to index into the <code>FutureGroup</code> type.\nIterate over items in the futures group with their …\nReturn the capacity of the <code>FutureGroup</code>.\nReturns <code>true</code> if the <code>FutureGroup</code> contains a value for the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nInsert a new future into the group.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if there are no futures currently active in …\nCreate a stream which also yields the key of each item.\nReturn the number of futures currently active in the group.\nCreate a new instance of <code>FutureGroup</code>.\nRemoves a stream from the group. Returns whether the value …\nReserves capacity for <code>additional</code> more futures to be …\nCreate a new instance of <code>FutureGroup</code> with a given capacity.\nTakes multiple streams and creates a new stream over all …\nConversion into a <code>Stream</code>.\nWhich kind of stream are we turning this into?\nWhat’s the return type of our stream?\nThe type of the elements being iterated over.\nThe resulting output type.\nWhat’s the return type of our stream?\nCombines multiple streams into a single stream of all …\nWhat stream do we return?\nThe stream type.\nWhat stream do we return?\nAn extension trait for the <code>Stream</code> trait.\nA growable group of streams which act as a single unit.\nDelay execution of a stream once for the specified …\n‘Zips up’ multiple streams into a single stream of …\nCombine multiple streams into a single stream.\nTakes two streams and creates a new stream over all in …\nConvert into a concurrent stream.\nConvert into a concurrent stream.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCreates a stream from a value.\nCombine multiple streams into a single stream.\nCombines two streams into a single stream of all their …\nA growable group of streams which act as a single unit.\nDelay the yielding of items from the stream until the …\nDelay the yielding of items from the stream until the …\n‘Zips up’ multiple streams into a single stream of …\nCombine multiple streams into a single stream.\nA key used to index into the <code>StreamGroup</code> type.\nIterate over items in the stream group with their …\nA growable group of streams which act as a single unit.\nReturn the capacity of the <code>StreamGroup</code>.\nReturns <code>true</code> if the <code>StreamGroup</code> contains a value for the …\nReturns the argument unchanged.\nReturns the argument unchanged.\nInsert a new future into the group.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns true if there are no futures currently active in …\nCreate a stream which also yields the key of each item.\nReturn the number of futures currently active in the group.\nCreate a new instance of <code>StreamGroup</code>.\nRemoves a stream from the group. Returns whether the value …\nReserves capacity for <code>additional</code> more streams to be …\nCreate a new instance of <code>StreamGroup</code> with a given capacity.\nA collection of errors.\nA stream that chains multiple streams one after another.\nConcurrent async iterator that moves out of a vector.\nA future which waits for multiple futures to complete.\nA stream that merges multiple streams into a single stream.\nA future which waits for the first future to complete.\nA future which waits for the first successful future to …\nA future which waits for all futures to complete …\nA stream that ‘zips up’ multiple streams into a single …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")