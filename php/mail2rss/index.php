<?
$options["imap"]=":143/imap/notls/novalidate-cert";
$options["imap-ssl"]=":993/imap/ssl/novalidate-cert";

// parse path_info
$params = explode("/", trim(addslashes(stripslashes($_SERVER["PATH_INFO"])), "/"));
$ptr = explode(":", $params[0]);
$host = $ptr[0];
$proto = $ptr[1];
$dir = $params[1];
if($host == "" or $proto == "")
	die("usage: host:protocoll[/dir], example: /frugalware.org:imap-ssl or " .
		"/frugalware.org:imap-ssl/INBOX.Lists.frugalware-devel");
if(!$dir)
	$dir = "INBOX";

if(!isset($_SERVER['PHP_AUTH_USER']))
{
	header("WWW-Authenticate: Basic realm=\"mail2rss\"");
	header('HTTP/1.0 401 Unauthorized');
	die("no pass");
}

$connect = "{".$host.$options[$proto]."}".$dir;
$box=@imap_open($connect,$_SERVER['PHP_AUTH_USER'], $_SERVER['PHP_AUTH_PW']);
if($box===false)
	die("wrong pass");
$list=imap_sort($box, SORTDATE, 1, SE_UID);

header('Content-Type: application/xml; charset=UTF-8');
print('<?xml version="1.0"  encoding="UTF-8"?>
<rss version="2.0">
<channel>');
print("<title>" . $_SERVER["PATH_INFO"] . "</title>");
print("<description>last 10 msgs</description>");
print("<link>https://frugalware.org/~vmiklos/mail2rss/</link>");

for($i=0;$i<10;$i++)
{
	$header=@imap_headerinfo($box, imap_msgno($box, $list[$i]));
	if($header==null)
		die("</channel>\n</rss>");
	print("<item>\n<title>".htmlspecialchars(imap_utf8($header->subject))."</title>\n");
	print("<author>" . imap_utf8($header->from[0]->personal) . " &lt;" .
		$header->from[0]->mailbox . "@" . $header->from[0]->host . "&gt;</author>\n");
	print("<pubDate>".preg_replace("|^(.*) \(.*\)|", "$1", $header->Date)."</pubDate>\n</item>\n");
}
print("</channel>\n</rss>");
?>
