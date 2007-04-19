<?
	include("lastRSS.php");
	$rss = new lastRSS; 

	if ($items = $rss->get("http://hup.hu/rss.xml"))
	{
		var_dump($items);
	}
?>
