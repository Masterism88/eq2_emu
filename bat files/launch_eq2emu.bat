@echo off
rem The following are the variables used, what they do, and the settings available.
rem All settings on by default.
rem SQLUPDATE Will update, the EQ2Emu World DB. 1 for on, 2 for updates only. anything else for off.
rem ******** SETTING SQLUPDATE = 1 WILL ERASE ALL DATA IN THE DATABASE use 2 TO SAVE CHARACTERS/GUILDS ************
rem EXEUPDATE Will update, EXE files. 1 for on, anything else for off. If we update we check for structs as well.
rem LUAUPDATE Will update, LUA files. 1 for on, anything else for off.
rem MAPUPDATE Will update, MAP files. 1 for on, anything else for off.
rem SENDBUGS  Will send your bug reports to an offsite server to be reviewed by staff. 1 for on, anything else is off.
rem LSUPDATE  Will check for new Loginserver updates SQL.
rem UPDATELS  Will sync world and loginserver data.

SET NAME=Zeklabs.com
TITLE %NAME%

rem Lets show the nice fancy header
echo 88888888888  ,ad8888ba,     ad888888b,  88888888888  88b           d88               
echo 88          d8"'    `"8b   d8"     "88  88           888b         d888               
echo 88         d8'        `8b          a8P  88           88`8b       d8'88               
echo 88aaaaa    88          88       ,d8P"   88aaaaa      88 `8b     d8' 88  88       88  
echo 88"""""    88          88     a8P"      88"""""      88  `8b   d8'  88  88       88  
echo 88         Y8,    "88,,8P   a8P'        88           88   `8b d8'   88  88       88  
echo 88          Y8a.    Y88P   d8"          88           88    `888'    88  "8a,   ,a88  
echo 88888888888  `"Y8888Y"Y8a  88888888888  88888888888  88     `8'     88   `"YbbdP'Y8  
echo     Windows Installer by Devn00b  [Launch EQ2EMU.bat]  https://www.eq2emu.com
echo[
echo[
rem Check for internet if not set vars to 0
echo [ Checking internet connection. ]
Ping zeklabs.com -n 1 -w 1000 >NUL
rem Please do not edit below.
if errorlevel 1 (
SET SQLUPDATE=0
SET EXEUPDATE=0
SET LUAUPDATE=0
SET MAPUPDATE=0
SET SENDBUGS=0
SET LSUPDATE=0
SET UPDATELS=0
SET DBEDITOR=1
echo [ Internet not found. All updates DISABLED. ] 
) else (
rem Edit these vars if you have internet.
SET SQLUPDATE=2
SET EXEUPDATE=1
SET LUAUPDATE=1
SET MAPUPDATE=2
SET SENDBUGS=0
SET LSUPDATE=1
SET UPDATELS=1
SET DBEDITOR=1
echo [ Internet found. All updates ENABLED. ]
)


rem Some people have older version of this script, and are missing options.bat so lets be sure they get it no matter what option is selected.
rem On each load delete options.bat and re-download it.
powershell write-host -fore Blue [Making sure you have an updated Options.bat]
move options.bat options.old >NUL
wget -q -N --no-check-certificate https://www.zeklabs.com/dl/Options.bat
echo [ Completed Option.bat Update.]
echo[

rem check if first run, if it is deal with it
SET FIRSTRUN=0
if not exist options\ranonce.txt SET FIRSTRUN=1

rem make sure we install the vcredist for apache2
if /I "%FIRSTRUN%" EQU "1" reg Query "HKLM\Hardware\Description\System\CentralProcessor\0" | find /i "x86" > NUL && set OS=32BIT || set OS=64BIT
if /I "%FIRSTRUN%" EQU "1" if %OS%==32BIT %~dp0redist\vc_redist.x86.exe /q /norestart
if /I "%FIRSTRUN%" EQU "1" if %OS%==64BIT %~dp0redist\vc_redist.x64.exe /q /norestart

if /I "%SQLUPDATE%" EQU "1" powershell write-host -fore Red ******** SETTING SQLUPDATE = 1 WILL ERASE ALL DATA IN THE DATABASE use 2 TO SAVE CHARACTERS/GUILDS ************
if /I "%SQLUPDATE%" EQU "1" echo [ Checking for SQL Full Update and Downloading. ]
if /I "%SQLUPDATE%" EQU "1" wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://zeklabs.com/dl/eq2emudb.rar
if /I "%SQLUPDATE%" EQU "1" unrar x -y -inul eq2emudb.rar
if /I "%SQLUPDATE%" EQU "1" del eq2emudb.rar >NUL
if /I "%SQLUPDATE%" EQU "1" echo [ Completed Download of SQL Full Update. ]
if /I "%SQLUPDATE%" EQU "1" echo[
if /I "%SQLUPDATE%" EQU "2" echo [ Checking for SQL Update and Downloading. ]
if /I "%SQLUPDATE%" EQU "2" wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://zeklabs.com/dl/eq2dbupdate.rar
if /I "%SQLUPDATE%" EQU "2" unrar x -y -inul eq2dbupdate.rar
if /I "%SQLUPDATE%" EQU "2" del eq2dbupdate.rar >NUL
if /I "%SQLUPDATE%" EQU "2" echo [ Completed Download of SQL Update. ]
if /I "%SQLUPDATE%" EQU "2" echo[

if /I "%EXEUPDATE%" EQU "1" echo [ Checking for EXE Updates. **NOTE** If something goes wrong, backups files can be found in the oldfiles folder. ]
if /I "%EXEUPDATE%" EQU "1" cd server
if /I "%EXEUPDATE%" EQU "1" mkdir oldfiles >NUL
if /I "%EXEUPDATE%" EQU "1" move eq2*.exe oldfiles >NUL
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/EQ2Login__Debug64.exe
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/EQ2World__Debug_x64.exe
if /I "%EXEUPDATE%" EQU "1" echo [ New EXE Could Mean NEW Structs. Checking... ]
if /I "%EXEUPDATE%" EQU "1" move SpawnStructs.xml oldfiles/ >NUL
if /I "%EXEUPDATE%" EQU "1" move WorldStructs.xml oldfiles/ >NUL
if /I "%EXEUPDATE%" EQU "1" move EQ2_Structs.xml oldfiles/ >NUL
if /I "%EXEUPDATE%" EQU "1" move ItemStructs.xml oldfiles/ >NUL
if /I "%EXEUPDATE%" EQU "1" move LoginStructs.xml oldfiles/ >NUL
if /I "%EXEUPDATE%" EQU "1" move CommonStructs.xml oldfiles/ >NUL
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/SpawnStructs.xml
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/WorldStructs.xml
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/EQ2_Structs.xml
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/ItemStructs.xml
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/LoginStructs.xml
if /I "%EXEUPDATE%" EQU "1" wget -q http://git.eq2emu.com:3000/devn00b/EQ2EMu/raw/master/server/CommonStructs.xml
if /I "%EXEUPDATE%" EQU "1" echo [ Completed Struct Updates. ]
if /I "%EXEUPDATE%" EQU "1" echo [ Completed EXE Updates. ]
if /I "%EXEUPDATE%" EQU "1" cd ..
if /I "%EXEUPDATE%" EQU "1" echo[

if /I "%LUAUPDATE%" EQU "1" echo [ Checking for LUA Updates. ]
if /I "%LUAUPDATE%" EQU "1" echo [ This will take a moment due to number of files. ]
if /I "%LUAUPDATE%" EQU "1" cd server
if /I "%LUAUPDATE%" EQU "1" wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://zeklabs.com/dl/eq2emulua.rar
if /I "%LUAUPDATE%" EQU "1" ..\unrar x -y -inul eq2emulua.rar
if /I "%LUAUPDATE%" EQU "1" del eq2emulua.rar >NUL
if /I "%LUAUPDATE%" EQU "1" echo [ Completed LUA Updates. ]
if /I "%LUAUPDATE%" EQU "1" cd ..
if /I "%LUAUPDATE%" EQU "1" echo[

if /I "%MAPUPDATE%" EQU "1" echo [ Checking for MAP Updates. ]
if /I "%MAPUPDATE%" EQU "1" echo [ This will take a LONG time. The maps are 2+ GB in size.]
if /I "%MAPUPDATE%" EQU "1" cd server
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part01.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part02.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part03.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part04.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part05.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part06.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part07.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part08.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part09.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part10.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part11.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part12.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part13.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part14.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part15.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part16.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part17.rar
if /I "%MAPUPDATE%" EQU "1" echo [ Any downloads below here means NEW maps!]
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part18.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part19.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part20.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part21.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part22.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part23.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part24.rar
if /I "%MAPUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate --show-progress --progress=bar:force:noscroll https://github.com/devn00b/EQ2EMu-Maps/raw/master/eq2emumaps.part25.rar
if /I "%MAPUPDATE%" EQU "1" ..\unrar x -o+ -inul eq2emumaps*.rar 
if /I "%MAPUPDATE%" EQU "1" del /f /q eq2emumaps*.rar >NUL
if /I "%MAPUPDATE%" EQU "1" echo [ Completed MAP Updates.]
if /I "%MAPUPDATE%" EQU "1" cd ..
if /I "%MAPUPDATE%" EQU "1" echo[

echo [ Loading MariaDB. ]
cd mariadb\bin
start db.bat
echo [ You Can Press Any Key To Continue Once MYSQL has Completed Loading. ]
timeout /t 60
echo [ MariaDB Loaded. ]
echo[
cd ..\..

if /I "%LSUPDATE%" EQU "1" echo [ Updating LoginServer SQL. ]
if /I "%LSUPDATE%" EQU "1" cd server
if /I "%LSUPDATE%" EQU "1" mkdir oldfiles >NUL
if /I "%LSUPDATE%" EQU "1" echo [ Saving Character and WorldServer Data before Import. ]
if /I "%LSUPDATE%" EQU "1" ..\mariadb\bin\mysqldump -ueq2emu -peq2emu eq2ls login_characters > lschars.sql
if /I "%LSUPDATE%" EQU "1" ..\mariadb\bin\mysqldump -ueq2emu -peq2emu eq2ls login_worldservers >> lschars.sql
if /I "%LSUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate https://zeklabs.com/dl/eq2emulssql.rar
if /I "%LSUPDATE%" EQU "1" ..\unrar x -y -inul eq2emulssql.rar
if /I "%LSUPDATE%" EQU "1" ..\mariadb\bin\mysql -ueq2emu -peq2emu --database=eq2emu < eq2emulssql.sql
if /I "%LSUPDATE%" EQU "1" echo [ Importing old character data (if any). ]
if /I "%LSUPDATE%" EQU "1" ..\mariadb\bin\mysql -ueq2emu -peq2emu --database=eq2ls <lschars.sql
if /I "%LSUPDATE%" EQU "1" echo [ Characters Backported to DB. ]
if /I "%LSUPDATE%" EQU "1" echo [ Making sure opcode table is correct. ]
if /I "%LSUPDATE%" EQU "1" ..\wget -q -N --no-check-certificate https://zeklabs.com/dl/ls.sql
if /I "%LSUPDATE%" EQU "1" ..\mariadb\bin\mysql -ueq2emu -peq2emu --database=eq2ls <ls.sql
if /I "%LSUPDATE%" EQU "1" del /f /q lschars.sql >NUL
if /I "%LSUPDATE%" EQU "1" del /f /q ls.sql >NUL
if /I "%LSUPDATE%" EQU "1" move eq2emuls* oldfiles/ >NUL
if /I "%LSUPDATE%" EQU "1" echo [ Completed Updating LoginServer SQL. ]
if /I "%LSUPDATE%" EQU "1" cd ..
if /I "%LSUPDATE%" EQU "1" echo[

if /I "%SENDBUGS%" EQU "1" echo [ Uploading BUG Reports to Devn00b. ]
if /I "%SENDBUGS%" EQU "1" cd mariadb\bin\
if /I "%SENDBUGS%" EQU "1" mysqldump -ueq2emu -peq2emu --no-create-db --no-create-info --complete-insert --skip-add-locks --skip-add-drop-table --skip-comments --compact eq2emu bugs >bugs.sql
if /I "%SENDBUGS%" EQU "1" mysql -ueq2emu -pidontgive2shits --host=eq2db.devn00b.com --database=eq2emu <bugs.sql
if /I "%SENDBUGS%" EQU "1" mysql -ueq2emu -peq2emu --database=eq2emu < options\clearbugs.sql
if /I "%SENDBUGS%" EQU "1" del bugs.sql >NUL
if /I "%SENDBUGS%" EQU "1" echo [ Completed BUG Report Upload. Thank You. ]
if /I "%SENDBUGS%" EQU "1" cd ..\..

if /I "%SQLUPDATE%" EQU "1" echo [ Importing Full DB Update. This Will Delete Characters And Guild Info. ]
if /I "%SQLUPDATE%" EQU "1" cd mariadb\bin\
if /I "%SQLUPDATE%" EQU "1" mysql -ueq2emu -peq2emu --database=eq2emu <..\..\eq2emu.sql
if /I "%SQLUPDATE%" EQU "1" cd ..\..
if /I "%SQLUPDATE%" EQU "1" del eq2emudb.rar >NUL
if /I "%SQLUPDATE%" EQU "1" del eq2emu.sql >NUL
if /I "%SQLUPDATE%" EQU "1" SET FIRSTRUN=1
if /I "%SQLUPDATE%" EQU "1" echo [ Flag set to import default LS/Characters. ]
if /I "%SQLUPDATE%" EQU "1" echo [ Completed Full DB Update. ]
if /I "%SQLUPDATE%" EQU "1" echo[

if /I "%SQLUPDATE%" EQU "2" echo [ Importing Partial DB Update. ]
if /I "%SQLUPDATE%" EQU "2" cd mariadb\bin\
if /I "%SQLUPDATE%" EQU "2" mysql -ueq2emu -peq2emu --database=eq2emu <..\..\eq2dbupdate.sql
if /I "%SQLUPDATE%" EQU "2" cd ..\..
if /I "%SQLUPDATE%" EQU "2" del eq2dbupdate.rar >NUL
if /I "%SQLUPDATE%" EQU "2" del eq2dbupdate.sql >NUL
if /I "%SQLUPDATE%" EQU "2" echo [ Completed Partial DB Update. ]
if /I "%SQLUPDATE%" EQU "2" echo[

if /I "%FIRSTRUN%" EQU "1" echo [ Fixing Windows Bug With Hostname. ]
if /I "%FIRSTRUN%" EQU "1" cd server
if /I "%FIRSTRUN%" EQU "1" set HOSTIP=%COMPUTERNAME%
if /I "%FIRSTRUN%" EQU "1" fart loginserver.ini 127.0.0.1 %HOSTIP%
if /I "%FIRSTRUN%" EQU "1" echo [ HostIP Should now be %HOSTIP% ]
if /I "%FIRSTRUN%" EQU "1" cd ..
if /I "%FIRSTRUN%" EQU "1" echo[

if /I "%FIRSTRUN%" EQU "1" echo [ Inserting Starter Admin Account. ]
if /I "%FIRSTRUN%" EQU "1" cd mariadb\bin\
if /I "%FIRSTRUN%" EQU "1" ..\..\wget -q --no-check-certificate https://www.zeklabs.com/dl/eq2emu-account-insert.sql
if /I "%FIRSTRUN%" EQU "1" mysql -ueq2emu -peq2emu <eq2emu-account-insert.sql
if /I "%FIRSTRUN%" EQU "1" del /f /q eq2emu-account-insert.sql >NUL
if /I "%FIRSTRUN%" EQU "1" cd ..\..
if /I "%FIRSTRUN%" EQU "1" echo [ Starter Account Inserted. Username: eq2emu Password: eq2emu Character: Eqtwoemu. Info can be found in Options Folder\ranonce.txt ] 
if /I "%FIRSTRUN%" EQU "1" echo [ Starter Account Inserted. Username: eq2emu Password: eq2emu Character: Eqtwoemu] > options\ranonce.txt
if /I "%FIRSTRUN%" EQU "1" echo[
if /I "%FIRSTRUN%" EQU "1" FIRSTRUN=0

if /I "%UPDATELS%" EQU "1" echo [ Syncing Login Characters to World. ]
if /I "%UPDATELS%" EQU "1" del /f /q .\server\oldfiles\worldtolsbak.sql >NUL
if /I "%UPDATELS%" EQU "1" .\mariadb\bin\mysqldump -ueq2emu -peq2emu eq2ls login_characters>.\server\oldfiles\worldtolsbak.sql
if /I "%UPDATELS%" EQU "1" echo delete from login_characters; >delchar.sql
if /I "%UPDATELS%" EQU "1" .\mariadb\bin\mysql -ueq2emu -peq2emu --database=eq2ls <delchar.sql
if /I "%UPDATELS%" EQU "1" del /f /q delchar.sql >NUL
if /I "%UPDATELS%" EQU "1" echo INSERT INTO eq2ls.login_characters (account_id,server_id,name,race,class,gender,body_size,body_age,current_zone_id,level,tradeskill_class,tradeskill_level,soga_wing_type,soga_chest_type,soga_legs_type,soga_hair_type,soga_facial_hair_type,soga_model_type,legs_type,chest_type,wing_type,hair_type,facial_hair_type,model_type,deleted,created_date,char_id) select account_id,server_id,name,race,class,gender,body_size,body_age,current_zone_id,level,tradeskill_class,tradeskill_level,soga_wing_type,soga_chest_type,soga_legs_type,soga_hair_type,soga_facial_hair_type,soga_model_type,legs_type,chest_type,wing_type,hair_type,facial_hair_type,model_type,deleted,created_date,id from eq2emu.characters; >lschars.sql
if /I "%UPDATELS%" EQU "1" .\mariadb\bin\mysql -ueq2emu -peq2emu --database=eq2emu <lschars.sql
if /I "%UPDATELS%" EQU "1" del /f /q lschars.sql >NUL
if /I "%UPDATELS%" EQU "1" echo [ Login Sync Completed. ]
if /I "%UPDATELS%" EQU "1" echo[
if /I "%DBEDITOR%" EQU "1" echo [ Starting EQ2EMu Database Editor. ]
if /I "%DBEDITOR%" EQU "1" cd binaries
if /I "%DBEDITOR%" EQU "1" start php8.3.bat
if /I "%DBEDITOR%" EQU "1" cd ..
if /I "%DBEDITOR%" EQU "1" echo[

echo [Starting World And Login]
start srvls.bat
start srvwrld.bat
