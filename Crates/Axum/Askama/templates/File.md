my_app/
│── Cargo.toml
│── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── users.rs
│   │   └── ml.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── prediction.rs
│   ├── services/
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   └── ml_service.rs
│   ├── db.rs
│   ├── config.rs
│   └── lib.rs






cemall/
│── gateway/               # API Gateway (entry point for all requests)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes.rs
│   │   └── auth_middleware.rs
│
│── common/                # Shared libraries
│   ├── Cargo.toml
│   ├── src/
│   │   ├── db.rs
│   │   ├── config.rs
│   │   ├── jwt.rs
│   │   └── models.rs
│
│── services/
│   ├── auth-service/      # Authentication & user management
│   │   ├── Cargo.toml
│   │   └── src/...
│   │
│   ├── shop-groceries/    # Grocery store microservice
│   │   ├── Cargo.toml
│   │   └── src/...
│   │
│   ├── shop-pharmacy/     # Pharmacy microservice
│   │   ├── Cargo.toml
│   │   └── src/...
│   │
│   ├── shop-electronics/  # Electronics store microservice
│   │   ├── Cargo.toml
│   │   └── src/...
│   │
│   └── ... more shops
│
│── pos-service/           # Shared POS (works online/offline)
│   ├── Cargo.toml
│   └── src/...
│
│── payment-service/       # Handles Mpesa, card payments, wallets
│   ├── Cargo.toml
│   └── src/...
│
│── shipping-service/      # Logistics & deliveries
│   ├── Cargo.toml
│   └── src/...
│
│── report-service/        # Analytics & reporting
│   ├── Cargo.toml
│   └── src/...
│
│── docker-compose.yml     # Runs all services locally
│── k8s/                   # Kubernetes manifests (for scaling)
