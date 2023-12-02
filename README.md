[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/kingwingfly/Concreter">
    <img src="images/screenshot.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Concreter</h3>

  <p align="center">
    A web app to enhance document with chatGPT and sympy.
    <br />
    <a href="https://github.com/kingwingfly/Concreter"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/kingwingfly/Concreter">View Demo</a>
    ·
    <a href="https://github.com/kingwingfly/Concreter/issues">Report Bug</a>
    ·
    <a href="https://github.com/kingwingfly/Concreter/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

[![Product Name Screen Shot][product-screenshot]](https://github.com/kingwingfly/Concreter)

### The Graph Database
![graph aliases](images/graph_aliases.png)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Rust][Rust]][Rust-url]
* [![Python][Python]][Python-url]
* [![typescript][typescript]][typescript-url]
* [![gRPC][gRPC]][gRPC-url]
* [![docker][docker]][docker-url]
* [![kubernetes][kubernetes]][kubernetes-url]
* [![postgresql][postgresql]][postgresql-url]
* [![agdb][agdb]][agdb-url]
* [![openai][openai]][openai-url]
* [![Next][Next.js]][Next-url]
* [![React][React.js]][React-url]
* [![reveal][reveal]][reveal-url]
* [![three][three]][three-url]


<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started
Here's how you can init and start the project.

Three ways: local docker k8s


### Local
- Postgres database
- AgDb database
- Python gRPC server
- Axum server

#### Postgres
```sh
# Init a Postgres database with the following settings:
initdb -D /path/to/pgdata --locale=en_US.UTF-8 -U $USER -W

# Start the database:
pg_ctl -D /path/to/pgdata -l /path/to/log.log start

# Create database:
createdb app_db -O $USER

# Set the PgUrl environment variable:
export PG_URL="postgres://$USER:password@localhost/app_db"

# Follows is the commands to end and clean the service:
pg_ctl -D /path/to/pgdata stop
rm -rf /path/to/pgdata
```

#### AgDb
[`AgDb`](https://github.com/agnesoft/agdb) is a graph database.

You need give it a `.agdb` suffix file to init the database. For example:

```sh
mkdir -p ag_db && touch ag_db/ag.agdb
export AG_FILE="path/to/ag.agdb"
```

#### python gRPC
To enable symbolic computation, I use python's [`sympy`](https://docs.sympy.org/latest/index.html). For time limitation, I don't have enough time to explore `pyO3`, so I just use `gRPC` to interact with python in Rust. Sympy only support python <= 3.10

```sh
# create a virtual environment
conda create -n py310 python=3.10

# activate the virtual environment
conda activate py310

# install the dependencies
pip install -r requirements.txt
# For ARM Mac, use
pip install socksio
conda install --file requirements.txt
pip install --upgrade openai

# generate the gRPC python code
export PB="./src_py" && python -m grpc_tools.protoc -I./proto --python_out=$PB --pyi_out=$PB --grpc_python_out=$PB proto/sym.proto

# set env varibles
export OPENAI_API_KEY=sk-123456

# Run gRPC service
python ./src_py/main.py

# Ctrl + C to end the service, and deactivate the virtual environment
conda deactivate
```
For both methods, you may need proxy configured. For docker, the host network should set proxy. For local machine, set the proxy in `src_py/openai_utils.py`:
```python
client = OpenAI(
    # In docker, do not need to set proxy, for it uses host network which does.
    http_client=Client(proxies="http://127.0.0.1:7890"), timeout=30, max_retries=0
)
```
#### fronted
```sh
cd frontend && npm i && cd ..
export WEB_FOLDER=path/to/fronted/out
export FRONTEND_FOLDER=path/to/frontend
```

#### start axum server
```sh
export RPC_ADDR=http://localhost:50051
export SERVICE_PWD_KEY=   # you can gernerate using
export SERVICE_TOKEN_KEY= # `cargo run --examples gen_key`
export SERVICE_TOKEN_DURATION_SEC=1800  # in seconds
cargo run
```

### Docker
#### postgres
```sh
docker pull postgres
docker run --name postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres
export PG_URL=postgres://postgres:postgres@localhost:5432/postgres
```
#### Agdb
```sh
mkdir -p ag_db && touch ag_db/ag.agdb
export AG_FILE="path/to/ag.agdb"
```

#### gRPC
```sh
# At the root of the project, run:
docker build -t rpc-py --target=rpc-py .
docker run -it -p 50051:50051 -e OPENAI_API_KEY=sk-123456 --rm --name rpc-py rpc-py
```

#### Frontend
```sh
cd frontend && npm i && cd ..
export WEB_FOLDER=path/to/fronted/out
export FRONTEND_FOLDER=path/to/frontend
```

#### start axum server
```sh
export RPC_ADDR=http://localhost:50051
export SERVICE_PWD_KEY=   # you can gernerate using
export SERVICE_TOKEN_KEY= # `cargo run --examples gen_key`
export SERVICE_TOKEN_DURATION_SEC=1800  # in seconds
cargo run
```

### Kubernetes
#### build images
```sh
docker build -t rpc-py --target rpc-py .
docker build -t axum --target axum .
docker pull postgres
```
Set the env variables in deploy/k8s:
- secrets in kustomization.yaml
- persist voloum in postgres-deployment.yaml and axum-deployment.yaml

And load the images rpc-py, axum, postgres to your cluster
```sh
cd deploy/k8s && kubectl apply -k ./
# init the postgres, see blow
# port-forward
kubectl port-forward svc/axum-xxxx-xxx 8080:8080
```

### Initialize the postgres table
Connect to db
```sh
psql postgres://postgres:postgres@localhost:5432/postgres
# For those using k8s
kubectl port-forward svc/postgres 5432:5432
```
Run sql in
[init sql](sql/dev_init/01-schema.sql)


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage
Register and login in.

Upload a markdown and enter the field the markdown about, you can see examples at examples folder.
<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- ROADMAP -->
## Roadmap

- [ ] Use [SeaORM](https://github.com/SeaQL/sea-orm) and openDAL
- [ ] Fix: service dropped in minikube but works properly on docker kube
- [ ] Use vector storage
- [ ] Train transformer latex-sympy translator
- [ ] Formular sub
- [ ] 3D model support

See the [open issues](https://github.com/kingwingfly/Concreter/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the Apache 2.0 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Louis - 20200581@cqu.edu.cn

Project Link: [https://github.com/kingwingfly/Concreter](https://github.com/kingwingfly/Concreter)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/kingwingfly/Concreter.svg?style=for-the-badge
[contributors-url]: https://github.com/kingwingfly/Concreter/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/kingwingfly/Concreter.svg?style=for-the-badge
[forks-url]: https://github.com/kingwingfly/Concreter/network/members
[stars-shield]: https://img.shields.io/github/stars/kingwingfly/Concreter.svg?style=for-the-badge
[stars-url]: https://github.com/kingwingfly/Concreter/stargazers
[issues-shield]: https://img.shields.io/github/issues/kingwingfly/Concreter.svg?style=for-the-badge
[issues-url]: https://github.com/kingwingfly/Concreter/issues
[license-shield]: https://img.shields.io/github/license/kingwingfly/Concreter.svg?style=for-the-badge
[license-url]: https://github.com/kingwingfly/Concreter/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/linkedin_username
[product-screenshot]: images/screenshot.png

[Rust]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=orange
[Rust-url]: https://www.rust-lang.org
[Python]: https://img.shields.io/badge/Python-000000?style=for-the-badge&logo=Python&logoColor=blue
[Python-url]: https://www.python.org
[axum]: https://img.shields.io/badge/axum-000000?style=for-the-badge&logo=axum&logoColor=white
[axum-url]: https://github.com/tokio-rs/axum
[typescript]: https://img.shields.io/badge/typescript-000000?style=for-the-badge&logo=typescript&logoColor=blue
[typescript-url]: https://www.typescriptlang.org
[gRPC]: https://img.shields.io/badge/gRPC-000000?style=for-the-badge&logo=gRPC&logoColor=yello
[gRPC-url]: https://grpc.io
[docker]: https://img.shields.io/badge/docker-000000?style=for-the-badge&logo=docker&logoColor=blue
[docker-url]: https://www.docker.com
[kubernetes]: https://img.shields.io/badge/kubernetes-000000?style=for-the-badge&logo=kubernetes&logoColor=blue
[kubernetes-url]: https://kubernetes.io
[postgresql]: https://img.shields.io/badge/postgresql-000000?style=for-the-badge&logo=postgresql&logoColor=blue
[postgresql-url]: https://www.postgresql.org
[agdb]: https://img.shields.io/badge/agdb-000000?style=for-the-badge&logo=agdb&logoColor=brown
[agdb-url]: https://github.com/agnesoft/agdb
[openai]: https://img.shields.io/badge/openai-000000?style=for-the-badge&logo=openai&logoColor=pink
[openai-url]: https://openai.com
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=white
[React-url]: https://reactjs.org/
[reveal]: https://img.shields.io/badge/reveal-000000?style=for-the-badge&logo=revealdotjs&logoColor=yellow
[reveal-url]: https://revealjs.com
[three]: https://img.shields.io/badge/three-000000?style=for-the-badge&logo=threedotjs&logoColor=yellow
[three-url]: https://threejs.org/docs/#manual/en/introduction/Installation
