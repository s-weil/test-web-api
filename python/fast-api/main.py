## pip install fastapi
## pip install "uvicorn[standard]"

# run it: uvicorn main:app --reload

from typing import Union
from fastapi import FastAPI
import time

app = FastAPI()

@app.get("/health")
async def health():
    return {}

@app.get("/delayed/{sleep_ms}")
async def delayed(sleep_ms: int):
    # TODO: randomize
    time.sleep(sleep_ms)
    return {}

@app.get("/items/{item_id}")
async def read_item(item_id: int, q: Union[str, None] = None):
    return {"item_id": item_id, "q": q}
