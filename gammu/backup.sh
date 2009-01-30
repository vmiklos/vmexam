date=`date +%Y%m%d`

echo -e "no\nALL" |gammu --backup backup-$date.txt
echo -e "no\nALL" |gammu --backupsms backupsms-$date.txt
gammu --geteachsms > eachsms-$date.txt
#python backup-fs.py
