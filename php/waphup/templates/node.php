<?
header('Content-Type: text/vnd.wap.wml; charset=iso-8859-2');
print('<?xml version="1.0" encoding="iso-8859-2"?>');
?>
<!DOCTYPE wml PUBLIC "-//WAPFORUM//DTD WML 1.1//EN"
"http://www.wapforum.org/DTD/wml_1.1.xml">

<wml>

<card id="XML" title="waphup">
<?
if($data!=null)
{
	ob_start();

	print(iconv("utf8", "latin2", str_replace("node/", "", $data)));

	$tidyconfig = array('output-xhtml'  => true,
			'input-encoding' => "utf8",
			'output-encoding' => "utf8");
	$buffer = ob_get_clean();
	$tidy = tidy_repair_string($buffer, $tidyconfig);

	// slice the header
	for($i=0;$i<7;$i++)
	{
		//$tidy = strstr($tidy, "\n");
		$tidy = substr($tidy, strpos($tidy, "\n")+1);
	}
	// slice the footer
	$tidy = substr($tidy, 0, strlen($tidy)-16);

	print($tidy);
}
else
{
	printf("lehalt a hup :/<br />\n");
}
?>
</card>

</wml>
