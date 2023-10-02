# NOTE MUST BE CALLED FROM ROOT

from ctypes import *

test_count = 100
fib_param = 47

# test method
def fibIter(n):
    if n < 2:
        return n
    fibPrev = 1
    fib = 1
    for _ in range(2, n):
        fibPrev, fib = fib, fib + fibPrev
    return fib

# start lib
dll = cdll.LoadLibrary("target\\debug\\rapl_lib.dll")

for i in range(test_count):
    # start recording
    dll.start_rapl()

    # run test
    fibIter(fib_param)

    # stop recording
    dll.stop_rapl()

print("job done")
