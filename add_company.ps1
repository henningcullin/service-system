Write-Host "Add a new Company, Database, Customer"

if (!($server = Read-Host "Enter Server / Host (127.0.0.1)")) { $server = "127.0.0.1" }
if (!($port = Read-Host "Enter Port (5432)")) { $port = "5432" }
$root_username = 'postgres'
$root_password = Read-Host "Enter Root Password"

$company_name = Read-Host "Enter company name"

# COMPANY USER CREDENTIALS
$company_user_name = $company_name
$company_user_password = Read-Host "Enter company password"

# POSTGRESQL COMMAND LINE CLIENT PATH <____________________________________________________________ CHANGE POSTGRESQL PATH HERE
$psql = "C:\Program Files\PostgreSQL\16\bin\psql.exe"

$env:PGPASSWORD = $root_password

# CREATE DATABASE
$createDatabaseQuery = "CREATE DATABASE $company_name;"
& $psql -U "$root_username" -h "$server" -c "$createDatabaseQuery"

# CREATE TABLES FROM FILES
$tableFilesPath = Join-Path (Get-Location).Path -ChildPath "structure\tables"
$tableFilesArr = Get-ChildItem $tableFilesPath -Filter *.sql | Sort-Object {
    $order = @{
        'procedures' = 1
        'machines'   = 2
        'roles'      = 3
        'users'      = 4
        'tasks'      = 5
        'reports'    = 6
    }
    $order[$_.BaseName.ToLower()]
}

foreach ($tableFile in $tableFilesArr) {
    $sqlContent = Get-Content $tableFile.FullName -Raw
    & $psql -U "$root_username" -h "$server" -d "$company_name" -c "$sqlContent"
}

# CREATE COMPANY USER
$createUserQuery = "CREATE USER $company_user_name WITH PASSWORD '$company_user_password';"
$grantQuery = "GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO $company_user_name;"
$grantQuery2 = "GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO $company_user_name;"

& $psql -U "$root_username" -h "$server" -d "$company_name" -c "$createUserQuery"
& $psql -U "$root_username" -h "$server" -d "$company_name" -c "$grantQuery"
& $psql -U "$root_username" -h "$server" -d "$company_name" -c "$grantQuery2"

#CREATE ENV FILE WITH DATABASE URL
$database_url = "DATABASE_URL=postgres://${company_user_name}:${company_user_password}@${server}:${port}/${company_name}"

$env_file_path = Join-Path (Get-Location).Path -ChildPath "envs\${company_name}.env"

$utf8NoBOM = New-Object System.Text.UTF8Encoding $false
$streamWriter = New-Object System.IO.StreamWriter $env_file_path, $false, $utf8NoBOM

try {
    $streamWriter.Write($database_url)
}
finally {
    $streamWriter.Close()
    Remove-Item Env:\PGPASSWORD
}

Read-Host "Press any key to exit"