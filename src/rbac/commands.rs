use crate::error::AppError;
use crate::rbac::models::{Permission, Role, RolePermission, UserRole};
use anyhow::Result;
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
    db.delete("roles", &role_id).await.map_err(AppError::from)?;
    Ok(())
}

pub async fn rbac_list_permissions(db: &JsonProvider) -> Result<Vec<Permission>, AppError> {
    let permissions = db.find_all("permissions").await.map_err(AppError::from)?;
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
            db.delete("user_roles", id).await.map_err(AppError::from)?;
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

pub async fn rbac_get_user_roles(
    db: &JsonProvider,
    user_id: String,
) -> Result<Vec<Role>, AppError> {
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
        .filter_map(|rp| {
            rp.get("permission_id")
                .and_then(|v| v.as_str())
                .map(String::from)
        })
        .collect();

    let permissions = db.find_all("permissions").await.map_err(AppError::from)?;

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
            rp.get("permission_id")
                .and_then(|v| v.as_str())
                .map(String::from)
        })
        .collect();

    let all_perms = db.find_all("permissions").await.map_err(AppError::from)?;
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
    use anyhow::Context;
    use tempfile::TempDir;

    async fn make_test_provider(path: &std::path::Path) -> Result<JsonProvider> {
        let provider = JsonProvider::new(path)
            .await
            .context("failed to create JsonProvider")?;
        Ok(provider)
    }

    #[tokio::test]
    async fn test_rbac_list_roles() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let result = rbac_list_roles(&provider).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_create_and_list_role() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let created = rbac_create_role(
            &provider,
            "admin".to_string(),
            "Administrator role".to_string(),
        )
        .await
        .context("failed to create admin role")?;
        assert!(!created.id.is_empty());

        let roles = rbac_list_roles(&provider).await?;
        assert_eq!(roles.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_create_and_delete_role() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let created = rbac_create_role(
            &provider,
            "temp_role".to_string(),
            "Temporary role".to_string(),
        )
        .await
        .context("failed to create temp role")?;
        let role = created;
        let deleted = rbac_delete_role(&provider, role.id)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(())
    }

    // -- Permissions -------------------------------------------------------------

    #[tokio::test]
    async fn test_rbac_list_permissions() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let result = rbac_list_permissions(&provider).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_create_and_list_permission() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let created = rbac_create_permission(
            &provider,
            "read_users".to_string(),
            "users".to_string(),
            "read".to_string(),
        )
        .await
        .context("failed to create read_users permission")?;
        assert!(!created.id.is_empty());

        let perms = rbac_list_permissions(&provider).await?;
        assert_eq!(perms.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_delete_permission() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let created = rbac_create_permission(
            &provider,
            "temp_perm".to_string(),
            "documents".to_string(),
            "write".to_string(),
        )
        .await
        .context("failed to create temp_perm")?;
        let deleted = rbac_delete_permission(&provider, created.id)
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;
        Ok(())
    }

    // -- Role assignment to users -----------------------------------------------

    #[tokio::test]
    async fn test_rbac_assign_and_get_user_roles() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(&provider, "editor".to_string(), "Editor".to_string())
            .await
            .context("failed to create editor role")?;
        let assigned = rbac_assign_role_to_user(&provider, "user42".to_string(), role.id.clone())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        let roles = rbac_get_user_roles(&provider, "user42".to_string())
            .await
            .context("failed to get user roles")?;
        assert!(roles.iter().any(|r| r.id == role.id));
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_remove_role_from_user() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(&provider, "viewer".to_string(), "Viewer".to_string())
            .await
            .context("failed to create viewer role")?;
        rbac_assign_role_to_user(&provider, "user99".to_string(), role.id.clone())
            .await
            .context("failed to assign role")?;
        let removed = rbac_remove_role_from_user(&provider, "user99".to_string(), role.id.clone())
            .await
            .map_err(|e| anyhow::anyhow!("{}", e))?;

        let roles = rbac_get_user_roles(&provider, "user99".to_string())
            .await
            .context("failed to get user roles")?;
        assert!(!roles.iter().any(|r| r.id == role.id));
        Ok(())
    }

    // -- Permission grants -------------------------------------------------------

    #[tokio::test]
    async fn test_rbac_grant_and_get_role_permissions() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(&provider, "admin".to_string(), "Admin".to_string())
            .await
            .context("failed to create admin role")?;
        let perm = rbac_create_permission(
            &provider,
            "manage_users".to_string(),
            "users".to_string(),
            "admin".to_string(),
        )
        .await
        .context("failed to create manage_users permission")?;
        let granted = rbac_grant_permission(&provider, role.id.clone(), perm.id.clone())
            .await
            .context("failed to grant permission")?;

        let perms = rbac_get_role_permissions(&provider, role.id.clone())
            .await
            .context("failed to get role permissions")?;
        assert!(perms.iter().any(|p| p.id == perm.id));
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_revoke_permission() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(&provider, "editor".to_string(), "Editor".to_string())
            .await
            .context("failed to create editor role")?;
        let perm = rbac_create_permission(
            &provider,
            "edit_docs".to_string(),
            "documents".to_string(),
            "edit".to_string(),
        )
        .await
        .context("failed to create edit_docs permission")?;
        rbac_grant_permission(&provider, role.id.clone(), perm.id.clone())
            .await
            .context("failed to grant permission")?;
        let revoked = rbac_revoke_permission(&provider, role.id.clone(), perm.id.clone())
            .await
            .context("failed to revoke permission")?;

        let perms = rbac_get_role_permissions(&provider, role.id.clone())
            .await
            .context("failed to get role permissions after revoke")?;
        assert!(!perms.iter().any(|p| p.id == perm.id));
        Ok(())
    }

    // -- Permission checks via roles --------------------------------------------

    #[tokio::test]
    async fn test_rbac_create_role_with_permissions() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(
            &provider,
            "content_manager".to_string(),
            "Content Manager".to_string(),
        )
        .await
        .context("failed to create content_manager role")?;
        let perm_create = rbac_create_permission(
            &provider,
            "create_posts".to_string(),
            "posts".to_string(),
            "create".to_string(),
        )
        .await
        .context("failed to create create_posts permission")?;
        let perm_delete = rbac_create_permission(
            &provider,
            "delete_posts".to_string(),
            "posts".to_string(),
            "delete".to_string(),
        )
        .await
        .context("failed to create delete_posts permission")?;
        rbac_grant_permission(&provider, role.id.clone(), perm_create.id.clone())
            .await
            .context("failed to grant create_posts")?;
        rbac_grant_permission(&provider, role.id.clone(), perm_delete.id.clone())
            .await
            .context("failed to grant delete_posts")?;

        let perms = rbac_get_role_permissions(&provider, role.id.clone())
            .await
            .context("failed to get role permissions")?;
        assert_eq!(perms.len(), 2);
        assert!(perms.iter().any(|p| p.name == "create_posts"));
        assert!(perms.iter().any(|p| p.name == "delete_posts"));
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_check_permission_user_has_permission() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(&provider, "viewer".to_string(), "Viewer".to_string())
            .await
            .context("failed to create viewer role")?;
        let perm = rbac_create_permission(
            &provider,
            "view_dashboard".to_string(),
            "dashboard".to_string(),
            "view".to_string(),
        )
        .await
        .context("failed to create view_dashboard permission")?;
        rbac_grant_permission(&provider, role.id.clone(), perm.id.clone())
            .await
            .context("failed to grant permission")?;
        rbac_assign_role_to_user(&provider, "user_abc".to_string(), role.id.clone())
            .await
            .context("failed to assign role to user")?;

        let has_perm = rbac_check_permission(
            &provider,
            "user_abc".to_string(),
            "dashboard".to_string(),
            "view".to_string(),
        )
        .await
        .context("failed to check permission")?;
        assert!(has_perm);
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_check_permission_user_without_permission() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(
            &provider,
            "basic_user".to_string(),
            "Basic User".to_string(),
        )
        .await
        .context("failed to create basic_user role")?;
        let perm = rbac_create_permission(
            &provider,
            "view_reports".to_string(),
            "reports".to_string(),
            "view".to_string(),
        )
        .await
        .context("failed to create view_reports permission")?;
        rbac_grant_permission(&provider, role.id.clone(), perm.id.clone())
            .await
            .context("failed to grant permission")?;
        rbac_assign_role_to_user(&provider, "user_xyz".to_string(), role.id.clone())
            .await
            .context("failed to assign role to user")?;

        let has_perm = rbac_check_permission(
            &provider,
            "user_xyz".to_string(),
            "settings".to_string(),
            "edit".to_string(),
        )
        .await
        .context("failed to check permission")?;
        assert!(!has_perm);
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_remove_permission_from_role() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role = rbac_create_role(&provider, "moderator".to_string(), "Moderator".to_string())
            .await
            .context("failed to create moderator role")?;
        let perm_ban = rbac_create_permission(
            &provider,
            "ban_users".to_string(),
            "users".to_string(),
            "ban".to_string(),
        )
        .await
        .context("failed to create ban_users permission")?;
        let perm_kick = rbac_create_permission(
            &provider,
            "kick_users".to_string(),
            "users".to_string(),
            "kick".to_string(),
        )
        .await
        .context("failed to create kick_users permission")?;
        rbac_grant_permission(&provider, role.id.clone(), perm_ban.id.clone())
            .await
            .context("failed to grant ban_users")?;
        rbac_grant_permission(&provider, role.id.clone(), perm_kick.id.clone())
            .await
            .context("failed to grant kick_users")?;

        let perms_before = rbac_get_role_permissions(&provider, role.id.clone())
            .await
            .context("failed to get role permissions before revoke")?;
        assert_eq!(perms_before.len(), 2);

        rbac_revoke_permission(&provider, role.id.clone(), perm_ban.id.clone())
            .await
            .context("failed to revoke ban_users")?;

        let perms_after = rbac_get_role_permissions(&provider, role.id.clone())
            .await
            .context("failed to get role permissions after revoke")?;
        assert_eq!(perms_after.len(), 1);
        assert!(!perms_after.iter().any(|p| p.id == perm_ban.id));
        assert!(perms_after.iter().any(|p| p.id == perm_kick.id));
        Ok(())
    }

    #[tokio::test]
    async fn test_rbac_user_with_multiple_roles() -> Result<()> {
        let temp_dir = TempDir::new().context("failed to create temp dir")?;
        let provider = make_test_provider(temp_dir.path()).await?;
        let role_admin = rbac_create_role(&provider, "admin".to_string(), "Admin".to_string())
            .await
            .context("failed to create admin role")?;
        let role_support =
            rbac_create_role(&provider, "support".to_string(), "Support".to_string())
                .await
                .context("failed to create support role")?;
        let perm_users = rbac_create_permission(
            &provider,
            "manage_users".to_string(),
            "users".to_string(),
            "manage".to_string(),
        )
        .await
        .context("failed to create manage_users permission")?;
        let perm_tickets = rbac_create_permission(
            &provider,
            "view_tickets".to_string(),
            "tickets".to_string(),
            "view".to_string(),
        )
        .await
        .context("failed to create view_tickets permission")?;
        rbac_grant_permission(&provider, role_admin.id.clone(), perm_users.id.clone())
            .await
            .context("failed to grant manage_users to admin")?;
        rbac_grant_permission(&provider, role_support.id.clone(), perm_tickets.id.clone())
            .await
            .context("failed to grant view_tickets to support")?;
        rbac_assign_role_to_user(&provider, "multi_user".to_string(), role_admin.id.clone())
            .await
            .context("failed to assign admin role")?;
        rbac_assign_role_to_user(&provider, "multi_user".to_string(), role_support.id.clone())
            .await
            .context("failed to assign support role")?;

        let roles = rbac_get_user_roles(&provider, "multi_user".to_string())
            .await
            .context("failed to get user roles")?;
        assert_eq!(roles.len(), 2);
        assert!(roles.iter().any(|r| r.name == "admin"));
        assert!(roles.iter().any(|r| r.name == "support"));

        let can_manage_users = rbac_check_permission(
            &provider,
            "multi_user".to_string(),
            "users".to_string(),
            "manage".to_string(),
        )
        .await
        .context("failed to check manage_users permission")?;
        assert!(can_manage_users);

        let can_view_tickets = rbac_check_permission(
            &provider,
            "multi_user".to_string(),
            "tickets".to_string(),
            "view".to_string(),
        )
        .await
        .context("failed to check view_tickets permission")?;
        assert!(can_view_tickets);
        Ok(())
    }
}
