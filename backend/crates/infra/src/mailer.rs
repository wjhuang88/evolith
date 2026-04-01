//! Email/Mailer Layer
//!
//! Provides email sending capabilities via SMTP (production) or console logging (development).

use async_trait::async_trait;
use common::error::{AppError, Result};
use lettre::{
    message::{header::ContentType, Mailbox, MessageBuilder},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use tracing::info;

use crate::config::SmtpConfig;

/// Trait for email sending operations.
///
/// Implementations can use SMTP for production or console logging for development.
#[async_trait]
pub trait Mailer: Send + Sync {
    /// Send an email verification email.
    ///
    /// # Arguments
    /// * `to` - Recipient email address
    /// * `username` - User's display name
    /// * `token` - Verification token
    /// * `base_url` - Base URL for the verification link
    async fn send_verification_email(
        &self,
        to: &str,
        username: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()>;

    /// Send a password reset email.
    ///
    /// # Arguments
    /// * `to` - Recipient email address
    /// * `username` - User's display name
    /// * `token` - Password reset token
    /// * `base_url` - Base URL for the reset link
    async fn send_password_reset_email(
        &self,
        to: &str,
        username: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()>;

    /// Send a tenant invitation email.
    ///
    /// # Arguments
    /// * `to` - Recipient email address
    /// * `inviter_name` - Name of the person sending the invitation
    /// * `tenant_name` - Name of the tenant/workspace
    /// * `token` - Invitation token
    /// * `base_url` - Base URL for the invitation link
    async fn send_invitation_email(
        &self,
        to: &str,
        inviter_name: &str,
        tenant_name: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()>;
}

/// SMTP-based mailer for production use.
///
/// Uses `lettre::AsyncSmtpTransport` with `Tokio1Executor` for async email delivery.
pub struct SmtpMailer {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_address: String,
    from_name: String,
}

impl SmtpMailer {
    /// Create a new SMTP mailer from configuration.
    ///
    /// # Errors
    /// Returns an error if the SMTP transport cannot be built.
    pub fn new(config: &SmtpConfig) -> Result<Self> {
        let credentials = Credentials::new(config.username.clone(), config.password.clone());

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
            .map_err(|e| AppError::ConfigError(format!("Failed to create SMTP transport: {}", e)))?
            .credentials(credentials)
            .port(config.port)
            .build();

        Ok(Self {
            transport,
            from_address: config.from_address.clone(),
            from_name: config.from_name.clone(),
        })
    }

    fn build_message(&self, to: &str, subject: &str) -> Result<MessageBuilder> {
        let from_mailbox: Mailbox = format!("{} <{}>", self.from_name, self.from_address)
            .parse()
            .map_err(|e| AppError::ValidationError(format!("Invalid from address: {}", e)))?;

        let to_mailbox: Mailbox = to
            .parse()
            .map_err(|e| AppError::ValidationError(format!("Invalid recipient address: {}", e)))?;

        Ok(Message::builder()
            .from(from_mailbox)
            .to(to_mailbox)
            .subject(subject))
    }

    async fn send_message(&self, message: Message) -> Result<()> {
        self.transport.send(message).await.map_err(|e| {
            AppError::external_service("smtp", format!("Failed to send email: {}", e))
        })?;
        Ok(())
    }
}

#[async_trait]
impl Mailer for SmtpMailer {
    async fn send_verification_email(
        &self,
        to: &str,
        username: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()> {
        let verification_link = format!("{}/verify-email?token={}", base_url, token);
        let subject = "Verify your Evolith account";

        let html_body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Verify your email</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
    <div style="background: #f9f9f9; border-radius: 8px; padding: 30px; margin-top: 20px;">
        <h1 style="color: #2c3e50; margin-bottom: 20px;">Welcome to Evolith, {}!</h1>
        <p style="margin-bottom: 20px;">Thank you for creating an account. Please verify your email address by clicking the button below:</p>
        <div style="text-align: center; margin: 30px 0;">
            <a href="{}" style="background: #3498db; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block; font-weight: bold;">Verify Email Address</a>
        </div>
        <p style="color: #666; font-size: 14px;">Or copy and paste this link into your browser:</p>
        <p style="background: #eee; padding: 10px; border-radius: 4px; word-break: break-all; font-size: 13px;">{}</p>
        <p style="margin-top: 30px; color: #666; font-size: 13px;">If you didn't create an account with Evolith, you can safely ignore this email.</p>
    </div>
    <p style="text-align: center; color: #999; font-size: 12px; margin-top: 20px;">© Evolith. All rights reserved.</p>
</body>
</html>
"#,
            username, verification_link, verification_link
        );

        let message = self
            .build_message(to, subject)?
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .map_err(|e| AppError::InternalError(format!("Failed to build email body: {}", e)))?;

        self.send_message(message).await
    }

    async fn send_password_reset_email(
        &self,
        to: &str,
        username: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()> {
        let reset_link = format!("{}/reset-password?token={}", base_url, token);
        let subject = "Reset your Evolith password";

        let html_body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Reset your password</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
    <div style="background: #f9f9f9; border-radius: 8px; padding: 30px; margin-top: 20px;">
        <h1 style="color: #2c3e50; margin-bottom: 20px;">Password Reset Request</h1>
        <p style="margin-bottom: 10px;">Hi {},</p>
        <p style="margin-bottom: 20px;">We received a request to reset your password. Click the button below to create a new password:</p>
        <div style="text-align: center; margin: 30px 0;">
            <a href="{}" style="background: #e74c3c; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block; font-weight: bold;">Reset Password</a>
        </div>
        <p style="color: #666; font-size: 14px;">Or copy and paste this link into your browser:</p>
        <p style="background: #eee; padding: 10px; border-radius: 4px; word-break: break-all; font-size: 13px;">{}</p>
        <p style="margin-top: 20px; color: #666; font-size: 13px;"><strong>This link will expire in 1 hour.</strong></p>
        <p style="margin-top: 10px; color: #666; font-size: 13px;">If you didn't request a password reset, you can safely ignore this email. Your password will remain unchanged.</p>
    </div>
    <p style="text-align: center; color: #999; font-size: 12px; margin-top: 20px;">© Evolith. All rights reserved.</p>
</body>
</html>
"#,
            username, reset_link, reset_link
        );

        let message = self
            .build_message(to, subject)?
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .map_err(|e| AppError::InternalError(format!("Failed to build email body: {}", e)))?;

        self.send_message(message).await
    }

    async fn send_invitation_email(
        &self,
        to: &str,
        inviter_name: &str,
        tenant_name: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()> {
        let invitation_link = format!("{}/accept-invitation?token={}", base_url, token);
        let subject = format!("You've been invited to {} on Evolith", tenant_name);

        let html_body = format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Workspace Invitation</title>
</head>
<body style="font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px;">
    <div style="background: #f9f9f9; border-radius: 8px; padding: 30px; margin-top: 20px;">
        <h1 style="color: #2c3e50; margin-bottom: 20px;">You're Invited!</h1>
        <p style="margin-bottom: 20px;"><strong>{}</strong> has invited you to join <strong>{}</strong> on Evolith.</p>
        <p style="margin-bottom: 20px;">Evolith is an AI agent development platform that helps teams build, share, and deploy AI-powered tools and skills.</p>
        <div style="text-align: center; margin: 30px 0;">
            <a href="{}" style="background: #27ae60; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; display: inline-block; font-weight: bold;">Accept Invitation</a>
        </div>
        <p style="color: #666; font-size: 14px;">Or copy and paste this link into your browser:</p>
        <p style="background: #eee; padding: 10px; border-radius: 4px; word-break: break-all; font-size: 13px;">{}</p>
        <p style="margin-top: 30px; color: #666; font-size: 13px;">This invitation will expire in 7 days. If you don't have an Evolith account yet, you'll be able to create one after clicking the link.</p>
    </div>
    <p style="text-align: center; color: #999; font-size: 12px; margin-top: 20px;">© Evolith. All rights reserved.</p>
</body>
</html>
"#,
            inviter_name, tenant_name, invitation_link, invitation_link
        );

        let message = self
            .build_message(to, &subject)?
            .header(ContentType::TEXT_HTML)
            .body(html_body)
            .map_err(|e| AppError::InternalError(format!("Failed to build email body: {}", e)))?;

        self.send_message(message).await
    }
}

/// Console-based mailer for development/testing.
///
/// Logs email content to console via `tracing::info!` instead of actually sending.
pub struct ConsoleMailer;

#[async_trait]
impl Mailer for ConsoleMailer {
    async fn send_verification_email(
        &self,
        to: &str,
        username: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()> {
        info!(
            to = %to,
            subject = "Verify your Evolith account",
            username = %username,
            link = format!("{}/verify-email?token={}", base_url, token),
            "[ConsoleMailer] Would send verification email"
        );
        Ok(())
    }

    async fn send_password_reset_email(
        &self,
        to: &str,
        username: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()> {
        info!(
            to = %to,
            subject = "Reset your Evolith password",
            username = %username,
            link = format!("{}/reset-password?token={}", base_url, token),
            "[ConsoleMailer] Would send password reset email"
        );
        Ok(())
    }

    async fn send_invitation_email(
        &self,
        to: &str,
        inviter_name: &str,
        tenant_name: &str,
        token: &str,
        base_url: &str,
    ) -> Result<()> {
        info!(
            to = %to,
            subject = format!("You've been invited to {} on Evolith", tenant_name),
            inviter = %inviter_name,
            tenant = %tenant_name,
            link = format!("{}/accept-invitation?token={}", base_url, token),
            "[ConsoleMailer] Would send invitation email"
        );
        Ok(())
    }
}

/// Factory function to create a mailer based on configuration.
///
/// Returns `SmtpMailer` if `config.enabled == true`, otherwise returns `ConsoleMailer`.
pub fn create_mailer(config: &SmtpConfig) -> Box<dyn Mailer> {
    if config.enabled {
        match SmtpMailer::new(config) {
            Ok(mailer) => Box::new(mailer),
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    "[create_mailer] Failed to create SmtpMailer, falling back to ConsoleMailer"
                );
                Box::new(ConsoleMailer)
            }
        }
    } else {
        info!("[create_mailer] SMTP disabled, using ConsoleMailer");
        Box::new(ConsoleMailer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_smtp_config() -> SmtpConfig {
        SmtpConfig {
            host: "localhost".to_string(),
            port: 1025,
            username: String::new(),
            password: String::new(),
            from_address: "noreply@evolith.io".to_string(),
            from_name: "Evolith".to_string(),
            enabled: true,
        }
    }

    #[test]
    fn test_smtp_mailer_creation() {
        let config = test_smtp_config();
        let result = SmtpMailer::new(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_smtp_mailer_invalid_host_does_not_panic() {
        let config = SmtpConfig {
            host: "invalid host with spaces".to_string(),
            ..test_smtp_config()
        };
        let _result = SmtpMailer::new(&config);
    }

    #[test]
    fn test_create_mailer_enabled() {
        let config = test_smtp_config();
        let mailer = create_mailer(&config);
        drop(mailer);
    }

    #[test]
    fn test_create_mailer_disabled() {
        let config = SmtpConfig {
            enabled: false,
            ..test_smtp_config()
        };
        let mailer = create_mailer(&config);
        drop(mailer);
    }

    #[tokio::test]
    async fn test_console_mailer_verification() {
        let mailer = ConsoleMailer;
        let result = mailer
            .send_verification_email(
                "test@example.com",
                "TestUser",
                "token123",
                "https://evolith.io",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_console_mailer_password_reset() {
        let mailer = ConsoleMailer;
        let result = mailer
            .send_password_reset_email(
                "test@example.com",
                "TestUser",
                "token123",
                "https://evolith.io",
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_console_mailer_invitation() {
        let mailer = ConsoleMailer;
        let result = mailer
            .send_invitation_email(
                "test@example.com",
                "Admin User",
                "Test Workspace",
                "token123",
                "https://evolith.io",
            )
            .await;
        assert!(result.is_ok());
    }
}
