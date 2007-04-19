<?
include("config.php");
include("functions.php");

$node = substr($_SERVER["PATH_INFO"], 1);
if (strpos($_SERVER["PATH_INFO"], "/") !== false)
	$node = preg_replace('|^([^/]*)/.*|', '$1', $node);

$param = preg_replace('|^[^/]*/([^/]*)|', '$1',
	substr($_SERVER["PATH_INFO"], 1));
if($param==$node)
	$param=null;

if ($node == "")
	list_nodes();
elseif ($param == "")
	display_node($node, $param);
else
	display_comment($node, $param);
?>
