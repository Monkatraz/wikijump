<?php


use Ozone\Framework\Database\Database;
use Ozone\Framework\PathManager;
use Ozone\Framework\Scheduler;
use Wikidot\Utils\GlobalProperties;

$applicationDir = $argv[1];
echo "The application dir is: " .$applicationDir."\n";

require_once ($applicationDir."/conf/GlobalProperties.php");
require_once ("../php/core/autoload.inc.php");
require_once ("../php/core/functions.php");

$tz = GlobalProperties::$SERVER_TIMEZONE;
putenv("TZ=$tz");
require_once(WIKIJUMP_ROOT.'/lib/date/Date.php');
require_once(WIKIJUMP_ROOT.'/lib/date/Date/TimeZone.php');
require_once(WIKIJUMP_ROOT.'/lib/date/Date/Span.php');

define('SMARTY_DIR', PathManager :: smartyDir());
require_once (SMARTY_DIR.'Smarty.class.php');

// connect to the database
Database::init();
$db = Database::connection();

$scheduler = new Scheduler();

$scheduler->setClassPath($applicationDir."/php/Jobs");

$schedulerFiles = ls($applicationDir."/conf/scheduler", "*-jobs.xml");

foreach ($schedulerFiles as $key => $file) {
	echo "----------------------------------------\n";
	echo "processing file $file:\n";
	echo "----------------------------------------\n";
	$xml = simplexml_load_file($applicationDir."/conf/scheduler/$file");

	##$database = new DBDatabase($xml);
	$scheduler->parseConfigXml($xml);

}

$scheduler->start();
