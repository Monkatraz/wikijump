<?php

namespace Wikidot\Modules\UserInfo;

use Ozone\Framework\Database\Criteria;
use Wikidot\DB\OzoneUserPeer;
use Wikidot\Utils\GlobalProperties;
use Wikidot\Utils\ProcessException;
use Wikidot\Utils\SmartyLocalizedModule;

class UserInfoModule extends SmartyLocalizedModule
{

    private $user; // nasty hack again...

    protected $processPage = true;

    public function build($runData)
    {

        // a hack to get unix user name
        $qs =  $_SERVER['QUERY_STRING'];
        $splited = explode("/", $qs);

        // WARNING!!! this is a hack! not a proper use of ParameterList object!
        $userUnixName = $splited[3];

        if ($userUnixName == null || $userUnixName == '') {
            throw new ProcessException(_("No user specified."), "no_user");
        }

        // get user
        $c = new Criteria();
        $c->add("unix_name", $userUnixName);
        $user = OzoneUserPeer::instance()->selectOne($c);

        if ($user == null) {
            throw new ProcessException(_("User does not exist."));
        }

        $runData->contextAdd("user", $user);
        $runData->contextAdd("userUnixName", $userUnixName);
        $runData->contextAdd("userId", $user->getUserId());

        $this->user = $user;

        // get the referring page too in case one wants to
        // flag an abusive user. than we set site_id of the flag
        // to the site which the user comes from if
        // this is a Wikijump site.

        $referer = $_SERVER['HTTP_REFERER'];

        if ($referer) {
            $referer = parse_url($referer);
            $referer = $referer['host'];
        }

        $runData->contextAdd("referer", $referer);

        $runData->contextAdd("uu", $runData->getUser());
    }

    public function processPage($out, $runData)
    {
        // modify title of the page
        $user = $this->user;
        if ($user != null) {
            $out = preg_replace("/<title>(.+?)<\/title>/is", "<title>".GlobalProperties::$SERVICE_NAME.": ".$user->getNickName()."</title>", $out);
            $out = preg_replace("/<div id=\"page-title\">(.*?)<\/div>/is", '', $out, 1);
        }
        return $out;
    }
}
