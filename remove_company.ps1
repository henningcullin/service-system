Write-Host "Remove a Company / Database / Customer"

if (!($server = Read-Host "Enter Server / Host (127.0.0.1)")) { $server = "127.0.0.1" }
$root_username = 'postgres'
$root_password = Read-Host "Enter Root Password"

$company_name = Read-Host "Enter company name"

$psql = "C:\Program Files\PostgreSQL\16\bin\psql.exe"          # CHANGE POSTGRESQL PATH HERE

$env:PGPASSWORD = $root_password

$drop_database_sql = "DROP DATABASE $company_name WITH (FORCE);"
& $psql -U "$root_username" -h "$server" -c "$drop_database_sql"

$drop_user_sql = "DROP USER $company_name;"
& $psql -U "$root_username" -h "$server" -c "$drop_user_sql"

Remove-Item Env:\PGPASSWORD

Read-Host "Press any key to exit"