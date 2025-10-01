# GET request to list users
$response = Invoke-RestMethod -Uri "http://127.0.0.1:3000/users" -Method Get
$response | Format-Table


Invoke-RestMethod -Uri "http://127.0.0.1:3000/users/1" -Method Delete
