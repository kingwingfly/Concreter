tonic::include_proto!("nlp");

use nlp_client::NlpClient;
use tokio::sync::OnceCell;
use tonic::transport::Channel;

use crate::config::config;

use super::error::RpcResult;

pub async fn nlp_client() -> NlpClient<Channel> {
    static NLP_CLIENT: OnceCell<NlpClient<Channel>> = OnceCell::const_new();
    NLP_CLIENT
        .get_or_init(|| async {
            let addr = config().RPC_ADDR.to_owned();
            let channel = Channel::from_shared(addr).unwrap().connect().await.unwrap();
            NlpClient::new(channel)
        })
        .await
        .clone()
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn openai_ner_test() {
        let mut client = nlp_client().await;
        let req = NerRequest {
            text: r#"域名解析
cqu.edu.cn 其中 cqu 是主机名；edu 为机构性域名；cn 是地理域名
大数据
指无法在一定时间范围内用常规软件工具进行捕捉、管理和处理的数据集合，需要新的处理模式才能具有更强的决策力、洞察力和流程优化能力来适应海量、高增长率和多样性的信息资产
特征
海量化：体量大
多样化：类型多样（视频、图片、文字等），无明显模式，不连贯的语义或句义
快速化：实时分析而非批量分析，处理快，增长快
价值化：低价值密度，高商业价值
思维方式
全样而非抽样
效率而非精确
相关而非因果
大数据技术起源
Google 的 GFS (Google file system)、MapReduce (并行计算编程)、BigTable (Google 的分布式数据储存系统) 但是不开源（用的cpp） 开源 Hadoop 系统（用的 java ，如今有大量其他语言的实现）： HDFS Hadoop MapReduce HBase
云计算
IaaS 基础设施即服务
PaaS 平台即服务
SaaS 软件即服务
FaaS 函数即服务
aa 即 as a"#.to_string(),
            region: "物联网".to_string(),
        };
        let resp = client.ner(req).await.unwrap().into_inner();
        println!("{}", resp.ner_ret)
    }
}
