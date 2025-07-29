# Asynchronous Programming with asyncio

import asyncio

# Basic async function
print("=== Basic Async Function ===")
# TODO: Practice creating async functions
# async def hello():
#     print("Hello")
#     await asyncio.sleep(1)
#     print("World")

# Running async functions
print("\n=== Running Async Functions ===")
# TODO: Practice running async functions
# asyncio.run(hello())

# Multiple coroutines
print("\n=== Multiple Coroutines ===")
# TODO: Practice running multiple coroutines concurrently
# async def task(name, delay):
#     await asyncio.sleep(delay)
#     print(f"Task {name} completed")
# 
# async def main():
#     await asyncio.gather(
#         task("A", 1),
#         task("B", 2),
#         task("C", 1.5)
#     )

# Async with aiohttp (web requests)
print("\n=== Async HTTP Requests ===")
# TODO: Practice async HTTP requests
# import aiohttp
# async def fetch(session, url):
#     async with session.get(url) as response:
#         return await response.text()

# Async generators
print("\n=== Async Generators ===")
# TODO: Practice async generators
# async def async_range(start, stop):
#     for i in range(start, stop):
#         await asyncio.sleep(0.1)
#         yield i

# Async context managers
print("\n=== Async Context Managers ===")
# TODO: Practice async context managers
# class AsyncContextManager:
#     async def __aenter__(self):
#         print("Entering async context")
#         return self
#     
#     async def __aexit__(self, exc_type, exc_val, exc_tb):
#         print("Exiting async context")
#         return False

if __name__ == "__main__":
    print("Practice your async programming here!")

