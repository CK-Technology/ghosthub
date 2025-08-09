# GhostHub

<div align="center">
  <img src="assets/icons/ghosthub-icon.png" alt="GhostHub Icon" width="128" height="128">

**Self-Hosted IT Operations Platform — Documentation, Ticketing, and Reporting in One**

![rust](https://img.shields.io/badge/Backend-Rust-orange?logo=rust)
![typescript](https://img.shields.io/badge/Frontend-TypeScript-blue?logo=typescript)
![svelte](https://img.shields.io/badge/UI-SvelteKit-ff3e00?logo=svelte)
![postgres](https://img.shields.io/badge/Database-PostgreSQL-336791?logo=postgresql)
![oidc](https://img.shields.io/badge/Auth-OIDC%20%2F%20SSO-brightgreen)
![docker](https://img.shields.io/badge/Deploy-Docker%20%7C%20K8s-2496ED?logo=docker)

</div>

---

## Overview

**GhostHub** is a self-hosted IT operations platform combining:

* **Hudu-style documentation**
* **Modern ticketing** inspired by BMS/HaloPSA
* **Reporting & asset management**
* **Developer-friendly API**

Designed for MSPs, internal IT teams, and DevOps groups that want control over their tooling without SaaS lock-in.

---

## ✨ Features

* **Documentation (Hudu-style)**

  * Nested docs, templates, and secrets fields
  * Revision history and attachments
  * Full-text search (Meilisearch)
* **Ticketing & Reporting**

  * Modern UI with drag-and-drop workflow changes
  * SLA tracking, custom statuses, priorities
  * Interactive dashboards and scheduled reports
* **Asset Management**

  * Clients, sites, devices, and relationships
  * Sync devices from GhostLink agents
* **Integrations**

  * CrowdSec / Wazuh event ingestion
  * Webhooks, REST API, and CLI
* **Auth & Security**

  * OIDC/SSO with Entra, GitHub, Google, Okta
  * Role-based access control (RBAC)

---

## 📦 Tech Stack

* **Backend:** Rust (`axum`, `tokio`, `sqlx`)
* **Frontend:** SvelteKit + TailwindCSS
* **Database:** PostgreSQL + Redis (cache & queues)
* **Search:** Meilisearch for instant KB/ticket search
* **Deploy:** Docker Compose or Helm chart for Kubernetes

---

## 🚀 Quick Start (Planned)

```bash
# Clone GhostHub
git clone https://github.com/resolve-technology/ghosthub
cd ghosthub

# Start with Docker Compose
docker compose up -d
```

**Example Config (`.env`)**

```env
POSTGRES_URL=postgres://ghosthub:password@db/ghosthub
REDIS_URL=redis://cache
MEILISEARCH_URL=http://search:7700
OIDC_PROVIDER_URL=https://login.microsoftonline.com/<tenant>/v2.0
OIDC_CLIENT_ID=<client-id>
OIDC_CLIENT_SECRET=<client-secret>
```

---

## 🗺 Roadmap

* [ ] MVP: KB, ticketing, client/site/device assets
* [ ] API & CLI tools
* [ ] Integrations: GhostLink, Ghostwarden, CrowdSec, Wazuh
* [ ] Reporting engine
* [ ] Role-based dashboards

---

## 🌐 Integrations

GhostHub is designed to be the **central hub** for:

* **GhostLink** → device info & remote actions
* **Ghostwarden** → security alerts & enforcement logs
* **CrowdSec/Wazuh** → create tickets or update assets automatically

