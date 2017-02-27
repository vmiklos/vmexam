rm -rf db
mkdir db
certutil -N -d db
certutil -S -f pwdfile.txt -d db -t "C,," -x -n "Server-Cert" -g 2048 -s "CN=nss.dev.example.com,O=Testing,L=example,ST=South Australia,C=AU,2.5.4.97=VATHU-10585560"
