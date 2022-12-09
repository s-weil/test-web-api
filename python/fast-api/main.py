## pip install fastapi
## pip install "uvicorn[standard]"


# run it: uvicorn main:app --reload

from typing import Union

from fastapi import FastAPI

app = FastAPI()

@app.get("/health")
async def read_root():
    return {}

@app.get("/")
async def read_root():
    return {"Hello": "World"}


@app.get("/items/{item_id}")
async def read_item(item_id: int, q: Union[str, None] = None):
    return {"item_id": item_id, "q": q}
