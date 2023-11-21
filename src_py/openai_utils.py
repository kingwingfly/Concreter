from time import sleep
from dotenv import load_dotenv
from typing import Optional
from openai import OpenAI
from httpx import Client
from sympy import Symbol, Expr, symbols

load_dotenv()

SYSTEM = "You are a helpful assistant designed to convert formulas in markdown or latex \
to a python function signatured `formula() -> sympy.Expr` with python's sympy library. \
Remember, no args in the function signature. Remember, use `symbols` to create symbols in function \
first if needed."

client = OpenAI(
    # In docker, do not need to set proxy, for it uses host network which does.
    # http_client=Client(proxies="http://127.0.0.1:7890"), timeout=30, max_retries=0
)


def convert(symbols: str | list[str], formula: str) -> str:
    q = f"Asking GPT: \n Only these are symbols: {symbols}, besides some common coefficients; \
You should only convert the right side of ${formula}$ to a python function \
signatured `formula() -> sympy.Expr` with python's sympy library."
    print(q)
    while True:
        try:
            completion = client.chat.completions.create(
                model="gpt-3.5-turbo-1106",
                messages=[
                    {
                        "role": "system",
                        "content": SYSTEM,
                    },
                    {
                        "role": "user",
                        "content": q,
                    },
                ],
            )
        except Exception as e:
            print(e)
            sleep(20)
            continue
        break
    content = completion.choices[0].message.content
    print(f"GPT answer:\n {content}\n")
    return extract(content if content else "")


def extract(content: str) -> str:
    flag = False
    ret = []
    for line in content.split("\n"):
        if line.startswith("```"):
            flag = not flag
            continue
        if flag:
            ret.append(line)
    return "\n".join(ret)


def run_code(code: str) -> Expr:
    buffer = {}
    print(f"exec: \n{code}\n")
    exec(code, buffer)
    expr = buffer["formula"]().factor()
    return expr


if __name__ == "__main__":
    print("Start test ...")
    code = convert(["h", "x"], r"f = h (h - 0.5 x)")
    print(code)
    expr = run_code(code)
    print(expr)
    k, h, x = symbols("k h x")
    expr = expr.subs(x, k * h)
    expr = expr.factor() if isinstance(expr, Expr) else expr
    print(expr)
