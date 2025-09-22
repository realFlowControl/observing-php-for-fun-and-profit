--TEST--
Basic function and extension being loaded
--FILE--
<?php
var_dump(hello_world("Longhorn PHP"));
?>
--EXPECT--
string(20) "Hello, Longhorn PHP!"
