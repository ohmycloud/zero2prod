use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use tracing::{Span, field::display};
use uuid::Uuid;

type PgTransaction = Transaction<'static, Postgres>;

struct NewsLetterIssue {
    title: String,
    text_content: String,
    html_content: String,
}

async fn get_issue(pool: &PgPool, issue_id: Uuid) -> Result<NewsLetterIssue, anyhow::Error> {
    let issue = sqlx::query_as!(
        NewsLetterIssue,
        r#"
        SELECT title, text_content, html_content
        FROM newsletter_issues
        WHERE
            newsletter_issue_id = $1
        "#,
        issue_id,
    )
    .fetch_one(pool)
    .await?;

    Ok(issue)
}

#[tracing::instrument(name = "DEQUEUE TASK", skip_all)]
async fn dequeue_task(
    pool: &PgPool,
) -> Result<Option<(PgTransaction, Uuid, String)>, anyhow::Error> {
    let mut transaction = pool.begin().await?;
    let record = sqlx::query!(
        r#"
        SELECT newsletter_issue_id, subscriber_email
        FROM issue_delivery_queue
        FOR UPDATE
        SKIP LOCKED
        LIMIT 1
        "#
    )
    .fetch_optional(&mut *transaction)
    .await?;

    if let Some(record) = record {
        Ok(Some((
            transaction,
            record.newsletter_issue_id,
            record.subscriber_email,
        )))
    } else {
        Ok(None)
    }
}

#[tracing::instrument(skip_all)]
async fn delete_task(
    mut transaction: PgTransaction,
    issue_id: Uuid,
    email: &str,
) -> Result<(), anyhow::Error> {
    let query = sqlx::query!(
        r#"
        DELETE FROM issue_delivery_queue
        WHERE
            newsletter_issue_id = $1 AND
            subscriber_email = $2
        "#,
        issue_id,
        email
    );
    transaction.execute(query).await?;
    transaction.commit().await?;
    Ok(())
}

#[tracing::instrument(
    skip_all,
    fields(
        newsletter_issue_id=tracing::field::Empty,
        subscriber_id=tracing::field::Empty),
    err
)]
async fn try_execute_task(pool: &PgPool, email_client: &EmailClient) -> Result<(), anyhow::Error> {
    if let Some((transaction, issue_id, email)) = dequeue_task(pool).await? {
        Span::current()
            .record("newsletter_issue_id", &display(issue_id))
            .record("subscriber_email", &display(&email));

        match SubscriberEmail::parse(email.clone()) {
            Ok(email) => {
                let issue = get_issue(pool, issue_id).await?;
                if let Err(e) = email_client
                    .send_email(
                        &email,
                        &issue.title,
                        &issue.html_content,
                        &issue.text_content,
                    )
                    .await
                {
                    tracing::error!(
                        error.cause_chain = ?e,
                        error.message = %e,
                        "Failed to deliver issue to a confirmed subscriber. Skipping."
                    );
                }
            }
            Err(e) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Skipping a confirmed subscriber. Their stored contact details are invalid"
                );
            }
        }
        delete_task(transaction, issue_id, &email).await?;
    }
    Ok(())
}
