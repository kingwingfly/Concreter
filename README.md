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
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>

<h3 align="center">Concreter</h3>

  <p align="center">
    An app for students' learning concrete construction built with knowledge graph and symbolic computation
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
![graph aliases](image/graph_aliases.png)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![Next][Next.js]][Next-url]
* [![React][React.js]][React-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

Here's how you can init and start the project.

### Prerequisites
- Postgres data base

#### Postgres
If you wanana use Postgres on your local machine, you can follow the steps below.

```sh
# Init a Postgres database with the following settings:
initdb -D /path/to/pgdata --locale=en_US.UTF-8 -U $USER -W

# Start the database:
pg_ctl -D /path/to/pgdata -l /path/to/log.log start

# Create database:
createdb concreter -O $USER

# Set the PgUrl environment variable:
export PgUrl=postgres://$USER:password@localhost:5432/concreter

# Follows is the commands to end the service:
pg_ctl -D /path/to/pgdata stop
rm -rf /path/to/pgdata
```

If you wanana use Postgres on docker, you can follow the steps below.
```sh
docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres

export PgUrl=postgres://postgres:password@localhost:5432/postgres

# To end the service:
docker stop postgres
docker rm postgres
```

### Installation

1. Get a free API Key at [https://example.com](https://example.com)
2. Clone the repo
   ```sh
   git clone https://github.com/kingwingfly/Concreter.git
   ```
3. Install NPM packages
   ```sh
   npm install
   ```
4. Enter your API in `config.js`
   ```js
   const API_KEY = 'ENTER YOUR API';
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

Use this space to show useful examples of how a project can be used. Additional screenshots, code examples and demos work well in this space. You may also link to more resources.

_For more examples, please refer to the [Documentation](https://example.com)_

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ROADMAP -->
## Roadmap

- [ ] Feature 1
- [ ] Feature 2
- [ ] Feature 3
    - [ ] Nested Feature

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



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* []()
* []()
* []()

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
[Next.js]: https://img.shields.io/badge/next.js-000000?style=for-the-badge&logo=nextdotjs&logoColor=white
[Next-url]: https://nextjs.org/
[React.js]: https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB
[React-url]: https://reactjs.org/
