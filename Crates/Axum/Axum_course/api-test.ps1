# Get vehicle basic example
Invoke-RestMethod -Uri http://localhost:3000/vehicle -Method Get

# Post New Vehicle
Invoke-RestMethod -Uri http://localhost:3000/vehicle -Method Post

# Using a WebRequets
Invoke-WebRequest -Uri http://localhost:3000/vehicle -Method Post
