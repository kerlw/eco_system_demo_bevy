use serde::Deserialize;

/// 关卡配置数据结构
#[derive(Debug, Deserialize)]
pub struct LevelConfig {
    /// 关卡ID
    pub id: u32,
    /// 关卡名称
    pub name: String,
    /// 关卡描述
    pub description: String,
    /// 初始实体配置
    pub entities: Vec<EntityConfig>,
    /// 胜利条件
    pub win_conditions: WinConditions,
}

/// 实体配置
#[derive(Debug, Deserialize)]
pub struct EntityConfig {
    /// 实体类型
    pub entity_type: String,
    /// 初始位置(x,y)
    pub position: (i32, i32),
    /// 初始属性
    pub properties: serde_json::Value,
}

/// 胜利条件
#[derive(Debug, Deserialize)]
pub struct WinConditions {
    /// 需要维持的生态平衡时间(秒)
    pub balance_duration: f32,
    /// 最小实体数量
    pub min_entities: u32,
    /// 最大实体数量
    pub max_entities: u32,
}