Write-Host "Remove a Company / Database / Customer"

if (!($server = Read-Host "Enter Server / Host (127.0.0.1)")) { $server = "127.0.0.1" }
$root_username = 'root'
$root_password = Read-Host "Enter Root Password"

$company_name = Read-Host "Enter company name"

$mysql = "C:\Program Files\MySQL\MySQL Server 8.0\bin\mysql"

$drop_database_sql = "DROP DATABASE $company_name;"
& $mysql -u "$root_username" -p"$root_password" -h "$server" -e "$drop_database_sql"
sleep 1

$drop_user_sql = "DROP USER $company_name;"
& $mysql -u "$root_username" -p"$root_password" -h "$server" -e "$drop_user_sql"