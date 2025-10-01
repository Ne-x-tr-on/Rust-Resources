@echo off
REM Start PostgreSQL server
"C:\Program Files\PostgreSQL\17\bin\pg_ctl.exe" -D "C:\PostgresData" -l "C:\PostgresData\logfile" start

REM Wait a few seconds for the server to start
timeout /t 3 /nobreak >nul

REM Open psql connected to cemall database
"C:\Program Files\PostgreSQL\17\bin\psql.exe" -U postgres -d cemall
