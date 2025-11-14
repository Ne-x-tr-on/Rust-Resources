# README (quick steps)
1. Create Postgres DB and run migrations (use psql or a migration tool)
2. Set environment variables in `.env`
3. `cargo run --release`
4. Endpoints:
   - POST /signup { email, password }
   - GET /verify_email?token=TOKEN
   - POST /login { email, password }
   - POST /resend_verification { email }
   - GET /me (Bearer TOKEN)