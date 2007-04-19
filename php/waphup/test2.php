<?
function googleize($matches)
{
	return("http://wmlproxy.google.com/wmltrans/h=hu/g=@26amp@3bwmlmode=url/u=" . strtr(urlencode(substr($matches[0], 1, strlen($matches[0])-2)), "%", "@"));
}
print(preg_replace_callback('|"http://[^"]*"|', "googleize", $text));
?>
