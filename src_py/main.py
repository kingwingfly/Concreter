import grpc
from concurrent import futures
import sym_pb2_grpc
import nlp_pb2_grpc
from sym_utils import SymServer
from nlp_utils import NlpServer

if __name__ == "__main__":
    from dotenv import load_dotenv
    load_dotenv()
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    sym_pb2_grpc.add_SymServicer_to_server(SymServer(), server)
    nlp_pb2_grpc.add_NlpServicer_to_server(NlpServer(), server)
    server.add_insecure_port("[::]:50051")
    print("listening on port [::]:50051")
    server.start()
    server.wait_for_termination()
