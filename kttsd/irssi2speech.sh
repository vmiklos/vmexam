if [ -z "`pidof kttsd`" ]; then
	kttsd &>/dev/null
fi
ssh genesis.frugalware.org tail -n 0 -f '$HOME/.irssi/logs/freenode/#debian.hu_'`date +%Y%m%d`'.log' |while IFS= read i
do
	num=$(dcop kttsd KSpeech setText "`echo $i|sed 's/[0-9:]\{5\}// '`" "hu" 2>/dev/null)
	dcop kttsd KSpeech startText $num &>/dev/null
	echo "$i"
done
