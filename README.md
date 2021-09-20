# Create DB
`sqlx database create`
# Add migrations
`sqlx migrate add create_subscriptions_table`

# Run migrations
`SKIP_DOCKER=true ./scripts/init_db.sh`