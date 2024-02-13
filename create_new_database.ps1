Write-Host "Create new database"

$server = Read-Host "Enter server name"
$root_username = "root"
$root_password = Read-Host "Enter root password"

$company_name = Read-Host "Enter company name"

# COMPANY USER CREDENTIALS
$company_user_name = $company_name
$company_user_password = Read-Host "Enter company password"

# MYSQL COMMAND LINE CLIENT PATH
$mysql = "C:\Program Files\MySQL\MySQL Server 8.0\bin\mysql"

# CREATE DATABASE
$createDatabaseQuery = "CREATE DATABASE IF NOT EXISTS $company_name;"
& $mysql -u $root_username -p$root_password -h $server -e $createDatabaseQuery

# CREATE TABLES FROM FILES
$tableFilesPath = (Get-Location).Path + "\structure\tables"
$tableFilesArr = Get-ChildItem $tableFilesPath -Filter *.sql

foreach ($tableFile in $tableFilesArr) {
    $sqlContent = Get-Content $tableFile.FullName -Raw
    & $mysql -u $root_username -p$root_password -h $server -D $company_name -e $sqlContent
}

# CREATE COMPANY USER
$createUserQuery = "CREATE USER '$company_user_name'@'%' IDENTIFIED BY '$company_user_password';"
$grantQuery = "GRANT INSERT, UPDATE, DELETE ON $company_name.* TO '$company_user_name'@'%';"

& $mysql -u $root_username -p$root_password -h $server -e $createUserQuery
& $mysql -u $root_username -p$root_password -h $server -e $grantQuery

Read-Host "Press any key to exit"
