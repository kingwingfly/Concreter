FROM python:3.10

WORKDIR /usr/src/app

COPY ./src_py/requirements.txt ./requirements.txt

VOLUME ["./src_py", "./proto" ]

RUN alias cls=clear

RUN pip install --upgrade pip

RUN pip install -r ./requirements.txt

RUN echo 'export PB="./src_py" && \
python -m grpc_tools.protoc -I./proto --python_out=$PB \
--pyi_out=$PB --grpc_python_out=$PB proto/sym.proto proto/nlp.proto && \
python ./src_py/main.py' > ./start.sh

RUN chmod +x ./start.sh

CMD ["/bin/bash", "-c", "./start.sh"]
