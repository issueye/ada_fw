//! 鉴权模块。
//!
//! 基于 **JWT + HttpOnly Cookie** 实现无状态会话：
//! - 登录成功后签发 JWT，写入 `HttpOnly` Cookie
//! - 受保护路由通过 [`middleware::require_auth`] 校验 Cookie 中的 Token
//!
//! 安全约定：
//! - Cookie 设置 `HttpOnly` + `SameSite=Lax`，生产环境追加 `Secure`
//! - 登录失败统一返回「邮箱或密码错误」，不区分具体原因，防用户枚举
//! - 密钥通过 `ADA_JWT_SECRET` 注入，缺失时使用开发默认值并告警

pub mod middleware;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

/// Cookie 名称。
pub const TOKEN_COOKIE: &str = "ada_token";

/// JWT 载荷。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户唯一标识。
    pub sub: String,
    /// 用户显示名。
    pub name: String,
    /// 用户角色。
    pub role: String,
    /// 过期时间（Unix 时间戳，秒）。
    pub exp: usize,
    /// 签发时间（Unix 时间戳，秒）。
    pub iat: usize,
}

/// 当前登录用户（从 [`Claims`] 提取，注入到请求扩展中）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    /// 用户唯一标识。
    pub id: String,
    /// 用户显示名。
    pub name: String,
    /// 用户角色。
    pub role: String,
}

impl From<Claims> for CurrentUser {
    fn from(c: Claims) -> Self {
        Self {
            id: c.sub,
            name: c.name,
            role: c.role,
        }
    }
}

/// 签发 JWT。
pub fn issue_token(
    secret: &str,
    user_id: &str,
    name: &str,
    role: &str,
    ttl_secs: u64,
) -> Result<String, AppError> {
    let now = now_secs();
    let claims = Claims {
        sub: user_id.to_string(),
        name: name.to_string(),
        role: role.to_string(),
        iat: now,
        exp: now + ttl_secs as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("签发 Token 失败: {e}")))
}

/// 校验 JWT 并返回载荷。
pub fn verify_token(secret: &str, token: &str) -> Result<Claims, AppError> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Unauthorized(format!("Token 无效: {e}")))?;

    Ok(data.claims)
}

/// 当前 Unix 时间戳（秒）。
fn now_secs() -> usize {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as usize)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &str = "test-secret-at-least-16-chars";

    #[test]
    fn 签发的_token_应可被正确校验() {
        let token = issue_token(SECRET, "u1", "张三", "admin", 3600).unwrap();
        let claims = verify_token(SECRET, &token).unwrap();
        assert_eq!(claims.sub, "u1");
        assert_eq!(claims.name, "张三");
        assert_eq!(claims.role, "admin");
    }

    #[test]
    fn 使用错误密钥校验应失败() {
        let token = issue_token(SECRET, "u1", "张三", "admin", 3600).unwrap();
        assert!(verify_token("another-secret-16chars", &token).is_err());
    }

    #[test]
    fn 篡改的_token_应校验失败() {
        let token = issue_token(SECRET, "u1", "张三", "admin", 3600).unwrap();
        let tampered = format!("{}x", token);
        assert!(verify_token(SECRET, &tampered).is_err());
    }

    #[test]
    fn 已过期的_token_应校验失败() {
        // 使用负 ttl 构造一个签发即过期的 Token。
        // 注意：jsonwebtoken 默认有 60s leeway，故需让过期时间明显早于当前。
        let token = issue_token(SECRET, "u1", "张三", "admin", 0).unwrap();
        // 手动构造一个 exp 远早于当前的 Token。
        let expired = {
            use jsonwebtoken::{encode, EncodingKey, Header};
            let claims = Claims {
                sub: "u1".into(),
                name: "张三".into(),
                role: "admin".into(),
                iat: 1_000_000,
                exp: 1_000_100, // 2001 年，早已过期
            };
            encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(SECRET.as_bytes()),
            )
            .unwrap()
        };
        assert!(verify_token(SECRET, &expired).is_err());
        // 正常签发的 Token 仍应有效。
        assert!(verify_token(SECRET, &token).is_ok());
    }

    #[test]
    fn claims_应可转换为_current_user() {
        let token = issue_token(SECRET, "u1", "张三", "admin", 3600).unwrap();
        let claims = verify_token(SECRET, &token).unwrap();
        let user: CurrentUser = claims.into();
        assert_eq!(user.id, "u1");
        assert_eq!(user.name, "张三");
        assert_eq!(user.role, "admin");
    }
}
