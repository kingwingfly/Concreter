import os
from dotenv import load_dotenv
from typing import Optional
from openai import OpenAI
from httpx import Client

if __name__ == "__main__":
    load_dotenv()
    print("Start test ...")
    client = OpenAI(http_client=Client(proxies="http://127.0.0.1:7890"), timeout=30, max_retries=0)
    completion = client.chat.completions.create(
      model="gpt-3.5-turbo-1106",
      messages=[
          {"role": "system", "content": "You are a helpful assistant designed to convert formulas in markdown to code that can run in python's sympy library."},
          {"role": "user", "content": "y = x ^ 2"}
      ]
    )
    print(completion.choices[0].message)
