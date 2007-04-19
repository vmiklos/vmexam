<?
include("lastRSS.php");

function list_nodes()
{
	global $url, $web;
	$rss = new lastRSS; 

	if ($items = $rss->get($url))
	{
		$nodes = $items['items'];
	}
	else
	{
		$nodes=null;
	}
	include("templates/nodes.php");
}

function googleize($matches)
{
	return("http://wap.google.com/gwt/n?mrestrict=wml&output=wml&u=" .
		urlencode(substr($matches[0], 1, strlen($matches[0])-2)));
}

function display_node($node, $param)
{
	global $web, $url;
	$rss = new lastRSS;

	if(is_numeric($node))
	{
		$raw = file_get_contents("$web/$node");
		// drop header
		$cikk = strstr($raw, '<div class="taxonomy">');
		if(!($cikk))
			// ez vmi blog lesz
			$cikk = strstr($raw, '<div class="node">');
		$data = substr(strstr($cikk, '<div class="content">'),
			strlen('<div class="content">'));
		// drop footer
		$data = substr($data, 0, strpos($data, '</div>'));
		// internal links
		$data = str_replace("node/", "", $data);
		// external links
		$data = preg_replace_callback('|"http://[^"]*"|', "googleize", $data);

		// comment link
		if (strstr($raw,"<table class=\"comment\""))
			$data .= "<p><a href=\"$node/comment\">commentek</a></p>";

		// pointer links
		if ($items = $rss->get($url))
		{
			foreach($items['items'] as $key => $value)
			{
				if(str_replace("$web/", "", $value['link'])==$node)
				{
					$data .= "<p><a href=\"" . str_replace("$web/", "",
						$items['items'][$key-1]['link']) . "\">elozo</a></p>";
					$data .= "<p><a href=\"" . str_replace("$web/", "",
						$items['items'][$key+1]['link']) . "\">kovetkezo</a></p>";
				}
			}
		}
	}
	else
		$data="nice try\n";
	include("templates/node.php");
}

function getdata($start, $end, $data)
{
	$ptr = strstr($data, $start);
	return substr($ptr, 0, strpos($ptr, $end));
}

function getcomment($begin,$data)
{
	if (!strstr($data,$begin))
		return("no comment<br />\n");
	$data .= $begin;

	for(;strpos($data,$begin,1)>0;$i++)
	{
		$start = strstr($data, $begin);
		$end = strpos($start, $begin,1);
		$comm = substr($start, 0, $end);

		if ($sor=strstr($comm,"<td style=\"text-align: left; vertical-align: top;\">"))
			$res .= "<p>" . ereg_replace("<td style=.text-align: left; vertical-align: top;.>([^<]*)<[^>]*>([^<]*).*",
			"\\1\\2",$sor)."</p>";
        	if ($sor=strstr($comm,"<td style=\"text-align: left;\">"))
			$res .= "<p>" . ereg_replace("<td style=.text-align: left;.>([^<]*).*","\\1",$sor)."</p>";
		if ($sor=strstr($comm,"<td style=\"background-color: #f6f6eb;\">"))
			$res .= "<p>" . substr(getdata("<td style=\"background-color: #f6f6eb;\">","</td>",$sor),
				strlen("<td style=\"background-color: #f6f6eb;\">"),4096)."</p>";
		$data=substr($start,$end,1024*512);
	}
	return($res);
}

function display_comment($node, $param)
{
	global $web, $url;
	$rss = new lastRSS;

	if(is_numeric($node))
	{
		$data = file_get_contents("$web/$node?mode=2");

		// get content
		$data = getdata("<!-- begin content -->","<!-- end content -->",$data);		

		$ize = getcomment("<table class=\"comment\"",$data);
		// internal links
		$data = str_replace("node/", "", $ize);
		// external links
		$data = preg_replace_callback('|"http://[^"]*"|', "googleize", $data);
	}
	else
		$data="nice try\n";
	include("templates/node.php");
}
?>
