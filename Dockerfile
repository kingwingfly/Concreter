FROM python:3.12

WORKDIR /usr/src/app

COPY ./src_py ./src_py
COPY ./proto ./proto

RUN pip install --upgrade pip
RUN pip install -r ./src_py/requirements.txt
RUN python -m grpc_tools.protoc -I./proto --python_out=./src_py --pyi_out=./src_py --grpc_python_out=./src_py proto/sym.proto

CMD [ "python", "src_py/main.py" ]
