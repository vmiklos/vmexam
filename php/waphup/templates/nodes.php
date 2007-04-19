<?
header('Content-Type: text/vnd.wap.wml; charset=iso-8859-2');
print('<?xml version="1.0" encoding="iso-8859-2"?>');
?>
<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
"http://www.wapforum.org/DTD/wml_1.1.xml">

<wml>

<card id="XML" title="waphup">
<p>
<?
if($nodes!=null)
{
	foreach($nodes as $node)
	{
		$ptr = explode("/", $node['link']);
		print(iconv("utf8", "latin2", preg_replace("/.*, (.*) \+.*/", '$1', $node['pubDate']) . "<a href=\"" . str_replace("$rssweb/", "", $ptr[count($ptr)-1]) . "\">" . $node['title'] . "</a><br />\n"));
	}
}
else
{
	printf("lehalt a hup :/<br />\n");
}
?>
</p>
</card>

</wml>
