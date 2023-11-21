import sym_pb2_grpc
from sym_pb2_grpc import SymServicer
from sym_pb2 import (
    ConvertMdRequest,
    ConvertMdReply,
    HelloReply,
    HelloRequest,
    ValueType,
)
import grpc
from sympy import Symbol, Expr
from openai_utils import convert, run_code

from concurrent import futures


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


if __name__ == "__main__":
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    sym_pb2_grpc.add_SymServicer_to_server(SymServer(), server)
    server.add_insecure_port("[::]:50051")
    print("listening on port [::]:50051")
    server.start()
    server.wait_for_termination()
