# Threading and Multiprocessing

import threading
import multiprocessing
import time
from concurrent.futures import ThreadPoolExecutor, ProcessPoolExecutor

# Basic threading
print("=== Basic Threading ===")
# TODO: Practice creating and running threads
# def worker(name):
#     for i in range(5):
#         print(f"Worker {name}: {i}")
#         time.sleep(1)
# 
# thread = threading.Thread(target=worker, args=("A",))
# thread.start()
# thread.join()

# Thread synchronization
print("\n=== Thread Synchronization ===")
# TODO: Practice using locks, semaphores, and conditions
# lock = threading.Lock()
# def safe_worker(name, lock):
#     with lock:
#         print(f"Worker {name} acquired lock")
#         time.sleep(1)
#         print(f"Worker {name} releasing lock")

# ThreadPoolExecutor
print("\n=== ThreadPoolExecutor ===")
# TODO: Practice using thread pools
# def task(n):
#     return n * n
# 
# with ThreadPoolExecutor(max_workers=4) as executor:
#     futures = [executor.submit(task, i) for i in range(10)]
#     results = [future.result() for future in futures]

# Basic multiprocessing
print("\n=== Basic Multiprocessing ===")
# TODO: Practice creating processes
# def cpu_intensive_task(n):
#     total = 0
#     for i in range(n):
#         total += i * i
#     return total
# 
# if __name__ == "__main__":
#     process = multiprocessing.Process(target=cpu_intensive_task, args=(1000000,))
#     process.start()
#     process.join()

# ProcessPoolExecutor
print("\n=== ProcessPoolExecutor ===")
# TODO: Practice using process pools
# with ProcessPoolExecutor(max_workers=4) as executor:
#     futures = [executor.submit(cpu_intensive_task, 100000) for _ in range(4)]
#     results = [future.result() for future in futures]

# Shared memory
print("\n=== Shared Memory ===")
# TODO: Practice sharing data between processes
# from multiprocessing import shared_memory, Array, Value

if __name__ == "__main__":
    print("Practice your threading and multiprocessing here!")

