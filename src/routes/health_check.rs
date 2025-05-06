use daoyi_cloud_common::{endpoint, salvo, CommonResult};

/// 健康检查接口
/// PING - > PONG
#[endpoint(tags("健康检查"))]
pub async fn health_check() -> CommonResult<String> {
    // 这里可以添加一些健康检查的逻辑，比如数据库连接、外部服务可用性等
    // 如果一切正常，返回200 OK
    CommonResult::ok(Some("PONG".to_string()))
}
