# pip install fastapi
# pip install "uvicorn[standard]"
# pip install pydantic

# run it: uvicorn main:app --reload

from typing import Union
from fastapi import FastAPI
import time
from pydantic import BaseModel

app = FastAPI()


class Item(BaseModel):
    name: str
    description: str | None = None
    price: float
    tax: float | None = None


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


@app.post("/items/")
async def create_item(item: Item):
    return item
