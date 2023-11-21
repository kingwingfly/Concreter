import os
from dotenv import load_dotenv
from typing import Optional
from openai import OpenAI
from httpx import Client
from sympy import Symbol, Expr

load_dotenv()

client = OpenAI(
    http_client=Client(proxies="http://127.0.0.1:7890"), timeout=30, max_retries=0
)

def convert(symbols: str | list[str], formula: str) -> str:
    completion = client.chat.completions.create(
        model="gpt-3.5-turbo-1106",
        messages=[
            {
                "role": "system",
                "content": "You are a helpful assistant designed to convert formulas in markdown to a python function signatured `formula() -> sympy.Expr` with python's sympy library.",
            },
            {"role": "user", "content": f"Symbols: {symbols}; Only convert ${formula}$ to a python function signatured `formula() -> sympy.Expr` with python's sympy library."},
        ],
    )
    content = completion.choices[0].message.content
    return extract(content if content else "")

def extract(content: str) -> str:
    flag = False
    ret = []
    for line in content.split('\n'):
        if line.startswith('```'):
            flag = not flag
            continue
        if flag:
            ret.append(line)
    return '\n'.join(ret)

def run_code(code: str) -> Expr:
    buffer = {}
    exec(code, buffer)
    expr = buffer["formula"]()
    return expr

if __name__ == "__main__":
    print("Start test ...")
    code = convert(["x", "y"], r"z = x ^ 2 + 2 \TIMES x \TIMES y + y ^ 2")
    print(code)
    expr = run_code(code).factor()
    print(expr)
    ret = expr.subs(Symbol("x"), 2).subs(Symbol("y"), 2)
    print(ret)
