//! 向量存储 - Qdrant 集成

use anyhow::Result;
use qdrant_client::qdrant::{PointStruct, QueryPoints, SearchPoints, Value};
use qdrant_client::Qdrant;
use tracing::info;

/// 向量存储 - Qdrant 客户端
pub struct VectorStore {
    client: Qdrant,
    collection: String,
}

impl VectorStore {
    /// 创建新的向量存储
    pub async fn new(url: &str, api_key: Option<&str>, collection: &str) -> Result<Self> {
        let client = if let Some(key) = api_key {
            Qdrant::from_url(url).with_api_key(key).build()?
        } else {
            Qdrant::from_url(url).build()?
        };
        
        // 检查集合是否存在
        if !client.collection_exists(collection).await? {
            // 创建集合
            client
                .create_collection(
                    collection,
                    qdrant_client::qdrant::vectors_config::Config::Params(
                        qdrant_client::qdrant::VectorParams {
                            size: 768, // BGE 模型维度
                            distance: qdrant_client::qdrant::Distance::Cosine.into(),
                            ..Default::default()
                        },
                    ),
                )
                .await?;
            info!("Qdrant 集合 '{}' 创建成功", collection);
        }
        
        Ok(Self {
            client,
            collection: collection.to_string(),
        })
    }
    
    /// 添加向量点
    pub async fn upsert(&self, points: Vec<PointStruct>) -> Result<()> {
        self.client
            .upsert_points(&self.collection, None, points, None)
            .await?;
        
        Ok(())
    }
    
    /// 搜索相似向量
    pub async fn search(&self, vector: Vec<f32>, limit: usize) -> Result<Vec<PointStruct>> {
        let result = self
            .client
            .search_points(SearchPoints {
                collection_name: self.collection.clone(),
                vector,
                limit: limit as u64,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await?;
        
        // 转换为 PointStruct
        let points = result
            .result
            .into_iter()
            .map(|scored_point| {
                PointStruct {
                    id: scored_point.id,
                    vectors: None,
                    payload: scored_point.payload,
                }
            })
            .collect();
        
        Ok(points)
    }
    
    /// 删除点
    pub async fn delete(&self, ids: Vec<&str>) -> Result<()> {
        self.client
            .delete_points(
                &self.collection,
                None,
                qdrant_client::qdrant::PointsSelector::PointsOptions(
                    qdrant_client::qdrant::points_selector::PointsOptions::Points(
                        qdrant_client::qdrant::PointsIdsList {
                            ids: ids.into_iter().map(|id| id.into()).collect(),
                        },
                    ),
                ),
                None,
            )
            .await?;
        
        Ok(())
    }
}
