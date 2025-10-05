cemall/
│── README.md
│── .gitignore
│
├── backend/                       # Rust backend
│   └── cemall_experiences/
│       ├── Cargo.toml             # workspace definition
│       ├── apps/
│       │   └── api-gateway/
│       │       ├── Cargo.toml
│       │       └── src/
│       │           └── main.rs    # nests all routes
│       │
│       └── services/
│           ├── shops/
│           │   ├── common/
│           │   │   ├── auth.rs
│           │   │   ├── middleware.rs
│           │   │   └── base_roles.rs
│           │   ├── restaurant/
│           │   │   ├── Cargo.toml
│           │   │   └── src/
│           │   │       ├── lib.rs
│           │   │       ├── routes.rs
│           │   │       ├── handlers.rs
│           │   │       ├── models.rs
│           │   │       ├── pos.rs
│           │   │       └── common/roles.rs
│           │   └── chemist/
│           │       ├── Cargo.toml
│           │       └── src/
│           │           ├── lib.rs
│           │           ├── routes.rs
│           │           ├── handlers.rs
│           │           ├── models.rs
│           │           ├── pos.rs
│           │           └── common/roles.rs
│           │
│           ├── crm/
│           │   ├── Cargo.toml
│           │   └── src/{lib.rs,routes.rs,handlers.rs,models.rs,analytics.rs}
│           │
│           ├── analytics/
│           │   ├── Cargo.toml
│           │   └── src/{lib.rs,routes.rs,handlers.rs,models.rs,training.rs}
│           │
│           ├── logistics/
│           │   ├── Cargo.toml
│           │   └── src/
│           │       ├── {lib.rs,routes.rs,handlers.rs,models.rs,tracking.rs}
│           │       └── integrations/{sendy.rs,glovo.rs,local_courier.rs}
│           │
│           └── marketplace/
│               ├── Cargo.toml
│               └── src/{lib.rs,routes.rs,handlers.rs,models.rs,payments.rs}
│
└── frontend/                       # Next.js frontend
    ├── package.json
    ├── next.config.js
    ├── public/                     # static files
    ├── pages/
    │   ├── index.tsx
    │   ├── login.tsx
    │   ├── register.tsx
    │   ├── dashboard/
    │   │   ├── index.tsx
    │   │   ├── shops/
    │   │   │   ├── restaurant.tsx
    │   │   │   ├── chemist.tsx
    │   │   │   └── retail.tsx
    │   │   ├── crm.tsx
    │   │   ├── analytics.tsx
    │   │   ├── marketplace.tsx
    │   │   └── logistics.tsx
    │
    ├── components/
    │   ├── Header.tsx
    │   ├── Sidebar.tsx
    │   ├── ProductCard.tsx
    │   ├── POSInterface.tsx
    │   └── AnalyticsChart.tsx
    │
    ├── services/                   # API clients
    │   ├── auth.ts
    │   ├── shops.ts
    │   ├── crm.ts
    │   ├── analytics.ts
    │   ├── logistics.ts
    │   └── marketplace.ts
    │
    ├── context/                    # state management
    │   └── AuthContext.tsx
    ├── hooks/
    │   └── useFetchOrders.ts
    ├── styles/
    │   └── globals.css
    └── utils/
        └── validators.ts
