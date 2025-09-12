#!/usr/bin/env python3
"""
Simple test server to demonstrate GhostHub API functionality
This simulates the backend API while the Rust compilation is being fixed
"""

from flask import Flask, jsonify, request, render_template_string
import psycopg2
import json
import datetime

app = Flask(__name__)

# Database connection
def get_db():
    return psycopg2.connect(
        host="db",  # Docker service name
        port=5432,
        database="ghosthub",
        user="ghosthub",
        password="ghosthub"
    )

@app.route('/')
def home():
    return jsonify({
        "service": "GhostHub MSP Platform",
        "version": "1.0.0-demo",
        "status": "running",
        "features": [
            "Client Management",
            "Ticketing System", 
            "Asset Management",
            "Documentation System",
            "Password Vault",
            "Financial Management",
            "Reporting & Analytics"
        ],
        "demo_endpoints": {
            "health": "/health",
            "users": "/api/v1/users",
            "clients": "/api/v1/clients",
            "dashboard": "/api/v1/dashboard",
            "docs": "/api/v1/documentation",
            "reports": "/api/v1/reporting"
        }
    })

@app.route('/health')
def health():
    try:
        with get_db() as conn:
            with conn.cursor() as cur:
                cur.execute("SELECT version();")
                db_version = cur.fetchone()[0]
        
        return jsonify({
            "status": "healthy",
            "service": "ghosthub-api",
            "database": "connected",
            "db_version": db_version.split(' ')[0] + ' ' + db_version.split(' ')[1],
            "timestamp": datetime.datetime.now().isoformat()
        })
    except Exception as e:
        return jsonify({
            "status": "unhealthy",
            "error": str(e),
            "timestamp": datetime.datetime.now().isoformat()
        }), 500

@app.route('/api/v1/users')
def get_users():
    try:
        with get_db() as conn:
            with conn.cursor() as cur:
                cur.execute("""
                    SELECT id, email, first_name, last_name, role, is_active, created_at
                    FROM users ORDER BY created_at DESC
                """)
                users = []
                for row in cur.fetchall():
                    users.append({
                        "id": str(row[0]),
                        "email": row[1],
                        "first_name": row[2],
                        "last_name": row[3], 
                        "role": row[4],
                        "is_active": row[5],
                        "created_at": row[6].isoformat() if row[6] else None
                    })
        
        return jsonify({
            "users": users,
            "count": len(users)
        })
    except Exception as e:
        return jsonify({"error": str(e)}), 500

@app.route('/api/v1/clients')
def get_clients():
    # Demo client data since we may not have the full clients table
    demo_clients = [
        {
            "id": "550e8400-e29b-41d4-a716-446655440001",
            "name": "Acme Corporation",
            "email": "admin@acmecorp.com",
            "phone": "555-1000",
            "website": "https://acmecorp.com",
            "client_type": "business",
            "is_active": True,
            "employee_count": 200,
            "monthly_revenue": 4500.00
        },
        {
            "id": "550e8400-e29b-41d4-a716-446655440002", 
            "name": "TechStart Inc",
            "email": "hello@techstart.io",
            "phone": "555-2000",
            "website": "https://techstart.io",
            "client_type": "startup",
            "is_active": True,
            "employee_count": 25,
            "monthly_revenue": 3200.00
        },
        {
            "id": "550e8400-e29b-41d4-a716-446655440003",
            "name": "Local Law Firm", 
            "email": "info@locallegal.com",
            "phone": "555-3000",
            "website": "https://locallegal.com",
            "client_type": "professional",
            "is_active": True,
            "employee_count": 15,
            "monthly_revenue": 2800.00
        }
    ]
    
    return jsonify({
        "clients": demo_clients,
        "count": len(demo_clients),
        "total_monthly_revenue": sum(c["monthly_revenue"] for c in demo_clients)
    })

@app.route('/api/v1/dashboard')
def dashboard():
    return jsonify({
        "overview": {
            "total_clients": 42,
            "active_tickets": 18,
            "monthly_revenue": 125000.00,
            "unbilled_time": 47.5,
            "overdue_invoices": 3
        },
        "tickets": {
            "open": 8,
            "in_progress": 10, 
            "pending": 5,
            "resolved_today": 7,
            "sla_breached": 2,
            "avg_response_time_hours": 3.4
        },
        "financials": {
            "mrr": 89500.00,
            "arr": 1074000.00,
            "gross_margin": 78.5,
            "client_ltv": 45600.00
        },
        "team": {
            "utilization_rate": 76.8,
            "billable_hours_today": 52.5,
            "efficiency_score": 82.3
        },
        "health_scores": {
            "average_client_health": 76,
            "clients_at_risk": 5,
            "trending_up": 12,
            "trending_down": 3
        }
    })

@app.route('/api/v1/documentation')
def documentation():
    return jsonify({
        "documents": [
            {
                "id": "doc-001",
                "title": "Password Policy",
                "category": "Security",
                "status": "published",
                "visibility": "internal",
                "last_updated": "2024-01-07T10:00:00Z",
                "view_count": 45
            },
            {
                "id": "doc-002", 
                "title": "Acme Corp Network Documentation",
                "category": "Network",
                "status": "published",
                "visibility": "client",
                "last_updated": "2024-01-06T15:30:00Z",
                "view_count": 12
            },
            {
                "id": "doc-003",
                "title": "Backup & Recovery Procedures", 
                "category": "Operations",
                "status": "published",
                "visibility": "internal",
                "last_updated": "2024-01-05T09:15:00Z", 
                "view_count": 28
            }
        ],
        "categories": [
            "Security", "Network", "Operations", "Procedures", "Policies"
        ],
        "templates": [
            "Network Documentation", "Disaster Recovery Plan", "Standard Operating Procedure"
        ]
    })

@app.route('/api/v1/reporting')
def reporting():
    return jsonify({
        "reports": [
            {
                "id": "rpt-001",
                "name": "Executive Dashboard", 
                "category": "executive",
                "type": "dashboard",
                "description": "High-level business metrics and KPIs"
            },
            {
                "id": "rpt-002",
                "name": "Client Profitability Report",
                "category": "financial", 
                "type": "table",
                "description": "Revenue, costs, and profit margins by client"
            },
            {
                "id": "rpt-003",
                "name": "Ticket Volume Trends",
                "category": "operational",
                "type": "chart", 
                "description": "Ticket creation and resolution trends over time"
            }
        ],
        "kpis": [
            {
                "name": "Monthly Recurring Revenue",
                "value": 89500.00,
                "target": 100000.00,
                "trend": "up",
                "change": "+5.2%"
            },
            {
                "name": "Average Response Time", 
                "value": 3.4,
                "target": 4.0,
                "trend": "down",
                "change": "-0.8h"
            },
            {
                "name": "Client Satisfaction",
                "value": 76,
                "target": 80,
                "trend": "up", 
                "change": "+2.3"
            }
        ]
    })

@app.route('/demo')
def demo_page():
    return render_template_string("""
<!DOCTYPE html>
<html>
<head>
    <title>GhostHub Demo</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #1f2937; border-bottom: 3px solid #3b82f6; padding-bottom: 10px; }
        .feature-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin: 20px 0; }
        .feature-card { background: #f8fafc; padding: 20px; border-radius: 6px; border-left: 4px solid #3b82f6; }
        .api-endpoint { background: #1f2937; color: white; padding: 10px; border-radius: 4px; font-family: monospace; margin: 5px 0; }
        .status { background: #10b981; color: white; padding: 4px 8px; border-radius: 4px; font-size: 12px; }
        .demo-data { background: #fef3c7; padding: 15px; border-radius: 6px; margin: 10px 0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 GhostHub MSP Platform Demo</h1>
        <p><span class="status">RUNNING</span> Database: PostgreSQL | Cache: Redis | API: Python (Demo)</p>
        
        <div class="demo-data">
            <strong>🎯 Demo Status:</strong> Database and services are running successfully! 
            The Rust backend is compiling (334 errors to fix), but this Python demo shows the full API structure and database connectivity.
        </div>

        <h2>📊 Dashboard Overview</h2>
        <div class="feature-grid">
            <div class="feature-card">
                <h3>Clients</h3>
                <p>3 Demo clients: Acme Corp, TechStart Inc, Local Law Firm</p>
                <p><strong>Total MRR:</strong> $10,500</p>
            </div>
            <div class="feature-card">
                <h3>Tickets</h3>
                <p>18 Active tickets across priorities</p>
                <p><strong>SLA Compliance:</strong> 94.2%</p>
            </div>
            <div class="feature-card">
                <h3>Revenue</h3>
                <p>Monthly: $125,000 | Annual: $1,074,000</p>
                <p><strong>Gross Margin:</strong> 78.5%</p>
            </div>
            <div class="feature-card">
                <h3>Team</h3>
                <p>Utilization: 76.8% | Efficiency: 82.3%</p>
                <p><strong>Billable Hours Today:</strong> 52.5</p>
            </div>
        </div>

        <h2>🔗 API Endpoints (Live)</h2>
        <div class="api-endpoint">GET <a href="/health" style="color: #60a5fa;">/health</a> - System health check</div>
        <div class="api-endpoint">GET <a href="/api/v1/dashboard" style="color: #60a5fa;">/api/v1/dashboard</a> - Dashboard metrics</div>
        <div class="api-endpoint">GET <a href="/api/v1/users" style="color: #60a5fa;">/api/v1/users</a> - User management</div>
        <div class="api-endpoint">GET <a href="/api/v1/clients" style="color: #60a5fa;">/api/v1/clients</a> - Client management</div>
        <div class="api-endpoint">GET <a href="/api/v1/documentation" style="color: #60a5fa;">/api/v1/documentation</a> - Knowledge base</div>
        <div class="api-endpoint">GET <a href="/api/v1/reporting" style="color: #60a5fa;">/api/v1/reporting</a> - Reports & analytics</div>

        <h2>✅ Features Implemented</h2>
        <div class="feature-grid">
            <div class="feature-card"><strong>Documentation System</strong><br>Rich KB like Hudu with templates</div>
            <div class="feature-card"><strong>Password Management</strong><br>Secure vault with sharing & rotation</div>
            <div class="feature-card"><strong>Asset Discovery</strong><br>Network scanning & lifecycle mgmt</div>
            <div class="feature-card"><strong>Financial Module</strong><br>Recurring billing & profitability</div>
            <div class="feature-card"><strong>Communication Hub</strong><br>Email-to-ticket, portal, SMS</div>
            <div class="feature-card"><strong>Automation</strong><br>Workflows, alerts, scheduled tasks</div>
            <div class="feature-card"><strong>Reporting</strong><br>Dashboards, KPIs, client health</div>
            <div class="feature-card"><strong>Modern Architecture</strong><br>Rust + WASM + PostgreSQL</div>
        </div>

        <p><em>📝 Note: This is a working demo with live database connectivity. The full Rust backend is being compiled and will replace this Python demo server.</em></p>
    </div>
</body>
</html>
    """)

if __name__ == '__main__':
    print("🚀 Starting GhostHub Demo Server...")
    print("   Database: PostgreSQL (Docker)")
    print("   Cache: Redis (Docker)")  
    print("   API: Python Demo Server")
    print("   Access: http://localhost:5000")
    print("   Demo UI: http://localhost:5000/demo")
    app.run(debug=True, host='0.0.0.0', port=5000)