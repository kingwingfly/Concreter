FROM python:3.12

WORKDIR /usr/src/app

COPY ./requirements.txt ./requirements.txt
COPY ./proto ./proto

VOLUME ["./src_py" ]

RUN pip install --upgrade pip
RUN pip install -r ./requirements.txt
RUN echo "python -m grpc_tools.protoc -I./proto --python_out=./src_py \
--pyi_out=./src_py --grpc_python_out=./src_py proto/sym.proto && \
python ./src_py/main.py" > ./start.sh
RUN chmod +x ./start.sh

CMD ["/bin/bash", "-c", "./start.sh"]
