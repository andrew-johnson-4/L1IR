from timeit import default_timer as timer

def pow2(x):
    if x == 0:
        return 1
    return pow2(x-1) + pow2(x-1)

start = timer()
for x in range(1000):
    pow2(20)
end = timer()

print("(Python) 1M 2^20 in {:.7f} seconds".format(end - start))
