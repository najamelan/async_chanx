# Single writer mpsc

ringbuffer

reader keeps track of the last item it read in AtomicUsize

each writer keeps track of the last item it wrote in an AtomicUsize

reader loops over writers to check if there are next items to read and can do so in batch mode.

each writer checks where the reader is to avoid overwriting tail

How do senders coordinate reserving the slots?
