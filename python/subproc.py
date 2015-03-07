import subprocess

sock = subprocess.Popen(["bc"], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
sock.stdin.write("2+2\n")
sock.stdin.close()
print sock.stdout.read().strip()
sock.stdout.close()
