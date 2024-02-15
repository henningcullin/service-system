Write-Host "Add a new Company, Database, Customer"

if (!($server = Read-Host "Enter Server / Host (127.0.0.1)")) { $server = "127.0.0.1" }
if (!($port = Read-Host "Enter Port (3306)")) { $port = "3306" }
$root_username = 'root'
$root_password = Read-Host "Enter Root Password"

$company_name = Read-Host "Enter company name"

# COMPANY USER CREDENTIALS
$company_user_name = $company_name
$company_user_password = Read-Host "Enter company password"

# MYSQL COMMAND LINE CLIENT PATH <____________________________________________________________ CHANGE MYSQL PATH HERE
$mysql = "E:\Program Files\MySQL\MySQL Server 8.0\bin\mysql"

# CREATE DATABASE
$createDatabaseQuery = "CREATE DATABASE IF NOT EXISTS $company_name;"
& $mysql -u "$root_username" -p"$root_password" -h "$server" -e "$createDatabaseQuery"

# CREATE TABLES FROM FILES
$tableFilesPath = Join-Path (Get-Location).Path -ChildPath "structure\tables"
$tableFilesArr = Get-ChildItem $tableFilesPath -Filter *.sql | Sort-Object {
    $order = @{
        'user'     = 1
        'worker'   = 2
        'machine'  = 3
        'task'     = 4
    }
    $order[$_.BaseName.ToLower()]
}

foreach ($tableFile in $tableFilesArr) {
    $sqlContent = Get-Content $tableFile.FullName -Raw
    & $mysql -u "$root_username" -p"$root_password" -h "$server" -D "$company_name" -e "$sqlContent"
}

# CREATE COMPANY USER
$createUserQuery = "CREATE USER '$company_user_name'@'%' IDENTIFIED BY '$company_user_password';"
$grantQuery = "GRANT SELECT, INSERT, UPDATE, DELETE ON $company_name.* TO '$company_user_name'@'%';"

& $mysql -u "$root_username" -p"$root_password" -h "$server" -D "$company_name" -e "$createUserQuery"
& $mysql -u "$root_username" -p"$root_password" -h "$server" -D "$company_name" -e "$grantQuery"

#CREATE ENV FILE WITH DATABASE URL
$database_url = "DATABASE_URL=mysql://${company_user_name}:${company_user_password}@${server}:${port}/${company_name}"

$env_file_path = Join-Path (Get-Location).Path -ChildPath "envs\${company_name}.env"

$utf8NoBOM = New-Object System.Text.UTF8Encoding $false
$streamWriter = New-Object System.IO.StreamWriter $env_file_path, $false, $utf8NoBOM

try {
    $streamWriter.Write($database_url)
}
finally {
    $streamWriter.Close()
}

Read-Host "Press any key to exit"