use crate::error::AppError;
use crate::rbac::models::{Permission, Role, RolePermission, UserRole};
use nosql_orm::prelude::*;
use nosql_orm::providers::JsonProvider;

pub async fn rbac_list_roles(db: &JsonProvider) -> Result<Vec<Role>, AppError> {
  let roles = db.find_all("roles").await.map_err(AppError::from)?;
  roles
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(AppError::from))
    .collect()
}

pub async fn rbac_create_role(
  db: &JsonProvider,
  name: String,
  description: String,
) -> Result<Role, AppError> {
  let role = Role::new(name, description);
  let role_value = serde_json::to_value(&role).map_err(AppError::from)?;
  db.insert("roles", role_value)
    .await
    .map_err(AppError::from)?;
  Ok(role)
}

pub async fn rbac_delete_role(db: &JsonProvider, role_id: String) -> Result<(), AppError> {
  db.delete("roles", &role_id)
    .await
    .map_err(AppError::from)?;
  Ok(())
}

pub async fn rbac_list_permissions(db: &JsonProvider) -> Result<Vec<Permission>, AppError> {
  let permissions = db
    .find_all("permissions")
    .await
    .map_err(AppError::from)?;
  permissions
    .into_iter()
    .map(|data| serde_json::from_value(data).map_err(AppError::from))
    .collect()
}

pub async fn rbac_create_permission(
  db: &JsonProvider,
  name: String,
  resource: String,
  action: String,
) -> Result<Permission, AppError> {
  let permission = Permission::new(name, resource, action);
  let perm_value = serde_json::to_value(&permission).map_err(AppError::from)?;
  db.insert("permissions", perm_value)
    .await
    .map_err(AppError::from)?;
  Ok(permission)
}

pub async fn rbac_delete_permission(db: &JsonProvider, perm_id: String) -> Result<(), AppError> {
  db.delete("permissions", &perm_id)
    .await
    .map_err(AppError::from)?;
  Ok(())
}

pub async fn rbac_assign_role_to_user(
  db: &JsonProvider,
  user_id: String,
  role_id: String,
) -> Result<UserRole, AppError> {
  let user_role = UserRole::new(user_id, role_id);
  let ur_value = serde_json::to_value(&user_role).map_err(AppError::from)?;
  db.insert("user_roles", ur_value)
    .await
    .map_err(AppError::from)?;
  Ok(user_role)
}

pub async fn rbac_remove_role_from_user(
  db: &JsonProvider,
  user_id: String,
  role_id: String,
) -> Result<(), AppError> {
  let user_roles = db.find_all("user_roles").await.map_err(AppError::from)?;
  let to_delete = user_roles.iter().find(|ur| {
    ur.get("user_id").and_then(|v| v.as_str()) == Some(&user_id)
      && ur.get("role_id").and_then(|v| v.as_str()) == Some(&role_id)
  });

  if let Some(ur) = to_delete {
    if let Some(id) = ur.get("id").and_then(|v| v.as_str()) {
      db.delete("user_roles", id)
        .await
        .map_err(AppError::from)?;
    }
  }
  Ok(())
}

pub async fn rbac_grant_permission(
  db: &JsonProvider,
  role_id: String,
  perm_id: String,
) -> Result<RolePermission, AppError> {
  let role_perm = RolePermission::new(role_id, perm_id);
  let rp_value = serde_json::to_value(&role_perm).map_err(AppError::from)?;
  db.insert("role_permissions", rp_value)
    .await
    .map_err(AppError::from)?;
  Ok(role_perm)
}

pub async fn rbac_revoke_permission(
  db: &JsonProvider,
  role_id: String,
  perm_id: String,
) -> Result<(), AppError> {
  let role_perms = db
    .find_all("role_permissions")
    .await
    .map_err(AppError::from)?;
  let to_delete = role_perms.iter().find(|rp| {
    rp.get("role_id").and_then(|v| v.as_str()) == Some(&role_id)
      && rp.get("permission_id").and_then(|v| v.as_str()) == Some(&perm_id)
  });

  if let Some(rp) = to_delete {
    if let Some(id) = rp.get("id").and_then(|v| v.as_str()) {
      db.delete("role_permissions", id)
        .await
        .map_err(AppError::from)?;
    }
  }
  Ok(())
}

pub async fn rbac_get_user_roles(db: &JsonProvider, user_id: String) -> Result<Vec<Role>, AppError> {
  let user_roles = db.find_all("user_roles").await.map_err(AppError::from)?;
  let role_ids: Vec<String> = user_roles
    .iter()
    .filter(|ur| ur.get("user_id").and_then(|v| v.as_str()) == Some(&user_id))
    .filter_map(|ur| ur.get("role_id").and_then(|v| v.as_str()).map(String::from))
    .collect();

  let all_roles = db.find_all("roles").await.map_err(AppError::from)?;
  all_roles
    .into_iter()
    .filter(|r| {
      r.get("id")
        .and_then(|v| v.as_str())
        .map(|id| role_ids.contains(&id.to_string()))
        .unwrap_or(false)
    })
    .map(|data| serde_json::from_value(data).map_err(AppError::from))
    .collect()
}

pub async fn rbac_check_permission(
  db: &JsonProvider,
  user_id: String,
  resource: String,
  action: String,
) -> Result<bool, AppError> {
  let user_roles = db.find_all("user_roles").await.map_err(AppError::from)?;
  let role_ids: Vec<String> = user_roles
    .iter()
    .filter(|ur| ur.get("user_id").and_then(|v| v.as_str()) == Some(&user_id))
    .filter_map(|ur| ur.get("role_id").and_then(|v| v.as_str()).map(String::from))
    .collect();

  let role_perms = db
    .find_all("role_permissions")
    .await
    .map_err(AppError::from)?;
  let perm_ids: Vec<String> = role_perms
    .iter()
    .filter(|rp| {
      rp.get("role_id")
        .and_then(|v| v.as_str())
        .map(|id| role_ids.contains(&id.to_string()))
        .unwrap_or(false)
    })
    .filter_map(|rp| rp.get("permission_id").and_then(|v| v.as_str()).map(String::from))
    .collect();

  let permissions = db
    .find_all("permissions")
    .await
    .map_err(AppError::from)?;

  let has_permission = permissions.iter().any(|p| {
    perm_ids.contains(
      &p.get("id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string(),
    ) && p.get("resource").and_then(|v| v.as_str()) == Some(&resource)
      && p.get("action").and_then(|v| v.as_str()) == Some(&action)
  });

  Ok(has_permission)
}

pub async fn rbac_get_role_permissions(
  db: &JsonProvider,
  role_id: String,
) -> Result<Vec<Permission>, AppError> {
  let role_perms = db
    .find_all("role_permissions")
    .await
    .map_err(AppError::from)?;
  let perm_ids: Vec<String> = role_perms
    .iter()
    .filter(|rp| rp.get("role_id").and_then(|v| v.as_str()) == Some(&role_id))
    .filter_map(|rp| {
      rp
        .get("permission_id")
        .and_then(|v| v.as_str())
        .map(String::from)
    })
    .collect();

  let all_perms = db
    .find_all("permissions")
    .await
    .map_err(AppError::from)?;
  all_perms
    .into_iter()
    .filter(|p| {
      p.get("id")
        .and_then(|v| v.as_str())
        .map(|id| perm_ids.contains(&id.to_string()))
        .unwrap_or(false)
    })
    .map(|data| serde_json::from_value(data).map_err(AppError::from))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use tempfile::TempDir;

  #[tokio::test]
  async fn test_rbac_list_roles() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_str().unwrap();
    let provider = JsonProvider::new(path).await.unwrap();
    let result = rbac_list_roles(&provider).await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_rbac_create_and_list_role() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_str().unwrap();
    let provider = JsonProvider::new(path).await.unwrap();
    let created = rbac_create_role(
      &provider,
      "admin".to_string(),
      "Administrator role".to_string(),
    )
    .await;
    assert!(created.is_ok());
    let roles = rbac_list_roles(&provider).await;
    assert!(roles.is_ok());
    assert_eq!(roles.unwrap().len(), 1);
  }

  #[tokio::test]
  async fn test_rbac_create_and_delete_role() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().to_str().unwrap();
    let provider = JsonProvider::new(path).await.unwrap();
    let created = rbac_create_role(
      &provider,
      "temp_role".to_string(),
      "Temporary role".to_string(),
    )
    .await;
    assert!(created.is_ok());
    let role = created.unwrap();
    let deleted = rbac_delete_role(&provider, role.id).await;
    assert!(deleted.is_ok());
  }

  // -- Permissions -------------------------------------------------------------

  #[tokio::test]
  async fn test_rbac_list_permissions() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    let result = rbac_list_permissions(&provider).await;
    assert!(result.is_ok());
  }

  #[tokio::test]
  async fn test_rbac_create_and_list_permission() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    let created = rbac_create_permission(
      &provider,
      "read_users".to_string(),
      "users".to_string(),
      "read".to_string(),
    )
    .await;
    assert!(created.is_ok());
    let perms = rbac_list_permissions(&provider).await;
    assert!(perms.is_ok());
    assert_eq!(perms.unwrap().len(), 1);
  }

  #[tokio::test]
  async fn test_rbac_delete_permission() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    let created = rbac_create_permission(
      &provider,
      "temp_perm".to_string(),
      "documents".to_string(),
      "write".to_string(),
    )
    .await
    .unwrap();
    let deleted = rbac_delete_permission(&provider, created.id).await;
    assert!(deleted.is_ok());
  }

  // -- Role assignment to users -----------------------------------------------

  #[tokio::test]
  async fn test_rbac_assign_and_get_user_roles() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    let role = rbac_create_role(&provider, "editor".to_string(), "Editor".to_string())
      .await
      .unwrap();
    let assigned = rbac_assign_role_to_user(&provider, "user42".to_string(), role.id.clone())
      .await;
    assert!(assigned.is_ok());
    let roles = rbac_get_user_roles(&provider, "user42".to_string()).await;
    assert!(roles.is_ok());
    let user_roles = roles.unwrap();
    assert!(user_roles.iter().any(|r| r.id == role.id));
  }

  #[tokio::test]
  async fn test_rbac_remove_role_from_user() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    let role = rbac_create_role(&provider, "viewer".to_string(), "Viewer".to_string())
      .await
      .unwrap();
    rbac_assign_role_to_user(&provider, "user99".to_string(), role.id.clone())
      .await
      .unwrap();
    let removed = rbac_remove_role_from_user(&provider, "user99".to_string(), role.id.clone())
      .await;
    assert!(removed.is_ok());
    let roles = rbac_get_user_roles(&provider, "user99".to_string()).await.unwrap();
    assert!(!roles.iter().any(|r| r.id == role.id));
  }

  // -- Permission grants -------------------------------------------------------

  #[tokio::test]
  async fn test_rbac_grant_and_get_role_permissions() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    let role = rbac_create_role(&provider, "admin".to_string(), "Admin".to_string())
      .await
      .unwrap();
    let perm = rbac_create_permission(
      &provider,
      "manage_users".to_string(),
      "users".to_string(),
      "admin".to_string(),
    )
    .await
    .unwrap();
    let granted = rbac_grant_permission(&provider, role.id.clone(), perm.id.clone())
      .await;
    assert!(granted.is_ok());
    let perms = rbac_get_role_permissions(&provider, role.id.clone()).await;
    assert!(perms.is_ok());
    assert!(perms.unwrap().iter().any(|p| p.id == perm.id));
  }

  #[tokio::test]
  async fn test_rbac_revoke_permission() {
    let temp_dir = TempDir::new().unwrap();
    let provider = JsonProvider::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    let role = rbac_create_role(&provider, "editor".to_string(), "Editor".to_string())
      .await
      .unwrap();
    let perm = rbac_create_permission(
      &provider,
      "edit_docs".to_string(),
      "documents".to_string(),
      "edit".to_string(),
    )
    .await
    .unwrap();
    rbac_grant_permission(&provider, role.id.clone(), perm.id.clone())
      .await
      .unwrap();
    let revoked = rbac_revoke_permission(&provider, role.id.clone(), perm.id.clone()).await;
    assert!(revoked.is_ok());
    let perms = rbac_get_role_permissions(&provider, role.id.clone()).await.unwrap();
    assert!(!perms.iter().any(|p| p.id == perm.id));
  }
}