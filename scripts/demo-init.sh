#!/bin/bash
# Demo initialization script for GhostHub

set -e

echo "🚀 Starting GhostHub Demo Initialization..."

# Wait for database to be ready
echo "⏳ Waiting for database connection..."
while ! pg_isready -h db -p 5432 -U ghosthub; do
  echo "Waiting for PostgreSQL..."
  sleep 2
done

echo "✅ Database is ready!"

# Check if database is already initialized
if PGPASSWORD=ghosthub psql -h db -U ghosthub -d ghosthub -t -c "SELECT 1 FROM information_schema.tables WHERE table_name='users' LIMIT 1;" | grep -q 1; then
  echo "🔄 Database already initialized, skipping migration..."
else
  echo "🔧 Running database migrations..."
  
  # Run migrations (would normally use sqlx migrate, but for demo we'll use a simple approach)
  for migration in /app/migrations/*.sql; do
    if [ -f "$migration" ]; then
      echo "Running migration: $(basename $migration)"
      PGPASSWORD=ghosthub psql -h db -U ghosthub -d ghosthub -f "$migration" || {
        echo "⚠️ Migration $(basename $migration) failed, continuing..."
      }
    fi
  done
  
  echo "✅ Database migrations completed!"
  
  # Load demo data
  echo "📊 Loading demo data..."
  if [ -f "/app/demo-data.sql" ]; then
    PGPASSWORD=ghosthub psql -h db -U ghosthub -d ghosthub -f "/app/demo-data.sql" || {
      echo "⚠️ Demo data loading failed, continuing..."
    }
    echo "✅ Demo data loaded!"
  fi
fi

# Create uploads directory
mkdir -p /app/uploads/documents /app/uploads/assets /app/uploads/avatars
chown -R ghosthub:ghosthub /app/uploads

# Generate demo certificates (for testing SSL features)
if [ ! -f "/app/data/demo.crt" ]; then
  echo "🔐 Generating demo SSL certificates..."
  openssl req -x509 -newkey rsa:2048 -keyout /app/data/demo.key -out /app/data/demo.crt -days 365 -nodes -subj "/CN=localhost" 2>/dev/null || echo "⚠️ SSL cert generation failed, continuing..."
fi

echo "🎉 GhostHub Demo initialization complete!"
echo ""
echo "📋 Demo Information:"
echo "   • Web Interface: http://localhost:8080"
echo "   • Admin User: admin@ghosthub.demo / demo123"
echo "   • Tech User: tech@ghosthub.demo / demo123"
echo "   • Client Portal: http://localhost:8080/portal"
echo "   • Database Admin: http://localhost:8081 (with --profile admin)"
echo "   • Mail Catcher: http://localhost:8025 (with --profile mail)"
echo ""
echo "🔧 Demo Features Enabled:"
echo "   • 3 Sample Clients with realistic data"
echo "   • 25+ Sample tickets across different priorities"
echo "   • Asset inventory with health scores"
echo "   • Documentation templates and examples"
echo "   • Password vault with sample entries"
echo "   • Financial data and recurring billing"
echo "   • Automation workflows and alerts"
echo "   • Reporting dashboards with KPIs"
echo ""

exit 0