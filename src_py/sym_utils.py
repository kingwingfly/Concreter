from time import sleep
from typing import Optional
from openai import OpenAI
from httpx import Client
from sympy import Symbol, Expr, symbols
import sym_pb2_grpc
from sym_pb2_grpc import SymServicer
from sym_pb2 import (
    ConvertMdRequest,
    ConvertMdReply,
    HelloReply,
    HelloRequest,
    ValueType,
)


class SymServer(SymServicer):
    def SayHello(self, request: HelloRequest, context):
        return HelloReply(message=f"Hello {request.name}!")

    def ConvertMdFormula(self, request: ConvertMdRequest, context):
        code = convert(request.symbols, request.md)
        formula = run_code(code)
        if request.symbol == "":  # Do not do substitution
            return ConvertMdReply(formula=str(formula))
        symbol = Symbol(request.symbol)
        match request.type:
            case ValueType.NUMBER:
                value = float(request.value)
            case ValueType.EXPR:
                code = convert(request.symbols, request.value)
                value = run_code(code)
            case _:
                raise NotImplementedError("unreachable")
        formula = formula.subs(symbol, value)
        formula = formula.factor() if isinstance(formula, Expr) else formula
        return ConvertMdReply(formula=str(formula))


SYSTEM = "You are a helpful assistant designed to convert formulas in markdown or latex \
to a python function signatured `formula() -> sympy.Expr` with python's sympy library. \
Remember, no args in the function signature. Remember, use `symbols` to create symbols in function \
first if needed."

from dotenv import load_dotenv

load_dotenv()
client = OpenAI(
    # In docker, do not need to set proxy, for it uses host network which does.
    # http_client=Client(proxies="http://127.0.0.1:7890"), timeout=30, max_retries=0
)


def convert(symbols: str | list[str], formula: str) -> str:
    q = f"Only these are symbols: {symbols}, besides some common coefficients; \
You should only convert the right side of ${formula}$ to a python function \
signatured `formula() -> sympy.Expr` with python's sympy library."
    print(f"Asking GPT: \n{q}")
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
    code = convert(["h", "x"], "f = h (h - 0.5 x)")
    print(code)
    expr = run_code(code)
    print(expr)
    k, h, x = symbols("k h x")
    expr = expr.subs(x, k * h)
    expr = expr.factor() if isinstance(expr, Expr) else expr
    print(expr)
