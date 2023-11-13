# Technology Selection

## 分析

作者是三大力学95分选手，但是败在了混凝土结构上， 这说明混凝土结构对于某类学生是一门困难的课程。

### 原因

一方面，部分学生听到“钢筋”二字就犯困；

另一方面，部分学生在此课程之前，并没有G-101图集的基础，往往并不清楚钢筋混凝土中有何种钢筋，而执教老师假设了学生具备这方面的知识；

最后，此学科存在一定数量的半经验公式，但公式、知识点在书中相对分散，同时，教材对于材料力学相关公式的应用并没有指明，存在理解困难。

**因此，围绕`图集、知识点、公式推导`三个方面构建知识图谱，并用算法、符号计算等辅助对其中较难的公式的理解，在一定程度上能够方便学生对混凝土结构的掌握**

### 学科特征

- G101图集：工整清晰 方便应用
- 数据量大：混凝土结构课程中存在大量图表，利用机器存储、查询是可实现的
- 机械性大：公式推导、公式运用的流程都是固定且机械的

### 目标

- 前沿技术的应用：知识图谱、软件开发
- 创新性：符号计算
- 易用性：教师添加知识容易；学生查询知识容易；具备开箱即用的功能：如自动配筋、钢筋选型

### 难点

- 跨语言调用
- 知识提取与知识图谱构建
- 用户易用的dashboard
- SeverLess 部署

## 架构

As for now, I suggest using `Rust`, `python` and `typescript` as the language we use.

### As to **frontend**
- [`nextjs`](https://nextjs.org) and `react` are both important.
- As for visible 3D concrete structure display, we need `revit` or `blender` to build a 3D model, and [`ifcjs`](https://github.com/IFCjs/web-ifc) to display it on the web.
-  As to knowledge display, [revealjs](https://revealjs.com/markdown/) should be use, so that the users could show their markdown through it easily.

### As to **backend**,
- To build a knowledge graph, neo4j in Java is a not a hyper choice, we are glad to try [agdb](https://github.com/agnesoft/agdb) as our graph database.
- For symbolic computation, python's [`sympy`](https://docs.sympy.org/latest/index.html) is the first choice while the other's far away useful enough. In rust neither [symbolic_polynomials](https://crates.io/crates/symbolic_polynomials) nor [savage](https://crates.io/crates/savage_core) is far from easily useful.
- Run python on server and send back the result. Here are two ways: use socket or severless to deploy text knowledge and formula separately on server side; Or just use [pyO3](https://crates.io/crates/pyo3) to build our server.

### Deploy
We could not use Vercel maybe, for our backend is almost in Rust. There are two choice, just using an `Aliyun` server or try serverless(`FaaS`) deployment. They are challengeable. 考虑到阿里云可笑的文档，可能应该选择`AWS`作为部署平台。

## Implementation
The most difficult part is formula computation or symbolic math as we know. Maybe users can do something like `zed`'s GPT assistant implementation: user select the formulas and select an operation, send it to backend and got the result.

This is quite difficult for our not knowing how to translate markdown's formula like `y = x^2` to sympy's expr `y = x ** 2`. The best solution I can post is to use ChatGPT's api for now.
