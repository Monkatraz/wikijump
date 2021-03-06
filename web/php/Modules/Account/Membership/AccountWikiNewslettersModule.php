<?php

namespace Wikidot\Modules\Account\Membership;




use Ozone\Framework\Database\Criteria;
use Wikidot\DB\MemberPeer;
use Wikidot\DB\UserSettingsPeer;
use Wikidot\Utils\AccountBaseModule;

class AccountWikiNewslettersModule extends AccountBaseModule
{

    public function build($runData)
    {

        $userId = $runData->getUserId();

        // get all membership - criteria with join - wooo!
        $c = new Criteria();
        $c->add("user_id", $userId);
        $c->addJoin("site_id", "site.site_id");
        $c->addOrderAscending("site.name");

        $mems = MemberPeer::instance()->select($c);
        if (count($mems)>0) {
            $runData->contextAdd("mems", $mems);
        }

        // get user settings
        $us = UserSettingsPeer::instance()->selectByPrimaryKey($runData->getUserId());
        $runData->contextAdd("defaultNewsletter", $us->getAllowSiteNewslettersDefault());
    }
}
