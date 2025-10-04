# Get vehicle basic example
Invoke-RestMethod -Uri http://localhost:3000/vehicle -Method Get

# Post New Vehicle
Invoke-RestMethod -Uri http://localhost:3000/vehicle -Method Post

# Post bank form
Invoke-RestMethod -Uri http://localhost:3000/bank_form -Method Post

# Using a WebRequets
Invoke-WebRequest -Uri http://localhost:3000/vehicle -Method Post

$params = @{
  Uri    = 'http://localhost:3000/vehicle'
  Method = 'POST'
  Body   = @{
    manufacturer = 'Tesla'
    model        = 'Model Y'
    year         = 2026
  } | ConvertTo-Json

Invoke-WebRequest @params

curl -X POST http://localhost:3000/vehicle\
-H "Content-Type: application/json"\
-d '{"manufacturer":"Tesla","model":"Model Y","year":"2026"}'