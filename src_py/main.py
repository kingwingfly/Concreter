from pb import sym_pb2_grpc
from pb.sym_pb2_grpc import SymServicer
from pb.sym_pb2 import HelloReply
import grpc

from concurrent import futures


class SymServer(SymServicer):
    def SayHello(self, request, context):
        return HelloReply(message=f"Hello, {request.name}!")


if __name__ == "__main__":
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    sym_pb2_grpc.add_SymServicer_to_server(SymServicer(), server)
    server.add_insecure_port("[::]:50051")
    server.start()
    server.wait_for_termination()
