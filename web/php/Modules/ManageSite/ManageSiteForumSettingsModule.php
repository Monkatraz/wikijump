<?php

namespace Wikidot\Modules\ManageSite;

use Wikidot\Utils\ManageSiteBaseModule;

class ManageSiteForumSettingsModule extends ManageSiteBaseModule
{

    public function build($runData)
    {

        $site = $runData->getTemp("site");

        //get forum settings
        $runData->contextAdd("forumSettings", $site->getForumSettings());
    }
}
