<?php

namespace Wikidot\Utils;

use Ozone\Framework\SmartyModule;

 abstract class ManageSiteBaseModule extends SmartyModule
{

    public function isAllowed($runData)
    {
        WDPermissionManager::instance()->hasPermission('manage_site', $runData->getUser(), $runData->getTemp("site"));
        return true;
    }
}
